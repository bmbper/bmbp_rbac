use bmbp_marco::bean;
use bmbp_marco::table_rdbc_tree_bean_orm_option;
use bmbp_rdbc::RdbcIdent;
use bmbp_rdbc::RdbcOrmRow;
use bmbp_rdbc::RdbcTable;
use bmbp_util::BmbpTree;
use serde::Deserialize;
use serde::Serialize;

#[bean]
pub struct BatchReqVo {
    ids: Option<Vec<String>>,
}

// 角色信息
#[table_rdbc_tree_bean_orm_option(BMBP_RBAC_ROLE, menu)]
pub struct BmbpRbacMenu {
    app_id: Option<String>,
}
