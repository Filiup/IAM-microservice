-- Create roles table
CREATE TABLE public.roles (
	role_id serial NOT NULL,
	role_name varchar NOT NULL,
	role_description text NULL,
	CONSTRAINT roles_pk PRIMARY KEY (role_id)
);

-- Create clients table 
CREATE TABLE public.clients (
	client_id serial4 NOT NULL,
	client_forename varchar NOT NULL,
	client_surname varchar NOT NULL,
	client_email varchar NULL,
	CONSTRAINT clients_pk PRIMARY KEY (client_id)
);

-- Create groups table 
CREATE TABLE public."groups" (
	group_id serial4 NOT NULL,
	group_name varchar NOT NULL,
	group_description text NULL,
	group_email varchar NULL,
	CONSTRAINT groups_pkey PRIMARY KEY (group_id)
);

-- Create groups clients table 
CREATE TABLE public.groups_clients (
	client_alias_id serial4 NOT NULL,
	client_id int4 NULL,
	group_id int4 NOT NULL,
	subscription_item_id int4 NULL,
	deleted bool DEFAULT false NOT NULL,
	suspended bool DEFAULT false NOT NULL,
	role_id int4 NULL,
	CONSTRAINT groups_clients_client_id_group_id_unique UNIQUE (client_id, group_id),
	CONSTRAINT groups_clients_pkey PRIMARY KEY (client_alias_id)
);
CREATE INDEX groups_clients_group_id_idx ON public.groups_clients USING btree (group_id);
CREATE INDEX groups_clients_subscription_item_id_idx ON public.groups_clients USING btree (subscription_item_id);

ALTER TABLE public.groups_clients ADD CONSTRAINT groups_clients_group_id_fkey FOREIGN KEY (group_id) REFERENCES public."groups"(group_id);
ALTER TABLE public.groups_clients ADD CONSTRAINT groups_clients_clients_fk FOREIGN KEY (client_id) REFERENCES public.clients(client_id);
ALTER TABLE public.groups_clients ADD CONSTRAINT groups_clients_roles_fk FOREIGN KEY (role_id) REFERENCES public.roles(role_id);
