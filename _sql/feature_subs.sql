-- Create subs shema
CREATE SCHEMA subs;

-- Create subscription period taype
CREATE TYPE subs."period" AS ENUM (
	'MONTHLY',
	'YEARLY');

-- Create subs status type 
CREATE TYPE subs."subs_status" AS ENUM (
	'TRIAL',
	'ACTIVE',
	'CANCELED',
	'UNPAID'
);

-- Create subscriptions table 
CREATE TABLE subs.subscriptions (
	subscription_id serial4 NOT NULL,
	subs_period subs."period" NOT NULL,
	status subs."subs_status" NOT NULL,
	current_period_start timestamp NOT NULL,
	current_period_end timestamp NOT NULL,
	CONSTRAINT subscriptions_pkey PRIMARY KEY (subscription_id)
);

-- Create subscription plans table 
CREATE TABLE subs."plans" (
	subs_plan_id serial4 NOT NULL,
	subs_plan_name varchar NOT NULL,
	description text NULL,
	CONSTRAINT plans_pk PRIMARY KEY (subs_plan_id),
	CONSTRAINT subs_plan_name_unique UNIQUE (subs_plan_name)
);

-- Create subscriptions items table 
CREATE TABLE subs.subscriptions_items (
	subscription_item_id serial4 NOT NULL,
	subscription_id int4 NOT NULL,
	subs_plan_id serial4 NOT NULL,
	CONSTRAINT subscriptions_items_pkey PRIMARY KEY (subscription_item_id)
);
CREATE INDEX subscriptions_items_subscription_id_idx ON subs.subscriptions_items USING btree (subscription_id);


ALTER TABLE subs.subscriptions_items ADD CONSTRAINT subs_running_subs_items_subscription_fkey FOREIGN KEY (subscription_id) REFERENCES subs.subscriptions(subscription_id) MATCH FULL;
ALTER TABLE subs.subscriptions_items ADD CONSTRAINT subscriptions_items_plans_fk FOREIGN KEY (subs_plan_id) REFERENCES subs."plans"(subs_plan_id);
