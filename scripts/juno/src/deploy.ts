import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import "./constants";

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import { instantiateContract } from "./util";
import { auto_denom, registryStakeCodeId, wrapperCodeId } from "./constants";

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    process.env.MNEMONIC || "",
    { prefix: "juno" }
  );

  const client = await SigningCosmWasmClient.connectWithSigner(
    process.env.MAIN_NETWORK || "localhost:26657",
    wallet,
    { gasPrice: GasPrice.fromString("0.025ujuno") }
  );

  const regisry = await instantiateContract(
    client,
    wallet,
    wallet,
    registryStakeCodeId,
    {
      auto: {
        native_token: {
          denom: auto_denom,
        },
      },
      fee_amount: "800000",
      fee_denom: "ujuno",
    }
  );
  console.log("regisry", regisry.contractAddress);
  const wrapper = await instantiateContract(
    client,
    wallet,
    wallet,
    wrapperCodeId,
    {}
  );
  console.log("wrapper", wrapper.contractAddress);
}

main().catch(console.error);
