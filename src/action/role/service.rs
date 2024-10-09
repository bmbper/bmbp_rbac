use bmbp_app_util::{parse_orm, parse_user_orm};
use bmbp_http_type::{BmbpPageReq, BmbpResp, BmbpRespErr, PageData};

use bmbp_rdbc::DeleteWrapper;
use bmbp_rdbc::InsertWrapper;
use bmbp_rdbc::QueryWrapper;
use bmbp_rdbc::RdbcColumn;
use bmbp_rdbc::RdbcIdent;
use bmbp_rdbc::RdbcOrm;
use bmbp_rdbc::RdbcTable;
use bmbp_rdbc::RdbcTableFilter;
use bmbp_rdbc::RdbcTableWrapper;
use bmbp_rdbc::UpdateWrapper;
use bmbp_util::{current_time, BmbpId, BmbpTreeUtil, BMBP_TREE_ROOT_NODE};
use salvo::Depot;

use bmbp_curd::{BmbpCurdDao, BmbpCurdService};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacRole;
use super::bean::BmbpRbacRoleColumn;

pub struct BmbpRbacRoleService;

impl BmbpRbacRoleService {
    pub(crate) async fn role_find_tree(
        depot: &mut Depot,
        params: &BmbpRbacRole,
    ) -> BmbpResp<Option<Vec<BmbpRbacRole>>> {
        let role_vec = Self::role_find_list(depot, params).await?;
        if let Some(dic) = role_vec {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacRole>(dic)))
        } else {
            Ok(None)
        }
    }
    pub(crate) async fn role_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpRbacRole>,
    ) -> BmbpResp<Option<PageData<BmbpRbacRole>>> {
        let query_wrapper = Self::build_role_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpRbacRole>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn role_find_list(
        depot: &mut Depot,
        params: &BmbpRbacRole,
    ) -> BmbpResp<Option<Vec<BmbpRbacRole>>> {
        let query_wrapper = Self::build_role_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpRbacRole>(orm, &query_wrapper).await
    }
    pub(crate) async fn role_find_tree_ignore(
        depot: &mut Depot,
        params: &BmbpRbacRole,
    ) -> BmbpResp<Option<Vec<BmbpRbacRole>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacRole>();
        if let Some(role_id) = params.get_data_id() {
            let role = Self::role_find_info(depot, Some(role_id)).await?;
            if role.is_none() {
                return Err(BmbpRespErr::err(
                    Some("DB".to_string()),
                    Some("未找到角色信息".to_string()),
                ));
            }
            query_wrapper.not_like_left_(
                BmbpRbacRoleColumn::RoleCodePath,
                role.unwrap().get_role_code_path(),
            );
        }
        if let Some(role_code) = params.get_role_code() {
            query_wrapper.not_like_left_(BmbpRbacRoleColumn::RoleCodePath, role_code.clone());
        }
        if let Some(role_parent_code) = params.get_role_parent_code() {
            query_wrapper
                .not_like_left_(BmbpRbacRoleColumn::RoleCodePath, role_parent_code.clone());
        }
        let orm = parse_orm(depot)?;
        let role_vec = BmbpCurdDao::execute_query_list::<BmbpRbacRole>(orm, &query_wrapper).await?;
        if role_vec.is_some() {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacRole>(
                role_vec.unwrap(),
            )))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn role_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacRole>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpRbacRole>(orm, params).await
    }

    pub(crate) async fn role_find_info_by_code(
        depot: &mut Depot,
        code: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacRole>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacRole>();
        query_wrapper.eq_(BmbpRbacRoleColumn::RoleCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpRbacRole>(orm, &query_wrapper).await
    }

    pub(crate) async fn role_save(
        depot: &mut Depot,
        params: &mut BmbpRbacRole,
    ) -> BmbpResp<Option<BmbpRbacRole>> {
        let role_info = Self::role_find_info(depot, params.get_data_id().as_ref()).await?;
        if role_info.is_none() {
            Self::role_insert(depot, params).await
        } else {
            Self::role_update(depot, params).await
        }
    }

    pub(crate) async fn role_insert(
        depot: &mut Depot,
        params: &mut BmbpRbacRole,
    ) -> BmbpResp<Option<BmbpRbacRole>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_role_code().as_ref().is_none()
            || params.get_role_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色编码".to_string()),
            ));
        } else {
            let mut role_code = params.get_role_code().clone().unwrap_or("".to_string());
            role_code = role_code.trim().to_string().replace(".", "-");
            params.set_role_code(Some(role_code));
        }

        if params.get_role_name().as_ref().is_none()
            || params.get_role_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色名称".to_string()),
            ));
        } else {
            let mut role_name = params.get_role_name().clone().unwrap_or("".to_string());
            role_name = role_name.trim().to_string().replace(".", "-");
            params.set_role_name(Some(role_name));
        }

        if params.get_role_parent_code().as_ref().is_none()
            || params.get_role_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_role_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }

        let role_name = params.get_role_name().clone().unwrap_or("".to_string());
        let role_code = params.get_role_code().clone().unwrap_or("".to_string());
        if let Some(parent_node) =
            Self::role_find_info_by_code(depot, params.get_role_parent_code().as_ref()).await?
        {
            let parent_code_path = parent_node
                .get_role_code_path()
                .clone()
                .unwrap_or("".to_string());
            let parent_name_path = parent_node
                .get_role_name_path()
                .clone()
                .unwrap_or("".to_string());
            if parent_code_path.is_empty() || parent_name_path.is_empty() {
                return Err(BmbpRespErr::err(
                    Some("VALID".to_string()),
                    Some("父级节点信息异常,请联系管理员".to_string()),
                ));
            }
            params.set_role_code_path(Some(format!("{}{}/", parent_code_path, role_code)));
            params.set_role_name_path(Some(format!("{}{}/", parent_name_path, role_name)));
        } else {
            params.set_role_code_path(Some(format!(
                "{}/{}/",
                params.get_role_parent_code().as_ref().unwrap(),
                role_code
            )));
            params.set_role_name_path(Some(format!(
                "{}/{}/",
                params.get_role_parent_code().as_ref().unwrap(),
                role_name
            )));
        }
        // tree_grade;
        let tree_grade = params
            .get_role_code_path()
            .as_ref()
            .unwrap()
            .split("/")
            .count()
            - 2;
        params.set_role_tree_grade(Some(tree_grade as u32));
        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_role_parent_code().clone().unwrap(),
            params.get_role_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_role_parent_code().clone().unwrap(),
            params.get_role_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpRbacRole::get_table());

        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleCode.get_ident(),
            params.get_role_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleParentCode.get_ident(),
            params.get_role_parent_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleName.get_ident(),
            params.get_role_name().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleCodePath.get_ident(),
            params.get_role_code_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleNamePath.get_ident(),
            params.get_role_name_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleTreeGrade.get_ident(),
            params.get_role_tree_grade().unwrap_or(1),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::RoleOrder.get_ident(),
            params.get_role_order().clone().unwrap_or(0usize),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataCreateTime.get_ident(),
            current_time(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataUpdateTime.get_ident(),
            current_time(),
        );
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacRoleColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpRbacRoleColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpRbacRoleColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpRbacRole>(orm.unwrap(), &insert_wrapper).await?;
        Self::role_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn role_update(
        depot: &mut Depot,
        params: &mut BmbpRbacRole,
    ) -> BmbpResp<Option<BmbpRbacRole>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色标识".to_string()),
            ));
        }

        let role_info_op = Self::role_find_info(depot, params.get_data_id().as_ref()).await?;
        if role_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let role_info = role_info_op.unwrap();

        let old_role_parent_code = role_info.get_role_parent_code().clone().unwrap();
        let old_role_name = role_info.get_role_name().clone().unwrap();
        let old_role_code_path = role_info.get_role_code_path().clone().unwrap();
        let old_role_name_path = role_info.get_role_name_path().clone().unwrap();
        if params.get_role_code().is_none() {
            params.set_role_code(role_info.get_role_code().clone());
        } else {
            let mut role_code = params.get_role_code().clone().unwrap_or("".to_string());
            role_code = role_code.trim().to_string().replace(".", "-");
            params.set_role_code(Some(role_code));
        }
        if params.get_role_parent_code().is_none() {
            params.set_role_parent_code(role_info.get_role_parent_code().clone());
        }
        if params.get_role_name().is_none() {
            params.set_role_name(role_info.get_role_name().clone());
        } else {
            let mut role_name = params.get_role_name().clone().unwrap_or("".to_string());
            role_name = role_name.trim().to_string().replace(".", "-");
            params.set_role_name(Some(role_name));
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(role_info.get_data_sort().clone());
        }
        if params.get_role_order().is_none() {
            params.set_role_order(role_info.get_role_order().clone());
        }
        let (role_code_path, role_name_path) = if params.get_role_parent_code().as_ref().unwrap()
            == BMBP_TREE_ROOT_NODE
        {
            (
                format!(
                    "{}/{}/",
                    BMBP_TREE_ROOT_NODE.to_string(),
                    params.get_role_code().as_ref().unwrap()
                ),
                format!(
                    "{}/{}/",
                    BMBP_TREE_ROOT_NODE.to_string(),
                    params.get_role_name().as_ref().unwrap()
                ),
            )
        } else {
            let parent_node_op =
                Self::role_find_info_by_code(depot, params.get_role_parent_code().as_ref()).await?;
            if parent_node_op.is_none() {
                return Err(BmbpRespErr::err(
                    Some("REQUEST".to_string()),
                    Some("未找到上级角色信息".to_string()),
                ));
            }

            let parent_node = parent_node_op.unwrap();
            (
                format!(
                    "{}{}/",
                    parent_node.get_role_code_path().clone().unwrap(),
                    params.get_role_code().clone().unwrap()
                ),
                format!(
                    "{}{}/",
                    parent_node.get_role_name_path().clone().unwrap(),
                    params.get_role_name().as_ref().unwrap()
                ),
            )
        };

        let tree_grade = &role_code_path.split("/").count() - 2;
        params.set_role_tree_grade(Some(tree_grade as u32));

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_role_parent_code().clone().unwrap(),
            params.get_role_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_role_parent_code().clone().unwrap(),
            params.get_role_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpRbacRole::get_table());
        update_wrapper.set(
            BmbpRbacRoleColumn::RoleCode,
            params.get_role_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacRoleColumn::RoleParentCode,
            params.get_role_parent_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacRoleColumn::RoleName,
            params.get_role_name().as_ref().unwrap(),
        );
        update_wrapper.set(BmbpRbacRoleColumn::RoleCodePath, &role_code_path);
        update_wrapper.set(BmbpRbacRoleColumn::RoleNamePath, &role_name_path);
        update_wrapper.set(
            BmbpRbacRoleColumn::RoleTreeGrade,
            params.get_role_tree_grade().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacRoleColumn::DataSort,
            params.get_data_sort().unwrap(),
        );
        update_wrapper.set(BmbpRbacRoleColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpRbacRoleColumn::DataUpdateUser, "");
        update_wrapper.set(
            BmbpRbacRoleColumn::RoleOrder,
            params.get_role_order().clone(),
        );

        update_wrapper.eq_(
            BmbpRbacRoleColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpRbacRole>(orm, &update_wrapper).await?;
        if &old_role_name != params.get_role_name().as_ref().unwrap()
            || &old_role_parent_code != params.get_role_parent_code().as_ref().unwrap()
        {
            Self::update_children_role_path(
                orm,
                &old_role_code_path,
                &role_code_path,
                &old_role_name_path,
                &role_name_path,
            )
            .await?;
        }
        Self::role_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn role_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let role_info: Option<BmbpRbacRole> =
            BmbpCurdService::find_info_by_id::<BmbpRbacRole>(orm, params).await?;
        if role_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let code_path = role_info
            .as_ref()
            .unwrap()
            .get_role_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let code_vec: Vec<&str> = code_path.split('/').filter(|&s| !s.is_empty()).collect();
        print!("==>{:#?}", code_vec);
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "1");
        update_wrapper.table(BmbpRbacRole::get_table().get_ident());
        update_wrapper.in_v(BmbpRbacRoleColumn::RoleCode, code_vec);
        BmbpCurdDao::execute_update::<BmbpRbacRole>(orm, &update_wrapper).await
    }

    pub(crate) async fn role_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::role_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn role_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let role_info: Option<BmbpRbacRole> =
            BmbpCurdService::find_info_by_id::<BmbpRbacRole>(orm, params).await?;
        if role_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let code_path = role_info
            .as_ref()
            .unwrap()
            .get_role_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "0");
        update_wrapper.table(BmbpRbacRole::get_table());
        update_wrapper.like_left_(BmbpRbacRoleColumn::RoleCodePath, code_path);
        BmbpCurdDao::execute_update::<BmbpRbacRole>(orm, &update_wrapper).await
    }

    pub(crate) async fn role_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::role_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn role_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;

        let role_info = BmbpCurdService::find_info_by_id::<BmbpRbacRole>(orm, params).await?;
        if role_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let code_path = role_info
            .as_ref()
            .unwrap()
            .get_role_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut delete_wrapper = DeleteWrapper::new();
        delete_wrapper.table(BmbpRbacRole::get_table());
        delete_wrapper.like_left_(BmbpRbacRoleColumn::RoleCodePath, code_path);
        let res = BmbpCurdDao::execute_delete::<BmbpRbacRole>(orm, &delete_wrapper).await?;
        Ok(res)
    }

    pub(crate) async fn role_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        let mut res = 0u64;
        for id in data_ids {
            let resp = Self::role_remove(depot, Some(&id)).await?;
            if resp.is_some() {
                res += resp.unwrap();
            }
        }
        Ok(Some(res))
    }
    pub(crate) async fn role_update_parent(
        depot: &mut Depot,
        params: &mut BmbpRbacRole,
    ) -> BmbpResp<Option<u64>> {
        if params.get_role_parent_code().is_none()
            || params.get_role_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_role_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }
        if params.get_data_id().is_none() || params.get_data_id().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("角色标识不能为空".to_string()),
            ));
        }
        Self::role_update(depot, params).await?;
        Ok(Some(1))
    }

    async fn build_role_query_wrapper(
        depot: &mut Depot,
        params_op: Option<&BmbpRbacRole>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacRole>();
        if let Some(params) = params_op {
            if let Some(role_id) = params.get_data_id() {
                let role = Self::role_find_info(depot, Some(role_id)).await?;
                if role.is_none() {
                    return Err(BmbpRespErr::err(
                        Some("DB".to_string()),
                        Some("未找到角色信息".to_string()),
                    ));
                }
                query_wrapper.like_left_(BmbpRbacRoleColumn::RoleCodePath, role_id.clone());
            }
            if let Some(role_code) = params.get_role_code() {
                query_wrapper.like_left_(BmbpRbacRoleColumn::RoleCodePath, role_code.clone());
            }
            if let Some(role_parent_code) = params.get_role_parent_code() {
                query_wrapper.like_(BmbpRbacRoleColumn::RoleCodePath, role_parent_code.clone());
            }

            if let Some(role_name) = params.get_role_name() {
                query_wrapper.like_(BmbpRbacRoleColumn::RoleName, role_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpRbacRoleColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpRbacRoleColumn::RoleTreeGrade, true);
        query_wrapper.order_by(BmbpRbacRoleColumn::RoleParentCode, true);
        query_wrapper.order_by(BmbpRbacRoleColumn::DataSort, true);
        query_wrapper.order_by(BmbpRbacRoleColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        parent_code: String,
        role_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacRole>();
        query.eq_(BmbpRbacRoleColumn::RoleCode, role_code.clone());
        query.eq_(BmbpRbacRoleColumn::RoleParentCode, parent_code.clone());
        query.ne_(BmbpRbacRoleColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacRole>(&query).await {
            Ok(role) => {
                if role.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("角色编码已存在".to_string()),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(BmbpRespErr::err(
                Some("DB".to_string()),
                Some(err.get_msg()),
            )),
        };
    }
    async fn check_same_name(
        orm: &RdbcOrm,
        parent_code: String,
        role_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacRole>();
        query.eq_(BmbpRbacRoleColumn::RoleName, role_name.clone());
        query.eq_(BmbpRbacRoleColumn::RoleParentCode, parent_code.clone());
        query.ne_(BmbpRbacRoleColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacRole>(&query).await {
            Ok(role) => {
                if role.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("角色名称已存在".to_string()),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(BmbpRespErr::err(
                Some("DB".to_string()),
                Some(err.get_msg()),
            )),
        };
    }
    async fn update_children_role_path(
        orm: &RdbcOrm,
        old_code_path: &String,
        new_code_path: &String,
        old_name_path: &String,
        new_name_path: &String,
    ) -> BmbpResp<Option<u64>> {
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpRbacRole::get_table().get_ident())
            .set(
                BmbpRbacRoleColumn::RoleNamePath,
                RdbcColumn::replace(
                    BmbpRbacRoleColumn::RoleNamePath.get_ident(),
                    old_name_path,
                    new_name_path,
                ),
            )
            .set(
                BmbpRbacRoleColumn::RoleCodePath,
                RdbcColumn::replace(
                    BmbpRbacRoleColumn::RoleCodePath.get_ident(),
                    old_code_path,
                    new_code_path,
                ),
            );
        update.like_left_(BmbpRbacRoleColumn::RoleCodePath, old_code_path);
        BmbpCurdDao::execute_update::<BmbpRbacRole>(orm, &update).await
    }
}
