#!/bin/bash
# docker build -t builder .
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp builder cargo +nightly test --release