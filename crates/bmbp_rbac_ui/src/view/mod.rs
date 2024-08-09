use crate::ctx::ctx_init;
use crate::init::DATA_TERA;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn organ_view(_: &mut Request, res: &mut Response) {
    let mut ctx = ctx_init();
    res.render(Text::Html((*DATA_TERA).render("organ.html", &ctx).unwrap()));
}

#[handler]
pub async fn user_view(_: &mut Request, resp: &mut Response) {
    let mut ctx = ctx_init();
    resp.render(Text::Html((*DATA_TERA).render("user.html", &ctx).unwrap()));
}

#[handler]
pub async fn app_view(_: &mut Request, res: &mut Response) {
    let mut ctx = ctx_init();
    res.render(Text::Html((*DATA_TERA).render("app.html", &ctx).unwrap()));
}

#[handler]
pub async fn menu_view(_: &mut Request, resp: &mut Response) {
    let mut ctx = ctx_init();
    resp.render(Text::Html((*DATA_TERA).render("menu.html", &ctx).unwrap()));
}

#[handler]
pub async fn role_res_view(_: &mut Request, resp: &mut Response) {
    let mut ctx = ctx_init();
    resp.render(Text::Html(
        (*DATA_TERA).render("role_res.html", &ctx).unwrap(),
    ));
}

#[handler]
pub async fn role_data_view(_: &mut Request, resp: &mut Response) {
    let mut ctx = ctx_init();
    resp.render(Text::Html(
        (*DATA_TERA).render("role_data.html", &ctx).unwrap(),
    ));
}
