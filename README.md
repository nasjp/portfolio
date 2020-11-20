# Portfolio

## build docker image

```sh
docker build -t docker.pkg.github.com/nasjp/dockerimages/rust-wasm-pack:latest .
docker push docker.pkg.github.com/nasjp/dockerimages/rust-wasm-pack:latest
```

## build wasm

```sh
docker run  -v $PWD:/app --workdir=/app docker.pkg.github.com/nasjp/dockerimages/rust-wasm-pack:latest bash -c "wasm-pack build --target web --out-name wasm --out-dir ./public"
```
