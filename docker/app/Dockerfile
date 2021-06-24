FROM rust:1.53-alpine3.13

ENV USER=rust

# install utilities
RUN apk add --no-cache \
    # needed by cargo-watch
    libc-dev ncurses

# create user
RUN adduser -D -g '' rust
USER rust

# install rust tools
RUN rustup component add \
    # formatting
    rustfmt \
    # linting
    clippy \
    # test watcher
    && cargo install cargo-watch

WORKDIR /app

