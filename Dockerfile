FROM rust:latest as builder

WORKDIR /build
COPY . .

RUN cargo install --path .

EXPOSE 8084

CMD /usr/local/cargo/bin/lightspeed-ingest
