use super::iam_model::AccessRightEntity;
use std::collections::HashMap;

pub fn create_access_right_map(
    entities: Vec<AccessRightEntity>,
) -> HashMap<String, AccessRightEntity> {
    entities.into_iter().fold(HashMap::new(), |mut acc, curr| {
        let curr_permission = curr.permission.as_deref().unwrap_or_default();
        acc.insert(curr_permission.to_string(), curr);

        acc
    })
}
