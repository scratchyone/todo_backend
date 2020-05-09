FROM rustlang/rust:nightly

WORKDIR /usr/src/rustapp
RUN apt update && apt install libssl-dev -y

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release 

RUN rm -rf src
COPY ./ .
CMD cargo run --release