use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacMenu;
use super::service::BmbpRbacMenuService;

#[handler]
pub async fn menu_find_tree(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacMenu>>> {
    let params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_find_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn menu_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpRbacMenu>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpRbacMenu>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpRbacMenuService::menu_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色分页成功!".to_string()))
}

#[handler]
pub async fn menu_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacMenu>>> {
    let params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn menu_find_tree_ignore(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacMenu>>> {
    let params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_find_tree_ignore(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn menu_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacMenu>> {
    let menu_id = req.query::<String>("dataId");
    let data = BmbpRbacMenuService::menu_find_info(depot, menu_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn menu_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacMenu>> {
    let mut params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存角色成功!".to_string()))
}

#[handler]
pub async fn menu_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacMenu>> {
    let mut params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增角色成功!".to_string()))
}

#[handler]
pub async fn menu_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacMenu>> {
    let mut params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新角色成功!".to_string()))
}

#[handler]
pub async fn menu_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let menu_id = req.query::<String>("dataId");
    let data = BmbpRbacMenuService::menu_enable(depot, menu_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn menu_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let menu_id = req.query::<String>("dataId");
    let data = BmbpRbacMenuService::menu_disable(depot, menu_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn menu_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let menu_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacMenuService::menu_batch_enable(depot, &menu_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn menu_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let menu_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacMenuService::menu_batch_disable(depot, &menu_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn menu_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let menu_id = req.query::<String>("dataId");
    let data = BmbpRbacMenuService::menu_remove(depot, menu_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn menu_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let menu_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacMenuService::menu_batch_remove(depot, &menu_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn menu_update_parent(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let mut params = req.parse_json::<BmbpRbacMenu>().await?;
    let data = BmbpRbacMenuService::menu_update_parent(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新上级角色成功!".to_string()))
}
