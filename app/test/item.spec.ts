import oasis from '@oasislabs/client';

jest.setTimeout(15000);

describe('AuctionItem', () => {
    let service: any;

    beforeAll(async () => {
        service = await oasis.workspace.AuctionItem.deploy({
            header: { confidential: false },
            gasLimit: '0x700000',
        });
    });

    it('transfer', async () => {
        const transferCap = await service.transferCap({
            gasLimit: '0x60000',
        });
        expect(transferCap).not.toBe(null);

        expect(
            await service.setOwner(transferCap, new Uint8Array(20), {
                gasLimit: '0x65000',
            }),
        ).toBe(true);

        expect(
            await service.transferCap({
                gasLimit: '0x40000',
            }),
        ).toBe(null);
    });

    afterAll(() => {
        oasis.disconnect();
    });
});
