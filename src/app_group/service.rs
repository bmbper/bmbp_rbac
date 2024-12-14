use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn};
use bmbp_abc::{BmbpError, BmbpResp};
use bmbp_bean::PageVo;
use bmbp_orm::{PageData, BMBP_ORM};
use bmbp_sql::RdbcQueryWrapper;

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
    pub(crate) async fn query_info(group: &mut BmbpAppGroup) -> BmbpResp<Option<BmbpAppGroup>> {
        if (&group.data_id).is_empty() {
            return Err(BmbpError::valid("data_id 不能为空"));
        }
        let mut query = RdbcQueryWrapper::with_table::<BmbpAppGroup>();
        query.eq(BmbpAppGroupColumn::DataId, &group.data_id);
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
}
