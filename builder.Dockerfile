FROM docker.io/rust:slim
RUN rm -rf /etc/apt/sources.list.d/* && \
    echo "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ stable main contrib non-free" > /etc/apt/sources.list && \
    echo "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ stable-updates main contrib non-free" >> /etc/apt/sources.list && \
    echo "deb http://mirrors.tuna.tsinghua.edu.cn/debian-security stable-security main contrib non-free" >> /etc/apt/sources.list
RUN apt-get update && apt-get install libssl-dev curl pkg-config -y && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN rustup add component rustfmt clippy && \
    cargo install cargo-audit cargo-tarpaulin && \
    rm -rf ~/.cargo/registry