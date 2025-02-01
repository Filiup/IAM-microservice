use super::iam_helper::create_access_right_map;
use crate::{
    common::{database::Database, services::cashing_service::CashingService},
    iam::iam_model::{AccessRightEntity, ColleagueEntity},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct IamDriver {
    cashing_service: CashingService,
}

impl IamDriver {
    pub async fn new() -> Self {
        let cashing_service = CashingService::new();
        Self { cashing_service }
    }

    pub async fn load_colleagues(
        &self,
        client_alias_id: i32,
    ) -> Result<Vec<ColleagueEntity>, sqlx::Error> {
        let colleagues_redis_key = format!("IAM_COLLEAGUES_{}", client_alias_id);
        let cached_colleagues = self
            .cashing_service
            .get::<Vec<ColleagueEntity>>("off") // TODO: Replace with colleagues_redis_key
            .await;

        match cached_colleagues {
            Err(err) => {
                tracing::error!(error = %err, "Failed to load colleagues from redis cache !")
            }
            Ok(Some(cached_colleagues)) => return Ok(cached_colleagues),
            _ => (),
        }
        let pool = Database::pg().await;
        let colleagues = sqlx::query_as!(
            ColleagueEntity,
            r#"
            SELECT
                gc.client_alias_id,
                gc.client_id,
                gc.group_id,
                CASE 
                    WHEN gc.deleted THEN 0
                    WHEN gc.suspended THEN 1
                    ELSE 2
                END AS status
            FROM groups_clients gc
            JOIN groups_clients gc2
                ON gc.group_id = gc2.group_id
            WHERE gc2.client_alias_id = $1;
        "#,
            client_alias_id
        )
        .fetch_all(&pool)
        .await?;

        let _ = self
            .cashing_service
            .set(colleagues_redis_key, &colleagues)
            .await;

        Ok(colleagues)
    }

    pub async fn load_access_rights(
        &self,
        client_alias_ids: &[i32],
    ) -> Result<HashMap<i32, HashMap<String, AccessRightEntity>>, sqlx::Error> {
        /*         let access_rights_redis_key = format!("IAM_AR_{}", client_alias_ids);
        let cached_access_rights = self
            .cashing_service
            .get::<Vec<AccessRightEntity>>("off") // TODO: Replace with access_rights_redis_key
            .await;

        match cached_access_rights {
            Err(err) => {
                tracing::error!(error = %err, "Failed to load access rights from redis cache !")
            }
            Ok(Some(cached_access_rights)) => {
                return Ok(create_access_right_map(cached_access_rights))
            }
            _ => (),
        } */

        let pool = Database::pg().await;
        let access_right_entities = sqlx::query_as!(
            AccessRightEntity,
            r#"
         WITH active_alias_ids AS (
                SELECT client_alias_id AS caid
                FROM groups_clients
                WHERE client_alias_id = ANY($1)
            ),
            running_subscriptions AS (
                SELECT spl.subs_plan_name, gc.client_alias_id AS caid
                FROM subs.plans AS spl
                JOIN subs.subscriptions_items AS ssi
                    ON ssi.subs_plan_id = spl.subs_plan_id
                JOIN subs.subscriptions AS ss
                    ON ss.subscription_id = ssi.subscription_id 
                    AND ss.current_period_end::date >= NOW()::date 
                    AND ss.current_period_start::date <= NOW()::date
                    AND ss.status NOT IN ('CANCELED', 'UNPAID')
                JOIN groups_clients AS gc 
                    ON gc.subscription_item_id = ssi.subscription_item_id
                WHERE gc.client_alias_id IN (SELECT caid FROM active_alias_ids) 
            ),
            c_feature_rights AS (
                SELECT
                	caid,
                    permission,
                    MAX(feature)  AS feature,
                    NULL::integer AS owner,
                    NULL::integer AS colleague,
                    NULL::integer AS suspended,
                    NULL::integer AS deleted
                FROM (
                    SELECT ac.caid AS caid, fr.permission, fr.feature
                    FROM acl.feature_rights AS fr
                    CROSS JOIN active_alias_ids AS ac
                    UNION ALL
                    SELECT gc.client_alias_id AS caid, fgr.permission, fgr.feature
                    FROM acl.feature_groups_rights AS fgr
                    JOIN public.groups_clients AS gc 
                        ON fgr.group_id = gc.group_id
                    WHERE gc.client_alias_id IN (SELECT caid FROM active_alias_ids)
                )
                GROUP BY PERMISSION, caid
            ),
            c_subscription_rights AS (
                SELECT
                	crs.caid,
                    acsr.permission,
                    acsr.feature,
                    acsr.owner,
                    acsr.colleague,
                    acsr.suspended,
                    acsr.deleted
                FROM acl.subscriptions_rights AS acsr
                JOIN running_subscriptions AS crs 
                    ON crs.subs_plan_name = acsr.subs_plan_name
            ),
            c_roles_rights AS (
                SELECT
                	gc.client_alias_id AS caid,
                    acr.permission,
                    acr.feature,
                    acr.owner,
                    acr.colleague,
                    NULL::integer AS suspended,
                    NULL::integer AS deleted
                FROM acl.roles_rights AS acr
                JOIN public.groups_clients AS gc 
                    ON acr.role_id = gc.role_id
                WHERE gc.client_alias_id IN (SELECT caid FROM active_alias_ids)
            )
            SELECT
            	caid,
                permission,
                MIN(feature)   AS feature,
                MIN(owner)     AS owner,
                MIN(colleague) AS colleague,
                MIN(suspended) AS suspended,
                MIN(deleted)   AS deleted
            FROM (
                SELECT * FROM c_feature_rights
                UNION ALL
                SELECT * FROM c_subscription_rights
                UNION ALL
                SELECT * FROM c_roles_rights
            )
            GROUP BY PERMISSION, caid;
       
            "#,
            client_alias_ids
        )
        .fetch_all(&pool)
        .await?;

        /*         let _ = self
        .cashing_service
        .set(access_rights_redis_key, &access_right_entities)
        .await; */
        Ok(create_access_right_map(access_right_entities))
    }
}
