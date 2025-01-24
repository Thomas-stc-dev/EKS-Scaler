FROM lambci/lambda:build-provided.al2


ARG RUST_VERSION=1.55.0

RUN yum install -y jq openssl-devel

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
| CARGO_HOME=/cargo RUSTUP_HOME=/rustup sh -s -- -y --profile minimal --default-toolchain $RUST_VERSION --target x86_64-unknown-linux-gnu

ADD build.sh /usr/local/bin/

VOLUME ["/code"]

WORKDIR /code

RUN ["chmod", "+x", "/usr/local/bin/build.sh"]

ENTRYPOINT ["/usr/local/bin/build.sh"]