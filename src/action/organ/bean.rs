use bmbp_marco::bean;
use bmbp_marco::table_rdbc_bean_orm_option;
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

// 组织信息
#[table_rdbc_tree_bean_orm_option(BMBP_RBAC_ORGAN, organ)]
pub struct BmbpRbacOrgan {
    organ_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BmbpRbacOrganInfo {
    GROUP(BmbpRbacOrganGroup),
    UNIT(BmbpRbacOrganUnit),
    DEPT(BmbpRbacOrganDept),
    POST(BmbpRbacOrganPost),
    PERSON(BmbpRbacOrganPerson),
}

// 组织分组信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ORGAN_GROUP)]
pub struct BmbpRbacOrganGroup {
    organ_id: Option<String>,
}
// 组织单位信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ORGAN_UNIT)]
pub struct BmbpRbacOrganUnit {
    organ_id: Option<String>,
}
// 组织部门信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ORGAN_DEPT)]
pub struct BmbpRbacOrganDept {
    organ_id: Option<String>,
}
// 组织岗位信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ORGAN_POST)]
pub struct BmbpRbacOrganPost {
    organ_id: Option<String>,
}
// 组织人员信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ORGAN_PERSON)]
pub struct BmbpRbacOrganPerson {
    organ_id: Option<String>,
}
