# spot

Brings all of the Spot components (currently [`spot-server`](https://github.com/teovoinea/spot-server) and [`spot-client`](https://github.com/teovoinea/spot-client)) together

## Architecture

Starts up `spot-server`

### hyper

Uses hyper to store compiled `spot-client` wasm and static html/css/js
