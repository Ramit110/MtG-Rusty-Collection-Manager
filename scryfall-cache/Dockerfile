# build the binaries
FROM rust:1.84 as compiler
WORKDIR /usr/src/scryfall-cache
COPY . .
RUN <<EOF
    apt-get update && apt-get upgrade
    cargo install --path .
    cargo build --locked --release
    cp target/release/mtg_lounge_collections
EOF

# run the rust
FROM debian as executor
COPY --from=compiler /usr/local/cargo/bin/scryfall-cache /usr/local/bin/scryfall-cache
RUN <<EOF
    apt-get update && apt-get upgrade
    scryfall-cache
EOF

# run the database
FROM mongo as deploy
WORKDIR /data
ENV MONGO_INITDB_ROOT_USERNAME root
ENV MONGO_INITDB_ROOT_PASSWORD password
RUN <<EOF
    apt-get update && apt-get upgrade
    for file in /initial; do; mongoimport --db scryfall --file "$file".json --batchSize 1; done
EOF
CMD ["mongod"]
