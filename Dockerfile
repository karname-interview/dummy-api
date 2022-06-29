FROM rust:1.61-buster as builder

WORKDIR /app
COPY ./ ./

RUN cargo build --release

FROM ubuntu:20.04
RUN apt update -y && apt-get install -y libpq-dev 
WORKDIR /app
COPY --from=builder /app/target/release/dummy-api ./dummy-api

CMD ["/app/dummy-api"]


