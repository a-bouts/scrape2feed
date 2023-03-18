FROM rust as builder
WORKDIR /usr/src/scrape2feed

COPY Cargo.toml Cargo.lock ./
RUN rustup default nightly

COPY src ./src
COPY migrations ./migrations
RUN cargo install --path .


FROM node as static-builder
WORKDIR /app

COPY web/package*.json ./
RUN yarn config set network-timeout 300000
RUN yarn install
COPY web/ .
RUN yarn build


FROM rust
VOLUME /var/lib/scrape2feed
ENV DATABASE_URL=/var/lib/scrape2feed/scrape2feed.db
USER 1000

COPY --from=builder /usr/local/cargo/bin/scrape2feed .
COPY --from=static-builder /app/dist /static

CMD ["./scrape2feed"]
