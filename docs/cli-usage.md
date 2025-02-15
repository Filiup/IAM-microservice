# `sign_jwt.sh`

This script is used to generate and sign a **JWT token**, which can then be used to authenticate **HTTP/RPC** requests to the IAM microservice.

### Usage

- To sign a JWT token, choose some user from [public.groups_clients](./database-structure.md) table and specify its `client_alias_id` and `group_id`
  - Input will be embedded in the **JWT payload** and specify the user for whom the request is being made.



# `dump.sh`

This script generates a **PostgreSQL** database dump, providing a backup of your data. It is particularly useful when adding custom **permissions** and **subscriptions**, ensuring that changes can be restored if needed.

### Usage

- To create new database backup, run this script on your machine and specify **backup file name** as the **first program argument**

  

# `restore.sh`

This script restores a **PostgreSQL** database dump. Available backup files are located in the `postgresql/backups` directory, with the default dump stored in `iam.backup` file.

### Usage

- To create restore database backup, run this script on your machine and specify **backup file name** as the **first program argument**











