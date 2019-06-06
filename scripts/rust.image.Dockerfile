FROM alpine:edge
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories && \
  sed -i 's/http:/https:/g' /etc/apk/repositories && \
  apk update && \
  apk add cargo gcc g++ cmake make curl vim zlib-dev openssl-dev linux-headers gperf
CMD sh

