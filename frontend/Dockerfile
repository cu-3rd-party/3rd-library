# !IMPORTANT
FROM node:22-alpine AS build
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm@latest && \
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
