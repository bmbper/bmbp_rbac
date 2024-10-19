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

use super::bean::BatchReqVo;
use super::bean::BmbpRbacAppGroup;
use super::bean::BmbpRbacAppGroupColumn;
use bmbp_curd::{BmbpCurdDao, BmbpCurdService};

pub struct BmbpRbacAppGroupService;

impl BmbpRbacAppGroupService {
    pub(crate) async fn app_group_find_tree(
        depot: &mut Depot,
        params: &BmbpRbacAppGroup,
    ) -> BmbpResp<Option<Vec<BmbpRbacAppGroup>>> {
        let app_group_vec = Self::app_group_find_list(depot, params).await?;
        if let Some(dic) = app_group_vec {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacAppGroup>(dic)))
        } else {
            Ok(None)
        }
    }
    pub(crate) async fn app_group_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpRbacAppGroup>,
    ) -> BmbpResp<Option<PageData<BmbpRbacAppGroup>>> {
        let query_wrapper =
            Self::build_app_group_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpRbacAppGroup>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn app_group_find_list(
        depot: &mut Depot,
        params: &BmbpRbacAppGroup,
    ) -> BmbpResp<Option<Vec<BmbpRbacAppGroup>>> {
        let query_wrapper = Self::build_app_group_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpRbacAppGroup>(orm, &query_wrapper).await
    }
    pub(crate) async fn app_group_find_tree_ignore(
        depot: &mut Depot,
        params: &BmbpRbacAppGroup,
    ) -> BmbpResp<Option<Vec<BmbpRbacAppGroup>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacAppGroup>();
        if let Some(app_group_id) = params.get_data_id() {
            let app_group = Self::app_group_find_info(depot, Some(app_group_id)).await?;
            if app_group.is_none() {
                return Err(BmbpRespErr::err(
                    Some("DB".to_string()),
                    Some("未找到应用分组信息".to_string()),
                ));
            }
            query_wrapper.not_like_left_(
                BmbpRbacAppGroupColumn::AppGroupCodePath,
                app_group.unwrap().get_app_group_code_path(),
            );
        }
        if let Some(app_group_code) = params.get_app_group_code() {
            query_wrapper.not_like_left_(
                BmbpRbacAppGroupColumn::AppGroupCodePath,
                app_group_code.clone(),
            );
        }
        if let Some(app_group_parent_code) = params.get_app_group_parent_code() {
            query_wrapper.not_like_left_(
                BmbpRbacAppGroupColumn::AppGroupCodePath,
                app_group_parent_code.clone(),
            );
        }
        let orm = parse_orm(depot)?;
        let app_group_vec =
            BmbpCurdDao::execute_query_list::<BmbpRbacAppGroup>(orm, &query_wrapper).await?;
        if app_group_vec.is_some() {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacAppGroup>(
                app_group_vec.unwrap(),
            )))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn app_group_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacAppGroup>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpRbacAppGroup>(orm, params).await
    }

    pub(crate) async fn app_group_find_info_by_code(
        depot: &mut Depot,
        code: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacAppGroup>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacAppGroup>();
        query_wrapper.eq_(BmbpRbacAppGroupColumn::AppGroupCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpRbacAppGroup>(orm, &query_wrapper).await
    }

    pub(crate) async fn app_group_save(
        depot: &mut Depot,
        params: &mut BmbpRbacAppGroup,
    ) -> BmbpResp<Option<BmbpRbacAppGroup>> {
        let app_group_info =
            Self::app_group_find_info(depot, params.get_data_id().as_ref()).await?;
        if app_group_info.is_none() {
            Self::app_group_insert(depot, params).await
        } else {
            Self::app_group_update(depot, params).await
        }
    }

    pub(crate) async fn app_group_insert(
        depot: &mut Depot,
        params: &mut BmbpRbacAppGroup,
    ) -> BmbpResp<Option<BmbpRbacAppGroup>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_app_group_code().as_ref().is_none()
            || params.get_app_group_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入应用分组编码".to_string()),
            ));
        } else {
            let mut app_group_code = params
                .get_app_group_code()
                .clone()
                .unwrap_or("".to_string());
            app_group_code = app_group_code.trim().to_string().replace(".", "-");
            params.set_app_group_code(Some(app_group_code));
        }

        if params.get_app_group_name().as_ref().is_none()
            || params.get_app_group_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入应用分组名称".to_string()),
            ));
        } else {
            let mut app_group_name = params
                .get_app_group_name()
                .clone()
                .unwrap_or("".to_string());
            app_group_name = app_group_name.trim().to_string().replace(".", "-");
            params.set_app_group_name(Some(app_group_name));
        }

        if params.get_app_group_parent_code().as_ref().is_none()
            || params
                .get_app_group_parent_code()
                .as_ref()
                .unwrap()
                .is_empty()
        {
            params.set_app_group_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }

        let app_group_name = params
            .get_app_group_name()
            .clone()
            .unwrap_or("".to_string());
        let app_group_code = params
            .get_app_group_code()
            .clone()
            .unwrap_or("".to_string());
        if let Some(parent_node) =
            Self::app_group_find_info_by_code(depot, params.get_app_group_parent_code().as_ref())
                .await?
        {
            let parent_code_path = parent_node
                .get_app_group_code_path()
                .clone()
                .unwrap_or("".to_string());
            let parent_name_path = parent_node
                .get_app_group_name_path()
                .clone()
                .unwrap_or("".to_string());
            if parent_code_path.is_empty() || parent_name_path.is_empty() {
                return Err(BmbpRespErr::err(
                    Some("VALID".to_string()),
                    Some("父级节点信息异常,请联系管理员".to_string()),
                ));
            }
            params
                .set_app_group_code_path(Some(format!("{}{}/", parent_code_path, app_group_code)));
            params
                .set_app_group_name_path(Some(format!("{}{}/", parent_name_path, app_group_name)));
        } else {
            params.set_app_group_code_path(Some(format!(
                "{}/{}/",
                BMBP_TREE_ROOT_NODE, app_group_code
            )));
            params.set_app_group_name_path(Some(format!(
                "{}/{}/",
                BMBP_TREE_ROOT_NODE, app_group_name
            )));
        }
        // tree_grade;
        let tree_grade = params
            .get_app_group_code_path()
            .as_ref()
            .unwrap()
            .split("/")
            .count()
            - 2;
        params.set_app_group_tree_grade(Some(tree_grade as u32));
        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_app_group_parent_code().clone().unwrap(),
            params.get_app_group_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_app_group_parent_code().clone().unwrap(),
            params.get_app_group_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpRbacAppGroup::get_table());

        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupCode.get_ident(),
            params.get_app_group_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupParentCode.get_ident(),
            params.get_app_group_parent_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupName.get_ident(),
            params.get_app_group_name().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupCodePath.get_ident(),
            params.get_app_group_code_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupNamePath.get_ident(),
            params.get_app_group_name_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupTreeGrade.get_ident(),
            params.get_app_group_tree_grade().unwrap_or(1),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupDesc.get_ident(),
            params.get_app_group_desc().as_ref(),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::AppGroupOrder.get_ident(),
            params.get_app_group_order().clone().unwrap_or(0usize),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataCreateTime.get_ident(),
            current_time(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataUpdateTime.get_ident(),
            current_time(),
        );
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppGroupColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpRbacAppGroupColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpRbacAppGroupColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpRbacAppGroup>(orm.unwrap(), &insert_wrapper).await?;
        Self::app_group_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn app_group_update(
        depot: &mut Depot,
        params: &mut BmbpRbacAppGroup,
    ) -> BmbpResp<Option<BmbpRbacAppGroup>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入应用分组标识".to_string()),
            ));
        }

        let app_group_info_op =
            Self::app_group_find_info(depot, params.get_data_id().as_ref()).await?;
        if app_group_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到应用分组信息".to_string()),
            ));
        }
        let app_group_info = app_group_info_op.unwrap();

        let old_app_group_parent_code = app_group_info.get_app_group_parent_code().clone().unwrap();
        let old_app_group_name = app_group_info.get_app_group_name().clone().unwrap();
        let old_app_group_code_path = app_group_info.get_app_group_code_path().clone().unwrap();
        let old_app_group_name_path = app_group_info.get_app_group_name_path().clone().unwrap();
        if params.get_app_group_code().is_none() {
            params.set_app_group_code(app_group_info.get_app_group_code().clone());
        } else {
            let mut app_group_code = params
                .get_app_group_code()
                .clone()
                .unwrap_or("".to_string());
            app_group_code = app_group_code.trim().to_string().replace(".", "-");
            params.set_app_group_code(Some(app_group_code));
        }
        if params.get_app_group_parent_code().is_none() {
            params.set_app_group_parent_code(app_group_info.get_app_group_parent_code().clone());
        }
        if params.get_app_group_name().is_none() {
            params.set_app_group_name(app_group_info.get_app_group_name().clone());
        } else {
            let mut app_group_name = params
                .get_app_group_name()
                .clone()
                .unwrap_or("".to_string());
            app_group_name = app_group_name.trim().to_string().replace(".", "-");
            params.set_app_group_name(Some(app_group_name));
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(app_group_info.get_data_sort().clone());
        }
        if params.get_app_group_desc().is_none() {
            params.set_app_group_desc(app_group_info.get_app_group_desc().clone());
        }
        if params.get_app_group_order().is_none() {
            params.set_app_group_order(app_group_info.get_app_group_order().clone());
        }
        let (app_group_code_path, app_group_name_path) =
            if params.get_app_group_parent_code().as_ref().unwrap() == BMBP_TREE_ROOT_NODE {
                (
                    format!(
                        "{}/{}/",
                        BMBP_TREE_ROOT_NODE.to_string(),
                        params.get_app_group_code().as_ref().unwrap()
                    ),
                    format!(
                        "{}/{}/",
                        BMBP_TREE_ROOT_NODE.to_string(),
                        params.get_app_group_name().as_ref().unwrap()
                    ),
                )
            } else {
                let parent_node_op = Self::app_group_find_info_by_code(
                    depot,
                    params.get_app_group_parent_code().as_ref(),
                )
                .await?;
                if parent_node_op.is_none() {
                    return Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("未找到上级应用分组信息".to_string()),
                    ));
                }

                let parent_node = parent_node_op.unwrap();
                (
                    format!(
                        "{}{}/",
                        parent_node.get_app_group_code_path().clone().unwrap(),
                        params.get_app_group_code().clone().unwrap()
                    ),
                    format!(
                        "{}{}/",
                        parent_node.get_app_group_name_path().clone().unwrap(),
                        params.get_app_group_name().as_ref().unwrap()
                    ),
                )
            };

        let tree_grade = &app_group_code_path.split("/").count() - 2;
        params.set_app_group_tree_grade(Some(tree_grade as u32));

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_app_group_parent_code().clone().unwrap(),
            params.get_app_group_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_app_group_parent_code().clone().unwrap(),
            params.get_app_group_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpRbacAppGroup::get_table());
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupCode,
            params.get_app_group_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupParentCode,
            params.get_app_group_parent_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupName,
            params.get_app_group_name().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupCodePath,
            &app_group_code_path,
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupNamePath,
            &app_group_name_path,
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupTreeGrade,
            params.get_app_group_tree_grade().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupDesc,
            params
                .get_app_group_desc()
                .as_ref()
                .unwrap_or(&"".to_string()),
        );
        update_wrapper.set(
            BmbpRbacAppGroupColumn::DataSort,
            params.get_data_sort().unwrap_or(0),
        );
        update_wrapper.set(BmbpRbacAppGroupColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpRbacAppGroupColumn::DataUpdateUser, "");
        update_wrapper.set(
            BmbpRbacAppGroupColumn::AppGroupOrder,
            params.get_app_group_order().clone(),
        );

        update_wrapper.eq_(
            BmbpRbacAppGroupColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpRbacAppGroup>(orm, &update_wrapper).await?;
        if &old_app_group_name != params.get_app_group_name().as_ref().unwrap()
            || &old_app_group_parent_code != params.get_app_group_parent_code().as_ref().unwrap()
        {
            Self::update_children_app_group_path(
                orm,
                &old_app_group_code_path,
                &app_group_code_path,
                &old_app_group_name_path,
                &app_group_name_path,
            )
            .await?;
        }
        Self::app_group_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn app_group_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let app_group_info: Option<BmbpRbacAppGroup> =
            BmbpCurdService::find_info_by_id::<BmbpRbacAppGroup>(orm, params).await?;
        if app_group_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到应用分组信息".to_string()),
            ));
        }
        let code_path = app_group_info
            .as_ref()
            .unwrap()
            .get_app_group_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let code_vec: Vec<&str> = code_path.split('/').filter(|&s| !s.is_empty()).collect();
        print!("==>{:#?}", code_vec);
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "1");
        update_wrapper.table(BmbpRbacAppGroup::get_table().get_ident());
        update_wrapper.in_v(BmbpRbacAppGroupColumn::AppGroupCode, code_vec);
        BmbpCurdDao::execute_update::<BmbpRbacAppGroup>(orm, &update_wrapper).await
    }

    pub(crate) async fn app_group_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_data_id_list() {
            for id in ids {
                let resp = Self::app_group_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn app_group_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let app_group_info: Option<BmbpRbacAppGroup> =
            BmbpCurdService::find_info_by_id::<BmbpRbacAppGroup>(orm, params).await?;
        if app_group_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到应用分组信息".to_string()),
            ));
        }
        let code_path = app_group_info
            .as_ref()
            .unwrap()
            .get_app_group_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "0");
        update_wrapper.table(BmbpRbacAppGroup::get_table());
        update_wrapper.like_left_(BmbpRbacAppGroupColumn::AppGroupCodePath, code_path);
        BmbpCurdDao::execute_update::<BmbpRbacAppGroup>(orm, &update_wrapper).await
    }

    pub(crate) async fn app_group_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_data_id_list() {
            for id in ids {
                let resp = Self::app_group_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn app_group_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;

        let app_group_info =
            BmbpCurdService::find_info_by_id::<BmbpRbacAppGroup>(orm, params).await?;
        if app_group_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到应用分组信息".to_string()),
            ));
        }

        let group_status = app_group_info
            .as_ref()
            .unwrap()
            .get_data_status()
            .as_ref()
            .unwrap_or(&"".to_string())
            .clone();
        if group_status == "1" {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("已启用状态的分组不能删除！".to_string()),
            ));
        }

        let code_path = app_group_info
            .as_ref()
            .unwrap()
            .get_app_group_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut delete_wrapper = DeleteWrapper::new();
        delete_wrapper.table(BmbpRbacAppGroup::get_table());
        delete_wrapper.like_left_(BmbpRbacAppGroupColumn::AppGroupCodePath, code_path);
        let res = BmbpCurdDao::execute_delete::<BmbpRbacAppGroup>(orm, &delete_wrapper).await?;
        Ok(res)
    }

    pub(crate) async fn app_group_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_data_id_list().clone().unwrap_or(vec![]);
        let orm = parse_orm(depot)?;
        if data_ids.is_empty() {
            return Ok(Some(0));
        }
        let group_vec = Self::app_group_find_list_by_ids(orm, &data_ids).await?;
        if group_vec.is_none() || group_vec.as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到应用分组信息".to_string()),
            ));
        }

        for group in group_vec.as_ref().unwrap() {
            let group_status = group
                .get_data_status()
                .as_ref()
                .unwrap_or(&"".to_string())
                .clone();
            if group_status == "1" {
                return Err(BmbpRespErr::err(
                    Some("SERVICE".to_string()),
                    Some("已启用状态的分组不能删除！".to_string()),
                ));
            }
        }

        let mut row_count = 0u64;
        for group in group_vec.as_ref().unwrap() {
            let code_path = group.get_app_group_code_path().as_ref().unwrap().clone();
            let mut delete_wrapper = DeleteWrapper::new();
            delete_wrapper.table(BmbpRbacAppGroup::get_table());
            delete_wrapper.like_left_(BmbpRbacAppGroupColumn::AppGroupCodePath, code_path);
            let res = BmbpCurdDao::execute_delete::<BmbpRbacAppGroup>(orm, &delete_wrapper).await?;
            if res.is_some() {
                row_count += res.unwrap();
            }
        }
        Ok(Some(row_count))
    }

    async fn app_group_find_list_by_ids(
        orm: &RdbcOrm,
        data_ids: &Vec<String>,
    ) -> BmbpResp<Option<Vec<BmbpRbacAppGroup>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacAppGroup>();
        query_wrapper.in_v(BmbpRbacAppGroupColumn::DataId, data_ids.clone());
        BmbpCurdDao::execute_query_list::<BmbpRbacAppGroup>(orm, &query_wrapper).await
    }

    pub(crate) async fn app_group_update_parent(
        depot: &mut Depot,
        params: &mut BmbpRbacAppGroup,
    ) -> BmbpResp<Option<u64>> {
        if params.get_app_group_parent_code().is_none()
            || params
                .get_app_group_parent_code()
                .as_ref()
                .unwrap()
                .is_empty()
        {
            params.set_app_group_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }
        if params.get_data_id().is_none() || params.get_data_id().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("应用分组标识不能为空".to_string()),
            ));
        }
        Self::app_group_update(depot, params).await?;
        Ok(Some(1))
    }

    async fn build_app_group_query_wrapper(
        _: &mut Depot,
        params_op: Option<&BmbpRbacAppGroup>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacAppGroup>();
        if let Some(params) = params_op {
            if let Some(app_group_id) = params.get_data_id() {
                query_wrapper.eq_(BmbpRbacAppGroupColumn::DataId, app_group_id.clone());
            }
            if let Some(app_group_code) = params.get_app_group_code() {
                query_wrapper.like_(BmbpRbacAppGroupColumn::AppGroupCode, app_group_code.clone());
            }
            if let Some(app_group_parent_code) = params.get_app_group_parent_code() {
                query_wrapper.like_(
                    BmbpRbacAppGroupColumn::AppGroupCodePath,
                    app_group_parent_code.clone(),
                );
            }
            if let Some(app_group_name) = params.get_app_group_name() {
                query_wrapper.like_(BmbpRbacAppGroupColumn::AppGroupName, app_group_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpRbacAppGroupColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpRbacAppGroupColumn::AppGroupTreeGrade, true);
        query_wrapper.order_by(BmbpRbacAppGroupColumn::AppGroupParentCode, true);
        query_wrapper.order_by(BmbpRbacAppGroupColumn::AppGroupOrder, true);
        query_wrapper.order_by(BmbpRbacAppGroupColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        parent_code: String,
        app_group_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacAppGroup>();
        query.eq_(BmbpRbacAppGroupColumn::AppGroupCode, app_group_code.clone());
        query.eq_(
            BmbpRbacAppGroupColumn::AppGroupParentCode,
            parent_code.clone(),
        );
        query.ne_(BmbpRbacAppGroupColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacAppGroup>(&query).await {
            Ok(app_group) => {
                if app_group.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("应用分组编码已存在".to_string()),
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
        app_group_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacAppGroup>();
        query.eq_(BmbpRbacAppGroupColumn::AppGroupName, app_group_name.clone());
        query.eq_(
            BmbpRbacAppGroupColumn::AppGroupParentCode,
            parent_code.clone(),
        );
        query.ne_(BmbpRbacAppGroupColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacAppGroup>(&query).await {
            Ok(app_group) => {
                if app_group.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("应用分组名称已存在".to_string()),
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
    async fn update_children_app_group_path(
        orm: &RdbcOrm,
        old_code_path: &String,
        new_code_path: &String,
        old_name_path: &String,
        new_name_path: &String,
    ) -> BmbpResp<Option<u64>> {
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpRbacAppGroup::get_table().get_ident())
            .set(
                BmbpRbacAppGroupColumn::AppGroupNamePath,
                RdbcColumn::replace(
                    BmbpRbacAppGroupColumn::AppGroupNamePath.get_ident(),
                    old_name_path,
                    new_name_path,
                ),
            )
            .set(
                BmbpRbacAppGroupColumn::AppGroupCodePath,
                RdbcColumn::replace(
                    BmbpRbacAppGroupColumn::AppGroupCodePath.get_ident(),
                    old_code_path,
                    new_code_path,
                ),
            );
        update.like_left_(BmbpRbacAppGroupColumn::AppGroupCodePath, old_code_path);
        BmbpCurdDao::execute_update::<BmbpRbacAppGroup>(orm, &update).await
    }
}
