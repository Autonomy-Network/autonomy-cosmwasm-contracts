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
  "osmo1yr7asvwnwtr5aedqt5k72229fxaclqyt8x8aw4mea60xxuesgkhqpes76u";

export const registryCodeId = 4699;
export const wrapperCodeId = 4700;

export const STAN_STAKE = 10000;
