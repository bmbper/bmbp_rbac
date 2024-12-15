use crate::app_group::bean::BmbpAppGroup;
use crate::app_group::service::Service;
use bmbp_abc::{BmbpError, BmbpErrorKind, Resp, VecResp};
use bmbp_bean::{BatchVo, PageVo, RespVo, VecRespVo};
use bmbp_orm::PageData;
use salvo::prelude::*;

#[handler]
pub async fn query_tree(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> VecResp<BmbpAppGroup> {
    let mut group_query = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let group_tree = Service::query_tree(&mut group_query).await?;
    Ok(Json(VecRespVo::<BmbpAppGroup>::ok_msg(
        group_tree,
        "查询成功",
    )))
}

#[handler]
pub async fn query_tree_exclude_node(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> VecResp<BmbpAppGroup> {
    let mut group_query = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let group_tree = Service::query_tree(&mut group_query).await?;
    Ok(Json(VecRespVo::<BmbpAppGroup>::ok_msg(
        group_tree,
        "查询成功",
    )))
}
#[handler]
pub async fn query_page(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Resp<PageData<BmbpAppGroup>> {
    let mut group_query = req
        .parse_json::<PageVo<BmbpAppGroup>>()
        .await
        .unwrap_or(PageVo::default());
    let group_tree = Service::query_page(&mut group_query).await?;
    Ok(Json(RespVo::<PageData<BmbpAppGroup>>::query_ok(group_tree)))
}

#[handler]
pub async fn query_info(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Resp<BmbpAppGroup> {
    let mut group_query = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let group_info = Service::query_info(&group_query.data_id).await?;
    Ok(Json(RespVo::<BmbpAppGroup>::query_op_ok(group_info)))
}

#[handler]
pub async fn query_list(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> VecResp<BmbpAppGroup> {
    let mut group_query = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let group_tree = Service::query_list(&mut group_query).await?;
    Ok(Json(VecRespVo::<BmbpAppGroup>::ok_msg(
        group_tree,
        "查询成功",
    )))
}

#[handler]
pub async fn save(req: &mut Request, resp: &mut Response, depot: &mut Depot) -> Resp<BmbpAppGroup> {
    let mut group_query = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let group_info = Service::save(&mut group_query).await?;
    Ok(Json(RespVo::<BmbpAppGroup>::ok_op_msg(
        group_info,
        "保存成功",
    )))
}

#[handler]
pub async fn insert(req: &mut Request, resp: &mut Response, depot: &mut Depot) -> Resp<String> {
    let mut group = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    Service::insert(&mut group).await?;
    Ok(Json(RespVo::<String>::ok_msg(
        group.data_id.to_string(),
        "新增成功",
    )))
}

#[handler]
pub async fn update(req: &mut Request, resp: &mut Response, depot: &mut Depot) -> Resp<String> {
    let mut group = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    Service::update(&mut group).await?;
    Ok(Json(RespVo::<String>::ok_msg(
        group.data_id.to_string(),
        "更新成功",
    )))
}

#[handler]
pub async fn enable(req: &mut Request, resp: &mut Response, depot: &mut Depot) -> Resp<usize> {
    let mut group = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let row_count = Service::enable(&group.data_id).await?;
    Ok(Json(RespVo::<usize>::ok_msg(row_count, "启用成功")))
}

#[handler]
pub async fn disable(req: &mut Request, resp: &mut Response, depot: &mut Depot) -> Resp<usize> {
    let mut group = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let row_count = Service::disable(&group.data_id).await?;
    Ok(Json(RespVo::<usize>::ok_msg(row_count, "停用成功")))
}
#[handler]
pub async fn remove(req: &mut Request, resp: &mut Response, depot: &mut Depot) -> Resp<usize> {
    let mut group = req
        .parse_json::<BmbpAppGroup>()
        .await
        .unwrap_or(BmbpAppGroup::default());
    let row_count = Service::remove(&group.data_id).await?;
    Ok(Json(RespVo::<usize>::ok_msg(row_count, "删除成功")))
}
#[handler]
pub async fn batch_enable(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Resp<usize> {
    let mut group = req
        .parse_json::<BatchVo<String>>()
        .await
        .unwrap_or(BatchVo::default());
    let row_count = Service::batch_enable(group.batch_vo.as_slice()).await?;
    Ok(Json(RespVo::<usize>::ok_msg(row_count, "批量启用成功")))
}

#[handler]
pub async fn batch_disable(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Resp<usize> {
    let mut group = req
        .parse_json::<BatchVo<String>>()
        .await
        .unwrap_or(BatchVo::default());
    let row_count = Service::batch_enable(group.batch_vo.as_slice()).await?;
    Ok(Json(RespVo::<usize>::ok_msg(row_count, "批量停用成功")))
}

#[handler]
pub async fn batch_remove(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Resp<usize> {
    let mut group = req
        .parse_json::<BatchVo<String>>()
        .await
        .unwrap_or(BatchVo::default());
    let row_count = Service::batch_remove(group.batch_vo.as_slice()).await?;
    Ok(Json(RespVo::<usize>::ok_msg(row_count, "批量删除成功")))
}

#[handler]
pub async fn update_parent(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> Resp<usize> {
    Ok(Json(RespVo::err_msg("接口未实现")))
}
