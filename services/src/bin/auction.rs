#![feature(slice_patterns)]

use map_vec::{Map, Set};
use oasis_std::{Address, AddressExt, Balance, Context};
use serde::{Deserialize, Serialize};

#[derive(oasis_std::Service)]
struct SealedAuction {
    /// The address of the auction item. The auction is assumed to be its owner.
    item_address: Address,

    /// The recipient of the payout.
    owner: Address,

    /// The unix timestamp at which this auction ends (or at least approximately).
    end_time: u64,

    /// A set of bidders sorted by bid price.
    /// Invariant: this Auction holds balance equal to the sum of bids' values.
    bids: Map<Address, Balance>,

    /// The set of participants (i.e. bidders ∪ { owner }) who have already been paid out.
    payed: Set<Address>,
}

impl SealedAuction {
    pub fn new(
        ctx: &Context,
        item_address: Address,
        item_transfer_cap: u64,
        auction_duration: u64,
    ) -> Result<Self, AuctionError> {
        let mut item_client = item::AuctionItemClient::at(item_address);
        match item_client.set_owner(&Context::default(), item_transfer_cap, ctx.address()) {
            Ok(true) => {}
            _ => return Err(AuctionError::BadItem),
        }
        Ok(Self {
            item_address,
            owner: ctx.sender(),
            end_time: now_ts() + auction_duration,
            bids: Map::new(),
            payed: Set::new(),
        })
    }

    /// Returns the address of the item under consideration.
    pub fn item(&self, _ctx: &Context) -> Address {
        self.item_address
    }

    /// Returns the unix timestamp at which this auction will (approximately) end.
    pub fn end_time(&self, _ctx: &Context) -> u64 {
        self.end_time
    }

    /// Adds the value sent by the sender to the sender's current bid.
    /// Returns the sender's current offer.
    pub fn bid(&mut self, ctx: &Context) -> Result<Balance, AuctionError> {
        if now_ts() >= self.end_time {
            return Err(AuctionError::TooLate);
        }

        let bidder = ctx.sender();
        let offer = ctx.value();

        let bid = self.bids.entry(bidder).or_default();
        *bid += offer;
        Ok(*bid)
    }

    /// Remits any escrowed balance to the sender and returns whether the bidder won.
    pub fn withdraw(&mut self, ctx: &Context) -> Result<bool, AuctionError> {
        if now_ts() < self.end_time {
            return Err(AuctionError::TooSoon);
        }

        let sender = ctx.sender();

        let sorted_bids = self.sorted_bids();

        // If there were no bids, allow the owner to retrieve the item from escrow.
        // Otherwise, short circuit, knowing that there's no balance to redistribute.
        if sorted_bids.is_empty() {
            if sender == self.owner {
                item::AuctionItemClient::at(self.item_address)
                    .set_owner(
                        &Context::default(),
                        0, /* null transfer cap */
                        self.owner,
                    )
                    .unwrap();
            }
            return Ok(false); // no bids, nothing
        }

        let sender_is_winner = sorted_bids
            .last()
            .map(|(bidder, _)| **bidder == sender)
            .unwrap_or_default();

        if self.payed.contains(&sender) {
            return Ok(sender_is_winner);
        }

        let item_value = match sorted_bids.as_slice() {
            [.., (_, &value), _] => value,
            [.., (_, &value)] => value,
            _ => unreachable!("`bids` is nonempty, as checked above."),
        };

        if sender == self.owner {
            self.owner.transfer(item_value).unwrap(); // panicking reverts.
        } else if let Some(&bid) = self.bids.get(&sender) {
            if sender_is_winner {
                sender.transfer(bid - item_value).unwrap();
            } else {
                sender.transfer(bid).unwrap();
            }
        }

        self.payed.insert(sender);
        Ok(sender_is_winner)
    }

    fn sorted_bids<'a>(&'a self) -> Vec<(&'a Address, &'a Balance)> {
        let mut bids: Vec<(&Address, &Balance)> = self.bids.iter().collect();
        bids.sort_unstable_by_key(|(_, value)| *value);
        bids
    }
}

/// Returns the current unix timestamp.
fn now_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap() // cannot actually fail
        .as_secs()
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuctionError {
    BadItem,
    TooSoon,
    TooLate,
}

fn main() {
    oasis_std::service!(SealedAuction);
}
