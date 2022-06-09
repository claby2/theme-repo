###########
# Backend #
###########
FROM rust:1.60.0-alpine AS backend-build

WORKDIR /usr/src/backend
COPY ./backend .

RUN apk add --no-cache musl-dev
RUN cargo install --path .


############
# Frontend #
############
FROM node:18.1-alpine AS frontend-build

WORKDIR /usr/src/frontend
COPY ./frontend .
RUN yarn install

ARG PUBLIC_BACKEND_URL
ENV PUBLIC_BACKEND_URL=$PUBLIC_BACKEND_URL

RUN yarn build


##############
# Deployment #
##############
FROM nginx:1.21-alpine

# Copy from backend stage
COPY --from=backend-build /usr/local/cargo/bin/theme-repo-backend /usr/local/bin/theme-repo-backend
COPY --from=backend-build /usr/src/backend/themes /usr/src/backend/themes
COPY --from=backend-build /usr/src/backend/templates /usr/src/backend/templates
VOLUME /usr/src/backend/themes
VOLUME /usr/src/backend/templates

# Copy from frontend stage
COPY --from=frontend-build /usr/src/frontend/dist /usr/share/nginx/html

COPY ./nginx.default.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

COPY ./entrypoint.sh /usr/src
ENTRYPOINT ["/usr/src/entrypoint.sh"]
