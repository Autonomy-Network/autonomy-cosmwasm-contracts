STAKEMGR_INIT=$(cat <<EOF
{
  "auto": {
    "native_token": {
      "denom": "uosmo"
    }
  }
}
EOF
)

REGISTRY_INIT=$(cat <<EOF
{
  "stake_manager": "osmo1p54qvfde6mpnqvz3dnpa78x2qyyr5k4sgw9qr97mxjgklc5gze9s75ygrc",
  "fee_amount": "1000",
  "fee_denom": "uosmo"
}
EOF
)

SWAP=$(cat <<EOF
{
  "swap": {
    "user": "osmo1phaxpevm5wecex2jyaqty2a4v02qj7qmlmzk5a",
    "pool_id": 1,
    "denom_in": "uosmo",
    "denom_out": "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
    "amount": "1000000",
    "min_output": "1",
    "max_output": "90000000"
  }
}
EOF
)

CREATE_REQUEST=$(cat <<EOF
{
  "create_request": {
      "target": "osmo1999u8suptza3rtxwk7lspve02m406xe7l622erg3np3aq05gawxsj2lrr5",
      "msg": "ewogICJzd2FwIjogewogICAgInVzZXIiOiAib3NtbzFwaGF4cGV2bTV3ZWNleDJqeWFxdHkyYTR2MDJxajdxbWxtems1YSIsCiAgICAicG9vbF9pZCI6IDEsCiAgICAiZGVub21faW4iOiAidW9zbW8iLAogICAgImRlbm9tX291dCI6ICJ1aW9uIiwKICAgICJhbW91bnQiOiAiMTAwMDAwMCIsCiAgICAibWluX291dHB1dCI6ICIxIiwKICAgICJtYXhfb3V0cHV0IjogIjkiCiAgfQp9",
      "input_asset": {
        "info": {
          "native_token": { "denom": "uosmo" }
        },
        "amount": "1000000"
      }
  }
}
EOF
)

