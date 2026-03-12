FROM debian:bookworm-slim

ARG TARGETARCH

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY bin/${TARGETARCH}/rustmail /app/rustmail

RUN mkdir -p /app/db && chmod 755 /app/db

RUN chmod +x /app/rustmail

RUN useradd -m -u 1000 rustmail && \
    chown -R rustmail:rustmail /app

USER rustmail

EXPOSE 3002

VOLUME ["/app/db", "/app/config.toml"]

ENTRYPOINT ["/app/rustmail"]