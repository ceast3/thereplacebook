FROM ubuntu:latest
LABEL authors="caitlineast"

ENTRYPOINT ["top", "-b"]