# This is the image for xmrbc-rs server deployment
# Uses fedora:36 image and podman version 4.2.0
# Command to build: podman build --network host -t xmrbc-rs:vx.x.x .
# Command to run:
# > podman run --rm -P -p 127.0.0.1:<SERVER_PORT>:<SERVER_PORT> --name xmrbc-rs \
# > xmrbc-rs:v0.1.0 /bin/bash -c "sh /home/dev/deploy.sh <SERVER_PORT>"

FROM fedora
LABEL name = support@hiahatf.org
# Update fedora
RUN dnf -y update
# Install necessary packages
RUN dnf install -y wget curl git gnupg pkg-config jq gcc \
    openssl openssl-libs openssl-devel
# Let's not use root
RUN adduser dev
# Clone xmrbc-rs source code
RUN su dev && cd /home/dev && git clone https://github.com/hyahatiph-labs/xmrbc-rs.git
COPY ./deploy.sh /home/dev
RUN chown dev:dev /home/dev/deploy.sh
RUN chmod +x /home/dev/deploy.sh
