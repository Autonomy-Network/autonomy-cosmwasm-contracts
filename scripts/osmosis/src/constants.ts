import "dotenv/config";

// environment
export const MNEMONIC = process.env.MNEMONIC || "";
export const MAIN_NETWORK = process.env.MAIN_NETWORK || "https://rpc-test.osmosis.zone";
export const PREFIX = process.env.PREFIX || "osmo";
export const GASPRICE = process.env.GASPRICE || "";

export const auto_denom =
  "factory/osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a/auto";

// testnet
export const registry =
  "osmo124as70630wa94lpyyqjczc4ne0hsryycg3qdp40s5ljh8qr2nv8szqka4q";
export const wrapper =
  "osmo1c5e5hn72fv24m35aavzv8z9unljfgm7yn0tp6uxtpdlyypmj530sm3sz03";

// export const registryCodeId = 4920;
// export const wrapperCodeId = 4921;

// export const STAN_STAKE = 10000;

// export const registryCodeId = 478;
// export const wrapperCodeId = 270;

export const registryCodeId = 6207;
export const wrapperCodeId = 6208;

export const STAN_STAKE = 10000;
