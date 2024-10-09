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
use super::bean::BmbpRbacOrgan;
use super::bean::BmbpRbacOrganColumn;

pub struct BmbpRbacOrganService;

impl BmbpRbacOrganService {
    pub(crate) async fn organ_find_tree(
        depot: &mut Depot,
        params: &BmbpRbacOrgan,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        let organ_vec = Self::organ_find_list(depot, params).await?;
        if let Some(dic) = organ_vec {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacOrgan>(dic)))
        } else {
            Ok(None)
        }
    }
    pub(crate) async fn organ_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpRbacOrgan>,
    ) -> BmbpResp<Option<PageData<BmbpRbacOrgan>>> {
        let query_wrapper = Self::build_organ_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpRbacOrgan>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn organ_find_list(
        depot: &mut Depot,
        params: &BmbpRbacOrgan,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        let query_wrapper = Self::build_organ_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpRbacOrgan>(orm, &query_wrapper).await
    }
    pub(crate) async fn organ_find_tree_ignore(
        depot: &mut Depot,
        params: &BmbpRbacOrgan,
    ) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacOrgan>();
        if let Some(organ_id) = params.get_data_id() {
            let organ = Self::organ_find_info(depot, Some(organ_id)).await?;
            if organ.is_none() {
                return Err(BmbpRespErr::err(
                    Some("DB".to_string()),
                    Some("未找到组织信息".to_string()),
                ));
            }
            query_wrapper.not_like_left_(
                BmbpRbacOrganColumn::OrganCodePath,
                organ.unwrap().get_organ_code_path(),
            );
        }
        if let Some(organ_code) = params.get_organ_code() {
            query_wrapper.not_like_left_(BmbpRbacOrganColumn::OrganCodePath, organ_code.clone());
        }
        if let Some(organ_parent_code) = params.get_organ_parent_code() {
            query_wrapper.not_like_left_(
                BmbpRbacOrganColumn::OrganCodePath,
                organ_parent_code.clone(),
            );
        }
        let orm = parse_orm(depot)?;
        let organ_vec =
            BmbpCurdDao::execute_query_list::<BmbpRbacOrgan>(orm, &query_wrapper).await?;
        if organ_vec.is_some() {
            Ok(Some(BmbpTreeUtil::build_tree::<BmbpRbacOrgan>(
                organ_vec.unwrap(),
            )))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn organ_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpRbacOrgan>(orm, params).await
    }

    pub(crate) async fn organ_find_info_by_code(
        depot: &mut Depot,
        code: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacOrgan>();
        query_wrapper.eq_(BmbpRbacOrganColumn::OrganCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpRbacOrgan>(orm, &query_wrapper).await
    }

    pub(crate) async fn organ_save(
        depot: &mut Depot,
        params: &mut BmbpRbacOrgan,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let organ_info = Self::organ_find_info(depot, params.get_data_id().as_ref()).await?;
        if organ_info.is_none() {
            Self::organ_insert(depot, params).await
        } else {
            Self::organ_update(depot, params).await
        }
    }

    pub(crate) async fn organ_insert(
        depot: &mut Depot,
        params: &mut BmbpRbacOrgan,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_organ_code().as_ref().is_none()
            || params.get_organ_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入组织编码".to_string()),
            ));
        } else {
            let mut organ_code = params.get_organ_code().clone().unwrap_or("".to_string());
            organ_code = organ_code.trim().to_string().replace(".", "-");
            params.set_organ_code(Some(organ_code));
        }

        if params.get_organ_name().as_ref().is_none()
            || params.get_organ_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入组织名称".to_string()),
            ));
        } else {
            let mut organ_name = params.get_organ_name().clone().unwrap_or("".to_string());
            organ_name = organ_name.trim().to_string().replace(".", "-");
            params.set_organ_name(Some(organ_name));
        }

        if params.get_organ_parent_code().as_ref().is_none()
            || params.get_organ_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_organ_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }

        let organ_name = params.get_organ_name().clone().unwrap_or("".to_string());
        let organ_code = params.get_organ_code().clone().unwrap_or("".to_string());
        if let Some(parent_node) =
            Self::organ_find_info_by_code(depot, params.get_organ_parent_code().as_ref()).await?
        {
            let parent_code_path = parent_node
                .get_organ_code_path()
                .clone()
                .unwrap_or("".to_string());
            let parent_name_path = parent_node
                .get_organ_name_path()
                .clone()
                .unwrap_or("".to_string());
            if parent_code_path.is_empty() || parent_name_path.is_empty() {
                return Err(BmbpRespErr::err(
                    Some("VALID".to_string()),
                    Some("父级节点信息异常,请联系管理员".to_string()),
                ));
            }
            params.set_organ_code_path(Some(format!("{}{}/", parent_code_path, organ_code)));
            params.set_organ_name_path(Some(format!("{}{}/", parent_name_path, organ_name)));
        } else {
            params.set_organ_code_path(Some(format!(
                "{}/{}/",
                params.get_organ_parent_code().as_ref().unwrap(),
                organ_code
            )));
            params.set_organ_name_path(Some(format!(
                "{}/{}/",
                params.get_organ_parent_code().as_ref().unwrap(),
                organ_name
            )));
        }
        // tree_grade;
        let tree_grade = params
            .get_organ_code_path()
            .as_ref()
            .unwrap()
            .split("/")
            .count()
            - 2;
        params.set_organ_tree_grade(Some(tree_grade as u32));
        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_organ_parent_code().clone().unwrap(),
            params.get_organ_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_organ_parent_code().clone().unwrap(),
            params.get_organ_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpRbacOrgan::get_table());

        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganCode.get_ident(),
            params.get_organ_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganParentCode.get_ident(),
            params.get_organ_parent_code().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganName.get_ident(),
            params.get_organ_name().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganCodePath.get_ident(),
            params.get_organ_code_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganNamePath.get_ident(),
            params.get_organ_name_path().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganTreeGrade.get_ident(),
            params.get_organ_tree_grade().unwrap_or(1),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::OrganOrder.get_ident(),
            params.get_organ_order().clone().unwrap_or(0usize),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataCreateTime.get_ident(),
            current_time(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataUpdateTime.get_ident(),
            current_time(),
        );
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacOrganColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpRbacOrganColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpRbacOrganColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpRbacOrgan>(orm.unwrap(), &insert_wrapper).await?;
        Self::organ_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn organ_update(
        depot: &mut Depot,
        params: &mut BmbpRbacOrgan,
    ) -> BmbpResp<Option<BmbpRbacOrgan>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入组织标识".to_string()),
            ));
        }

        let organ_info_op = Self::organ_find_info(depot, params.get_data_id().as_ref()).await?;
        if organ_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到组织信息".to_string()),
            ));
        }
        let organ_info = organ_info_op.unwrap();

        let old_organ_parent_code = organ_info.get_organ_parent_code().clone().unwrap();
        let old_organ_name = organ_info.get_organ_name().clone().unwrap();
        let old_organ_code_path = organ_info.get_organ_code_path().clone().unwrap();
        let old_organ_name_path = organ_info.get_organ_name_path().clone().unwrap();
        if params.get_organ_code().is_none() {
            params.set_organ_code(organ_info.get_organ_code().clone());
        } else {
            let mut organ_code = params.get_organ_code().clone().unwrap_or("".to_string());
            organ_code = organ_code.trim().to_string().replace(".", "-");
            params.set_organ_code(Some(organ_code));
        }
        if params.get_organ_parent_code().is_none() {
            params.set_organ_parent_code(organ_info.get_organ_parent_code().clone());
        }
        if params.get_organ_name().is_none() {
            params.set_organ_name(organ_info.get_organ_name().clone());
        } else {
            let mut organ_name = params.get_organ_name().clone().unwrap_or("".to_string());
            organ_name = organ_name.trim().to_string().replace(".", "-");
            params.set_organ_name(Some(organ_name));
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(organ_info.get_data_sort().clone());
        }
        if params.get_organ_order().is_none() {
            params.set_organ_order(organ_info.get_organ_order().clone());
        }
        let (organ_code_path, organ_name_path) =
            if params.get_organ_parent_code().as_ref().unwrap() == BMBP_TREE_ROOT_NODE {
                (
                    format!(
                        "{}/{}/",
                        BMBP_TREE_ROOT_NODE.to_string(),
                        params.get_organ_code().as_ref().unwrap()
                    ),
                    format!(
                        "{}/{}/",
                        BMBP_TREE_ROOT_NODE.to_string(),
                        params.get_organ_name().as_ref().unwrap()
                    ),
                )
            } else {
                let parent_node_op =
                    Self::organ_find_info_by_code(depot, params.get_organ_parent_code().as_ref())
                        .await?;
                if parent_node_op.is_none() {
                    return Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("未找到上级组织信息".to_string()),
                    ));
                }

                let parent_node = parent_node_op.unwrap();
                (
                    format!(
                        "{}{}/",
                        parent_node.get_organ_code_path().clone().unwrap(),
                        params.get_organ_code().clone().unwrap()
                    ),
                    format!(
                        "{}{}/",
                        parent_node.get_organ_name_path().clone().unwrap(),
                        params.get_organ_name().as_ref().unwrap()
                    ),
                )
            };

        let tree_grade = &organ_code_path.split("/").count() - 2;
        params.set_organ_tree_grade(Some(tree_grade as u32));

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_organ_parent_code().clone().unwrap(),
            params.get_organ_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_organ_parent_code().clone().unwrap(),
            params.get_organ_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpRbacOrgan::get_table());
        update_wrapper.set(
            BmbpRbacOrganColumn::OrganCode,
            params.get_organ_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacOrganColumn::OrganParentCode,
            params.get_organ_parent_code().as_ref().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacOrganColumn::OrganName,
            params.get_organ_name().as_ref().unwrap(),
        );
        update_wrapper.set(BmbpRbacOrganColumn::OrganCodePath, &organ_code_path);
        update_wrapper.set(BmbpRbacOrganColumn::OrganNamePath, &organ_name_path);
        update_wrapper.set(
            BmbpRbacOrganColumn::OrganTreeGrade,
            params.get_organ_tree_grade().unwrap(),
        );
        update_wrapper.set(
            BmbpRbacOrganColumn::DataSort,
            params.get_data_sort().unwrap(),
        );
        update_wrapper.set(BmbpRbacOrganColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpRbacOrganColumn::DataUpdateUser, "");
        update_wrapper.set(
            BmbpRbacOrganColumn::OrganOrder,
            params.get_organ_order().clone(),
        );

        update_wrapper.eq_(
            BmbpRbacOrganColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpRbacOrgan>(orm, &update_wrapper).await?;
        if &old_organ_name != params.get_organ_name().as_ref().unwrap()
            || &old_organ_parent_code != params.get_organ_parent_code().as_ref().unwrap()
        {
            Self::update_children_organ_path(
                orm,
                &old_organ_code_path,
                &organ_code_path,
                &old_organ_name_path,
                &organ_name_path,
            )
            .await?;
        }
        Self::organ_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn organ_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let organ_info: Option<BmbpRbacOrgan> =
            BmbpCurdService::find_info_by_id::<BmbpRbacOrgan>(orm, params).await?;
        if organ_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到组织信息".to_string()),
            ));
        }
        let code_path = organ_info
            .as_ref()
            .unwrap()
            .get_organ_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let code_vec: Vec<&str> = code_path.split('/').filter(|&s| !s.is_empty()).collect();
        print!("==>{:#?}", code_vec);
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "1");
        update_wrapper.table(BmbpRbacOrgan::get_table().get_ident());
        update_wrapper.in_v(BmbpRbacOrganColumn::OrganCode, code_vec);
        BmbpCurdDao::execute_update::<BmbpRbacOrgan>(orm, &update_wrapper).await
    }

    pub(crate) async fn organ_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::organ_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn organ_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        let organ_info: Option<BmbpRbacOrgan> =
            BmbpCurdService::find_info_by_id::<BmbpRbacOrgan>(orm, params).await?;
        if organ_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到组织信息".to_string()),
            ));
        }
        let code_path = organ_info
            .as_ref()
            .unwrap()
            .get_organ_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.set("data_status", "0");
        update_wrapper.table(BmbpRbacOrgan::get_table());
        update_wrapper.like_left_(BmbpRbacOrganColumn::OrganCodePath, code_path);
        BmbpCurdDao::execute_update::<BmbpRbacOrgan>(orm, &update_wrapper).await
    }

    pub(crate) async fn organ_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::organ_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn organ_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;

        let organ_info = BmbpCurdService::find_info_by_id::<BmbpRbacOrgan>(orm, params).await?;
        if organ_info.is_none() {
            return Err(BmbpRespErr::err(
                Some("SERVICE".to_string()),
                Some("未找到组织信息".to_string()),
            ));
        }
        let code_path = organ_info
            .as_ref()
            .unwrap()
            .get_organ_code_path()
            .as_ref()
            .unwrap()
            .clone();
        let mut delete_wrapper = DeleteWrapper::new();
        delete_wrapper.table(BmbpRbacOrgan::get_table());
        delete_wrapper.like_left_(BmbpRbacOrganColumn::OrganCodePath, code_path);
        let res = BmbpCurdDao::execute_delete::<BmbpRbacOrgan>(orm, &delete_wrapper).await?;
        Ok(res)
    }

    pub(crate) async fn organ_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        let mut res = 0u64;
        for id in data_ids {
            let resp = Self::organ_remove(depot, Some(&id)).await?;
            if resp.is_some() {
                res += resp.unwrap();
            }
        }
        Ok(Some(res))
    }
    pub(crate) async fn organ_update_parent(
        depot: &mut Depot,
        params: &mut BmbpRbacOrgan,
    ) -> BmbpResp<Option<u64>> {
        if params.get_organ_parent_code().is_none()
            || params.get_organ_parent_code().as_ref().unwrap().is_empty()
        {
            params.set_organ_parent_code(Some(BMBP_TREE_ROOT_NODE.to_string()));
        }
        if params.get_data_id().is_none() || params.get_data_id().as_ref().unwrap().is_empty() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("组织标识不能为空".to_string()),
            ));
        }
        Self::organ_update(depot, params).await?;
        Ok(Some(1))
    }

    async fn build_organ_query_wrapper(
        depot: &mut Depot,
        params_op: Option<&BmbpRbacOrgan>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacOrgan>();
        if let Some(params) = params_op {
            if let Some(organ_id) = params.get_data_id() {
                let organ = Self::organ_find_info(depot, Some(organ_id)).await?;
                if organ.is_none() {
                    return Err(BmbpRespErr::err(
                        Some("DB".to_string()),
                        Some("未找到组织信息".to_string()),
                    ));
                }
                query_wrapper.like_left_(BmbpRbacOrganColumn::OrganCodePath, organ_id.clone());
            }
            if let Some(organ_code) = params.get_organ_code() {
                query_wrapper.like_left_(BmbpRbacOrganColumn::OrganCodePath, organ_code.clone());
            }
            if let Some(organ_parent_code) = params.get_organ_parent_code() {
                query_wrapper.like_(
                    BmbpRbacOrganColumn::OrganCodePath,
                    organ_parent_code.clone(),
                );
            }

            if let Some(organ_name) = params.get_organ_name() {
                query_wrapper.like_(BmbpRbacOrganColumn::OrganName, organ_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpRbacOrganColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpRbacOrganColumn::OrganTreeGrade, true);
        query_wrapper.order_by(BmbpRbacOrganColumn::OrganParentCode, true);
        query_wrapper.order_by(BmbpRbacOrganColumn::DataSort, true);
        query_wrapper.order_by(BmbpRbacOrganColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        parent_code: String,
        organ_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacOrgan>();
        query.eq_(BmbpRbacOrganColumn::OrganCode, organ_code.clone());
        query.eq_(BmbpRbacOrganColumn::OrganParentCode, parent_code.clone());
        query.ne_(BmbpRbacOrganColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacOrgan>(&query).await {
            Ok(organ) => {
                if organ.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("组织编码已存在".to_string()),
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
        organ_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacOrgan>();
        query.eq_(BmbpRbacOrganColumn::OrganName, organ_name.clone());
        query.eq_(BmbpRbacOrganColumn::OrganParentCode, parent_code.clone());
        query.ne_(BmbpRbacOrganColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacOrgan>(&query).await {
            Ok(organ) => {
                if organ.is_some() {
                    Err(BmbpRespErr::err(
                        Some("REQUEST".to_string()),
                        Some("组织名称已存在".to_string()),
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
    async fn update_children_organ_path(
        orm: &RdbcOrm,
        old_code_path: &String,
        new_code_path: &String,
        old_name_path: &String,
        new_name_path: &String,
    ) -> BmbpResp<Option<u64>> {
        let mut update = UpdateWrapper::new();
        update
            .table(BmbpRbacOrgan::get_table().get_ident())
            .set(
                BmbpRbacOrganColumn::OrganNamePath,
                RdbcColumn::replace(
                    BmbpRbacOrganColumn::OrganNamePath.get_ident(),
                    old_name_path,
                    new_name_path,
                ),
            )
            .set(
                BmbpRbacOrganColumn::OrganCodePath,
                RdbcColumn::replace(
                    BmbpRbacOrganColumn::OrganCodePath.get_ident(),
                    old_code_path,
                    new_code_path,
                ),
            );
        update.like_left_(BmbpRbacOrganColumn::OrganCodePath, old_code_path);
        BmbpCurdDao::execute_update::<BmbpRbacOrgan>(orm, &update).await
    }
}
