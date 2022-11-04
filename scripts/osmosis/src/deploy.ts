import { instantiateContract } from "./util";
import { auto_denom, registryCodeId, wrapperCodeId } from "./constants";
import { getConnection } from "./connection";

async function main() {
  const { client, wallet } = await getConnection();

  const regisry = await instantiateContract(
    client,
    wallet,
    wallet,
    registryCodeId,
    {
      auto: {
        native_token: {
          denom: auto_denom,
        },
      },
      fee_amount: "1000",
      fee_denom: "uosmo",
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
