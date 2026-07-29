FROM rust:1.97-bookworm@sha256:77fac8b98f9f46062bb680b6d25d5bcaabfc400143952ebc572e924bcbedc3fa AS build

WORKDIR /app
COPY . .
RUN cargo build --release --locked --bin sourceright

FROM debian:bookworm-slim@sha256:67b30a61dc87758f0caf819646104f29ecbda97d920aaf5edc834128ac8493d3

LABEL org.opencontainers.image.source="https://github.com/edithatogo/sourceright"
LABEL org.opencontainers.image.description="Sourceright MCP stdio server"
LABEL org.opencontainers.image.url="https://github.com/edithatogo/sourceright"
LABEL org.opencontainers.image.version="0.1.20"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
LABEL io.modelcontextprotocol.server.name="io.github.edithatogo/sourceright"

COPY --from=build /app/target/release/sourceright /usr/local/bin/sourceright

ENTRYPOINT ["sourceright"]
CMD ["mcp"]
