# NET=mainnet
# URL=https://rpc.geth.mainnet.eth.blockfun.online

# NET=mainnet
# URL=https://eth-mainnet.g.alchemy.com/v2/qYWu6fiQJFaXsbgluccotBWlQ0yxywSW

# export RISC0_DEV_MODE=1
NET=dev
URL=http://192.168.50.158:8545

# NET=dev
# URL=https://testnet-hub-rpc.bsquared.network

# NET=optimism
# URL=https://opt-mainnet.g.alchemy.com/v2/qYWu6fiQJFaXsbgluccotBWlQ0yxywSW

shopt -s expand_aliases
alias elrpc="curl -X POST --url $URL -H 'Content-Type: application/json;' --data ${2} "
alias alchemyrpc='curl -X POST https://eth-mainnet.g.alchemy.com/v2/qYWu6fiQJFaXsbgluccotBWlQ0yxywSW'

set -x

init() {
    rzup install cargo-risczero v1.2.1
    return
}

build() {
    # exec >"$FUNCNAME.log" 2>&1
    time RUST_BACKTRACE=full \
        CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true \
        just build
    return
}

info() {
    just --dry-run build
    return
}

versions() {
    cargo risczero --version
    rzup --version --verbose
    r0vm --version --verbose
    # cargo risczero --help
    # r0vm --help
    return
}

testEthereumCache() {
    exec >"$FUNCNAME.log" 2>&1
    just ethereum build \
        --cache=bin/ethereum/data \
        --block-number=1
    return
}

testOnline() {
    logName="$FUNCNAME-$(date +%Y%m%d-%H%M%S).log"
    exec > $logName 2>&1
    testBatchBlockEthereumOnline $NET 100 1

    # block=$(elrpc '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' | jq .result | tr -d '"' | cast to-dec)
    # testBatchBlockEthereumOnline $NET $((block - 10)) 1

    # testBatchBlockOptimismOnline $NET 10 1

    # block=$(elrpc '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' | jq .result | tr -d '"' | cast to-dec)
    # testBatchBlockOptimismOnline $NET $((block - 100)) 1
    return
}

testBatchBlockOptimismOnline() {
    chain=$1
    startNum=$2
    cnt=$3
    elrpc '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' | jq .
    proveLogFile=tmp-$chain-$startNum-$cnt.prove.log
    time RUST_BACKTRACE=full \
        just optimism prove \
        --rpc=$URL \
        --block-count=$cnt \
        --block-number=$startNum >$proveLogFile 2>&1

    ZKP_FILE=$(grep -ior 'risc0.*.zkp' $proveLogFile)
    time RUST_BACKTRACE=full \
        just optimism  verify \
        --rpc=$URL \
        --block-count=$cnt \
        --block-number=$startNum \
        --file=$ZKP_FILE

    return
}

testBatchBlockEthereumOnline() {
    chain=$1
    startNum=$2
    cnt=$3
    elrpc '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' | jq .
    proveLogFile=tmp-$chain-$startNum-$cnt.prove.log
    time RUST_BACKTRACE=full \
        just ethereum prove \
        --rpc=$URL \
        --chain=$chain \
        --block-count=$cnt \
        --block-number=$startNum >$proveLogFile 2>&1

    ZKP_FILE=$(grep -ior 'risc0.*.zkp' $proveLogFile)
    time RUST_BACKTRACE=full \
        just ethereum verify \
        --rpc=$URL \
        --chain=$chain \
        --block-count=$cnt \
        --block-number=$startNum \
        --file=$ZKP_FILE

    return
}

verify() {
    # exec >"$FUNCNAME.log" 2>&1
    ZKP=risc0-1.2.1-0x7f06c2cd4f156b9d6bab70a51fcd3d9dbd26325dbfecf132d557fbabcd6b4ae9.zkp
    du -sh $ZKP
    stat $ZKP
    file $ZKP
    sha1sum $ZKP

    time RUST_BACKTRACE=full \
        just ethereum verify \
        --rpc=$URL \
        --chain=mainnet \
        --block-count=2 \
        --block-number=1 \
        --file=$ZKP
    return
}

help() {
    for item in build run prove verify; do
        just ethereum $item --help >tmp-$item.log
    done
    return
}


debug() {
    cargo run \
        --package=zeth-ethereum \
        --bin=zeth-ethereum \
        --message-format=json \
        --color=always \
        -- \
        prove \
        --rpc=http://192.168.50.158:8545 \
        --chain=dev \
        --block-count=1 \
        --block-number=2 > tmp-message-format-$(date +%Y%m%d-%H%M%S).log 2>&1
    return
}

$@
