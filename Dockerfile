FROM rust:latest AS builder

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install cargo-generate

ADD gb_core /usr/src/gb/gb_core
ADD gb_wasm /usr/src/gb/gb_wasm

WORKDIR /usr/src/gb/gb_wasm

RUN rustup default nightly
RUN wasm-pack build


FROM node:16 AS web_builder

COPY ./gb_wasm/www/  /home/node/app/
COPY --from=builder /usr/src/gb/gb_wasm/pkg /home/node/pkg
WORKDIR /home/node/app
RUN npm install
RUN npm run build

FROM nginx:alpine
COPY --from=web_builder /home/node/app/dist/. /usr/share/nginx/html/
COPY ./gb_wasm/www/super_mario.gb  /usr/share/nginx/html/super_mario.gb
COPY ./nginx.conf /etc/nginx/conf.d/default.conf