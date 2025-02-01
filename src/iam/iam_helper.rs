use super::iam_model::AccessRightEntity;
use std::collections::HashMap;

pub fn create_access_right_map(
    entities: Vec<AccessRightEntity>,
) -> HashMap<i32, HashMap<String, AccessRightEntity>> {
    entities.into_iter().fold(HashMap::new(), |mut acc, curr| {
        let caid = curr.caid.unwrap_or_default();
        let permissions = acc.entry(caid).or_default();
        let curr_permission = curr.permission.as_deref().unwrap_or_default();

        permissions.insert(curr_permission.to_string(), curr);
        acc
    })
}
