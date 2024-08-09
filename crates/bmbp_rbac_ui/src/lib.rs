use crate::init::build_bmbp_static_router;
use crate::view::*;
use salvo::prelude::*;

mod action;
mod ctx;
mod init;
mod view;

pub fn build_bmbp_rbac_router() -> Router {
    let mut router = Router::new();
    router = router.push(build_bmbp_static_router());

    let view_router = Router::with_path("bmbp/rbac/view")
        .push(Router::with_path("organ.view").get(organ_view))
        .push(Router::with_path("user.view").get(user_view))
        .push(Router::with_path("app.view").get(app_view))
        .push(Router::with_path("menu.view").get(menu_view))
        .push(Router::with_path("role_res.view").get(role_res_view))
        .push(Router::with_path("role_data.view").get(role_data_view));

    router = router.push(view_router);
    return router;
}
