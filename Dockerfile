FROM rust:latest

WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Бінарник буде тут: /app/target/x86_64-unknown-linux-gnu/release/oxford_dictionary_bot
