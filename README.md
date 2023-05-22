# Mempools Backend

## Run dev server
Using docker
```
docker build -t mempools-server-dev --file ./deployments/dockerfiles/dev/Dockerfile .
docker run --init -p 8123:8123 mempools-server-dev
```
or run server using cargo
```
cargo run -p mempools-server
```
Server will start running on port 8123

Note: This is a dev server, it has features like auto auth enabled - you can refer to [this](./deployments/dockerfiles/prod/Dockerfile) if your looking to run the server in a production environment.

## Install CLI - DEPRECATED - request postman collection
```
cargo install --path mempools-cli
```

## Create an alert
This should send you an alert anytime someone send funds 
on the JUNO blockchain.
```
mempools-cli create-alert send-funds-alert --destination-type email --destination-addr YOUR_EMAIL_HERE
```
You can add from and to address to narrow down the alert.
```
mempools-cli create-alert send-funds-alert --from FROM_ADDRESS --destination-type email --destination-addr YOUR_EMAIL_HERE
```

## Generate grpc-web ts client
```
export DIR=mempools-proto/proto
export OUT_DIR=mempools-proto/js
protoc -I=$DIR api.proto \
  --js_out=import_style=commonjs,binary:$OUT_DIR \
  --grpc-web_out=import_style=typescript,mode=grpcweb:$OUT_DIR
```