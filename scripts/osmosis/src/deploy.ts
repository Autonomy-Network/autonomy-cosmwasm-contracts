import { instantiateContract } from "./util";
import { auto_denom, registryCodeId, STAN_STAKE, wrapperCodeId } from "./constants";
import { getConnection } from "./connection";

async function main() {
  const { client, wallet } = await getConnection();

  const regisry = await instantiateContract(
    client,
    wallet,
    wallet,
    registryCodeId,
    {
      config: {
        owner: (await wallet.getAccounts())[0].address,
        auto: {
          native_token: {
            denom: auto_denom,
          },
        },
        fee_amount: "1000",
        fee_denom: "uosmo",
        stake_amount: STAN_STAKE.toString(),
        blocks_in_epoch: 1000,
      },
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
