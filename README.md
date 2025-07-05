# Cowboy Example Programs

### Quick Start
1. Run
```shell
docker run -p 1881:1881 --platform linux/amd64   ghcr.io/project-cowboy/cowboy-prover:latest
```
2. Separate terminal:
```shell
cargo run run
```

### How to generate TLSN proof of tiktok data from frontend

## Proxy Setup
For many web servers, a flexible websockets to TCP proxy will need to be run, until Cowboy runs hosted ones for convenience:
1. Download wstcp
2. wstcp --bind-addr 127.0.0.1:55688 www.tiktok.com:443
3. Tell your browser extension about the proxy. In the options menu of the browser extension, set `http://127.0.0.1:55688` for the proxy api field.

## Get data
1. Authenticate to Tiktok in browser
2. Ensure that you have analytics enabled, and that the data is ready(Tiktok will tell you you need to wait if not ready)
3. Browse to the analytics page(https://www.tiktok.com/tiktokstudio/analytics/overview?from=dropdown_button)
4. Open the TLS Notary Extension
5. Search in the filter bar for "follower_num"
6. There should be two results. Find the one which features the `follower_num` object at the top-level of the `response` field.
7. Click "Notarize". Then, the next "Notarize" button as well.


4. Filter the network request
1.
follower_num