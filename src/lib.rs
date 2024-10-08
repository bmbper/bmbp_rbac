use action::action::*;
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
    let action_router = Router::new().push(
        Router::with_path("rbac")
            .push(
                Router::with_path("organ")
                    .push(Router::with_path("tree").post(organ_find_tree))
                    .push(Router::with_path("page").post(organ_find_page))
                    .push(Router::with_path("list").post(organ_find_list))
                    .push(Router::with_path("info").post(organ_find_info))
                    .push(Router::with_path("tree/ignore").post(organ_find_tree_ignore))
                    .push(Router::with_path("save").post(organ_save))
                    .push(Router::with_path("insert").post(organ_insert))
                    .push(Router::with_path("update").post(organ_update))
                    .push(Router::with_path("enable").post(organ_enable))
                    .push(Router::with_path("batch/enable").post(organ_batch_enable))
                    .push(Router::with_path("disable").post(organ_disable))
                    .push(Router::with_path("batch/disable").post(organ_batch_disable))
                    .push(Router::with_path("remove").post(organ_remove))
                    .push(Router::with_path("batch/remove").post(organ_batch_remove))
                    .push(Router::with_path("update/parent").post(organ_update_parent))
                    .push(Router::with_path("index.view").get(organ_view)),
            )
            .push(
                Router::with_path("role")
                    .push(Router::with_path("tree").post(organ_find_tree))
                    .push(Router::with_path("page").post(organ_find_page))
                    .push(Router::with_path("list").post(organ_find_list))
                    .push(Router::with_path("info").post(organ_find_info))
                    .push(Router::with_path("tree/ignore").post(organ_find_tree_ignore))
                    .push(Router::with_path("save").post(organ_save))
                    .push(Router::with_path("insert").post(organ_insert))
                    .push(Router::with_path("update").post(organ_update))
                    .push(Router::with_path("enable").post(organ_enable))
                    .push(Router::with_path("batch/enable").post(organ_batch_enable))
                    .push(Router::with_path("disable").post(organ_disable))
                    .push(Router::with_path("batch/disable").post(organ_batch_disable))
                    .push(Router::with_path("remove").post(organ_remove))
                    .push(Router::with_path("batch/remove").post(organ_batch_remove))
                    .push(Router::with_path("update/parent").post(organ_update_parent))
                    .push(Router::with_path("index.view").get(organ_view)),
            ),
    );
    router = router.push(action_router);
    // view router
    let view_router = Router::with_path("rbac")
        .push(Router::with_path("user.view").get(user_view))
        .push(Router::with_path("app.view").get(app_view))
        .push(Router::with_path("menu.view").get(menu_view));

    router = router.push(view_router);
    return router;
}
