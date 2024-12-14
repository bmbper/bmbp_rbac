use crate::app_group::bean::BmbpAppGroup;
use crate::app_group::service::Service;
use bmbp_abc::{Resp, VecResp};
use bmbp_bean::{PageVo, RespVo, VecRespVo};
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
    Ok(Json(RespVo::<PageData<BmbpAppGroup>>::query_ok(
        group_tree,
        "查询成功",
    )))
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
    let group_info = Service::query_info(&mut group_query).await?;
    Ok(Json(RespVo::<BmbpAppGroup>::query_op_ok(
        group_info,
        "查询成功",
    )))
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
pub async fn save(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("save");
}

#[handler]
pub async fn insert(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("insert");
}

#[handler]
pub async fn update(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("update");
}

#[handler]
pub async fn enable(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("enable");
}

#[handler]
pub async fn disable(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("disable");
}

#[handler]
pub async fn remove(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("remove");
}

#[handler]
pub async fn batch_enable(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("enable");
}

#[handler]
pub async fn batch_disable(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("disable");
}

#[handler]
pub async fn batch_remove(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("remove");
}

#[handler]
pub async fn update_parent(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("remove");
}
