FROM rust:latest as deps

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

FROM deps as builder

WORKDIR /app
COPY . .

RUN echo "export ARCHITECTURE=`dpkg --print-architecture`" >> /architecturefile
RUN  . /architecturefile; if [ $ARCHITECTURE="arm64"]; then cd frontend && ./tailwindcss_arm64 src/static/style/input.css -o src/static/style/output.css --minify;\
    # elif [$ARCHITECTURE="arm64"]; then cd frontend && ./tailwindcss_arm64  src/static/style/input.css -o src/static/style/output.css --minify;\
    fi
RUN  cd frontend && trunk build --release

RUN cargo build --release

FROM builder as runner

COPY --from=builder /app/frontend/dist /usr/local/bin/dist
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

WORKDIR /usr/local/bin
CMD ["backend"]