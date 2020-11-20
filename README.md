# Portfolio

## run dev

```sh
wasm-pack build --target web --out-name wasm --out-dir public --no-typescript
miniserve public --index index.html
```

## build docker image

```sh
cat ~/.dockerimagegithubsecret | docker login ghcr.io -u nasjp --password-stdin
docker build -t ghcr.io/nasjp/dockerimages/rust-wasm-pack:latest .
docker push ghcr.io/nasjp/dockerimages/rust-wasm-pack:latest
```
