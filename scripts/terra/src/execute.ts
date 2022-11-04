import {
  Coin,
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
  Wallet,
} from "@terra-money/terra.js";

import "./constants";
import { auto, registry, registryCodeId, STAN_STAKE, wrapperAstroport } from "./constants";
import { sendMessage, toBase64, upgradeContract } from "./util";

async function stakeAuto(client: LCDClient, wallet: Wallet, numStakes: number) {
  const msgs = [
    new MsgExecuteContract(wallet.key.accAddress, auto, {
      send: {
        amount: (STAN_STAKE * numStakes).toString(),
        contract: registry,
        msg: toBase64({
          stake: {
            num_stakes: numStakes,
          },
        }),
      },
    }),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}

async function unstakeAuto(client: LCDClient, wallet: Wallet, idxs: number[]) {
  const msgs = [
    new MsgExecuteContract(wallet.key.accAddress, registry, {
      unstake: {
        idxs,
      },
    }),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}

async function updateExecutor(client: LCDClient, wallet: Wallet) {
  const msgs = [
    new MsgExecuteContract(wallet.key.accAddress, registry, {
      update_executor: {},
    }),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}

async function swapAstroport(client: LCDClient, wallet: Wallet) {
  const input_asset = {
    amount: "100000000",
    info: {
      token: {
        contract_addr:
          "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv",
      },
    },
  };

  const swapMsg = {
    swap: { max_spread: "0.005", belief_price: "475.088722818986445718" },
  };

  const msg = {
    swap: {
      user: wallet.key.accAddress,
      contract_addr: "terra1udsua9w6jljwxwgwsegvt6v657rg3ayfvemupnes7lrggd28s0wq7g8azm",
      swap_msg: toBase64(swapMsg),
      offer_asset: input_asset,
      output_asset: {
        native_token: {
          denom: "uluna",
        },
      },
      min_output: "1",
      max_output: "100000000",
      recipient_exist: false,
    },
  };

  const msgs = [
    new MsgExecuteContract(wallet.key.accAddress, wrapperAstroport, msg),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}

async function approve(client: LCDClient, wallet: Wallet) {
  const msgs = [
    new MsgExecuteContract(
      wallet.key.accAddress,
      "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv",
      {
        increase_allowance: {
          spender: registry,
          amount: "100000000000",
        },
      }
    ),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}

async function cancelRequest(
  client: LCDClient,
  wallet: Wallet,
  requestId: number
) {
  const msgs = [
    new MsgExecuteContract(wallet.key.accAddress, registry, {
      cancel_request: {
        id: requestId,
      },
    }),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}

async function createRequest(client: LCDClient, wallet: Wallet) {
  const input_asset = {
    amount: "100000000",
    info: {
      token: {
        contract_addr:
          "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv",
      },
    },
  };

  const swapMsg = {
    swap: { max_spread: "0.005", belief_price: "475.088722818986445718" },
  };

  const msg = {
    swap: {
      user: wallet.key.accAddress,
      contract_addr:
        "terra1udsua9w6jljwxwgwsegvt6v657rg3ayfvemupnes7lrggd28s0wq7g8azm",
      swap_msg: toBase64(swapMsg),
      offer_asset: input_asset,
      output_asset: {
        native_token: {
          denom: "uluna",
        },
      },
      min_output: "1",
      max_output: "100000000",
      recipient_exist: false,
    },
  };

  const msgs = [
    new MsgExecuteContract(
      wallet.key.accAddress,
      registry,
      {
        create_request: {
          target: wrapperAstroport,
          msg: toBase64(msg),
          input_asset,
        },
      },
      [new Coin("uluna", "10000")]
    ),
  ];
  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
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

  console.log(`Wallet is ${wallet.key.accAddress}`);

  // await cancelRequest(client, wallet, 2);
  // await stakeAuto(client, wallet, 2);
  // await updateExecutor(client, wallet);
  // await unstakeAuto(client, wallet, [0]);
  // await approve(client, wallet);
  await createRequest(client, wallet);
  // await upgradeContract(client, wallet, registry, registryCodeId);
}

main().catch(console.error);