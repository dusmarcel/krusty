FROM rust:latest AS build
WORKDIR /app
ARG BACKEND_PROTOCOL
ARG BACKEND_HOST
ARG BACKEND_PORT
ENV BACKEND_PROTOCOL=$BACKEND_PROTOCOL
ENV BACKEND_HOST=$BACKEND_HOST
ENV BACKEND_PORT=$BACKEND_PORT
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN cargo install trunk
COPY . .
RUN trunk build --release

FROM nginx:latest AS final
COPY --from=build /app/dist /usr/share/nginx/html