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
use super::bean::BmbpRbacApp;
use super::bean::BmbpRbacAppColumn;

pub struct BmbpRbacAppService;

impl BmbpRbacAppService {
    pub(crate) async fn app_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpRbacApp>,
    ) -> BmbpResp<Option<PageData<BmbpRbacApp>>> {
        let query_wrapper = Self::build_app_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpRbacApp>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn app_find_list(
        depot: &mut Depot,
        params: &BmbpRbacApp,
    ) -> BmbpResp<Option<Vec<BmbpRbacApp>>> {
        let query_wrapper = Self::build_app_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpRbacApp>(orm, &query_wrapper).await
    }

    pub(crate) async fn app_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacApp>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpRbacApp>(orm, params).await
    }

    pub(crate) async fn app_find_info_by_code(
        depot: &mut Depot,
        code: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacApp>> {
        if code.is_none() || code.as_ref().unwrap().is_empty() {
            return Ok(None);
        }
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacApp>();
        query_wrapper.eq_(BmbpRbacAppColumn::AppCode, code.unwrap().clone());
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_one::<BmbpRbacApp>(orm, &query_wrapper).await
    }

    pub(crate) async fn app_save(
        depot: &mut Depot,
        params: &mut BmbpRbacApp,
    ) -> BmbpResp<Option<BmbpRbacApp>> {
        let app_info = Self::app_find_info(depot, params.get_data_id().as_ref()).await?;
        if app_info.is_none() {
            Self::app_insert(depot, params).await
        } else {
            Self::app_update(depot, params).await
        }
    }

    pub(crate) async fn app_insert(
        depot: &mut Depot,
        params: &mut BmbpRbacApp,
    ) -> BmbpResp<Option<BmbpRbacApp>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_app_code().as_ref().is_none()
            || params.get_app_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色编码".to_string()),
            ));
        } else {
            let mut app_code = params.get_app_code().clone().unwrap_or("".to_string());
            app_code = app_code.trim().to_string().replace(".", "-");
            params.set_app_code(Some(app_code));
        }

        if params.get_app_name().as_ref().is_none()
            || params.get_app_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色名称".to_string()),
            ));
        } else {
            let mut app_name = params.get_app_name().clone().unwrap_or("".to_string());
            app_name = app_name.trim().to_string().replace(".", "-");
            params.set_app_name(Some(app_name));
        }

        let app_name = params.get_app_name().clone().unwrap_or("".to_string());
        let app_code = params.get_app_code().clone().unwrap_or("".to_string());

        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_app_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_app_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpRbacApp::get_table());

        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::AppCode.get_ident(),
            params.get_app_code().as_ref().unwrap(),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::AppName.get_ident(),
            params.get_app_name().as_ref().unwrap(),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataCreateTime.get_ident(),
            current_time(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataUpdateTime.get_ident(),
            current_time(),
        );
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacAppColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpRbacAppColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpRbacAppColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpRbacApp>(orm.unwrap(), &insert_wrapper).await?;
        Self::app_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn app_update(
        depot: &mut Depot,
        params: &mut BmbpRbacApp,
    ) -> BmbpResp<Option<BmbpRbacApp>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色标识".to_string()),
            ));
        }

        let app_info_op = Self::app_find_info(depot, params.get_data_id().as_ref()).await?;
        if app_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let app_info = app_info_op.unwrap();

        if params.get_app_code().is_none() {
            params.set_app_code(app_info.get_app_code().clone());
        } else {
            let mut app_code = params.get_app_code().clone().unwrap_or("".to_string());
            app_code = app_code.trim().to_string().replace(".", "-");
            params.set_app_code(Some(app_code));
        }

        if params.get_app_name().is_none() {
            params.set_app_name(app_info.get_app_name().clone());
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(app_info.get_data_sort().clone());
        }

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_app_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_app_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpRbacApp::get_table());
        update_wrapper.set(
            BmbpRbacAppColumn::AppCode,
            params.get_app_code().as_ref().unwrap(),
        );

        update_wrapper.set(
            BmbpRbacAppColumn::AppName,
            params.get_app_name().as_ref().unwrap(),
        );

        update_wrapper.set(BmbpRbacAppColumn::DataSort, params.get_data_sort().unwrap());
        update_wrapper.set(BmbpRbacAppColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpRbacAppColumn::DataUpdateUser, "");

        update_wrapper.eq_(
            BmbpRbacAppColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpRbacApp>(orm, &update_wrapper).await?;
        Self::app_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn app_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::enable::<BmbpRbacApp>(orm, params).await
    }

    pub(crate) async fn app_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::app_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn app_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::disable::<BmbpRbacApp>(orm, params).await
    }

    pub(crate) async fn app_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::app_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn app_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::remove::<BmbpRbacApp>(orm, params).await
    }

    pub(crate) async fn app_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        let mut res = 0u64;
        for id in data_ids {
            let resp = Self::app_remove(depot, Some(&id)).await?;
            if resp.is_some() {
                res += resp.unwrap();
            }
        }
        Ok(Some(res))
    }

    async fn build_app_query_wrapper(
        depot: &mut Depot,
        params_op: Option<&BmbpRbacApp>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacApp>();
        if let Some(params) = params_op {
            if let Some(app_name) = params.get_app_name() {
                query_wrapper.like_(BmbpRbacAppColumn::AppName, app_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpRbacAppColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpRbacAppColumn::DataSort, true);
        query_wrapper.order_by(BmbpRbacAppColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        app_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacApp>();
        query.eq_(BmbpRbacAppColumn::AppCode, app_code.clone());
        query.ne_(BmbpRbacAppColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacApp>(&query).await {
            Ok(app) => {
                if app.is_some() {
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
        app_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacApp>();
        query.eq_(BmbpRbacAppColumn::AppName, app_name.clone());
        query.ne_(BmbpRbacAppColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacApp>(&query).await {
            Ok(app) => {
                if app.is_some() {
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
}
