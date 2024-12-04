# DigiDoc Server

## Install libdigidocpp

```sh
$ sudo apt install build-essential cmake curl libssl-dev libxml2-dev libxmlsec1-dev pkg-config zlib1g-dev
$
$ curl -O https://github.com/open-eid/libdigidocpp/releases/download/v4.0.0/libdigidocpp-4.0.0.tar.gz
$ tar xzvf libdigidocpp-4.0.0.tar.gz
$
$ cd libdigidocpp-4.0.0
$ cmake -B build -S . -DCMAKE_INSTALL_PREFIX=/opt/digidoc
$ cmake --build build
$ sudo cmake --build build --target install
```

## Run server

```sh
$ ./run.sh
```
