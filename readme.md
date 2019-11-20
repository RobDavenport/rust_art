server -> server files

client -> wasm/js project

first build the client library
> wasm-pack build -d ./../server/src/pkg

build the server
> sudo webpack --entry=./src/app.js --output=./static/app.bundle.js --mode=production
