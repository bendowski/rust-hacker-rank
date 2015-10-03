FROM buildpack-deps:stretch-scm

ENV USER root
ENV RUST_VERSION=1.3.0

RUN curl -sO https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
  rm -rf \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /tmp/* \
    /var/tmp/* && \
  mkdir /source

RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  build-essential \
  cmake \
  libedit-dev \
  llvm-3.7  \
  llvm-3.7-dev \
  zlib1g-dev \
  && \
  update-alternatives --install /usr/bin/llvm-config llvm-config /usr/bin/llvm-config-3.7 200 && \
  rm -rf \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/*

VOLUME ["/source"]
WORKDIR /source
CMD ["/bin/bash"]
