# Watch run
    cargo install cargo-watch
    cargo-watch -x run
	cargo-watch -x "run -p rssserver"

Add your application client id to config.toml
.cargo/config.toml

    [env]
    "AUD" = "<GUID>"

docker build --progress=plain --no-cache -t rssserver .
docker build -t rssserver .
docker run -p 3030:3030 -e AUD=<GUID> --name=rssserver rssserver