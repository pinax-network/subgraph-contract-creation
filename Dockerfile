# Use an official Node.js image as base
FROM node:18

# Set a working directory
WORKDIR /app

# Install necessary dependencies, including graph-cli
RUN npm install -g @graphprotocol/graph-cli

# Install other essential packages
RUN apt-get update && apt-get install -y \
    git \
    curl \
    wget \
    tar \
    make \
    && rm -rf /var/lib/apt/lists/*

# Download and install the latest Substreams CLI release
RUN curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | \
    grep "browser_download_url.*linux_x86_64.tar.gz" | \
    cut -d '"' -f 4 | \
    wget -qi - && \
    tar -xzf substreams_linux_x86_64.tar.gz && \
    mv substreams /usr/local/bin/substreams && \
    rm substreams_linux_x86_64.tar.gz

# Install Buf CLI
RUN npm install -g @bufbuild/buf

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install target for substreams
RUN rustup target add wasm32-unknown-unknown

# Clone the subgraph repository
RUN git clone https://github.com/pinax-network/subgraph-contract-creation.git

# Set the working directory to the root of the cloned repository
WORKDIR /app/subgraph-contract-creation

# Run substreams build command in the root of the git repo
RUN substreams build

# Set the working directory to the eth subgraph folder
WORKDIR /app/subgraph-contract-creation/subgraphs/eth

# Run the make command on startup
CMD ["make"]