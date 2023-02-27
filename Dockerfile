FROM rust
WORKDIR /root/development/becho
COPY . .
RUN make install

