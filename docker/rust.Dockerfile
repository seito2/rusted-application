FROM rust:1.57
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
RUN cargo install cargo-edit
WORKDIR /app

EXPOSE 3001
VOLUME ["/usr/local/cargo"]
