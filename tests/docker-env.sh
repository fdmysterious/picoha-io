

docker build -t pza_test .

docker run \
    -v $PWD/../..:/work \
    --user $(id -u):$(id -g)\
    --privileged
    -it pza_test bash

