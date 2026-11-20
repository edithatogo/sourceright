FROM rust:1.100-bookworm@sha256:ade39a710ef85f3506cffcfec3e8a24ba090badef1dff2fb798f71d878833fea AS build

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
