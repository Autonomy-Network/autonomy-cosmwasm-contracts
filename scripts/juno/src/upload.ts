import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import "./constants";

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import { storeCode } from "./util";

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

  const registryCodeId = await storeCode(
    client,
    wallet,
    "../../artifacts/registry_stake.wasm"
  );
  console.log("registry:", registryCodeId);
  const wrapperCodeId = await storeCode(
    client,
    wallet,
    "../../artifacts/wrapper_junoswap.wasm"
  );
  console.log("wrapper:", wrapperCodeId);
}

main().catch(console.error);
