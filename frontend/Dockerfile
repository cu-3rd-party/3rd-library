# syntax=docker/dockerfile:1.7
FROM node:22-alpine AS build
ARG PNPM_VERSION=11.1.2
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
ENV CI=true
RUN corepack enable && corepack prepare pnpm@${PNPM_VERSION} --activate
RUN --mount=type=cache,target=/pnpm/store \
    pnpm config set store-dir /pnpm/store && \
    pnpm install --frozen-lockfile
COPY . .
ARG VITE_API_URL=http://localhost:5173
ENV VITE_API_URL=${VITE_API_URL}
RUN pnpm build

FROM nginx:alpine
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80 5173
CMD ["nginx", "-g", "daemon off;"]
