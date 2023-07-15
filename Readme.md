# Watch run
    cargo install cargo-watch
    cargo-watch -x run
	cargo-watch -x "run -p rssserver"

# Environment settings
Add your application client id to config.toml
.cargo/config.toml

    [env]
    "AUD" = "<GUID>"

# Docker
    docker build -t rssserver .
    docker run -p 3030:3030 -e AUD=<GUID> --name=rssserver rssserver

Run Docker build without cache
    docker build --progress=plain --no-cache -t rssserver .