set -x

build() {
    time RUST_BACKTRACE=full \
        CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true \
        just build >tmp.log 2>&1
    return
}

info(){
    just --dry-run build
    return
}

$@
