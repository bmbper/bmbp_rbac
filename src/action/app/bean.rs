use bmbp_marco::bean;
use bmbp_marco::table_rdbc_bean_orm_option;
use bmbp_rdbc::RdbcIdent;
use bmbp_rdbc::RdbcOrmRow;
use bmbp_rdbc::RdbcTable;
use serde::Deserialize;
use serde::Serialize;

#[bean]
pub struct BatchReqVo {
    ids: Option<Vec<String>>,
}

// 角色信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_APP)]
pub struct BmbpRbacApp {
    app_code: Option<String>,
    app_name: Option<String>,
}
