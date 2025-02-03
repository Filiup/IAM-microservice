# Creating new client

To create a new user, use the tables within the **public** schema.

- Add the new client to the `public.clients` table.
- If the client requires a separate group, create it within the `public.groups` table.
  - Otherwise, they can be assigned to a **predefined group** within the same table.




### `public.groups_clients` Table

This table defines the relationship between **clients** and **groups**.

- A **client** can belong to **multiple groups**.
- **Columns**:
  - `client_alias_id` column
    - Serves as the **identifier** for the client within a specific group.
  - `subscription_item_id` column
    - Specifies the **subscription** assigned to the client.
    - **Note:** This can be left empty and set later when a **new subscription** is created.
  - `role_id` column
    - Defines the **client’s role** within the group.
    - **Note:** This can be left empty and set later when a **new role** is created.



# Creating new role

Roles are defined in the `public.roles` table and are used to **restrict a client's access** within a **group**.

-  Add the role’s **name** and **description** to the `public.roles` table.
- Navigate back to the `public.groups_clients` table and assign the newly created role to the appropriate `client alias`
  - `role_id` column



# Creating new subscription/subscription plan

Subscriptions are managed within the `subs` schema.

- Create a new **subscription plan** in the `subs.subs_plans` table.
  - The plan can be named as desired.
- Define a new **subscription** in the `subs.subscriptions` table
  - This table tracks the **status** and **duration** of the subscription.
- Link the **subscription** and **subscription plan** in the `subs.subscription_items` table.
-  Navigate to the `public.groups_clients` table and assign the newly created **subscription item** to the appropriate `client alias`
  - `subscription_item_id` column







