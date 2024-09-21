# Build
FROM docker.io/library/rust:1.79.0-bullseye as builder

WORKDIR /app
ADD . /app

RUN apt update
RUN apt install sqlite3 

RUN ./sqlite/setup.sh

ENV QUOTES_DATABASE_URL "sqlite://quotes.db"

RUN cargo build --release

# Run
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/avatarapi /
COPY --from=builder /app/quotes.db /
ENV QUOTES_DATABASE_URL "sqlite://quotes.db"
CMD ["./avatarapi"]
