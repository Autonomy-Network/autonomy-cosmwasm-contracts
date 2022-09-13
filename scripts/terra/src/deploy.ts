import { LCDClient, MnemonicKey, Wallet } from "@terra-money/terra.js";

import "./constants";
import { auto, registryCodeId, wrapperAstroportCodeId } from "./constants";
import { deployContract } from "./util";

async function deployRegistry(client: LCDClient, wallet: Wallet) {
  const initMsg = {
    auto: {
      token: {
        contract_addr: auto,
      },
    },
    fee_amount: "10000",
    fee_denom: "uluna",
  };
  const registry = await deployContract(
    client,
    wallet,
    registryCodeId,
    initMsg
  );
  console.log("Registry:", registry);
}

async function deployAstroWrapper(client: LCDClient, wallet: Wallet) {
  const initMsg = {};
  const wrapper = await deployContract(
    client,
    wallet,
    wrapperAstroportCodeId,
    initMsg
  );
  console.log("Wrapper astro:", wrapper);
}

async function main() {
  const client = new LCDClient({
    URL: process.env.MAIN_NETWORK || "",
    chainID: process.env.CHAINID || "columbus-5",
  });

  const wallet = client.wallet(
    new MnemonicKey({
      mnemonic: process.env.MNEMONIC || "",
    })
  );

  console.log(`Deployer is ${wallet.key.accAddress}`);

  await deployAstroWrapper(client, wallet);
}

main().catch(console.error);
