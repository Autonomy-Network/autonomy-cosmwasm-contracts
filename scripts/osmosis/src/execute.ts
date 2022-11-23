import { coin } from "@cosmjs/proto-signing";
import { getRegistryClient } from "./connection";
import "./constants";
import { toBase64 } from "./util";

async function main() {
  const client = await getRegistryClient();

  const swap = {
    user: "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
    amount: "10000000",
    min_output: "1",
    max_output: "18446744073709551615",
    first: {
      pool_id: "1",
      denom_in: "uosmo",
      denom_out:
        "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
    },
    route: [],
  };

  await client.createRequest(
    {
      requestInfo: {
        target:
          "osmo1dwpdh2clk7c8csf9ql2xj36336xsryyg4j7622jhaert9htp48gsh8u9ve",
        msg: toBase64(swap),
        input_asset: {
          info: {
            native_token: {
              denom: "uosmo",
            },
          },
          amount: "10000000",
        },
        is_recurring: false,
      },
    },
    "auto",
    undefined,
    [coin("11000000", "uosmo")]
  );
}

main().catch(console.error);
