FROM rust:1.77.2-alpine3.19 as builder
RUN apk add --update alpine-sdk
WORKDIR /build
COPY ./peaq-client /build/peaq-client
COPY ./peaq-gen /build/peaq-gen
COPY ./provisioner /build/provisioner
COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock
RUN cargo build --package provisioner


FROM alpine:3.19
COPY --from=builder /build/target/debug/provisioner /iod/provisioner
ENTRYPOINT ["/iod/provisioner"]
