use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn};
use bmbp_abc::{BmbpError, BmbpResp};
use bmbp_orm::BMBP_ORM;
use bmbp_sql::RdbcQueryWrapper;

pub struct Service;

impl Service {
    pub async fn query_tree(group: &mut BmbpAppGroup) -> BmbpResp<Vec<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        query.like_if(
            BmbpAppGroupColumn::AppGroupName,
            &group.app_group_name,
            (&group.app_group_name).is_empty(),
        );
        match BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .read()
            .await
            .find_list_by_query(&query)
            .await
        {
            Ok(group_vec) => Ok(group_vec),
            Err(err) => Err(BmbpError::orm(err.msg)),
        }
    }
}
