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

// 角色信息
#[table_rdbc_tree_bean_orm_option(BMBP_RBAC_ROLE, role)]
pub struct BmbpRbacRole {
    // 角色类型
    role_type: Option<String>,
    // 角色描述
    role_desc: Option<String>,
    // 管理角色
    is_admin: Option<String>,
}
// 角色互斥信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_EXCLUDE)]
pub struct BmbpRbacRoleExclude {
    // 源角色ID
    source_role_id: Option<String>,
    // 互斥角色ID
    target_role_id: Option<String>,
}

// 角色用户信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_USER)]
pub struct BmbpRbacRoleUser {
    // 角色ID
    role_id: Option<String>,
    // 用户ID
    user_id: Option<String>,
}

// 角色用户组
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_USER_GROUP)]
pub struct BmbpRbacRoleUserGroup {
    // 角色ID
    role_id: Option<String>,
    // 用户组ID
    group_id: Option<String>,
}

// 角色组织信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_ORGAN)]
pub struct BmbpRbacRoleOrgan {
    // 角色ID
    role_id: Option<String>,
    // 组织ID
    organ_id: Option<String>,
}
// 角色组织组信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_ORGAN_GROUP)]
pub struct BmbpRbacRoleOrganGorup {
    // 角色ID
    role_id: Option<String>,
    // 组织组ID
    organ_id: Option<String>,
}

// 角色应用信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_APP)]
pub struct BmbpRbacRoleApp {
    // 角色ID
    role_id: Option<String>,
    // 应用ID
    app_id: Option<String>,
}

// 角色菜单信息
#[table_rdbc_bean_orm_option(BMBP_RBAC_ROLE_APP_MENU)]
pub struct BmbpRbacRoleAppMenu {
    // 角色ID
    role_id: Option<String>,
    // 应用ID
    app_id: Option<String>,
    // 应用ID
    menu_id: Option<String>,
}
