use crate::ctx::ctx_init;
use crate::init::CTX_TERA;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn app_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    let view_html = "app.html";
    resp.render(Text::Html((*CTX_TERA).render(view_html, &ctx).unwrap()));
}
#[handler]
pub async fn app_group_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    let view_html = "app_group.html";
    resp.render(Text::Html((*CTX_TERA).render(view_html, &ctx).unwrap()));
}

#[handler]
pub async fn menu_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    let view_html = "menu.html";
    resp.render(Text::Html((*CTX_TERA).render(view_html, &ctx).unwrap()));
}

#[handler]
pub async fn organ_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    let view_html = "organ.html";
    resp.render(Text::Html((*CTX_TERA).render(view_html, &ctx).unwrap()));
}

#[handler]
pub async fn user_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    let view_html = "user.html";
    resp.render(Text::Html((*CTX_TERA).render(view_html, &ctx).unwrap()));
}

#[handler]
pub async fn role_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    let view_html = "role.html";
    resp.render(Text::Html((*CTX_TERA).render(view_html, &ctx).unwrap()));
}
