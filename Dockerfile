FROM rust
RUN apt update; apt upgrade -y
RUN apt install -y sudo
WORKDIR /root/development/becho
COPY . .
RUN make install
