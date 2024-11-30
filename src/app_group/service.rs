use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn, BmbpAppGroupTable};
use bmbp_abc::BmbpTreeUtil;
use bmbp_app_orm::BMBP_APP_ORM;
use bmbp_bean::{BmbpError, BmbpResp};
use bmbp_sql::RdbcQueryWrapper;

pub struct Service;

impl Service {
    pub async fn query_tree(group: &mut BmbpAppGroup) -> BmbpResp<Vec<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::with_table_columns(
            BmbpAppGroupTable::name(),
            BmbpAppGroupTable::columns(),
        );
        query.like_if(
            BmbpAppGroupColumn::NodeName,
            &group.group.node_name,
            group.group.node_name.is_empty(),
        );
        match BMBP_APP_ORM.read()?.find_list_by_query(&query) {
            Ok(group_vec) => Ok(group_vec),
            Err(err) => Err(BmbpError::new(err.kind, "3001".to_string(), err.msg)),
        }
    }
}
