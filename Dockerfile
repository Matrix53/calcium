FROM rust:latest
COPY ./* /calcium/
WORKDIR /calcium/
RUN cargo build