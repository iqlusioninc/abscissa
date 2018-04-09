# Rust CI Dockerfile (iqlusion)
#
# Resulting image is published as iqlusion/rust-ci on Docker Hub

FROM centos:7.4.1708

# Include cargo in the path
ENV PATH "$PATH:/root/.cargo/bin"

# Install/update RPMs
RUN yum update -y && \
    yum groupinstall -y "Development Tools" && \
    yum install -y centos-release-scl epel-release openssl-devel rpm-devel xz-devel && \
    yum install -y --enablerepo=epel libsodium-devel && \
    yum install -y --enablerepo=centos-sclo-rh llvm-toolset-7

# rustup configuration
ENV RUSTUP_INIT_VERSION "2018-02-13"
ENV RUSTUP_INIT "rustup-init-$RUSTUP_INIT_VERSION"
ENV RUSTUP_INIT_DIGEST "d8823472cd91d102bb469dec4d05bc8808116cd5c8ac51d87685687d6c124757"

# Install rustup
WORKDIR /root
RUN curl -O https://storage.googleapis.com/iqlusion-prod-artifacts/rust/$RUSTUP_INIT.xz
RUN echo "$RUSTUP_INIT_DIGEST $RUSTUP_INIT.xz" | sha256sum -c
RUN unxz $RUSTUP_INIT.xz && chmod +x $RUSTUP_INIT
RUN ./$RUSTUP_INIT -y

# Rust nightly version to install
ENV RUST_NIGHTLY_VERSION "nightly-2018-04-05"

# Install Rust nightly
RUN rustup install $RUST_NIGHTLY_VERSION

RUN bash -l -c "echo $(rustc --print sysroot)/lib >> /etc/ld.so.conf"
RUN ldconfig

ENV RUSTFMT_NIGHTLY_VERSION "0.4.1"
RUN rustup run $RUST_NIGHTLY_VERSION cargo install rustfmt-nightly --vers $RUSTFMT_NIGHTLY_VERSION --force

ENV CLIPPY_VERSION "0.0.192"
RUN rustup run $RUST_NIGHTLY_VERSION cargo install clippy --vers $CLIPPY_VERSION --force

# Set environment variables to enable SCL packages (llvm-toolset-7)
ENV LD_LIBRARY_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64
ENV PATH "/opt/rh/llvm-toolset-7/root/usr/bin:/opt/rh/llvm-toolset-7/root/usr/sbin:$PATH"
ENV PKG_CONFIG_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64/pkgconfig
ENV X_SCLS llvm-toolset-7

# Configure RUSTFLAGS
ENV RUSTFLAGS "-Ctarget-feature=+aes"
