FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y build-essential ethtool iproute2 iptables iputils-ping sudo && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ARG USERNAME=docker
ARG UID=1000
ARG GROUPNAME=docker-users
ARG GID=1000
RUN groupadd -g $GID docker-users && \
    useradd -m -l -u $UID -g $GID -G sudo $USERNAME && \
    echo "${USERNAME}:password" | chpasswd
RUN echo 'Defaults visiblepw' >> /etc/sudoers
RUN echo 'iganari ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

USER $USERNAME

WORKDIR /home/$USERNAME/
#WORKDIR /root/

CMD ["bash"]
