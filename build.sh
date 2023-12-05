set -xe
PATH="/usr/local/opt/ruby@3.1/bin:$PATH"
(
    cd omnibus
    [[ $1 == "debug" ]] && wasm-pack build -t web --debug || wasm-pack build -t web --release
)
(
    cd site
    if [[ $1 == "publish" ]] ; then
        bundle exec jekyll build -b /~mmullins/adventofcode_2023
    else
        bundle exec jekyll build
    fi
)
