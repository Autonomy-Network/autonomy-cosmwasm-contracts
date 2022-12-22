import "dotenv/config";

// environment
export const MNEMONIC = process.env.MNEMONIC || "";
export const MAIN_NETWORK = process.env.MAIN_NETWORK || "";
export const PREFIX = process.env.PREFIX || "";
export const GASPRICE = process.env.GASPRICE || "";

export const auto_denom = "uosmo";

// testnet
export const registry =
  "osmo1nnxzdevdh2q2vrxteug6g0vfkz8qawg7xhn5tc7v8aawrq4qacasymx8fg";
export const wrapper =
  "osmo1g85sgdggdhez4htjj3d9m2zw734dt4ap6gdcymp6ymrveq5d48nswqkuww";

export const registryCodeId = 4920;
export const wrapperCodeId = 4921;

export const STAN_STAKE = 10000;
