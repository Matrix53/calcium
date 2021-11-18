FROM matrix53/rust:latest
COPY ./ /calcium/
WORKDIR /calcium/
RUN cargo build