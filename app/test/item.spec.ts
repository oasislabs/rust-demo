import oasis from '@oasislabs/client';

jest.setTimeout(15000);

const C10L = false;

describe('AuctionItem', () => {
    let item: any;

    beforeAll(async () => {
        item = await oasis.workspace.AuctionItem.deploy({
            header: { confidential: C10L },
            gasLimit: '0x700000',
        });
    });

    it('transfer', async () => {
        const transferCap = await item.transferCap({
            gasLimit: '0x60000',
        });
        expect(transferCap).not.toBe(null);

        expect(
            await item.setOwner(transferCap, new Uint8Array(20), {
                gasLimit: '0x65000',
            }),
        ).toBe(true);

        expect(
            await item.transferCap({
                gasLimit: '0x40000',
            }),
        ).toBe(null);
    });

    afterAll(() => {
        oasis.disconnect();
    });
});
