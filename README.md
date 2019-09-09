# RDF Worker

## Requirements

The following tool must be installed on your computer:

* Rust development environment (see installation [here](https://www.rust-lang.org/learn/get-started))
	* Rust >= 1.36.0
	* Cargo  >= 1.36.0
* Rust CI tools:
	* Tarpaulin (Code coverage for unit tests) >= 0.8.4 / see installation [here](https://github.com/xd009642/tarpaulin)
	* Rustfmt (code format) => 1.2.2-stable / see installation [here](https://github.com/rust-lang/rustfmt)
	* Clippy (Rust linter) >= 0.0.212 / see installation [here](https://github.com/rust-lang/rust-clippy)
* JQ (see installation [here](https://stedolan.github.io/jq/download/))

## Launch worker locally

Before to launch the worker you need to set these environment variables:

| Variable name          | Default value                | Description                                 |
|------------------------|------------------------------|---------------------------------------------|
| `AMQP_HOSTNAME`        | `127.0.0.1`                  | IP or host of AMQP server                   |
| `AMQP_PORT`            | `5672`                       | AMQP server port                            |
| `AMQP_USERNAME`        | `guest`                      | User name used to connect to AMQP server    |
| `AMQP_PASSWORD`        | `guest`                      | Password used to connect to AMQP server     |
| `AMQP_VHOST`           | `/`                          | AMQP vhost                                  |
| `AMQP_QUEUE`           | `job_undefined`              | AMQP queue                                  |
| `BACKEND_HOSTNAME`     | `http://127.0.0.1:4000/api`  | URL used to connect to backend server       |
| `BACKEND_USERNAME`     |                              | User name used to connect to backend server |
| `BACKEND_PASSWORD`     |                              | Password used to connect to backend server  |

Once these environment variables are set, you can start your worker:
```bash
make run
```

### Trick to set environment variables easily

You could create a file named `.env` (or you can copy the file `.env.dist`) end edit it with the correct values.
The `make run` command will automatically take it into account if it exists.

## Makefile targets

Commands below will be used for both stacks (backend & workers):

| Command                     | Description                                              |
|-----------------------------|----------------------------------------------------------|
| `make build`                | Build the application                                    |
| `make ci-code-format`       | Check the code format according to the rust format rules |
| `make ci-code-coverage`     | Launch tests and returns code coverage for tests         |
| `make ci-lint`              | Launch the rust linter                                   |
| `make ci-tests`             | Launch tests                                             |
| `make docker-build`         | Build locally a docker image                             |
| `make docker-clean`         | Remove locally the built docker image                    |
| `make docker-push-registry` | Push the locally built docker image                      |
| `make run`                  | Run locally the worker                                   |
| `make version`              | Display the version defined in `cargo.toml` file         |

## CI / CD

A `.gitlab-ci.yml` file is provided for the gitlab CI/CD feature.
This file will instantiate te following pipeline:

<!-- language: lang-none -->
    /-----------\      /---------\             /----------\             /----------\
    |  Compile  |------|  Tests  |-------------| Quality  |-------------|  Docker  |
    \-----------/      \---------/             \----------/             \----------/
         |                  |                       |                        |
     +------+           +-------+              +---------------+          +-------+
     | lint |           | tests |              | code-coverage |          | build |
     +------+           +-------+              +---------------+          +-------+
                                                    |
                                               +-------------+
                                               | code-format |
                                               +-------------+
<!-- language: markdown -->

### Docker

The command `make docker-build` will build an image named `mediacloudai/rdf_worker`.

The command `make push-docker-registry` will logged in and push the built image in the official docker registry. The login must be set with the following environment variables:

| Variable name           | Default value              | Description                                      |
|-------------------------|----------------------------|--------------------------------------------------|
| `DOCKER_REGISTRY_LOGIN` |                            | User name used to connect to the docker registry |
| `DOCKER_REGISTRY_PWD`   |                            | Password used to connect to the docker registry  |





