FROM ubuntu:20.04 AS compiler

# Install prereqisite packages for building openssl
RUN apt-get update && apt-get install -y \
    build-essential \
    gcc-aarch64-linux-gnu \
    curl

WORKDIR /root

# Get openssl source
RUN curl https://www.openssl.org/source/openssl-1.1.1e.tar.gz -o openssl-1.1.1e.tar.gz \
    && tar zxf openssl-1.1.1e.tar.gz \
    && rm openssl-1.1.1e.tar.gz

WORKDIR /root/openssl-1.1.1e

# Build openssl for aarch64
RUN ./Configure linux-aarch64 \
        --cross-compile-prefix=/bin/aarch64-linux-gnu- \
        --prefix=/opt/openssl-1.1.1e-aarch64 \
        --openssldir=/opt/openssl-1.1.1e-aarch64 \
        -static \
    && make install

FROM ubuntu:20.04

WORKDIR /root

# Install prereqisite packages for building rust binaries
RUN apt-get update && apt-get install -y \
    gcc \
    gcc-aarch64-linux-gnu \
    git \
    curl

# Get our prebuild aarch64 openssl library
COPY --from=compiler /opt/openssl-1.1.1e-aarch64 /opt/openssl-1.1.1e-aarch64

# Install minimal rust tools to target aarch64
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal \
    && /root/.cargo/bin/rustup target add aarch64-unknown-linux-gnu

# Configure the aarch64 linker for cargo
COPY ./config.toml .cargo

ENV PATH="/root/.cargo/bin:$PATH" \
    # Fix cargo/rustup home directories to be independant of $HOME
    CARGO_HOME="/root/.cargo" \
    RUSTUP_HOME="/root/.rustup" \
    # Points openssl-sys crate to our openssl installation
    OPENSSL_DIR="/opt/openssl-1.1.1e-aarch64"

CMD ["/bin/bash"]
