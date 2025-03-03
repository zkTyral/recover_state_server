# rename this example file to `.env` and put it in the current directory

# [rust]
# Config the log level, see https://docs.rs/env_logger/0.9.1/env_logger/#enabling-logging
# For local development, we could set "sqlx::query=info" to see sql output details
# Or just set RUST_LOG="debug" to see more detail logs
# For testnet or mainnet, we should set "sqlx::query=error" to reduce sqlx log output
RUST_LOG="info,sqlx::query=error"

# [runtime]
# ZKLINK HOME path for some data files
RUNTIME_CONFIG_ZKLINK_HOME="/home/user/zklink/recover_state_server"
# Current directory for the circuit keys.
RUNTIME_CONFIG_KEY_DIR="zklink_keys"

# [api]
API_CONFIG_SERVER_HTTP_PORT=8080
API_CONFIG_WORKERS_NUM=4
API_CONFIG_ENABLE_HTTP_CORS=true

# [database]
# set `USER_NAME`, `HOST` of database url to your postgresql user name
DATABASE_URL="postgres://postgres:postgres@localhost/plasma"
# Amount of open connections to the database.
DATABASE_POOL_SIZE=10

# [chains]
# Chains that supported, the chain id is defined by zkLink
# We can add new chains, but can't drop an exist chain, that means
# We could set this option to "1,2" and then add a new chain(3)
# But we could not drop chain(1) or chain(2)
# NOTE, the max value of `CHAIN_IDS` must not be greater than `MAX_CHAIN_ID` defined in `params.rs`
CHAIN_IDS=1,2

# [chain_1.chain]
# Chain id defined by zkLink, must be equal to the placeholder of `CHAIN_{CHAIN_ID}_CHAIN_ID`
CHAIN_1_CHAIN_ID=1
# Layer one chain type, for example, the chain type of Ethereum is EVM
CHAIN_1_CHAIN_TYPE=EVM
# Gas token price symbol
CHAIN_1_GAS_TOKEN=MATIC
# Whether sender should commit compressed block
# It must be keep same with the constant `ENABLE_COMMIT_COMPRESSED_BLOCK` defined in zkLink contract
CHAIN_1_IS_COMMIT_COMPRESSED_BLOCKS=true

# [chain_1.CONTRACT]
# The block number of CONTRACT deployed
CHAIN_1_CONTRACT_DEPLOYMENT_BLOCK=33377564
# The zkLink main contract address
CHAIN_1_CONTRACT_ADDRESS="0x517aa9dec0E297B744aC7Ac8ddd8B127c1993055"
# The zkLink contract deployed tx hash, used for recover data
CHAIN_1_CONTRACT_GENESIS_TX_HASH="0x5c576039ffefce307ffbc5556899ee0772efcf2046051cc4fe9ca633987061ca"

# [chain_1.client]
# Chain id defined in layer one
CHAIN_1_CLIENT_CHAIN_ID=80001
# RPC Server url of blockchain1.
CHAIN_1_CLIENT_WEB3_URL="https://rpc.ankr.com/polygon_mumbai"
# The rpc service provider asked for a delay in the request because the number of requests was too frequent.
# It is configured according to the documentation of the rpc service
# The default configuration comes from the Infura docs(https://docs.infura.io/infura/networks/ethereum/how-to/avoid-rate-limiting).
CHAIN_1_CLIENT_REQUEST_RATE_LIMIT_DELAY=30

# [chain_2.chain]
# Chain id defined by zkLink, must be equal to the placeholder of `CHAIN_{CHAIN_ID}_CHAIN_ID`
CHAIN_2_CHAIN_ID=2
# Layer one chain type, for example, the chain type of Ethereum is EVM
CHAIN_2_CHAIN_TYPE=EVM
# Gas token price symbol
CHAIN_2_GAS_TOKEN=AVAX
# Whether sender should commit compressed block
# It must be keep same with the constant `ENABLE_COMMIT_COMPRESSED_BLOCK` defined in zkLink contract
CHAIN_2_IS_COMMIT_COMPRESSED_BLOCKS=false

# [chain_2.CONTRACT]
# The block number of CONTRACT deployed
CHAIN_2_CONTRACT_DEPLOYMENT_BLOCK=20072376
# The zkLink main contract address
CHAIN_2_CONTRACT_ADDRESS="0x331a96b91F35051706680d96251931E26f4ba58A"
# The zkLink contract deployed tx hash, used for recover data
CHAIN_2_CONTRACT_GENESIS_TX_HASH="0xce20f9d8eeea9b9eb378d1ce4960c4aa89701f4ed0ae24638c57984f8af3f6ef"

# [chain_2.client]
# Chain id defined in layer one
CHAIN_2_CLIENT_CHAIN_ID=43113
# RPC Server url of blockchain1.
CHAIN_2_CLIENT_WEB3_URL="https://rpc.ankr.com/avalanche_fuji"
# The rpc service provider asked for a delay in the request because the number of requests was too frequent.
# It is configured according to the documentation of the rpc service
# The default configuration comes from the Infura docs(https://docs.infura.io/infura/networks/ethereum/how-to/avoid-rate-limiting).
CHAIN_2_CLIENT_REQUEST_RATE_LIMIT_DELAY=30

# Core applications settings
# [prover.core]
# Timeout(in Milliseconds) to consider prover gone.
PROVER_CORE_GONE_TIMEOUT=60000
# Amount of provers in the cluser if there is no pending jobs.
PROVER_CORE_IDLE_PROVERS=1