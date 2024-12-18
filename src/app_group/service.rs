use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn};
use bmbp_abc::{BmbpError, BmbpResp};
use bmbp_bean::PageVo;
use bmbp_orm::{PageData, BMBP_ORM};
use bmbp_sql::RdbcWhereCondition;
use bmbp_sql::{RdbcDeleteWrapper, RdbcQueryWrapper, RdbcUpdateWrapper};
use bmbp_util::BmbpTreeUtil;
use salvo::Writer;
pub struct Service;

impl Service {
    pub async fn query_tree(group: &mut BmbpAppGroup) -> BmbpResp<Vec<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        query.like(BmbpAppGroupColumn::AppGroupName, &group.app_group_name);
        let group_vec = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .read()
            .await
            .find_list_by_query(&query)
            .await?;
        let group_tree_vec = BmbpTreeUtil::build::<BmbpAppGroup>(group_vec);
        Ok(group_tree_vec)
    }
    pub async fn query_list(group: &mut BmbpAppGroup) -> BmbpResp<Vec<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        query.like(BmbpAppGroupColumn::AppGroupName, &group.app_group_name);
        let group_vec = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .read()
            .await
            .find_list_by_query(&query)
            .await?;
        Ok(group_vec)
    }
    pub(crate) async fn query_page(
        page_vo: &mut PageVo<BmbpAppGroup>,
    ) -> BmbpResp<PageData<BmbpAppGroup>> {
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        if let Some(group) = page_vo.params.as_ref() {
            query.like(BmbpAppGroupColumn::AppGroupCode, &group.app_group_code);
        }
        let group_page = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .read()
            .await
            .find_page_by_query::<BmbpAppGroup>(
                &query,
                page_vo.page_num.clone(),
                page_vo.page_size.clone(),
            )
            .await?;
        Ok(group_page)
    }
    pub(crate) async fn query_info(data_id: &String) -> BmbpResp<Option<BmbpAppGroup>> {
        if data_id.is_empty() {
            return Ok(None);
        }
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        query.eq(BmbpAppGroupColumn::DataId, data_id);
        let group_info = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .read()
            .await
            .find_one_by_query::<BmbpAppGroup>(&query)
            .await?;
        Ok(group_info)
    }

    pub(crate) async fn save(group: &mut BmbpAppGroup) -> BmbpResp<Option<BmbpAppGroup>> {
        if let Some(old_group) = Self::query_info(&group.data_id).await? {
            Self::update(group).await?;
        } else {
            Self::insert(group).await?;
        }
        Self::query_info(&group.data_id).await
    }

    pub(crate) async fn update(group: &mut BmbpAppGroup) -> BmbpResp<()> {
        Ok(())
    }

    pub(crate) async fn insert(group: &mut BmbpAppGroup) -> BmbpResp<()> {
        let orm_lock = BMBP_ORM.get().as_ref().unwrap().write().await;
        let mut conn = orm_lock.get_conn().await?;
        let mut trans = conn.get_transaction().await?;
        trans.commit().await?;
        Ok(())
    }

    pub(crate) async fn enable(data_id: &String) -> BmbpResp<usize> {
        let group_info = Self::query_info(data_id).await?;
        if group_info.is_none() {
            return Err(BmbpError::valid("数据不存在").into());
        }
        let mut group = group_info.unwrap();
        group.data_status = "1".to_string();
        let mut update_wrapper = RdbcUpdateWrapper::with_table::<BmbpAppGroup>();
        update_wrapper.set(BmbpAppGroupColumn::DataStatus, group.data_status);
        update_wrapper.eq(BmbpAppGroupColumn::DataId, data_id);
        let row_count = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .write()
            .await
            .execute_update_by_wrapper(&update_wrapper)
            .await?;
        Ok(row_count)
    }
    pub(crate) async fn disable(data_id: &String) -> BmbpResp<usize> {
        let group_info = Self::query_info(data_id).await?;
        if group_info.is_none() {
            return Err(BmbpError::valid("数据不存在").into());
        }
        let mut group = group_info.unwrap();
        group.data_status = "0".to_string();
        let mut update_wrapper = RdbcUpdateWrapper::with_table::<BmbpAppGroup>();
        update_wrapper.set(BmbpAppGroupColumn::DataStatus, group.data_status);
        update_wrapper.eq(BmbpAppGroupColumn::DataId, data_id);
        let row_count = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .write()
            .await
            .execute_update_by_wrapper(&update_wrapper)
            .await?;
        Ok(row_count)
    }
    pub(crate) async fn remove(data_id: &String) -> BmbpResp<usize> {
        let group_info = Self::query_info(data_id).await?;
        if group_info.is_none() {
            return Err(BmbpError::valid("数据不存在").into());
        }
        let mut delete_wrapper = RdbcDeleteWrapper::with_table::<BmbpAppGroup>();
        delete_wrapper.eq(BmbpAppGroupColumn::DataId, data_id);
        let row_count = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .write()
            .await
            .execute_delete_by_wrapper(&delete_wrapper)
            .await?;
        Ok(row_count)
    }
    pub(crate) async fn batch_disable(data_ids: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
    pub(crate) async fn batch_enable(data_ids: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
    pub(crate) async fn batch_remove(data_ids: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
}
