FROM alpine:latest

RUN apk update && apk add --no-cache bash

COPY target/x86_64-unknown-linux-musl/release/url2gif /usr/local/bin/url2gif

ENV PATH="/usr/local/bin:${PATH}"
