use bmbp_app_util::{parse_orm, parse_user_orm};
use bmbp_http_type::{BmbpPageReq, BmbpResp, BmbpRespErr, PageData};

use bmbp_rdbc::InsertWrapper;
use bmbp_rdbc::QueryWrapper;

use bmbp_rdbc::RdbcIdent;
use bmbp_rdbc::RdbcOrm;
use bmbp_rdbc::RdbcTable;
use bmbp_rdbc::RdbcTableFilter;
use bmbp_rdbc::RdbcTableWrapper;
use bmbp_rdbc::UpdateWrapper;
use bmbp_util::{current_time, BmbpId};
use salvo::Depot;

use bmbp_curd::{BmbpCurdDao, BmbpCurdService};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacUser;
use super::bean::BmbpRbacUserColumn;

pub struct BmbpRbacUserService;

impl BmbpRbacUserService {
    pub(crate) async fn user_find_page(
        depot: &mut Depot,
        page_req: &BmbpPageReq<BmbpRbacUser>,
    ) -> BmbpResp<Option<PageData<BmbpRbacUser>>> {
        let query_wrapper = Self::build_user_query_wrapper(depot, page_req.get_params()).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_page::<BmbpRbacUser>(
            orm,
            Some(page_req.get_page_no().clone()),
            Some(page_req.get_page_size().clone()),
            &query_wrapper,
        )
        .await
    }
    pub(crate) async fn user_find_list(
        depot: &mut Depot,
        params: &BmbpRbacUser,
    ) -> BmbpResp<Option<Vec<BmbpRbacUser>>> {
        let query_wrapper = Self::build_user_query_wrapper(depot, Some(params)).await?;
        let orm = parse_orm(depot)?;
        BmbpCurdDao::execute_query_list::<BmbpRbacUser>(orm, &query_wrapper).await
    }

    pub(crate) async fn user_find_info(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<BmbpRbacUser>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::find_info_by_id::<BmbpRbacUser>(orm, params).await
    }

    pub(crate) async fn user_save(
        depot: &mut Depot,
        params: &mut BmbpRbacUser,
    ) -> BmbpResp<Option<BmbpRbacUser>> {
        let user_info = Self::user_find_info(depot, params.get_data_id().as_ref()).await?;
        if user_info.is_none() {
            Self::user_insert(depot, params).await
        } else {
            Self::user_update(depot, params).await
        }
    }

