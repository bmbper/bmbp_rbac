use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacUser;
use super::service::BmbpRbacUserService;

#[handler]
pub async fn user_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpRbacUser>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpRbacUser>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpRbacUserService::user_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色分页成功!".to_string()))
}

#[handler]
pub async fn user_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacUser>>> {
    let params = req.parse_json::<BmbpRbacUser>().await?;
    let data = BmbpRbacUserService::user_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn user_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacUser>> {
    let user_id = req.query::<String>("dataId");
    let data = BmbpRbacUserService::user_find_info(depot, user_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn user_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacUser>> {
    let mut params = req.parse_json::<BmbpRbacUser>().await?;
    let data = BmbpRbacUserService::user_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存角色成功!".to_string()))
}

#[handler]
pub async fn user_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacUser>> {
    let mut params = req.parse_json::<BmbpRbacUser>().await?;
    let data = BmbpRbacUserService::user_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增角色成功!".to_string()))
}

#[handler]
pub async fn user_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacUser>> {
    let mut params = req.parse_json::<BmbpRbacUser>().await?;
    let data = BmbpRbacUserService::user_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新角色成功!".to_string()))
}

#[handler]
pub async fn user_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let user_id = req.query::<String>("dataId");
    let data = BmbpRbacUserService::user_enable(depot, user_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn user_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let user_id = req.query::<String>("dataId");
    let data = BmbpRbacUserService::user_disable(depot, user_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn user_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let user_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacUserService::user_batch_enable(depot, &user_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn user_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let user_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacUserService::user_batch_disable(depot, &user_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn user_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let user_id = req.query::<String>("dataId");
    let data = BmbpRbacUserService::user_remove(depot, user_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn user_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let user_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacUserService::user_batch_remove(depot, &user_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}
