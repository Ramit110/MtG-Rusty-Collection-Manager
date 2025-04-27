FROM debian AS elm-runner
RUN apt-get update && apt-get upgrade -y
WORKDIR /setup

RUN apt-get install curl -y
RUN curl -L -o elm.gz https://github.com/elm/compiler/releases/latest/download/binary-for-linux-64-bit.gz
RUN gunzip elm.gz
RUN chmod +x elm
RUN mv elm /usr/local/bin/

WORKDIR /usr/src
CMD [""]
