use crate::app_group::bean::BmbpAppGroup;
use crate::app_group::service::Service;
use bmbp_abc::{BmbpResp, PageResp, VecResp};
use bmbp_bean::{PageRespVo, VecRespVo};
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
    return Ok(VecRespVo::ok_msg(group_tree, "查询成功"));
}

#[handler]
pub async fn query_tree_exclude_node(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("find_tree_with_out");
}
#[handler]
pub async fn query_page(
    req: &mut Request,
    resp: &mut Response,
    depot: &mut Depot,
) -> PageResp<BmbpAppGroup> {
    Ok(PageRespVo::default())
}

#[handler]
pub async fn query_info(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("query_info");
}

#[handler]
pub async fn query_list(req: &mut Request, resp: &mut Response, depot: &mut Depot) {
    resp.render("query_list");
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
