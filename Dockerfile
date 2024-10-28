FROM lukemathwalker/cargo-chef:latest-rust-1.81.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build our project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json
# up to this point, if our dependency tree stays the same, all layers should be cached
COPY . .
ENV SQLX_OFFLINE true
# build our project
RUN cargo build --release --bin zero2prod

# ---

FROM debian:bookworm-slim AS runtime
WORKDIR /app
# install OpenSSL - it is dynamically linked by some of our dependencies
# install ca-certificates - it is needed to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# copy the build artifact from the builder stage
COPY --from=builder /app/target/release/zero2prod zero2prod
# we need config files in runtime
COPY config config
ENV APP_ENVIRONMENT prod
ENTRYPOINT ["./zero2prod"]