import "dotenv/config";

// environment
export const MNEMONIC = process.env.MNEMONIC || "";
export const MAIN_NETWORK = process.env.MAIN_NETWORK || "";
export const PREFIX = process.env.PREFIX || "";
export const GASPRICE = process.env.GASPRICE || "";

export const auto_denom = "uosmo";

// testnet
export const registry =
  "osmo1m5e3fu3tjv7j5txuv65p89tpqruzh29le5e926fssax2tzcf59nqd9fmxf";
export const wrapper =
  "osmo1zkgtuakfpxw00m7rn0e2aphtxe34znr2ewajr04ch6h4usmcf6tqdhvejw";

export const registryCodeId = 478;
export const wrapperCodeId = 270;

export const STAN_STAKE = 1000000;
