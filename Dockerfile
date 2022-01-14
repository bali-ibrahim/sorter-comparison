
# run with:
# docker build . -t rust_cross_compile/windows && docker run --rm -ti -v `pwd`:/app rust_cross_compile/windows
FROM rust:slim-buster
RUN apt update && apt upgrade -y
RUN apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu
WORKDIR /app
CMD ["cargo", "install", "--target", "x86_64-pc-windows-gnu", "--path", ".", "--root", "."]
