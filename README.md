# RDF Worker

## Requirements

The following tool must be installed on your computer:

* Rust development environment (see installation [here](https://www.rust-lang.org/learn/get-started))
	* Rustc >= 1.36.0
	* Cargo  >= 1.36.0
* Rust CI tools:
	* Tarpaulin (Code coverage for unit tests) >= 0.8.4
	* Rustfmt (code format) => 1.2.2-stable
	* Clippy (Rust linter) >= 0.0.212
* JQ (see installation [here](https://stedolan.github.io/jq/download/))
	
## Launch worker locally

Before to launch the worker you need to set these environment variables:

| Variable name        | Default value              | Description                                 |
|----------------------|----------------------------|---------------------------------------------|
| AMQP_HOSTNAME        | 127.0.0.1                  | IP or host of AMQP server                   |
| AMQP_PORT            | 5672                       | AMQP server port                            |
| AMQP_USERNAME        | guest                      | User name used to connect to AMQP server    |
| AMQP_PASSWORD        | guest                      | Password used to connect to AMQP server     |
| AMQP_VHOST           | /                          | AMQP vhost                                  |
| AMQP_QUEUE           | job_undefined              | AMQP queue                                  |
| BACKEND_HOSTNAME     | http://127.0.0.1:4000/api  | URL used to connect to backend server       |
| BACKEND_USERNAME     |                            | User name used to connect to backend server |
| BACKEND_PASSWORD     |                            | Password used to connect to backend server  |

Once these environment variables are set, you can start your worker:
```bash
make run
```

### Trick to set environment variables easily

You could create a file named `.env` (or you can copy the file `.env.dist`) end edit it with the correct values.
The `make run` command will automatically take it into account if it exists.
