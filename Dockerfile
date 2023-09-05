FROM rust:1.70

SHELL ["/bin/sh", "-c"]

ARG USERNAME=rust
ARG UID=1000
ARG GID=${UID}
ENV APPROOT=/opt/app-root
ENV EXROOT=${APPROOT}/examples
ENV PKGROOT=${APPROOT}/leptos-leaflet-hotline

RUN groupadd --gid ${GID} ${USERNAME} && useradd --uid ${UID} --gid ${GID} -m ${USERNAME}

RUN mkdir -p ${APPROOT} && mkdir -p ${PKGROOT}/src \
    && mkdir -p ${EXROOT}/ssr-example \
    ${EXROOT}/ssr-example/app ${EXROOT}/ssr-example/server ${EXROOT}/ssr-example/frontend

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

WORKDIR ${APPROOT}/ssr-example
RUN rustup toolchain install nightly --allow-downgrade
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-generate
RUN cargo install cargo-leptos
RUN npm install -g sass
RUN cd ${PKGROOT} && cargo build
RUN cd ${EXROOT}/ssr-example && cargo build

WORKDIR ${EXROOT}/ssr-example
EXPOSE 3000 3000

CMD ["cargo leptos build --release"]
