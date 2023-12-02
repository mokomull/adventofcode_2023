set -xe
PATH="/usr/local/opt/ruby@3.1/bin:$PATH"
(
    cd omnibus
    [[ $1 == "debug" ]] && wasm-pack build -t web --debug || wasm-pack build -t web --release
)
(cd site && bundle exec jekyll build)
