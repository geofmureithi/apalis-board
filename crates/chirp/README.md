
# Apalis Chirp

`apalis-chirp` is a command runner application that is designed to be invoked via HTTP or cron jobs. It leverages multiple storage backends and supports running tasks defined in a config file.

## Features

- **HTTP and Cron Job Integration**: Run tasks through HTTP requests or scheduled cron jobs.
- **Storage Support**: Supports Redis, MySQL, PostgreSQL, and SQLite for storing job data.
- **Docker Integration**: Run commands in Docker containers.
- **Real-time Event Streaming**: SSE (Server-Sent Events) support for real-time updates.
- **Structured Logging**: JSON-formatted logs for easy monitoring and debugging.

## Installation

To build and run the `apalis-chirp` application, you need to have Rust and Cargo installed. Clone the repository and run the following command:

```sh
cargo build --release
```

## Configuration

The application configuration is done via a YAML file. The configuration file should define the jobs to be executed. Below is an example configuration:

```yaml
jobs:
  example_job:
    source:
      Cron: "0 0 * * * *"
    task:
      Command:
        steps:
          step1: "echo 'Hello, World!'"
        docker: "alpine:latest"
```

## Running the Application

To run the application, use the following command, specifying your configuration file:

```sh
./target/release/apalis-chirp --config path/to/config.yaml
```

See [this example](../../examples/standalone/chirpy.yml)

## API Endpoints

### POST /api/v1/backend/{namespace}/{job}

Trigger a job to run immediately.

Example
```s=
curl -X PUT \
  http://127.0.0.1:8000/api/v1/backend/send-email/job \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' \
  -d '{ "Custom": { "value": "test" } }'
```

### GET /api/v1/events

Listen to real-time updates via Server-Sent Events (SSE).

## Development

### Prerequisites

- Rust
- Cargo

### Building from Source

Clone the repository and build the project:

```sh
git clone <repository-url>
cd crates/chirp
cargo build --release
```

### Running Tests

To run tests, use the following command:

```sh
cargo test
```

## License

This project is licensed under the MIT License.
