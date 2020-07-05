FROM ubuntu:20.04

LABEL maintainer="Xiangpeng Hao <haoxiangpeng@hotmail.com>"

COPY . /usr/src/pibench-online
WORKDIR /usr/src/pibench-online/backend

ENV TZ="America/Vancouver"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt --assume-yes install git g++ cmake curl \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly 

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release

ENTRYPOINT ["cargo", "run", "--release"]
