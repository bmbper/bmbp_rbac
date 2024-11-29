use bmbp_abc::{base_ctx, BMBP_TERA};
use salvo::prelude::*;

#[handler]
pub async fn app_group_view(_: &mut Request, resp: &mut Response) {
    let ctx = base_ctx();
    let view_html = "bmbp_rbac/app_group.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}

#[handler]
pub async fn app_app_view(_: &mut Request, resp: &mut Response) {
    let ctx = base_ctx();
    let view_html = "bmbp_rbac/app.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}

#[handler]
pub async fn app_menu_view(_: &mut Request, resp: &mut Response) {
    let ctx = base_ctx();
    let view_html = "bmbp_rbac/app_menu.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}

#[handler]
pub async fn organ_view(_: &mut Request, resp: &mut Response) {
    let ctx = base_ctx();
    let view_html = "bmbp_rbac/organ.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}

#[handler]
pub async fn user_view(_: &mut Request, resp: &mut Response) {
    let ctx = base_ctx();
    let view_html = "bmbp_rbac/user.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}

#[handler]
pub async fn role_view(_: &mut Request, resp: &mut Response) {
    let ctx = base_ctx();
    let view_html = "bmbp_rbac/role.html";
    resp.render(Text::Html(
        BMBP_TERA.read().unwrap().render(view_html, &ctx).unwrap(),
    ));
}
