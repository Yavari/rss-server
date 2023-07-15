# Watch run
    cargo install cargo-watch
    cargo-watch -x run
	cargo-watch -x "run -p rssserver"

# Run
    cargo run --bin rssreader
    cargo run --bin rssserver
    
# Docker
    docker build -t rssserver .
    docker run -p 3030:3030 --name=rssserver rssserver

Run Docker build without cache

    docker build --progress=plain --no-cache -t rssserver .