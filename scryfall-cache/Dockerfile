# build the binaries
FROM rust:1.84 AS compiler
WORKDIR /usr/src/scryfall-cache
COPY . .
RUN apt-get update && apt-get upgrade -y
RUN <<EOF
    cargo install --path .
    cargo build --release
EOF

# run the rust
FROM debian AS executor
RUN apt-get update &&  apt-get upgrade -y
COPY --from=compiler /usr/src/scryfall-cache/target/release /scryfall-cache
WORKDIR /scryfall-cache
RUN ls -l
RUN ./scryfall-cache

# run the database
FROM mongo AS deploy
COPY --from=executor /initial /initial
WORKDIR /initial
ENV MONGO_INITDB_ROOT_USERNAME=root
ENV MONGO_INITDB_ROOT_PASSWORD=password
RUN apt-get update && apt-get upgrade -y
RUN find rb-script -name '*.json' -type f -exec mongoimport --db scryfall --file "{}" --batchSize 1 \
CMD ["mongod"]
