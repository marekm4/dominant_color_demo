FROM rust:1.58-slim-bullseye as builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
ENV PORT=8080
COPY --from=builder /target/release/dominant_color_demo dominant_color_demo
COPY files files
CMD ["/dominant_color_demo"]
