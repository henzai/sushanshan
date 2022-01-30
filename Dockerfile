FROM rust:1.58
WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .
CMD ["sushanshan"]