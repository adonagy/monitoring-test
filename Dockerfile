FROM debian:10-slim as build-env
USER root
RUN apt-get update && apt-get install -y libssl-dev pkg-config libsodium-dev git curl

# 
# Checkout and compile source code
ARG repository="https://github.com/adonagy/monitoring-test.git"
ARG rust_toolchain="stable-x86_64-unknown-linux-gnu"
ARG SOURCE_BRANCH
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain ${rust_toolchain} -y
ENV PATH=/root/.cargo/bin:$PATH
ENV SODIUM_USE_PKG_CONFIG=1
RUN apt-get install -y clang libclang-dev
RUN git clone ${repository} --branch ${SOURCE_BRANCH-master} && cd monitoring-test && cargo build --release #8

FROM debian:10-slim

USER root
RUN apt-get update && apt-get install -y libssl-dev openssl curl

COPY --from=build-env /monitoring-test/target/release/monitoring-test /monitoring-test

ENTRYPOINT [ "/monitoring-test" ]