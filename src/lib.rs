use init::build_bmbp_static_router;
use salvo::prelude::*;
use view::{app_view, menu_view, organ_view, role_view, user_view};
mod action;
mod ctx;
mod init;
mod view;
pub fn build_bmbp_rbac_router() -> Router {
    let mut router = Router::new();
    router = router.push(build_bmbp_static_router());
    // action router
    let action_router = Router::new();
    router = router.push(action_router);
    // view router
    let view_router = Router::with_path("rbac")
        .push(Router::with_path("organ.view").get(organ_view))
        .push(Router::with_path("user.view").get(user_view))
        .push(Router::with_path("role.view").get(role_view))
        .push(Router::with_path("app.view").get(app_view))
        .push(Router::with_path("menu.view").get(menu_view));

    router = router.push(view_router);
    return router;
}
