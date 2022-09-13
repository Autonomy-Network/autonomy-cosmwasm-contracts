import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Coin, coin, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import "./constants";
import { auto_denom, registryStake, STAN_STAKE, wrapper } from "./constants";
import { Asset } from "./types";
import { toBase64 } from "./util";

async function stakeAuto(
  client: SigningCosmWasmClient,
  wallet: DirectSecp256k1HdWallet,
  numStakes: number
) {
  const [account] = await wallet.getAccounts();
  const amount = STAN_STAKE * numStakes;
  await client.execute(
    account.address,
    registryStake,
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
    registryStake,
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
    registryStake,
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
    registryStake,
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

