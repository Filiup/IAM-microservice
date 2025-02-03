## Creating new permission

To create a new permission, add its name and description to the `acl.permissions` table.



## Creating feature flag

To enable the permission, add it to the `acl.feature_rights` table. In this table, the `feature` column accepts values of 0 or 1, where 1 indicates the permission is **enabled** and 0 indicates it is **disabled**. If you wish to restrict the permission to a **specific group**, disable it in the `acl.feature_rights` table (set to 0) and enable it for that group in the `acl.feature_groups_rights` table (set to 1).



## Managing permission access rights

Permission access is managed within the `acl.subscriptions_rights` and `acl.roles_rights` tables.

- In the `acl.subscriptions_rights` table, rights are assigned to specific **subscription plans** through the `subs_plan_name` column.
  - Subscription plans are defined in the `subs.plans` table.
- The `acl.roles_rights` table is used to manage rights for specific **roles** via the `role_id` column.
  - Roles are defined in the `public.roles` table.



### Both of the tables contain these columns

| column    | Description                                                  |
| --------- | ------------------------------------------------------------ |
| feature   | Used to enable or disable permission. Can have value only 0 or 1 |
| owner     | Manages permission access rights for the owner ( Client for which request was made for  ). |
| colleague | Manages permission access rights for a colleague.            |
| deleted   | Manages permission access rights for a deleted user.         |
| suspended | Manages permission access rights for a suspended user.       |



Access rights are managed using a **Linux-like octal notation**, where:

- The **read** permission is represented by the value `4`.
- The **edit** permission is represented by the value `2`.
- The **delete** permission is represented by the value `1`.

Permissions can be combined in a manner similar to Linux. For example, to grant **read** and **edit** rights to a `colleague`, you would assign the value `6`:

- READ (4) + EDIT (2).

To grant **full access** to an `owner`, you would assign the value `7`:

- READ (4) + EDIT (2) + DELETE (1).







