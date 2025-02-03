## Parent/Child Rights Hierarchy

There are two types of rights: **parent** rights and **child** rights. A **parent** right is a right that does not contain a dot (`.`) in its name (e.g., the right `payments`).

For the parent right `payments`, we can define a child right `payments.online`.

A **child** right must always include:

- The name of its parent
- A dot (`.`) following the parent name
- Its own unique name



## Access Right Merging

Let's consider an example where the table `acl.subscriptions_rights` defines two rights:

- The **parent** right `payments`, where the `owner` column has a value of `4` (READ).
- The **child** right `payments.online`, where the `owner` column has a value of `7` (READ + EDIT + DELETE).

When determining whether a user has access to `payments.online`, the application first checks the permissions assigned to the parent (`payments`). In this case, the user has a permission value of `4` for the parent (`payments`) and a value of `7` for the child (`payments.online`).
The application merges these two values by always selecting the **minimum value**.

In practice, this means that even though the user has a permission level of `7` for `payments.online`, the application determines that their effective permission is only `4` (READ).

This happens because the application considers both the **parent** and the **child** and applies the **lowest** value, which in this case is `4` from the parent (`payments`).



### Multiple descendants

A parent right can have multiple layers of descendants. In addition to the child right `payments.online`, there can also be a deeper nested right such as `payments.online.stripe`.

In this case, the application merges the permissions of `payments.online.stripe` with `payments.online` first and then further merges the result with the parent right `payments`.



### Key Takeaways:

- A child right can never have a higher permission level than its parent
  - If it does, the application will enforce the parent’s permission level instead.
  
- A child right can have a lower permission level than its parent
  - For example, a user may have **full access** (`7`) to `payments`, but only **read access** (`4`) to `payments.online`.
  
    

This merging principle applies only **from the child to the parent**. It does not work the other way around.

For example, if:

- The **parent** right `payments` has an `owner` value of `7` (READ + EDIT + DELETE).
- The **child** right `payments.online` has an `owner` value of `4` (READ).

Then when checking permissions for the **parent**, the result will still be `7`, since the parent is **never merged** with its child.

This merging principle applies to all shared columns in the `acl.subscriptions_rights` and `acl.roles_rights` tables:

- `feature`
- `owner`
- `colleague`
- `suspended`
- `deleted`

For more details about these columns and their purpose, refer to the [Managing Permissions](managing-permissions.md) section.



## Cross-table right merging

Consider a scenario where the same right is defined in both the `acl.subscriptions_rights` and `acl.roles_rights` tables:

- In the `acl.subscriptions_rights` table, the `owner` column for the `payments` right is set to `7`.
- In the `acl.roles_rights` table, the `owner` column for the `payments` right is set to `4`.

In this case, the same **merging principle** applies as with **parent-child rights**—the application will always take the lowest value.

As a result, the effective permission for `payments` will be determined by the `owner` column in the `acl.roles_rights` table, which holds the lower value of `4`.



### Merging Exception

An exception to this rule applies to the `acl.feature_rights` and `acl.roles_rights` tables. When merging values from these tables, the application selects the **highest** value instead of the lowest for the `feature` column.





