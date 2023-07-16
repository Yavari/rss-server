FROM rust:1.71 AS base
WORKDIR /app
EXPOSE 3030

FROM rust:1.71 AS builder
WORKDIR /source

RUN mkdir -p libs/blogparser/src
RUN mkdir -p rssreader/src
RUN mkdir -p rssserver/src

RUN echo '' > libs/blogparser/src/lib.rs
RUN echo 'fn main() {println!("RSS Client build main");}' > rssreader/src/main.rs
RUN echo 'fn main() {println!("RSS Client build main");}' > rssserver/src/main.rs

COPY src/Cargo.toml Cargo.toml
COPY src/Cargo.lock Cargo.lock
COPY src/libs/blogparser/Cargo.toml libs/blogparser/Cargo.toml
COPY src/rssreader/Cargo.toml rssreader/Cargo.toml
COPY src/rssserver/Cargo.toml rssserver/Cargo.toml

RUN cargo build --bin rssserver --release

COPY src/ .
RUN touch libs/blogparser/src/lib.rs
RUN touch rssreader/src/main.rs
RUN touch rssserver/src/main.rs
RUN cargo build --bin rssserver --release 

FROM base AS final
WORKDIR /app
COPY --from=builder /source/target/release/rssserver .
COPY --from=builder /source/target/release/rssserver.d .
CMD ["/app/rssserver"]