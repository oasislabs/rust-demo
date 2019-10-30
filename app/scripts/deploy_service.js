const chalk = require('chalk');
const oasis = require('@oasislabs/client');

oasis.workspace.SealedAuction.deploy({
  header: {confidential: false},
})
  .then(res => {
    let addrHex = Buffer.from(res.address).toString('hex');
    console.log(`    ${chalk.green('Deployed')} SealedAuction at 0x${addrHex}`);
  })
  .catch(err => {
    console.error(chalk.red('error'), err);
  })
  .finally(() => {
    oasis.disconnect();
  });
