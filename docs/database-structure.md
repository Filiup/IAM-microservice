## Database structure 

### 1. Public schema

- Schema used to store application clients, their groups.

| Table          | Description                                                  |
| -------------- | ------------------------------------------------------------ |
| clients        | Contains all application clients.                            |
| groups         | Contains all  application groups.                            |
| roles          | Contains all roles created by group owners.                  |
| groups_clients | Determines which client belongs to which group, assigns the client's role within a specific group and links the client to a **subscription item**. A single client can be a member of multiple groups. |



### 2.  Subs schema

- Schema used for managing client subscriptions

| Table               | Description                                                  |
| ------------------- | :----------------------------------------------------------- |
| plans               | Stores all subscription plans which are currently available inside of the application. |
| subscriptions       | Stores all subscriptions which were created inside of the app together with their period |
| subscriptions_items | Associative table that establishes a relationship between the "subscriptions" and "plans" tables. |



### 3. ACL schema

- Schema used for managing permissions and access 

| Table                 | Description                                                  |
| --------------------- | :----------------------------------------------------------- |
| permissions           | Stores all permissions available inside of the application.  |
| feature_rights        | Used to enable or disable application features ( **feature flags** ). |
| feature_groups_rights | Used to enable or disable application features for specific **user groups**. |
| subscription_rights   | Used to manage feature access for designated subscriptions.  |
| roles_rights          | Used to manage feature access for designated roles.          |