    pub(crate) async fn user_insert(
        depot: &mut Depot,
        params: &mut BmbpRbacUser,
    ) -> BmbpResp<Option<BmbpRbacUser>> {
        if params.get_data_id().is_none() {
            params.set_data_id(Some(BmbpId::simple_uuid()));
        }

        if params.get_user_code().as_ref().is_none()
            || params.get_user_code().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色编码".to_string()),
            ));
        } else {
            let mut user_code = params.get_user_code().clone().unwrap_or("".to_string());
            user_code = user_code.trim().to_string().replace(".", "-");
            params.set_user_code(Some(user_code));
        }

        if params.get_user_name().as_ref().is_none()
            || params.get_user_name().as_ref().unwrap().is_empty()
        {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色名称".to_string()),
            ));
        } else {
            let mut user_name = params.get_user_name().clone().unwrap_or("".to_string());
            user_name = user_name.trim().to_string().replace(".", "-");
            params.set_user_name(Some(user_name));
        }

        let (user, orm) = parse_user_orm(depot);
        // 校验编码是否重复
        Self::check_same_code(
            orm.unwrap(),
            params.get_user_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm.unwrap(),
            params.get_user_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut insert_wrapper = InsertWrapper::new();
        insert_wrapper.table(BmbpRbacUser::get_table());

        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::UserCode.get_ident(),
            params.get_user_code().as_ref().unwrap(),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::UserName.get_ident(),
            params.get_user_name().as_ref().unwrap(),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataId.get_ident(),
            params.get_data_id().as_ref().unwrap(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataLevel.get_ident(),
            params.get_data_level().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataFlag.get_ident(),
            params.get_data_flag().clone().unwrap_or("0".to_string()),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataSort.get_ident(),
            params.get_data_sort().unwrap_or(0),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataStatus.get_ident(),
            params.get_data_status().clone().unwrap_or("0".to_string()),
        );

        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataCreateTime.get_ident(),
            current_time(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataUpdateTime.get_ident(),
            current_time(),
        );
        let current_user = match user {
            Some(u) => u.get_id().clone().unwrap_or("".to_string()),
            None => "".to_string(),
        };
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataCreateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(
            BmbpRbacUserColumn::DataUpdateUser.get_ident(),
            current_user.clone(),
        );
        insert_wrapper.insert_column_value(BmbpRbacUserColumn::DataOwnerOrg.get_ident(), "");
        insert_wrapper.insert_column_value(BmbpRbacUserColumn::DataSign.get_ident(), "");

        BmbpCurdDao::execute_insert::<BmbpRbacUser>(orm.unwrap(), &insert_wrapper).await?;
        Self::user_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn user_update(
        depot: &mut Depot,
        params: &mut BmbpRbacUser,
    ) -> BmbpResp<Option<BmbpRbacUser>> {
        if params.get_data_id().is_none() {
            return Err(BmbpRespErr::err(
                Some("VALID".to_string()),
                Some("请传入角色标识".to_string()),
            ));
        }

        let user_info_op = Self::user_find_info(depot, params.get_data_id().as_ref()).await?;
        if user_info_op.is_none() {
            return Err(BmbpRespErr::err(
                Some("REQUEST".to_string()),
                Some("未找到角色信息".to_string()),
            ));
        }
        let user_info = user_info_op.unwrap();

        if params.get_user_code().is_none() {
            params.set_user_code(user_info.get_user_code().clone());
        } else {
            let mut user_code = params.get_user_code().clone().unwrap_or("".to_string());
            user_code = user_code.trim().to_string().replace(".", "-");
            params.set_user_code(Some(user_code));
        }

        if params.get_user_name().is_none() {
            params.set_user_name(user_info.get_user_name().clone());
        }
        if params.get_data_sort().is_none() {
            params.set_data_sort(user_info.get_data_sort().clone());
        }

        // 校验别名是否重复
        let orm = parse_orm(depot)?;
        // 校验编码是否重复
        Self::check_same_code(
            orm,
            params.get_user_code().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        Self::check_same_name(
            orm,
            params.get_user_name().clone().unwrap(),
            params.get_data_id().clone(),
        )
        .await?;
        let mut update_wrapper = UpdateWrapper::new();
        update_wrapper.table(BmbpRbacUser::get_table());
        update_wrapper.set(
            BmbpRbacUserColumn::UserCode,
            params.get_user_code().as_ref().unwrap(),
        );

        update_wrapper.set(
            BmbpRbacUserColumn::UserName,
            params.get_user_name().as_ref().unwrap(),
        );

        update_wrapper.set(
            BmbpRbacUserColumn::DataSort,
            params.get_data_sort().unwrap(),
        );
        update_wrapper.set(BmbpRbacUserColumn::DataUpdateTime, current_time());
        update_wrapper.set(BmbpRbacUserColumn::DataUpdateUser, "");

        update_wrapper.eq_(
            BmbpRbacUserColumn::DataId,
            params.get_data_id().as_ref().unwrap(),
        );

        BmbpCurdDao::execute_update::<BmbpRbacUser>(orm, &update_wrapper).await?;
        Self::user_find_info(depot, params.get_data_id().as_ref()).await
    }

    pub(crate) async fn user_enable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::enable::<BmbpRbacUser>(orm, params).await
    }

    pub(crate) async fn user_batch_enable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::user_enable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn user_disable(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::disable::<BmbpRbacUser>(orm, params).await
    }

    pub(crate) async fn user_batch_disable(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let mut u64 = 0;
        if let Some(ids) = params.get_ids() {
            for id in ids {
                let resp = Self::user_disable(depot, Some(id)).await?;
                if resp.is_some() {
                    u64 += resp.unwrap();
                }
            }
        }
        Ok(Some(u64))
    }

    pub(crate) async fn user_remove(
        depot: &mut Depot,
        params: Option<&String>,
    ) -> BmbpResp<Option<u64>> {
        let orm = parse_orm(depot)?;
        BmbpCurdService::remove::<BmbpRbacUser>(orm, params).await
    }

    pub(crate) async fn user_batch_remove(
        depot: &mut Depot,
        params: &BatchReqVo,
    ) -> BmbpResp<Option<u64>> {
        let data_ids = params.get_ids().clone().unwrap_or(vec![]);
        let mut res = 0u64;
        for id in data_ids {
            let resp = Self::user_remove(depot, Some(&id)).await?;
            if resp.is_some() {
                res += resp.unwrap();
            }
        }
        Ok(Some(res))
    }

    async fn build_user_query_wrapper(
        _depot: &mut Depot,
        params_op: Option<&BmbpRbacUser>,
    ) -> BmbpResp<QueryWrapper> {
        let mut query_wrapper = QueryWrapper::new_from::<BmbpRbacUser>();
        if let Some(params) = params_op {
            if let Some(user_name) = params.get_user_name() {
                query_wrapper.like_(BmbpRbacUserColumn::UserName, user_name.clone());
            }
            if let Some(data_status) = params.get_data_status() {
                query_wrapper.eq_(BmbpRbacUserColumn::DataStatus, data_status.clone());
            }
        }
        query_wrapper.order_by(BmbpRbacUserColumn::DataSort, true);
        query_wrapper.order_by(BmbpRbacUserColumn::DataCreateTime, true);
        Ok(query_wrapper)
    }

    async fn check_same_code(
        orm: &RdbcOrm,
        user_code: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacUser>();
        query.eq_(BmbpRbacUserColumn::UserCode, user_code.clone());
        query.ne_(BmbpRbacUserColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacUser>(&query).await {
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
        user_name: String,
        data_id: Option<String>,
    ) -> BmbpResp<()> {
        let mut query = QueryWrapper::new_from::<BmbpRbacUser>();
        query.eq_(BmbpRbacUserColumn::UserName, user_name.clone());
        query.ne_(BmbpRbacUserColumn::DataId, data_id.clone());
        return match orm.select_one_by_query::<BmbpRbacUser>(&query).await {
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
