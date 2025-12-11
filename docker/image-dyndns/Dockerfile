FROM debian:13-slim
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y locales locales-all
ENV LANG=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8
ENV LC_CTYPE=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8
ENV LANGUAGE=en_US.UTF-8

ENV DOMAIN_NAME=example.com
ENV CONFIG_FILE_PATH=/config.yaml
ENV INTERVAL=1800

RUN echo "LC_ALL=en_US.UTF-8" >> /etc/environment
RUN apt-get update && apt-get upgrade -y
RUN apt-get update && apt-get install -y curl tmux nano net-tools procps htop git build-essential cmake pkg-config libssl-dev gosu

COPY entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

ARG USER_ID=3883
ARG GROUP_ID=3883
RUN groupadd --gid ${GROUP_ID} runner
RUN useradd -m -u ${USER_ID} -g runner -s /bin/false runner
RUN passwd -l runner

USER runner

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cd /home/runner/
WORKDIR /home/runner/

ENV PATH="/home/runner/.cargo/bin:$PATH"
RUN echo "export /home/runner/.cargo/bin:$PATH" > /home/runner/.bashrc

COPY ../../ /home/runner/sam-dns-tools/
RUN cd /home/runner/sam-dns-tools/
WORKDIR /home/runner/sam-dns-tools/
RUN (cargo update && cargo build --all) || (git checkout Cargo.lock && cargo build --all --locked)

RUN mkdir /home/runner/app/
RUN cp /home/runner/sam-dns-tools/target/debug/sam-dns-tools /home/runner/app/
RUN cd /home/runner/app/
WORKDIR /home/runner/app/

# Clean up the source and build dirs
RUN rm -rf /home/runner/sam-dns-tools/

USER root

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

CMD ["sh", "-c", "while true; do \
    /home/runner/app/sam-dns-tools dyndns \
    --account-domain-name=\"$DOMAIN_NAME\" \
    ${PROXY:+--subdomain=$SUBDOMAIN} \
    ${PROXY:+--proxy=$PROXY} \
    --config-file-path \"$CONFIG_FILE_PATH\"; \
    sleep \"$INTERVAL\"; \
    done"]
