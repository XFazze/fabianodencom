FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build /app/target/release/backend /usr/local/bin/backend
COPY --from=build /app/frontend/dist /usr/local/bin/dist

EXPOSE 8080

WORKDIR /usr/local/bin
CMD ["backend"]