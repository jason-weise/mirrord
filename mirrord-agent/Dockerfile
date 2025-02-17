FROM rustlang/rust:nightly as build-env
RUN apt update && apt install -y libpcap-dev cmake
WORKDIR /app
COPY Cargo.toml Cargo.lock CHANGELOG.md README.md LICENSE rust-toolchain.toml /app/
COPY sample/rust /app/sample/rust
COPY mirrord-auth /app/mirrord-auth
COPY mirrord-config /app/mirrord-config
COPY mirrord-protocol /app/mirrord-protocol
COPY mirrord-agent /app/mirrord-agent
COPY mirrord-macro /app/mirrord-macro
COPY mirrord-layer /app/mirrord-layer
COPY mirrord-cli /app/mirrord-cli
COPY tests /app/tests
COPY .cargo /app/.cargo
RUN cargo +nightly build -Z bindeps --manifest-path /app/mirrord-agent/Cargo.toml --release

FROM debian:stable
COPY --from=build-env /app/target/release/mirrord-agent /
RUN apt update && apt install -y libpcap-dev iptables
RUN update-alternatives --set iptables /usr/sbin/iptables-legacy \
    && update-alternatives --set ip6tables /usr/sbin/ip6tables-legacy

CMD ["./mirrord-agent"]