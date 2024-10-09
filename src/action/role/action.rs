use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacRole;
use super::service::BmbpRbacRoleService;

#[handler]
pub async fn role_find_tree(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacRole>>> {
    let params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_find_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn role_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpRbacRole>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpRbacRole>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpRbacRoleService::role_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色分页成功!".to_string()))
}

#[handler]
pub async fn role_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacRole>>> {
    let params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn role_find_tree_ignore(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacRole>>> {
    let params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_find_tree_ignore(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn role_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacRole>> {
    let role_id = req.query::<String>("dataId");
    let data = BmbpRbacRoleService::role_find_info(depot, role_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn role_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacRole>> {
    let mut params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存角色成功!".to_string()))
}

#[handler]
pub async fn role_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacRole>> {
    let mut params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增角色成功!".to_string()))
}

#[handler]
pub async fn role_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacRole>> {
    let mut params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新角色成功!".to_string()))
}

#[handler]
pub async fn role_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let role_id = req.query::<String>("dataId");
    let data = BmbpRbacRoleService::role_enable(depot, role_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn role_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let role_id = req.query::<String>("dataId");
    let data = BmbpRbacRoleService::role_disable(depot, role_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn role_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let role_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacRoleService::role_batch_enable(depot, &role_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn role_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let role_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacRoleService::role_batch_disable(depot, &role_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn role_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let role_id = req.query::<String>("dataId");
    let data = BmbpRbacRoleService::role_remove(depot, role_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn role_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let role_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacRoleService::role_batch_remove(depot, &role_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn role_update_parent(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let mut params = req.parse_json::<BmbpRbacRole>().await?;
    let data = BmbpRbacRoleService::role_update_parent(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新上级角色成功!".to_string()))
}
