# Меняем rust:1.85-slim на rust:latest
FROM rust:latest as builder

WORKDIR /app

# Устанавливаем системные зависимости
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Копируем исходники
COPY . .

# Собираем релиз
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Минимальный финальный образ
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/imperia_pizza_back /app/imperia_pizza_back

EXPOSE 8080

CMD ["./imperia_pizza_back"]