import oasis from '@oasislabs/client';

jest.setTimeout(15000);

const C10L = false;

describe('SealedAuction', () => {
    let item: any;
    let auction: any;

    beforeAll(async () => {
        item = await oasis.workspace.AuctionItem.deploy({
            header: { confidential: C10L },
            gasLimit: '0x900000',
        });
        const transferCap = await item.transferCap({
            gasLimit: '0x600000',
        });
        auction = await oasis.workspace.SealedAuction.deploy(
            item.address.bytes,
            transferCap,
            60 * 60 * 24 /* one day */,
            {
                header: { confidential: C10L },
                gasLimit: '0x900000',
            },
        );
    });

    it('transferred', async () => {
        expect(
            await item.transferCap({
                gasLimit: '0x60000',
            }),
        ).toBe(null);
    });

    it('cannot immediately withdraw', async () => {
        return expect(
            auction.withdraw({
                gasLimit: '0x60000',
            }),
        ).rejects.toThrow();
    });

    afterAll(() => {
        oasis.disconnect();
    });
});
