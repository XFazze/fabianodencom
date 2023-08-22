FROM rust:latest as deps

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

FROM deps as frontend-builder

WORKDIR /frontend
COPY ./frontend .

RUN  ./tailwindcss src/static/style/input.css -o src/static/style/output.css --watch
RUN  trunk build --release


FROM  frontend-builder as backend-builder

WORKDIR /app
COPY . .


RUN cargo build --release

FROM backend-builder as runner

COPY --from=frontend-builder /frontend/dist /usr/local/bin/dist
COPY --from=backend-builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

WORKDIR /usr/local/bin
CMD ["backend"]