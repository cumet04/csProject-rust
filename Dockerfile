FROM rust:1.43

RUN apt-get update && apt-get install -y \
    cmake \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    mingw-w64 \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

# app user
RUN mkdir /home/app /app && \
    useradd app -d /home/app -s /bin/bash && \
    chown app:app /home/app /app
USER app
WORKDIR /app

# for windows build
RUN rustup target add x86_64-pc-windows-gnu

# rust tools
RUN rustup component add rustfmt && \
    cargo install cargo-edit
