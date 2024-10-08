use bmbp_http_type::PageData;
use bmbp_http_type::RespVo;
use bmbp_http_type::{BmbpPageReq, BmbpResp};
use salvo::{handler, Depot, Request, Response};

use super::bean::BatchReqVo;
use super::bean::BmbpRbacOrgan;
use super::service::BmbpRbacOrganService;

#[handler]
pub async fn organ_find_tree(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    let params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_find_tree(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn organ_find_page(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<PageData<BmbpRbacOrgan>>> {
    let params = req.parse_json::<BmbpPageReq<BmbpRbacOrgan>>().await?;
    tracing::debug!("page params:{:#?}", params);
    let data = BmbpRbacOrganService::organ_find_page(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数分页成功!".to_string()))
}

#[handler]
pub async fn organ_find_list(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    let params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_find_list(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn organ_find_tree_ignore(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<Vec<BmbpRbacOrgan>>> {
    let params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_find_tree_ignore(depot, &params).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn organ_find_info(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    let organ_id = req.query::<String>("dataId");
    let data = BmbpRbacOrganService::organ_find_info(depot, organ_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "查询参数成功!".to_string()))
}

#[handler]
pub async fn organ_save(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    let mut params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_save(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "保存参数成功!".to_string()))
}

#[handler]
pub async fn organ_insert(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    let mut params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_insert(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "新增参数成功!".to_string()))
}

#[handler]
pub async fn organ_update(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<BmbpRbacOrgan>> {
    let mut params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_update(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新参数成功!".to_string()))
}

#[handler]
pub async fn organ_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let organ_id = req.query::<String>("dataId");
    let data = BmbpRbacOrganService::organ_enable(depot, organ_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "启用参数成功!".to_string()))
}

#[handler]
pub async fn organ_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let organ_id = req.query::<String>("dataId");
    let data = BmbpRbacOrganService::organ_disable(depot, organ_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "停用参数成功!".to_string()))
}

#[handler]
pub async fn organ_batch_enable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let organ_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacOrganService::organ_batch_enable(depot, &organ_req).await?;
    Ok(RespVo::ok_data_msg(data, "启用参数成功!".to_string()))
}

#[handler]
pub async fn organ_batch_disable(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let organ_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacOrganService::organ_batch_disable(depot, &organ_req).await?;
    Ok(RespVo::ok_data_msg(data, "停用参数成功!".to_string()))
}

#[handler]
pub async fn organ_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let organ_id = req.query::<String>("dataId");
    let data = BmbpRbacOrganService::organ_remove(depot, organ_id.as_ref()).await?;
    Ok(RespVo::ok_data_msg(data, "删除参数成功!".to_string()))
}

#[handler]
pub async fn organ_batch_remove(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let organ_req = req.parse_json::<BatchReqVo>().await?;
    let data = BmbpRbacOrganService::organ_batch_remove(depot, &organ_req).await?;
    Ok(RespVo::ok_data_msg(data, "删除参数成功!".to_string()))
}

#[handler]
pub async fn organ_update_parent(
    req: &mut Request,
    _resp: &mut Response,
    depot: &mut Depot,
) -> BmbpResp<RespVo<u64>> {
    let mut params = req.parse_json::<BmbpRbacOrgan>().await?;
    let data = BmbpRbacOrganService::organ_update_parent(depot, &mut params).await?;
    Ok(RespVo::ok_data_msg(data, "更新上级参数成功!".to_string()))
}
