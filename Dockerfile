FROM rust:1-buster

WORKDIR /usr/src/sketchy-art

VOLUME /usr/src/sketchy-art

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli