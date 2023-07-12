# Watch run
    cargo install cargo-watch
    cargo-watch -x run
	cargo-watch -x "run -p rssserver"

Add your application client id to config.toml
.cargo/config.toml

    [env]
    "AUD" = "<GUID>"