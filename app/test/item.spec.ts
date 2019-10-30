import oasis from '@oasislabs/client';

jest.setTimeout(20000);

describe('SealedAuction', () => {
    let service: any;

    beforeAll(async () => {
        service = await oasis.workspace.SealedAuction.deploy({
            header: { confidential: false },
        });
    });

    it('says hello', async () => {
        expect(await service.sayHello()).toMatch(/^Hello/);
    });

    afterAll(() => {
        oasis.disconnect();
    });
});
