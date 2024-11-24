ARG RUST_VERSION=1.82

FROM rust:${RUST_VERSION}-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:latest AS final
WORKDIR /app
COPY --from=build /app/target/release/backend backend
USER 1001
EXPOSE 8161
CMD [ "/app/backend" ]