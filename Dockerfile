FROM nginx:alpine

WORKDIR /

# Nginx & NodeJS
RUN apk update && apk add --no-cache --update nginx
RUN apk add nodejs npm

# Rust deps
RUN apk add curl && apk add gcc
RUN curl -proto '=https' -tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

COPY . .

RUN npm install --global pnpm
RUN pnpm install
RUN pnpm run build

COPY ./nginx.conf /etc/nginx/nginx.conf