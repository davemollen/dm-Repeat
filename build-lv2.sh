DOCKER_BUILDKIT=1 docker build --rm --file lv2/docker/modduo/Dockerfile --output lv2/out/modduo . && \
DOCKER_BUILDKIT=1 docker build --rm --file lv2/docker/moddwarf/Dockerfile --output lv2/out/moddwarf .