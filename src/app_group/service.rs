use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn, BmbpAppGroupTable};
use bmbp_abc::BmbpTreeUtil;
use bmbp_app_orm::BMBP_APP_ORM;
use bmbp_bean::BmbpResp;
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
        let mut group_vec = BMBP_APP_ORM.read()?.find_list_by_query(&query)?;
        group_vec = BmbpTreeUtil::build_tree(group_vec);
        Ok(group_vec)
    }
}
