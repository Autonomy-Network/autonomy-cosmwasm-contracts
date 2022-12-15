import "dotenv/config";

// environment
export const MNEMONIC = process.env.MNEMONIC || "";
export const MAIN_NETWORK = process.env.MAIN_NETWORK || "";
export const CHAINID = process.env.CHAINID || "";

export const STAN_STAKE = 1000;

// phoenix mainnet

// export const auto = "terra1jh37p2akmwprfwr2235kjyt3wuz0s2sfenhtytg9cajsxnxfvdasnaurdp";
// export const registry = "terra17lxkgqs30guzngnk9yf7ncgrvcrjkn38x9g2we90utfhmk3xmgcq79amda";
// export const wrapperAstroport = "terra1twpa03hsm0vdnqahu5hud4jvuv8h7c0ed9guvy53wkfae4r90g2qp5dp8q";

// export const auto_denom = "uluna";

// export const registryCodeId = 865;
// export const wrapperAstroportCodeId = 866;

// pisco testnet

export const auto = "terra1679n3rthcvghet6tc29ugqtn5f3axp0lvceshq0c0r482pps3u4qggfyax";
export const registry = "terra1s78hgz6gmqgrvsltwf6tlzjzp9xql7a07532g5g86edg00krfjds40tfla";
export const wrapperAstroport = "terra1mc0wr7n2mmfu3crp4xqxun6spgtenacwtzzz249p6r9p8yx2ctysc2gc0d";

export const auto_denom = "uluna";

export const registryCodeId = 6290;
export const wrapperAstroportCodeId = 6291;
