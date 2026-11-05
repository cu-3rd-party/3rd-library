# !IMPORTANT
FROM node:22-alpine AS build
WORKDIR /app
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN corepack enable && corepack prepare pnpm@11.1.2 --activate

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
# Пусто — API на том же домене (/api), как в self-service; локально — адрес бэкенда
ARG VITE_API_URL=""
ENV VITE_API_URL=${VITE_API_URL}
RUN pnpm build

FROM nginx:alpine
COPY --chmod=644 nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
