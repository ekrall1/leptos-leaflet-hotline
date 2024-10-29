FROM rust:1.76

SHELL ["/bin/sh", "-c"]

ARG USERNAME=rust
ARG UID=1000
ARG GID=1000
ENV APPROOT=/opt/app-root EXROOT=/opt/app-root/examples PKGROOT=/opt/app-root/leptos-leaflet-hotline

RUN apt update -y && apt upgrade -y && apt clean -y && \
    groupadd --gid ${GID} ${USERNAME} && useradd --uid ${UID} --gid ${GID} -m ${USERNAME} && \
    mkdir -p ${APPROOT} ${PKGROOT}/src \
    ${EXROOT}/ssr-example \
    ${EXROOT}/ssr-example/app ${EXROOT}/ssr-example/server ${EXROOT}/ssr-example/frontend \
    ${EXROOT}/ssr-example/style ${EXROOT}/ssr-example/public && \
    chown ${UID}:${GID} -R ${APPROOT} && chmod -R 774 ${APPROOT}

COPY --chown=${UID}:${GID} ./leptos-leaflet-hotline/ /${PKGROOT}/
COPY --chown=${UID}:${GID} ./examples/ /${EXROOT}/

RUN cd ${EXROOT}/ssr-example && \
    sed -i 's/site-addr = "127.0.0.1/site-addr = "0.0.0.0/g' ${EXROOT}/ssr-example/Cargo.toml && \
    apt-get update && apt-get install -y ca-certificates curl gnupg && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    NODE_MAJOR=20 && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list && \
    apt-get update -y && apt-get install nodejs -y && apt-get clean -y && \
    npm install -g sass && npm cache clean \
    rustup toolchain install nightly --allow-downgrade && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install cargo-generate cargo-leptos@0.2.7 wasm-pack && rm -rf /usr/local/cargo/registry/** &&\
    chown 1000:1000 -R /usr/local/cargo/registry && chmod -R 774 /usr/local/cargo/registry

WORKDIR ${EXROOT}/ssr-example/app
EXPOSE 3000 3000
EXPOSE 3001 3001

USER ${USERNAME}

CMD ["/bin/bash", "-c", "cargo update -p time && cargo leptos watch"]
