use bmbp_marco_rdbc::table_rdbc_tree_bean;
use bmbp_rdbc_type::RdbcIdent;
use bmbp_rdbc_type::RdbcTableIdent;
use bmbp_util::BmbpTree;
use serde::{Deserialize, Serialize};
#[table_rdbc_tree_bean(table = BMBP_RBAC_APP_GROUP,tree=app_group)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpAppGroup {}
