/* eslint-disable @typescript-eslint/ban-types */
import { is } from "ramda";
import {
  Wallet,
  LCDClient,
  Tx,
  MsgInstantiateContract,
  Coins,
  isTxError,
  MsgMigrateContract,
  AccAddress,
} from "@terra-money/terra.js";

export const omitEmpty = (object: object): object =>
  Object.entries(object).reduce((acc, [key, value]) => {
    const next = is(Object, value) ? omitEmpty(value) : value;
    const valid = Number.isFinite(value) || value || value === false;
    return Object.assign({}, acc, valid && { [key]: next });
  }, {});

export const toBase64 = (object: object) => {
  try {
    return Buffer.from(JSON.stringify(omitEmpty(object))).toString("base64");
  } catch (error) {
    return "";
  }
};

export const fromBase64 = <T>(string: string): T => {
  try {
    return JSON.parse(Buffer.from(string, "base64").toString());
  } catch (error) {
    return {} as T;
  }
};

export async function sendMessage(client: LCDClient, tx: Tx) {
  const res = await client.tx.broadcast(tx);
  if (isTxError(res)) {
    throw new Error(`Sending Message failed with ${res.raw_log}`);
  }
  return res;
}

export async function deployContract(
  client: LCDClient,
  deployer: Wallet,
  codeId: number,
  init_msg: object | string,
  init_coins?: Coins.Input
) {
  const msgs = [
    new MsgInstantiateContract(
      deployer.key.accAddress,
      deployer.key.accAddress,
      codeId,
      init_msg,
      init_coins,
      "instantiate"
    ),
  ];

  const instantiateTxResult = await sendMessage(
    client,
    await deployer.createAndSignTx({ msgs })
  );

  const {
    instantiate_contract: { contract_address },
  } = instantiateTxResult.logs[0].eventsByType;

  return contract_address[0];
}

export async function upgradeContract(
  client: LCDClient,
  wallet: Wallet,
  contract: AccAddress,
  codeId: number
) {
  const msgs = [
    new MsgMigrateContract(wallet.key.accAddress, contract, codeId, {}),
  ];

  await sendMessage(client, await wallet.createAndSignTx({ msgs }));
}
