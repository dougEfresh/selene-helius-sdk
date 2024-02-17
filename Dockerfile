FROM rust:1-bookworm as builder

WORKDIR /usr/src/selene-helius-bot
COPY . .
RUN cd bot && cargo install --path . --root /usr

FROM debian:bookworm

RUN apt-get update -y \
  && apt-get install -y curl   \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
  
COPY --from=builder /usr/bin/selene-helius-bot /usr/bin/selene-helius-bot
EXPOSE 3030
RUN selene-helius-bot version
CMD ["selene-helius-bot"]

