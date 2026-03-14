# Этап сборки
FROM rust:1.91-slim-bookworm AS builder

# Устанавливаем необходимые системные зависимости
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

ENV DATABASE_URL=sqlite:///app/data/database.db

WORKDIR /app
# Копируем только файлы зависимостей
COPY Cargo.toml Cargo.lock ./
# COPY database.db database.db
# Создаем фиктивный main.rs для предварительной компиляции зависимостей
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs

# Создаем пустой проект для кэширования зависимостей
RUN mkdir src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

# Копируем исходный код
COPY src ./src
COPY migrations ./migrations
# COPY templates ./templates
# Пересобираем приложение (только наш код, зависимости уже закэшированы)
# RUN cargo build --release --frozen
RUN cargo build --release --frozen

# Второй этап: минимальный runtime образ

FROM alpine:latest
# Устанавливаем необходимые библиотеки
RUN apk add --no-cache libgcc openssl
# Создаем пользователя для безопасности
# RUN addgroup -g 1000 -S appuser && \
# adduser -u 1000 -S appuser -G appuser
# Копируем бинарник из стадии сборки
# COPY --from=builder /app/target/release/back /usr/local/bin/back
COPY --from=builder /app/target/release/back /app/back
# Копируем миграции
COPY --from=builder /app/migrations /app/migrations
# Создаем директорию для базы данных
# RUN mkdir -p /app/data && chown -R appuser:appuser /app
RUN mkdir -p /app/data
# Переключаемся на непривилегированного пользователя
# USER appuser
WORKDIR /app
# Открываем порт
EXPOSE 5005
# Запускаем приложение
CMD ["./back"]
