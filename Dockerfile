FROM rust:1.94-bookworm AS build

WORKDIR /app
COPY . .
RUN cargo build --release --locked --bin sourceright

FROM debian:bookworm-slim

LABEL org.opencontainers.image.source="https://github.com/edithatogo/sourceright"
LABEL org.opencontainers.image.description="Sourceright MCP stdio server"
LABEL org.opencontainers.image.url="https://github.com/edithatogo/sourceright"
LABEL org.opencontainers.image.version="0.1.19"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
LABEL io.modelcontextprotocol.server.name="io.github.edithatogo/sourceright"

COPY --from=build /app/target/release/sourceright /usr/local/bin/sourceright

ENTRYPOINT ["sourceright"]
CMD ["mcp"]
