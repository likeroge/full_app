FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
# Если используете pnpm/yarn: замените на соответствующие команды
RUN npm ci --silent

COPY . .

RUN npm run build

FROM nginx:1.24-alpine

COPY --from=builder /app/dist /usr/share/nginx/html

# Копируем кастомную конфигурацию nginx
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
