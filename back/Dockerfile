# Этап сборки
FROM rust:1.91-slim-bookworm AS builder

# ENV DATABASE_URL=file:sqlite:./database.db
# Устанавливаем необходимые системные зависимости
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Копируем только файлы зависимостей
COPY Cargo.toml Cargo.lock ./
COPY database.db ./database.db
# Создаем фиктивный main.rs для предварительной компиляции зависимостей
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs
# Компилируем зависимости (это будет закэшировано)
RUN cargo build --release
# Удаляем фиктивный main.rs
RUN rm -rf src

# Копируем исходный код
COPY src ./src
# COPY templates ./templates

# Пересобираем приложение (только наш код, зависимости уже закэшированы)
RUN cargo build --release --frozen

# Второй этап: минимальный runtime образ
FROM debian:bookworm-slim

# Устанавливаем runtime зависимости
RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
  && rm -rf /var/lib/apt/lists/*

# Создаем не-root пользователя для безопасности
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Копируем собранный бинарник из builder этапа
COPY --from=builder /app/target/release/back /app/back
COPY --from=builder /app/target/release/back /app/back
# COPY --from=builder /app/dar /app/back

# COPY --from=builder /app/database.db /app/database.db
# Копируем статические файлы если есть
# COPY --chown=appuser:appuser static ./static
# COPY --chown=appuser:appuser templates ./templates
COPY --chown=appuser:appuser database.db ./database.db


# Меняем владельца
RUN chown -R appuser:appuser /app

# Переключаемся на не-root пользователя
USER appuser

# Экспонируем порт
EXPOSE 5005


CMD ["./back"]
