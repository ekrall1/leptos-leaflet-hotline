FROM rust:1.70

SHELL ["/bin/sh", "-c"]

ARG USERNAME=rust UID=1000 GID=1000
ENV APPROOT=/opt/app-root EXROOT=/opt/app-root/examples PKGROOT=/opt/app-root/leptos-leaflet-hotline

RUN apt update -y && apt upgrade -y && \
    groupadd --gid ${GID} ${USERNAME} && useradd --uid ${UID} --gid ${GID} -m ${USERNAME} && \
    mkdir -p ${APPROOT} ${PKGROOT}/src \
    ${EXROOT}/ssr-example \
    ${EXROOT}/ssr-example/app ${EXROOT}/ssr-example/server ${EXROOT}/ssr-example/frontend \
    ${EXROOT}/ssr-example/style ${EXROOT}/ssr-example/public

COPY --chown=${UID}:${GID} ./leptos-leaflet-hotline/Cargo.toml \
    ./leptos-leaflet-hotline/Cargo.lock ${PKGROOT}
COPY --chown=${UID}:${GID} ./leptos-leaflet-hotline/src/*.rs ${PKGROOT}/src/
COPY --chown=${UID}:${GID} ./examples/ssr-example/Cargo.* ./examples/ssr-example/rust-toolchain.toml ${EXROOT}/ssr-example/
COPY --chown=${UID}:${GID} ./examples/ssr-example/app/Cargo.toml ${EXROOT}/ssr-example/app/
COPY --chown=${UID}:${GID} ./examples/ssr-example/app/src/*.rs ${EXROOT}/ssr-example/app/src/
COPY --chown=${UID}:${GID} ./examples/ssr-example/server/Cargo.toml ${EXROOT}/ssr-example/server/
COPY --chown=${UID}:${GID} ./examples/ssr-example/server/src/*.rs ${EXROOT}/ssr-example/server/src/
COPY --chown=${UID}:${GID} ./examples/ssr-example/frontend/Cargo.toml ${EXROOT}/ssr-example/frontend/
COPY --chown=${UID}:${GID} ./examples/ssr-example/frontend/src/*.rs ${EXROOT}/ssr-example/frontend/src/
COPY --chown=${UID}:${GID} ./examples/ssr-example/style/*.scss ${EXROOT}/ssr-example/style/
COPY --chown=${UID}:${GID} ./examples/ssr-example/public/favicon.ico ${EXROOT}/ssr-example/public/

WORKDIR ${APPROOT}/ssr-example
RUN sed -i 's/site-addr = "127.0.0.1/site-addr = "0.0.0.0/g' ${EXROOT}/ssr-example/Cargo.toml && \
    apt-get update && apt-get install -y ca-certificates curl gnupg && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    NODE_MAJOR=20 && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list && \
    apt-get update -y && apt-get install nodejs -y && \
    npm install -g sass && \
    rustup toolchain install nightly --allow-downgrade && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install cargo-generate && \
    cargo install cargo-leptos@0.1.11 wasm-pack && \
    cd ${PKGROOT} && cargo build && \
    cd ${EXROOT}/ssr-example && cargo build

WORKDIR ${EXROOT}/ssr-example/app
EXPOSE 3000 3000
EXPOSE 3001 3001

RUN chown 1000:1000 -R ${APPROOT} && chmod -R 774 ${APPROOT} && \
    chown 1000:1000 -R /usr/local/cargo/registry && chmod -R 774 /usr/local/cargo/registry
USER ${USERNAME}

CMD ["/bin/bash", "-c", "cargo leptos watch"]
