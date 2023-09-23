FROM rust:alpine as build
WORKDIR /usr/src/app

RUN apk add libressl-dev libc-dev

COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /root

COPY --from=build /usr/src/app/target/release/centerdash-api ./

EXPOSE 8080

CMD ["./centerdash-api"]