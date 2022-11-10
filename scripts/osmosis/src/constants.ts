import "dotenv/config";

// environment
export const MNEMONIC = process.env.MNEMONIC || "";
export const MAIN_NETWORK = process.env.MAIN_NETWORK || "";
export const PREFIX = process.env.PREFIX || "";
export const GASPRICE = process.env.GASPRICE || "";

export const auto_denom = "uosmo";

// testnet
export const registry =
  "osmo1ulqdnqjkm6v6kdehh37md4a6fs4ppyrv9jqr0ypcud9p8efygzmsstn2sy";
export const wrapper =
  "osmo1dwpdh2clk7c8csf9ql2xj36336xsryyg4j7622jhaert9htp48gsh8u9ve";

export const registryCodeId = 3620;
export const wrapperCodeId = 3621;

export const STAN_STAKE = 10000;
