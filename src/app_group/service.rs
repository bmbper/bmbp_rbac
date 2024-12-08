use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn};
use bmbp_bean::{BmbpError, BmbpResp};
use bmbp_orm::BMBP_ORM;
use bmbp_rdbc_type::RdbcTableIdent;
use bmbp_sql::RdbcQueryWrapper;
pub struct Service;

impl Service {
    pub async fn query_tree(group: &mut BmbpAppGroup) -> BmbpResp<Vec<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        query.like_if(
            BmbpAppGroupColumn::AppGroupName,
            group
                .get_app_group_name()
                .as_ref()
                .unwrap_or(&"".to_string()),
            group
                .get_app_group_name()
                .as_ref()
                .unwrap_or(&"".to_string())
                .is_empty(),
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
            Err(err) => Err(BmbpError::new(err.kind, "3001".to_string(), err.msg)),
        }
    }
}
