use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacApp;
use super::service::BmbpRbacAppService;

#[handler]
pub async fn app_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpRbacApp>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpRbacApp>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpRbacAppService::app_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色分页成功!".to_string()))
}

#[handler]
pub async fn app_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacApp>>> {
    let params = req.parse_json::<BmbpRbacApp>().await?;
    let data = BmbpRbacAppService::app_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn app_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacApp>> {
    let app_id = req.query::<String>("dataId");
    let data = BmbpRbacAppService::app_find_info(depot, app_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn app_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacApp>> {
    let mut params = req.parse_json::<BmbpRbacApp>().await?;
    let data = BmbpRbacAppService::app_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存角色成功!".to_string()))
}

#[handler]
pub async fn app_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacApp>> {
    let mut params = req.parse_json::<BmbpRbacApp>().await?;
    let data = BmbpRbacAppService::app_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增角色成功!".to_string()))
}

#[handler]
pub async fn app_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacApp>> {
    let mut params = req.parse_json::<BmbpRbacApp>().await?;
    let data = BmbpRbacAppService::app_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新角色成功!".to_string()))
}

#[handler]
pub async fn app_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_id = req.query::<String>("dataId");
    let data = BmbpRbacAppService::app_enable(depot, app_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn app_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_id = req.query::<String>("dataId");
    let data = BmbpRbacAppService::app_disable(depot, app_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn app_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacAppService::app_batch_enable(depot, &app_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn app_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacAppService::app_batch_disable(depot, &app_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn app_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_id = req.query::<String>("dataId");
    let data = BmbpRbacAppService::app_remove(depot, app_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn app_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacAppService::app_batch_remove(depot, &app_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}
