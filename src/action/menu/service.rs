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
use super::bean::BmbpRbacMenu;
use super::bean::BmbpRbacMenuColumn;

pub struct BmbpRbacMenuService;

impl BmbpRbacMenuService {
    pub(crate) async fn menu_find_tree(
        depot: &mut Depot,
        params: &BmbpRbacMenu,
    ) -> BmbpResp<Option<Vec<BmbpRbacMenu>>> {
        let menu_vec = Self::menu_find_list(depot, params).await?;
        if let Some(dic) = menu_vec {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacMenu>(dic)))
        } else {
            Ok(None)
        }
    }
    pub(crate) async fn menu_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpRbacMenu>,
    ) -> BmbpResp<Option<PageData<BmbpRbacMenu>>> {
        let query_wrapper = Self::build_menu_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpRbacMenu>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn menu_find_list(
        depot: &mut Depot,
        params: &BmbpRbacMenu,
    ) -> BmbpResp<Option<Vec<BmbpRbacMenu>>> {
        let query_wrapper = Self::build_menu_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpRbacMenu>(orm, &query_wrapper).await
    }
    pub(crate) async fn menu_find_tree_ignore(
        depot: &mut Depot,
        params: &BmbpRbacMenu,
    ) -> BmbpResp<Option<Vec<BmbpRbacMenu>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacMenu>();
        if let Some(menu_id) = params.get_data_id() {
            let menu = Self::menu_find_info(depot, Some(menu_id)).await?;
            if menu.is_none() {
                return Err(BmbpRespErr::err(
                    Some("DB".to_string()),
                    Some("未找到角色信息".to_string()),
                ));
            }
            query_wrapper.not_like_left_(
                BmbpRbacMenuColumn::MenuCodePath,
                menu.unwrap().get_menu_code_path(),
            );
        }
        if let Some(menu_code) = params.get_menu_code() {
            query_wrapper.not_like_left_(BmbpRbacMenuColumn::MenuCodePath, menu_code.clone());
        }
        if let Some(menu_parent_code) = params.get_menu_parent_code() {
            query_wrapper
                .not_like_left_(BmbpRbacMenuColumn::MenuCodePath, menu_parent_code.clone());
        }
        let orm = parse_orm(depot)?;
        let menu_vec = BmbpCurdDao::execute_query_list::<BmbpRbacMenu>(orm, &query_wrapper).await?;
        if menu_vec.is_some() {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacMenu>(
                menu_vec.unwrap(),
            )))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn menu_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacMenu>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpRbacMenu>(orm, params).await
    }

    pub(crate) async fn menu_find_info_by_code(
        depot: &mut Depot,
        code: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacMenu>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacMenu>();
        query_wrapper.eq_(BmbpRbacMenuColumn::MenuCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpRbacMenu>(orm, &query_wrapper).await
    }

    pub(crate) async fn menu_save(
        depot: &mut Depot,
        params: &mut BmbpRbacMenu,
    ) -> BmbpResp<Option<BmbpRbacMenu>> {
        let menu_info = Self::menu_find_info(depot, params.get_data_id().as_ref()).await?;
        if menu_info.is_none() {
            Self::menu_insert(depot, params).await
        } else {
            Self::menu_update(depot, params).await
        }
    }

    pub(crate) async fn menu_insert(
        depot: &mut Depot,
        params: &mut BmbpRbacMenu,
    ) -> BmbpResp<Option<BmbpRbacMenu>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_menu_code().as_ref().is_none()
            || params.get_menu_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色编码".to_string()),
            ));
        } else {
            let mut menu_code = params.get_menu_code().clone().unwrap_or("".to_string());
            menu_code = menu_code.trim().to_string().replace(".", "-");
            params.set_menu_code(Some(menu_code));
        }

        if params.get_menu_name().as_ref().is_none()
            || params.get_menu_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色名称".to_string()),
            ));
        } else {
            let mut menu_name = params.get_menu_name().clone().unwrap_or("".to_string());
            menu_name = menu_name.trim().to_string().replace(".", "-");
            params.set_menu_name(Some(menu_name));
        }

        if params.get_menu_parent_code().as_ref().is_none()
            || params.get_menu_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_menu_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }

        let menu_name = params.get_menu_name().clone().unwrap_or("".to_string());
        let menu_code = params.get_menu_code().clone().unwrap_or("".to_string());
        if let Some(parent_node) =
            Self::menu_find_info_by_code(depot, params.get_menu_parent_code().as_ref()).await?
        {
            let parent_code_path = parent_node
                .get_menu_code_path()
                .clone()
                .unwrap_or("".to_string());
            let parent_name_path = parent_node
                .get_menu_name_path()
                .clone()
                .unwrap_or("".to_string());
            if parent_code_path.is_empty() || parent_name_path.is_empty() {
                return Err(BmbpRespErr::err(
                    Some("VALID".to_string()),
                    Some("父级节点信息异常,请联系管理员".to_string()),
                ));
            }
            params.set_menu_code_path(Some(format!("{}{}/", parent_code_path, menu_code)));
            params.set_menu_name_path(Some(format!("{}{}/", parent_name_path, menu_name)));
        } else {
            params.set_menu_code_path(Some(format!(
                "{}/{}/",
                params.get_menu_parent_code().as_ref().unwrap(),
                menu_code
            )));
            params.set_menu_name_path(Some(format!(
                "{}/{}/",
                params.get_menu_parent_code().as_ref().unwrap(),
                menu_name
            )));
        }
        // tree_grade;
        let tree_grade = params
            .get_menu_code_path()
            .as_ref()
            .unwrap()
            .split("/")
            .count()
            - 2;
        params.set_menu_tree_grade(Some(tree_grade as u32));
        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_menu_parent_code().clone().unwrap(),
            params.get_menu_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_menu_parent_code().clone().unwrap(),
            params.get_menu_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpRbacMenu::get_table());

        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuCode.get_ident(),
            params.get_menu_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuParentCode.get_ident(),
            params.get_menu_parent_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuName.get_ident(),
            params.get_menu_name().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuCodePath.get_ident(),
            params.get_menu_code_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuNamePath.get_ident(),
            params.get_menu_name_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuTreeGrade.get_ident(),
            params.get_menu_tree_grade().unwrap_or(1),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::MenuOrder.get_ident(),
            params.get_menu_order().clone().unwrap_or(0usize),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataCreateTime.get_ident(),
            current_time(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataUpdateTime.get_ident(),
            current_time(),
        );
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacMenuColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpRbacMenuColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpRbacMenuColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpRbacMenu>(orm.unwrap(), &insert_wrapper).await?;
        Self::menu_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn menu_update(
        depot: &mut Depot,
        params: &mut BmbpRbacMenu,
    ) -> BmbpResp<Option<BmbpRbacMenu>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色标识".to_string()),
            ));
        }

        let menu_info_op = Self::menu_find_info(depot, params.get_data_id().as_ref()).await?;
        if menu_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let menu_info = menu_info_op.unwrap();

        let old_menu_parent_code = menu_info.get_menu_parent_code().clone().unwrap();
        let old_menu_name = menu_info.get_menu_name().clone().unwrap();
        let old_menu_code_path = menu_info.get_menu_code_path().clone().unwrap();
        let old_menu_name_path = menu_info.get_menu_name_path().clone().unwrap();
        if params.get_menu_code().is_none() {
            params.set_menu_code(menu_info.get_menu_code().clone());
        } else {
            let mut menu_code = params.get_menu_code().clone().unwrap_or("".to_string());
            menu_code = menu_code.trim().to_string().replace(".", "-");
            params.set_menu_code(Some(menu_code));
        }
        if params.get_menu_parent_code().is_none() {
            params.set_menu_parent_code(menu_info.get_menu_parent_code().clone());
        }
        if params.get_menu_name().is_none() {
            params.set_menu_name(menu_info.get_menu_name().clone());
        } else {
            let mut menu_name = params.get_menu_name().clone().unwrap_or("".to_string());
            menu_name = menu_name.trim().to_string().replace(".", "-");
            params.set_menu_name(Some(menu_name));
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(menu_info.get_data_sort().clone());
        }
        if params.get_menu_order().is_none() {
            params.set_menu_order(menu_info.get_menu_order().clone());
        }
        let (menu_code_path, menu_name_path) = if params.get_menu_parent_code().as_ref().unwrap()
            == BMBP_TREE_ROOT_NODE
        {
            (
                format!(
                    "{}/{}/",
                    BMBP_TREE_ROOT_NODE.to_string(),
                    params.get_menu_code().as_ref().unwrap()
                ),
                format!(
                    "{}/{}/",
                    BMBP_TREE_ROOT_NODE.to_string(),
                    params.get_menu_name().as_ref().unwrap()
                ),
            )
        } else {
            let parent_node_op =
                Self::menu_find_info_by_code(depot, params.get_menu_parent_code().as_ref()).await?;
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
                    parent_node.get_menu_code_path().clone().unwrap(),
                    params.get_menu_code().clone().unwrap()
                ),
                format!(
                    "{}{}/",
                    parent_node.get_menu_name_path().clone().unwrap(),
                    params.get_menu_name().as_ref().unwrap()
                ),
            )
        };

        let tree_grade = &menu_code_path.split("/").count() - 2;
        params.set_menu_tree_grade(Some(tree_grade as u32));

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_menu_parent_code().clone().unwrap(),
            params.get_menu_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_menu_parent_code().clone().unwrap(),
            params.get_menu_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpRbacMenu::get_table());
        update_wrapper.set(
            BmbpRbacMenuColumn::MenuCode,
            params.get_menu_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacMenuColumn::MenuParentCode,
            params.get_menu_parent_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacMenuColumn::MenuName,
            params.get_menu_name().as_ref().unwrap(),
        );
        update_wrapper.set(BmbpRbacMenuColumn::MenuCodePath, &menu_code_path);
        update_wrapper.set(BmbpRbacMenuColumn::MenuNamePath, &menu_name_path);
        update_wrapper.set(
            BmbpRbacMenuColumn::MenuTreeGrade,
            params.get_menu_tree_grade().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacMenuColumn::DataSort,
            params.get_data_sort().unwrap(),
        );
        update_wrapper.set(BmbpRbacMenuColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpRbacMenuColumn::DataUpdateUser, "");
        update_wrapper.set(
            BmbpRbacMenuColumn::MenuOrder,
            params.get_menu_order().clone(),
        );

        update_wrapper.eq_(
            BmbpRbacMenuColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpRbacMenu>(orm, &update_wrapper).await?;
        if &old_menu_name != params.get_menu_name().as_ref().unwrap()
            || &old_menu_parent_code != params.get_menu_parent_code().as_ref().unwrap()
        {
            Self::update_children_menu_path(
                orm,
                &old_menu_code_path,
                &menu_code_path,
                &old_menu_name_path,
                &menu_name_path,
            )
            .await?;
        }
        Self::menu_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn menu_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let menu_info: Option<BmbpRbacMenu> =
            BmbpCurdService::find_info_by_id::<BmbpRbacMenu>(orm, params).await?;
        if menu_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let code_path = menu_info
            .as_ref()
            .unwrap()
            .get_menu_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let code_vec: Vec<&str> = code_path.split('/').filter(|&s| !s.is_empty()).collect();
        print!("==>{:#?}", code_vec);
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "1");
        update_wrapper.table(BmbpRbacMenu::get_table().get_ident());
        update_wrapper.in_v(BmbpRbacMenuColumn::MenuCode, code_vec);
        BmbpCurdDao::execute_update::<BmbpRbacMenu>(orm, &update_wrapper).await
    }

    pub(crate) async fn menu_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::menu_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn menu_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let menu_info: Option<BmbpRbacMenu> =
            BmbpCurdService::find_info_by_id::<BmbpRbacMenu>(orm, params).await?;
        if menu_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let code_path = menu_info
            .as_ref()
            .unwrap()
            .get_menu_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "0");
        update_wrapper.table(BmbpRbacMenu::get_table());
        update_wrapper.like_left_(BmbpRbacMenuColumn::MenuCodePath, code_path);
        BmbpCurdDao::execute_update::<BmbpRbacMenu>(orm, &update_wrapper).await
    }

    pub(crate) async fn menu_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::menu_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn menu_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;

        let menu_info = BmbpCurdService::find_info_by_id::<BmbpRbacMenu>(orm, params).await?;
        if menu_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let code_path = menu_info
            .as_ref()
            .unwrap()
            .get_menu_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut delete_wrapper = DeleteWrapper::new();
        delete_wrapper.table(BmbpRbacMenu::get_table());
        delete_wrapper.like_left_(BmbpRbacMenuColumn::MenuCodePath, code_path);
        let res = BmbpCurdDao::execute_delete::<BmbpRbacMenu>(orm, &delete_wrapper).await?;
        Ok(res)
    }

    pub(crate) async fn menu_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        let mut res = 0u64;
        for id in data_ids {
            let resp = Self::menu_remove(depot, Some(&id)).await?;
            if resp.is_some() {
                res += resp.unwrap();
            }
        }
        Ok(Some(res))
    }
    pub(crate) async fn menu_update_parent(
        depot: &mut Depot,
        params: &mut BmbpRbacMenu,
    ) -> BmbpResp<Option<u64>> {
        if params.get_menu_parent_code().is_none()
            || params.get_menu_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_menu_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }
        if params.get_data_id().is_none() || params.get_data_id().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("角色标识不能为空".to_string()),
            ));
        }
        Self::menu_update(depot, params).await?;
        Ok(Some(1))
    }

    async fn build_menu_query_wrapper(
        depot: &mut Depot,
        params_op: Option<&BmbpRbacMenu>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacMenu>();
        if let Some(params) = params_op {
            if let Some(menu_id) = params.get_data_id() {
                let menu = Self::menu_find_info(depot, Some(menu_id)).await?;
                if menu.is_none() {
                    return Err(BmbpRespErr::err(
                        Some("DB".to_string()),
                        Some("未找到角色信息".to_string()),
                    ));
                }
                query_wrapper.like_left_(BmbpRbacMenuColumn::MenuCodePath, menu_id.clone());
            }
            if let Some(menu_code) = params.get_menu_code() {
                query_wrapper.like_left_(BmbpRbacMenuColumn::MenuCodePath, menu_code.clone());
            }
            if let Some(menu_parent_code) = params.get_menu_parent_code() {
                query_wrapper.like_(BmbpRbacMenuColumn::MenuCodePath, menu_parent_code.clone());
            }

            if let Some(menu_name) = params.get_menu_name() {
                query_wrapper.like_(BmbpRbacMenuColumn::MenuName, menu_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpRbacMenuColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpRbacMenuColumn::MenuTreeGrade, true);
        query_wrapper.order_by(BmbpRbacMenuColumn::MenuParentCode, true);
        query_wrapper.order_by(BmbpRbacMenuColumn::DataSort, true);
        query_wrapper.order_by(BmbpRbacMenuColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        parent_code: String,
        menu_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacMenu>();
        query.eq_(BmbpRbacMenuColumn::MenuCode, menu_code.clone());
        query.eq_(BmbpRbacMenuColumn::MenuParentCode, parent_code.clone());
        query.ne_(BmbpRbacMenuColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacMenu>(&query).await {
            Ok(menu) => {
                if menu.is_some() {
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
        menu_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacMenu>();
        query.eq_(BmbpRbacMenuColumn::MenuName, menu_name.clone());
        query.eq_(BmbpRbacMenuColumn::MenuParentCode, parent_code.clone());
        query.ne_(BmbpRbacMenuColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacMenu>(&query).await {
            Ok(menu) => {
                if menu.is_some() {
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
    async fn update_children_menu_path(
        orm: &RdbcOrm,
        old_code_path: &String,
        new_code_path: &String,
        old_name_path: &String,
        new_name_path: &String,
    ) -> BmbpResp<Option<u64>> {
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpRbacMenu::get_table().get_ident())
            .set(
                BmbpRbacMenuColumn::MenuNamePath,
                RdbcColumn::replace(
                    BmbpRbacMenuColumn::MenuNamePath.get_ident(),
                    old_name_path,
                    new_name_path,
                ),
            )
            .set(
                BmbpRbacMenuColumn::MenuCodePath,
                RdbcColumn::replace(
                    BmbpRbacMenuColumn::MenuCodePath.get_ident(),
                    old_code_path,
                    new_code_path,
                ),
            );
        update.like_left_(BmbpRbacMenuColumn::MenuCodePath, old_code_path);
        BmbpCurdDao::execute_update::<BmbpRbacMenu>(orm, &update).await
    }
}
