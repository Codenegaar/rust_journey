# Hyper Microservice
A simple microservice written with hyper, just to see how things work under the hood.

The HTTP server has been written with hyper `0.13.8` and with the help of
[their guide](https://hyper.rs/guides/).

The server also supports configuration and logging.

## Configuration

### Using the `.env` file:

```
## Address to bind to
ADDRESS=127.0.0.1
```

The `.env` file has to be omitted from the repository (using `.gitignore`)
but has been kept for educational purposes. Lines starting with `#` has been commented.

### Using command-line arguments

The supported command-line arguments are:

* address (`-a`, `--address`): Address to bind to, in the following format:
`127.0.0.1:8080`

### Change logging level

You can choose one of these logging levels: `trace`, `debug`, `info`, `warn`, `error`.

To set the logging level, simply set the `RUST_LOG` env. variable.

* For the current session: `export RUST_LOG=trace`
* For a single run: `RUST_LOG=trace cargo run`
