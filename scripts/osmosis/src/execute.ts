import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Coin, coin, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import "./constants";
import { auto_denom, registry, registryCodeId, STAN_STAKE, wrapper } from "./constants";
import { Asset } from "./types";
import { toBase64, migrateContract } from "./util";

async function stakeAuto(
  client: SigningCosmWasmClient,
  wallet: DirectSecp256k1HdWallet,
  numStakes: number
) {
  const [account] = await wallet.getAccounts();
  const amount = STAN_STAKE * numStakes;
  await client.execute(
    account.address,
    registry,
    {
      stake_denom: {
        num_stakes: numStakes,
      },
    },
    "auto",
    undefined,
    [coin(amount, auto_denom)]
  );
}

async function unstakeAuto(
  client: SigningCosmWasmClient,
  wallet: DirectSecp256k1HdWallet,
  idxs: number[]
) {
  const [account] = await wallet.getAccounts();
  await client.execute(
    account.address,
    registry,
    {
      unstake: {
        idxs: idxs,
      },
    },
    "auto"
  );
}

async function updateExecutor(
  client: SigningCosmWasmClient,
  wallet: DirectSecp256k1HdWallet
) {
  const [account] = await wallet.getAccounts();
  await client.execute(
    account.address,
    registry,
    {
      update_executor: {},
    },
    "auto"
  );
}

async function createRequest(
  client: SigningCosmWasmClient,
  wallet: DirectSecp256k1HdWallet,
  swapMsg: Record<string, unknown>,
  inputAsset: Asset,
  funds: Coin[]
) {
  const [account] = await wallet.getAccounts();
  await client.execute(
    account.address,
    registry,
    {
      create_request: {
        target: wrapper,
        msg: toBase64(swapMsg),
        input_asset: inputAsset,
      },
    },
    "auto",
    undefined,
    funds
  );
}

async function swap(
  client: SigningCosmWasmClient,
  wallet: DirectSecp256k1HdWallet
) {
  const [account] = await wallet.getAccounts();
  await client.execute(
    account.address,
    wrapper,
    {
      swap: {
        user: "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
        amount: "1000",
        min_output: "0",
        max_output: "3275571",
        first: {
          pool_id: "1",
          denom_in:
            "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
          denom_out: "uosmo",
        },
        route: [
          // {
          //   pool_id: 2,
          //   denom_out: "uion",
          // },
        ],
      },
    },
    "auto",
    undefined,
    [
      coin(
        "1000",
        "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
      ),
    ]
  );
}

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    process.env.MNEMONIC || "",
    { prefix: "osmo" }
  );

  const client = await SigningCosmWasmClient.connectWithSigner(
    process.env.MAIN_NETWORK || "localhost:26657",
    wallet,
    { gasPrice: GasPrice.fromString("0.025uosmo") }
  );

  // await stakeAuto(client, wallet, 2);
  // await createRequest(
  //   client,
  //   wallet,
  //   {
  //     swap: {
  //       user: "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
  //       first: {
  //         pool_id: 1,
  //         denom_in:
  //           "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
  //         denom_out: "uosmo",
  //       },
  //       route: [
  //         // {
  //         //   pool_id: 2,
  //         //   denom_out: "uion",
  //         // },
  //       ],
  //       amount: "100000",
  //       min_output: "1",
  //       max_output: "90000000",
  //     },
  //   },
  //   {
  //     info: {
  //       native_token: {
  //         denom:
  //           "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
  //       },
  //     },
  //     amount: "100000",
  //   },
  //   [
  //     coin(
  //       "100000",
  //       "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
  //     ),
  //     coin("1000", "uosmo"),
  //   ]
  // );

  const request = await client.queryContractSmart(registry, {
    request_info: {
      id: 66,
    },
  });
  console.log(request);
  return;

  const requests = await client.queryContractSmart(registry, {
    requests: {},
  });
  console.log(requests);

  const stakes = await client.queryContractSmart(registry, {
    stakes: {
      start: 0,
      limit: 10,
    },
  });
  const state = await client.queryContractSmart(registry, {
    state: {},
  });
  const epochInfo = await client.queryContractSmart(registry, {
    epoch_info: {},
  });
  console.log(epochInfo);
  console.log(state);
  console.log(stakes);
}

main().catch(console.error);
