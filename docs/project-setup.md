# Running the Project in Docker

## Prerequisites

Ensure you have one of the following installed:

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Docker Engine](https://docs.docker.com/engine/)



## Running the Project in Development Mode

To start the project in development mode:

1. Clone the repository to your preferred location.

2. Navigate to the project directory and run:

   ```sh
   docker compose up -d
   ```

   This command will start three **docker containers**:

   - **Redis** – accessible on port `6370`
   - **PostgreSQL** – accessible on port `5430`
   - **IAM microservice** – exposes a gRPC service on port `9501` and a REST API on port `9101`

## Running project inside of production mode

- **TODO** (Production setup details to be added)

  

## Project configuration

All configuration settings are managed through the `.env` file. The most critical variables include:

- **`DATABASE_URL`**
  - Used by the [SQLx library](https://github.com/launchbadge/sqlx) for compile-time database query validation.
  - If **SQLx** fails to connect to the database, the project will not compile.
- `JWT_ACCESS_PUBLIC_KEY`
  - Base64-encoded RSA public key for JWT authentication.
  - To replace it, generate a new key pair using the `openssl rsa` command.
  - The private key is stored in the `jwtRS256.key` file.
  - **Note:** The RSA key included in this project is for testing purposes only and should not be used in production.