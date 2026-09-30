FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    lld \
    make \
    nasm \
    binutils \
    wget \
    curl \
    git \
    xorriso \
    qemu-system-x86 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /work

CMD ["/bin/bash"]
