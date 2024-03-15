# FROM rust:1.67

# WORKDIR /usr/src/app
# COPY . .

# RUN cargo install --path .

# CMD ["app"]

FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD cargo run