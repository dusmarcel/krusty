FROM rust:latest AS build
WORKDIR /build
ARG POSTGRES_HOST
ARG POSTGRES_DATABASE
ARG POSTGRES_USER
ARG POSTGRES_PASSWORD
ARG BACKEND_IF
ARG POSTGRES_HOST=$POSTGRES_HOST
ARG POSTGRES_DATABASE=$POSTGRES_DATABASE
ARG POSTGRES_USER=$POSTGRES_USER
ARG POSTGRES_PASSWORD=$POSTGRES_PASSWORD
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