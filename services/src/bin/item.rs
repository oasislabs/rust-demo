use oasis_std::{Address, Context};

#[derive(oasis_std::Service)]
struct AuctionItem {
    /// The owner of the item.
    owner: Address,

    /// A capability that can be used to transfer ownership of this item.
    transfer_cap: [u8; 16],
}

impl AuctionItem {
    pub fn new(ctx: &Context) -> Self {
        Self {
            owner: ctx.sender(),
            transfer_cap: rand::random(),
        }
    }

    /// Returns the address of the item under consideration.
    /// Only the owner is privy to this information.
    pub fn transfer_cap(&self, ctx: &Context) -> Option<[u8; 16]> {
        if ctx.sender() == self.owner {
            Some(self.transfer_cap)
        } else {
            None
        }
    }

    /// Sets the owner of this item if the sender has the capability to do so.
    /// Returns whether the call succeeded.
    pub fn set_owner(&mut self, ctx: &Context, transfer_cap: [u8; 16], new_owner: Address) -> bool {
        if ctx.sender() == self.owner || transfer_cap == self.transfer_cap {
            self.transfer_cap = rand::random();
            self.owner = new_owner;
            true
        } else {
            false
        }
    }
}

fn main() {
    oasis_std::service!(AuctionItem);
}
