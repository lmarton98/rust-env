FROM rust:latest
RUN mkdir -p /root/.ssh
RUN apt-get update -y && export DEBIAN_FRONTEND=noninteractive
RUN apt-get install -y --no-install-recommends git openssh-client vim && \
    apt-get clean && \
    rm -r /var/cache/* /var/lib/apt/lists/*
RUN rustup component add rustfmt clippy
RUN rustup completions bash >> "/root/.bashrc"
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/root/.history/.bash_history" && echo $SNIPPET >> "/root/.bashrc"