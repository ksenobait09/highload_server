FROM ubuntu:18.04
MAINTAINER Altunin Nikita

# sudo docker build -t highload .
# docker run --name db -it -p 5432:5432 highload:latest /bin/bash

RUN apt-get -y update

RUN apt-get install -y curl gcc

RUN curl https://sh.rustup.rs -sSf > installer.sh

RUN sh ./installer.sh -y

WORKDIR /server

COPY . .
RUN rm -rf target

# Объявлем порт сервера
EXPOSE 80
RUN ls -la 
CMD $HOME/.cargo/bin/cargo run
