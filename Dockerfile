FROM rust
COPY . /tmp/app
WORKDIR /tmp/app
RUN cargo install --path .