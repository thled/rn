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

# set aliases
ENV ENV="/home/rust/.profile"
RUN echo 'echo -e "Welcome! The following aliases exist:\n \
run_lint (rl): Lint the code.\n \
run_fmt (rf): Fix and format the code.\n \
run_test (rt): Run all tests.\n \
run_watch (rw): Watch all tests.\n \
run_ci (rc): Run pipeline steps."' >> ~/.profile \
    && echo 'alias run_lint="cargo check; cargo clippy"' >> ~/.profile \
    && echo 'alias rl=run_lint' >> ~/.profile \
    && echo 'alias run_fmt="cargo fix --allow-no-vcs; cargo fmt"' >> ~/.profile \
    && echo 'alias rf=run_fmt' >> ~/.profile \
    && echo 'alias run_test="cargo test"' >> ~/.profile \
    && echo 'alias rt=run_test' >> ~/.profile \
    && echo 'alias run_watch="cargo watch -cx test -i tests/data"' >> ~/.profile \
    && echo 'alias rw=run_watch' >> ~/.profile \
    && echo 'run_ci() { run_lint; run_test; }' >> ~/.profile \
    && echo 'alias rc=run_ci' >> ~/.profile
