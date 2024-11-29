use crate::app_group::bean::{BmbpAppGroup, TABLE_NAME};
use bmbp_abc::BmbpTreeUtil;
use bmbp_app_orm::BMBP_APP_ORM;
use bmbp_bean::BmbpResp;
use bmbp_sql::RdbcQueryWrapper;

pub struct Service;

impl Service {
    pub async fn query_tree(group: &mut BmbpAppGroup) -> BmbpResp<Vec<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::new();
        query.select("*");
        query.from(TABLE_NAME);
        if group.tree.node_name != "" {
            query.like("node_name", &group.tree.node_name);
        }
        let mut group_vec = (&BMBP_APP_ORM).find_list_by_query(&query).await?;
        group_vec = BmbpTreeUtil::build_tree(group_vec);
        Ok(group_vec)
    }
}
