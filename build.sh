set -xe
PATH="/usr/local/opt/ruby@3.1/bin:$PATH"
(cd omnibus ; wasm-pack build -t web --debug)
(cd site && bundle exec jekyll build)
