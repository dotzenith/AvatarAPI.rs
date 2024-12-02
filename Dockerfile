# Build
FROM docker.io/library/rust:1.82.0-bullseye as builder

WORKDIR /app
ADD . /app

RUN cargo build --release

# Run
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/avatarapi /
COPY --from=builder /app/submodules/AvatarApi/Quotes.csv /
CMD ["./avatarapi", "Quotes.csv"]
