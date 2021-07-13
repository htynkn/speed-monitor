FROM ekidd/rust-musl-builder:1.51.0 AS BUILDER

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:3

RUN apk add curl && mkdir /data

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/speed-monitor \
    /usr/local/bin/

HEALTHCHECK --interval=5s --timeout=3s CMD curl -fs http://localhost:8080/health || exit 1

ENV WEB_PORT=8080
VOLUME /data
EXPOSE 8080

ENTRYPOINT ["speed-monitor"]
