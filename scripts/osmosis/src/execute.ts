import { coin } from "@cosmjs/proto-signing";
import { getRegistryClient } from "./connection";
import "./constants";

// const swap = {
//   user: "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
//   amount: "1000",
//   min_output: "0",
//   max_output: "3275571",
//   first: {
//     pool_id: "1",
//     denom_in:
//       "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
//     denom_out: "uosmo",
//   },
//   route: [
//     // {
//     //   pool_id: 2,
//     //   denom_out: "uion",
//     // },
//   ],
// };

async function main() {
  const client = await getRegistryClient();
  await client.stakeDenom({ numStakes: 1 }, "auto", undefined, [
    coin(10000, "uosmo"),
  ]);
}

main().catch(console.error);
