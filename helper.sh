# URL=https://rpc.geth.mainnet.eth.blockfun.online
URL=https://eth-mainnet.g.alchemy.com/v2/qYWu6fiQJFaXsbgluccotBWlQ0yxywSW
shopt -s expand_aliases
alias elrpc="curl -X POST --url $URL -H 'Content-Type: application/json;' --data ${2} "
alias alchemyrpc='curl -X POST https://eth-mainnet.g.alchemy.com/v2/qYWu6fiQJFaXsbgluccotBWlQ0yxywSW'
alias run=''
set -x

probeAlchemy() {
    alchemyrpc -H "Content-Type: application/json" \
        -d '{
  "jsonrpc": "2.0",
  "method": "eth_getBlockByNumber",
  "params": ["0x1", false],
  "id": 1
}' >tmp.json
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
    cargo risczero --help
    r0vm --help
    return
}

testEthereum() {
    exec >"$FUNCNAME.log" 2>&1
    # just ethereum build \
    #     --cache=bin/ethereum/data \
    #     --block-number=1
    # return
    elrpc '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' | jq .
    elrpc '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x1",true],"id":1}' | jq .

    # time RUST_BACKTRACE=full \
    just ethereum prove \
        --rpc=$URL \
        --chain=mainnet \
        --block-count=2 \
        --block-number=1
    return
}

help() {
    for item in build run prove verify; do
        just ethereum $item --help >tmp-$item.log
    done
    return
}
$@
