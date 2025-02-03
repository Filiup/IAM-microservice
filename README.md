## Introduction 

This project is a lightweight and efficient identity and access management ( IAM ) solution designed for micro-service architectures. It determines what actions a user is authorized to perform based on their assigned roles and subscriptions. Other micro-services can use it to verify permissions quickly and securely.

## How does it work

### Roles and Subscriptions

In the IAM system, each user can have a **role** or **subscription**, which determines their **permissions** for specific functionalities. Based on these permissions, IAM evaluates what operations a user can or cannot perform within the application.

### Feature Flags

Developers can also enable or disable application features using **feature flags**. These are particularly useful when a functionality is not fully tested or fine-tuned. A feature can be deployed to production with its flag turned off, ensuring that end users cannot access it until it is ready. Feature flags can also be enabled for specific user groups, restricting access to certain clients only. This is useful for testing, controlled rollouts, or showcasing features to selected customers.



## Project features 

- compile time checker
- cashing with Redis
- automaticky generovane openAPI
- CLI pre SQL dumps
- gRPC
- JWT Guard
