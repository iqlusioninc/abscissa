FROM centos:7.4.1708

# Install/update RPMs
RUN yum update -y && yum groupinstall -y "Development Tools"

ENV PATH "$PATH:/root/.cargo/bin"
ENV RUSTUP_INIT_VERSION "2018-02-13"
ENV RUSTUP_INIT "rustup-init-$RUSTUP_INIT_VERSION"
ENV RUSTUP_INIT_DIGEST "ad0dd8442b61faa319e9fe29108535359f6318744a800fac1e76118bbad81d2b"
ENV RUST_NIGHTLY_VERSION "nightly-2018-03-05"

WORKDIR /root

# Install Rust
RUN curl -O https://storage.googleapis.com/iqlusion-prod-artifacts/rust/$RUSTUP_INIT.bz2
RUN echo "$RUSTUP_INIT_DIGEST $RUSTUP_INIT.bz2" | sha256sum -c
RUN bunzip2 $RUSTUP_INIT.bz2 && chmod +x $RUSTUP_INIT
RUN ./$RUSTUP_INIT -y
RUN rustup install $RUST_NIGHTLY_VERSION

RUN bash -l -c "echo $(rustc --print sysroot)/lib >> /etc/ld.so.conf"
RUN ldconfig

ENV RUSTFMT_NIGHTLY_VERSION "0.4.0"
RUN rustup run $RUST_NIGHTLY_VERSION cargo install rustfmt-nightly --vers $RUSTFMT_NIGHTLY_VERSION --force

ENV CLIPPY_VERSION "0.0.187"
RUN rustup run $RUST_NIGHTLY_VERSION cargo install clippy --vers $CLIPPY_VERSION --force
