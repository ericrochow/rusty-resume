################################################################################
#                               Build stage                                    #
################################################################################
FROM rust:latest as builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new rusty-resume
WORKDIR /usr/src/rusty-resume
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
COPY templates ./templates
RUN cargo install --target x86-unknown-linux-musl --path .

################################################################################
#                             Final build stage                                #
################################################################################
FROM scratch as final-build
COPY --from=builder /usr/local/cargo/bin/rusty-resume .
USER 1000
CMD ["./rusty-resume"]