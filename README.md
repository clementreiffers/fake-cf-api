# rust-fake-cf-api

## Run it without build it

create at the root directory of this project a file named ".env" and write this values:

```text
ACCESS_KEY=<your_access_key>
SECRET_KEY=<your_secret_key>
REGION=<your_region>
ENDPOINT=<your_endpoint>
BUCKET_NAME=<your_bucket_name>
```

then, you can run it using `cargo run`

> **Note**: if you need file-watching, install cargo-watch : `cargo install cargo-watch`
> and then run `cargo watch -x run`

## Run it in production mode

run `cargo build --release`, copy the `.env` file near to the executable and run it.
