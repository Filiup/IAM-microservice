-- Create acl schema
CREATE SCHEMA acl;

-- Create permissions tanle
CREATE TABLE acl.permissions (
	"permission" varchar NOT NULL,
	"name" varchar NOT NULL,
	description text NULL,
	CONSTRAINT permissions_pk PRIMARY KEY (permission)
);

-- Create groups_rights table
CREATE TABLE acl.feature_groups_rights (
	"permission" varchar NOT NULL,
	feature int4 NOT NULL,
	group_id serial4 NOT NULL,
	CONSTRAINT feature_groups_rights_pk PRIMARY KEY (permission, group_id)
);

ALTER TABLE acl.feature_groups_rights ADD CONSTRAINT feature_groups_rights_fk FOREIGN KEY ("permission") REFERENCES acl.permissions("permission");


-- Create feature_rights table
CREATE TABLE acl.feature_rights (
	"permission" varchar NOT NULL,
	feature int4 NOT NULL,
	CONSTRAINT feature_rights_pkey PRIMARY KEY (permission)
);

ALTER TABLE acl.feature_rights ADD CONSTRAINT feature_rights_fk FOREIGN KEY ("permission") REFERENCES acl.permissions("permission");


-- Create subscription_rights table
CREATE TABLE acl.subscriptions_rights (
	subs_plan_name varchar NOT NULL,
	"permission" varchar NOT NULL,
	feature int4 NULL,
	"owner" int4 NULL,
	colleague int4 NULL,
	suspended int4 NULL,
	deleted int4 NULL,
	CONSTRAINT subscriptions_rights_unique UNIQUE (subs_plan_name, permission)
);
ALTER TABLE acl.subscriptions_rights ADD CONSTRAINT subscriptions_rights_fk FOREIGN KEY ("permission") REFERENCES acl.permissions("permission");
ALTER TABLE acl.subscriptions_rights ADD CONSTRAINT subscriptions_rights_plans_fk FOREIGN KEY (subs_plan_name) REFERENCES subs."plans"(subs_plan_name);

-- Create roles rights table 
CREATE TABLE acl.roles_rights (
	role_id int4 NOT NULL,
	"permission" varchar NOT NULL,
	"owner" int NULL,
	feature int4 NULL,
	colleague int4 NULL,
	deleted int4 NULL,
	suspended int4 NULL,
	CONSTRAINT acl_roles_rights_unq UNIQUE (role_id, permission)
);
ALTER TABLE acl.roles_rights ADD CONSTRAINT roles_rights_fk FOREIGN KEY ("permission") REFERENCES acl.permissions("permission");
ALTER TABLE acl.roles_rights ADD CONSTRAINT roles_rights_roles_fk FOREIGN KEY (role_id) REFERENCES public.roles(role_id);