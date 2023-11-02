FROM rust:1.73.0-bullseye

WORKDIR /usr/src/avatarapi
COPY . .

RUN apt update
RUN apt install sqlite3 

RUN ./sqlite/setup.sh

ENV DATABASE_URL "sqlite://quotes.db"

RUN cargo build --release
CMD ["./target/release/avatarapi"]
