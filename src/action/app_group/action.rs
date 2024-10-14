use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacAppGroup;
use super::service::BmbpRbacAppGroupService;

#[handler]
pub async fn app_group_find_tree(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacAppGroup>>> {
    let params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_find_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn app_group_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpRbacAppGroup>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpRbacAppGroup>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpRbacAppGroupService::app_group_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色分页成功!".to_string()))
}

#[handler]
pub async fn app_group_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacAppGroup>>> {
    let params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn app_group_find_tree_ignore(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacAppGroup>>> {
    let params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_find_tree_ignore(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn app_group_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacAppGroup>> {
    let app_group_id = req.query::<String>("dataId");
    let data = BmbpRbacAppGroupService::app_group_find_info(depot, app_group_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询角色成功!".to_string()))
}

#[handler]
pub async fn app_group_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacAppGroup>> {
    let mut params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存角色成功!".to_string()))
}

#[handler]
pub async fn app_group_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacAppGroup>> {
    let mut params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增角色成功!".to_string()))
}

#[handler]
pub async fn app_group_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacAppGroup>> {
    let mut params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新角色成功!".to_string()))
}

#[handler]
pub async fn app_group_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_group_id = req.query::<String>("dataId");
    let data = BmbpRbacAppGroupService::app_group_enable(depot, app_group_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn app_group_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_group_id = req.query::<String>("dataId");
    let data = BmbpRbacAppGroupService::app_group_disable(depot, app_group_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn app_group_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_group_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacAppGroupService::app_group_batch_enable(depot, &app_group_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用角色成功!".to_string()))
}

#[handler]
pub async fn app_group_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_group_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacAppGroupService::app_group_batch_disable(depot, &app_group_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用角色成功!".to_string()))
}

#[handler]
pub async fn app_group_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_group_id = req.query::<String>("dataId");
    let data = BmbpRbacAppGroupService::app_group_remove(depot, app_group_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn app_group_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let app_group_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacAppGroupService::app_group_batch_remove(depot, &app_group_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除角色成功!".to_string()))
}

#[handler]
pub async fn app_group_update_parent(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let mut params = req.parse_json::<BmbpRbacAppGroup>().await?;
    let data = BmbpRbacAppGroupService::app_group_update_parent(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新上级角色成功!".to_string()))
}
