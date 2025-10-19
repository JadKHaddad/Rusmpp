FROM busybox

RUN mkdir /tmp/build/
COPY . /tmp/build/

# docker build -t build-context -f docker/context.Dockerfile .
# docker run --rm -it build-context tree /tmp/build/
