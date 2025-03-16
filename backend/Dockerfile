# build the binaries
FROM rust:1.84 AS compiler
RUN apt-get update && apt-get upgrade -y
WORKDIR /usr/src/mtg-lounge-collections
COPY . .
RUN <<EOF
    cargo install --path .
    cargo build --release
EOF

# run the rust
FROM debian AS executor
RUN apt-get update &&  apt-get upgrade -y
WORKDIR /mtg-lounge-collections
COPY --from=compiler /usr/src/scryfall-cache/target/mtg-lounge-collections /mtg-lounge-collections
CMD ["./mtg-lounge-collections"]
