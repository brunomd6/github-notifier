FROM rust:1.93-bookworm

ENV USER=root

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl wget git python3 python3-pip \
    libudev-dev pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install espup --locked
RUN espup install

RUN cargo install espflash --locked
RUN cargo install ldproxy --locked

# bruno:
# RUN cargo install cargo-generate --locked
RUN cargo install esp-generate --locked
RUN cargo install esp-config --features=tui --locked
RUN rustup component add rustfmt
RUN rustup component add clippy
# end bruno

RUN echo "source /root/export-esp.sh" >> /root/.bashrc

WORKDIR /project
CMD ["/bin/bash"]