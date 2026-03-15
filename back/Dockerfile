# Этап сборки
FROM rust:1.91-alpine AS builder

# Устанавливаем необходимые системные зависимости
# RUN apk add --no-cache \
#   pkgconfig \
#   openssl-dev \
#   ca-certificates \
#   clang \
#   && rm -rf /var/cache/apk/*
RUN apk add --no-cache \
  musl-dev \
  pkgconf \
  openssl-dev \
  openssl-libs-static \
  ca-certificates \
  build-base \
  curl \
  && rm -rf /var/cache/apk/*


WORKDIR /app

ENV DATABASE_URL=sqlite:file:data/database.db

# Копируем только файлы зависимостей
COPY Cargo.toml Cargo.lock ./
# COPY database.db database.db
# Создаем фиктивный main.rs для предварительной компиляции зависимостей

# Создаем пустой проект для кэширования зависимостей
# RUN mkdir src && \
#   echo "fn main() {}" > src/main.rs && \
#   cargo build --release && \
#   rm -rf src
RUN cargo fetch
COPY data data/
COPY src src/
COPY .sqlx .sqlx/
COPY migrations migrations/
# Копируем исходный код
# COPY src src/
# COPY migrations migrations/
# COPY templates ./templates
# Пересобираем приложение (только наш код, зависимости уже закэшированы)
# RUN cargo build --release --frozen
# RUN cargo build --release --frozen
RUN cargo build --release --locked --no-default-features
# RUN cargo build --release --frozen
# Второй этап: минимальный runtime образ

FROM alpine:latest
# Устанавливаем необходимые библиотеки
RUN apk add --no-cache libgcc openssl ca-certificates
# Создаем пользователя для безопасности
# RUN addgroup -g 1000 -S appuser && \
# adduser -u 1000 -S appuser -G ar
WORKDIR /app

# RUN mkdir data && touch data/database.db
# Копируем бинарник из стадии сборки
# COPY --from=builder /app/target/release/back /usr/local/bin/back
COPY --from=builder /app/target/release/app .
COPY --from=builder /app/data/database.db data/database.db
# Копируем миграции
# COPY --from=builder /app/migrations .
# Создаем директорию для базы данных
# RUN mkdir -p /app/data && chown -R appuser:appuser /app
# RUN mkdir -p data
# Переключаемся на непривилегированного пользователя
# USER appuser
# Открываем порт
EXPOSE 5005
# Запускаем приложение
CMD ["./app"]
