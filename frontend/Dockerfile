# Production image: static Vite build served by nginx, which also proxies /api to the backend.
# Built by CI (frontend-ci.yml) and scripts/ci/frontend/build-image.sh.
# The dev-server image for local compose is Dockerfile.dev.
# Build: docker build -t audio-dsp-web .   (context = frontend/)

FROM node:20-alpine AS build
WORKDIR /app
RUN npm install -g pnpm@10

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
# Same-origin API: the browser calls /api/..., nginx forwards it. Keeps one image for every env.
ARG VITE_API_BASE_URL=/api
ENV VITE_API_BASE_URL=${VITE_API_BASE_URL}
RUN pnpm build

FROM nginx:1.27-alpine AS runtime
# Backend address inside the cluster; override per environment in the deployment manifest.
ENV API_UPSTREAM=http://rust-api:3080
COPY nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 8080
