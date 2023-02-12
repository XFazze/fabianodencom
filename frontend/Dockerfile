FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /app
COPY . .



RUN cd frontend && ./tailwindcss src/static/style/input.css -o src/static/style/output.css --watch
RUN cd frontend && trunk build --release