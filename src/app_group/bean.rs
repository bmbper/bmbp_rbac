use bmbp_orm::RdbcOrmRow;
use bmbp_orm_marco::tree_table;
use bmbp_sql::RdbcColumnIdent;
use bmbp_sql::RdbcTableIdent;
use bmbp_util::BmbpTree;
use serde::{Deserialize, Serialize};
#[tree_table(table = BMBP_RBAC_APP_GROUP,tree=app_group)]
pub struct BmbpAppGroup {}
