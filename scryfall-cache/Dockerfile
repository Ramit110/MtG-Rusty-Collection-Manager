# build the binaries
FROM rust:1.84 AS compiler
RUN apt-get update && apt-get upgrade -y
WORKDIR /usr/src/scryfall-cache
COPY . .
RUN cargo install --path .
RUN cargo build --release

# run the rust
FROM debian AS executor
RUN apt-get update && apt-get upgrade -y
RUN apt-get -y install libssl3 ca-certificates
WORKDIR /scryfall-cache
COPY --from=compiler /usr/src/scryfall-cache/target/release /scryfall-cache
RUN ./scryfall-cache

# prepare the database
FROM mongo AS deploy
ENV MONGO_INITDB_ROOT_USERNAME=root
ENV MONGO_INITDB_ROOT_PASSWORD=password
RUN apt-get update && apt-get upgrade -y
WORKDIR /initial
COPY --from=executor /initial /initial
RUN mongod --fork --logpath /var/log/mongod.log
RUN mongosh  --username=root --password=password --eval 'show dbs'
RUN find . -name '*.json' -type f -exec mongoimport --username=root --password=password --db=scryfall --file='{}' \;

# run the daabase
CMD ["mongod"]
