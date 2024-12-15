use crate::app_group::bean::{BmbpAppGroup, BmbpAppGroupColumn};
use bmbp_abc::{BmbpError, BmbpResp, Resp};
use bmbp_bean::PageVo;
use bmbp_orm::{PageData, RdbcTransaction, BMBP_ORM};
use bmbp_sql::{RdbcInsertWrapper, RdbcQueryWrapper};
use bmbp_util::BmbpTreeUtil;

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
        let group_tree_vec = BmbpTreeUtil::build_tree::<BmbpAppGroup>(group_vec);
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

    async fn update(group: &mut BmbpAppGroup) -> BmbpResp<()> {
        let trans_conn: RdbcTransaction = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .write()
            .await
            .get_conn()
            .await?
            .get_transaction()?;
        Ok(())
    }

    async fn insert(group: &mut BmbpAppGroup) -> BmbpResp<()> {
        let trans_conn: RdbcTransaction = BMBP_ORM
            .get()
            .as_ref()
            .unwrap()
            .write()
            .await
            .get_conn()
            .await?
            .get_transaction()?;
        Ok(())
    }
}
