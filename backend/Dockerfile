FROM rust:latest AS build
WORKDIR /build
ARG BACKEND_IF
ENV BACKEND_IF=$BACKEND_IF
COPY . .
RUN cargo build --release

FROM debian:stable-slim AS final
RUN apt update && apt install -y libssl3
WORKDIR /app
COPY --from=build /build/target/release/backend backend
USER 1001
EXPOSE 8161
CMD [ "/app/backend" ]