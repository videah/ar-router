ARG RUST_VERSION=1.83.0

FROM rust:${RUST_VERSION}-alpine AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev npm
RUN npm install tailwindcss@next @tailwindcss/cli@next
COPY . .
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
    cp ./target/release/ar-router /

FROM alpine:3 AS final
WORKDIR /app
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /ar-router .
USER myuser
CMD ["./ar-router"]