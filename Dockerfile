FROM rust:1.70

SHELL ["/bin/sh", "-c"]
RUN apt update -y && apt upgrade -y

ARG USERNAME=rust
ARG UID=1000
ARG GID=${UID}
ENV APPROOT=/opt/app-root
ENV EXROOT=${APPROOT}/examples
ENV PKGROOT=${APPROOT}/leptos-leaflet-hotline

RUN groupadd --gid ${GID} ${USERNAME} && useradd --uid ${UID} --gid ${GID} -m ${USERNAME}

RUN mkdir -p ${APPROOT} && mkdir -p ${PKGROOT}/src \
    && mkdir -p ${EXROOT}/ssr-example \
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

RUN sed -i 's/site-addr = "127.0.0.1/site-addr = "0.0.0.0/g' ${EXROOT}/ssr-example/Cargo.toml

WORKDIR ${APPROOT}/ssr-example
RUN apt-get update && apt-get install -y ca-certificates curl gnupg
RUN mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
RUN NODE_MAJOR=20 && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
RUN apt-get update -y && apt-get install nodejs -y
RUN npm install -g sass
RUN rustup toolchain install nightly --allow-downgrade
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-generate
RUN cargo install cargo-leptos@0.1.11 wasm-pack
RUN cd ${PKGROOT} && cargo build
RUN cd ${EXROOT}/ssr-example && cargo build

WORKDIR ${EXROOT}/ssr-example/app
EXPOSE 3000 3000
EXPOSE 3001 3001

CMD ["/bin/bash", "-c", "cargo leptos watch"]
