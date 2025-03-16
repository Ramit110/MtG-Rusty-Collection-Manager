# build the binaries
FROM rust:1.84 AS compiler
RUN apt-get update && apt-get upgrade -y
WORKDIR /usr/src/scryfall-cache
COPY . .
RUN <<EOF
    cargo install --path .
    cargo build --release
EOF

# run the rust
FROM debian AS executor
RUN apt-get update &&  apt-get upgrade -y
WORKDIR /scryfall-cache
COPY --from=compiler /usr/src/scryfall-cache/target/release /scryfall-cache
RUN ./scryfall-cache

# run the database
FROM mongo AS deploy
ENV MONGO_INITDB_ROOT_USERNAME=root
ENV MONGO_INITDB_ROOT_PASSWORD=password
RUN apt-get update && apt-get upgrade -y
WORKDIR /initial
COPY --from=executor /initial /initial
RUN find rb-script -name '*.json' -type f -exec mongoimport --db scryfall --file "{}" --batchSize 1 \
CMD ["mongod"]
