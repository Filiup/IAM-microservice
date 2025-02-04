## Introduction 

This project is a lightweight and efficient identity and access management ( IAM ) solution designed for micro-service architectures. It determines what actions a user is authorized to perform based on their assigned roles and subscriptions. Other micro-services can use it to verify permissions quickly and securely.

## How does it work

### Roles and Subscriptions

In the IAM system, each user can have a **role** or **subscription**, which determines their **permissions** for specific functionalities. Based on these permissions, IAM evaluates what operations a user can or cannot perform within the application.

### Feature Flags

Developers can also enable or disable application features using **feature flags**. These are particularly useful when a functionality is not fully tested or fine-tuned. A feature can be deployed to production with its flag turned off, ensuring that end users cannot access it until it is ready. Feature flags can also be enabled for specific user groups, restricting access to certain clients only. This is useful for testing, controlled rollouts, or showcasing features to selected customers.



## Project features 

- **Efficient Data Storage with PostgreSQL**: Leverages [PostgreSQL](https://www.postgresql.org/) as the primary database, optimizing for high-performance queries and data integrity.
- **Compile-time Checked Queries**: Ensures query safety at **compile time** using [SQLx crate](https://github.com/launchbadge/sqlx), preventing runtime SQL errors.
- **Containerized Deployment**: Uses [**Docker containers**](https://www.docker.com/) to ensure consistency across different environments and simplify deployment.
- **Automated OpenAPI Generation**: [OpenAPI](https://www.openapis.org/) documentation is generated directly from **Rust code** using [Poem OpenAPI crate](https://github.com/poem-web/poem/blob/master/poem-openapi/README.md).
- **Comprehensive API Documentation**: A dedicated documentation page is generated using [Swagger UI](https://swagger.io/tools/swagger-ui/), providing an interactive **API exploration tool**.
- **Optimized Performance with Caching**: Utilizes [Redis](https://redis.io/) to cache database queries and reduce response times.
- **High-Concurrency Execution**: Built on [Tokio](https://tokio.rs/), utilizing asynchronous tasks for **peak concurrency**.
- **Integrated CLI Tools**: Supports essential operations like **JWT token generation** and **database dumps** via **command-line utilities**.
- **RPC Protocol Support**: Enables inter-service communication using [gRPC Framework](https://grpc.io/).
- **Robust Authentication System**: Features an integrated **Auth Guard** that validates **JWT tokens** and extracts **User objects** from the **JWT payload**.



## Project documentation

| Section                                                      | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [1. Project setup](./docs/project-setup.md)                  | Instructions on how to initialize and run the project using Docker. |
| [2 .Database structure](./docs/database-structure.md)        | Overview of all **PostgreSQL** tables and their usage in the system. |
| [3. Managing Roles & Subscriptions](./docs/managing-subscriptions-roles.md) | Guide to creating new **clients** and assigning them to **roles/subscriptions**. |
| [4. Managing permissions](./docs/managing-permissions.md)    | Instructions on defining new **permissions** and **managing access rights**. |
| [5. Access right merging](./docs/access-right-merging.md)    | Explanation of the **Parent/Child Rights Hierarchy** and how permissions inherit from parent to child. |
| [6. Project usage](./docs/project-usage.md)                  | Guide on interacting with the **IAM microservice** using **HTTP** and **RPC** protocols. |
| [7. CLI usage](./docs/cli-usage.md)                          | Instructions on utilizing **CLI tools** for generating **JWT tokens** and creating **database dumps**. |

