import "dotenv/config";

// environment
export const MNEMONIC = process.env.MNEMONIC || "";
export const MAIN_NETWORK = process.env.MAIN_NETWORK || "";
export const PREFIX = process.env.PREFIX || "";
export const GASPRICE = process.env.GASPRICE || "";

export const auto_denom = "uosmo";

// testnet
export const registry =
  "osmo1dhkyxu0g9u6zr3wm3d3s875atnffvr60hg2gnsz80s48j33atmxq3ptp4c";
export const wrapper =
  "osmo16ca3u3l8mrgrmz3u48rrfh7dhf3e9y8d9xrxf3w0lla6xy073v7stz3r4e";

export const registryCodeId = 3620;
export const wrapperCodeId = 3621;

export const STAN_STAKE = 10000;
