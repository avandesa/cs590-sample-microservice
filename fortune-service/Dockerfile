FROM rust:latest
LABEL maintainer="Alex van de Sandt <alex@avandesa.dev>"

ENV TERM=linux

# Install 'fortune'
RUN apt-get update -y && apt-get install -y fortune-mod

WORKDIR /build

# Copy files
RUN mkdir src
COPY Cargo.* ./
COPY src/* ./src/

# Build
RUN cargo build --release && cp ./target/release/fortune-service ./service && cargo clean

EXPOSE 8082

ENTRYPOINT ["./service"]
