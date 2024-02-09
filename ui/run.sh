#!/bin/bash

exec docker run --rm \
  --user "$(id -u):$(id -g)" \
  -p 5173:5173 \
  -it \
  -v "${PWD}":/staex-iod \
  --entrypoint="" \
  --workdir /staex-iod \
  oven/bun "$@"
