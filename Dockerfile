FROM rust:latest
MAINTAINER Altunin Nikita

# sudo docker build -t highload .
# docker run --name db -it -p 5432:5432 highload:latest /bin/bash

WORKDIR /server
COPY . .

RUN cargo install --path .

# Объявлем порт сервера
EXPOSE 80

CMD ["highload_server"]
