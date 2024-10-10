use action::app::*;
use action::menu::*;
use action::organ::*;
use action::role::*;
use action::user::*;
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
                    .push(Router::with_path("tree").post(role_find_tree))
                    .push(Router::with_path("page").post(role_find_page))
                    .push(Router::with_path("list").post(role_find_list))
                    .push(Router::with_path("info").post(role_find_info))
                    .push(Router::with_path("tree/ignore").post(role_find_tree_ignore))
                    .push(Router::with_path("save").post(role_save))
                    .push(Router::with_path("insert").post(role_insert))
                    .push(Router::with_path("update").post(role_update))
                    .push(Router::with_path("enable").post(role_enable))
                    .push(Router::with_path("batch/enable").post(role_batch_enable))
                    .push(Router::with_path("disable").post(role_disable))
                    .push(Router::with_path("batch/disable").post(role_batch_disable))
                    .push(Router::with_path("remove").post(role_remove))
                    .push(Router::with_path("batch/remove").post(role_batch_remove))
                    .push(Router::with_path("update/parent").post(role_update_parent))
                    .push(Router::with_path("index.view").get(role_view)),
            )
            .push(
                Router::with_path("app")
                    .push(Router::with_path("page").post(app_find_page))
                    .push(Router::with_path("list").post(app_find_list))
                    .push(Router::with_path("info").post(app_find_info))
                    .push(Router::with_path("save").post(app_save))
                    .push(Router::with_path("insert").post(app_insert))
                    .push(Router::with_path("update").post(app_update))
                    .push(Router::with_path("enable").post(app_enable))
                    .push(Router::with_path("batch/enable").post(app_batch_enable))
                    .push(Router::with_path("disable").post(app_disable))
                    .push(Router::with_path("batch/disable").post(app_batch_disable))
                    .push(Router::with_path("remove").post(app_remove))
                    .push(Router::with_path("batch/remove").post(app_batch_remove))
                    .push(Router::with_path("index.view").get(app_view)),
            )
            .push(
                Router::with_path("menu")
                    .push(Router::with_path("tree").post(menu_find_tree))
                    .push(Router::with_path("page").post(menu_find_page))
                    .push(Router::with_path("list").post(menu_find_list))
                    .push(Router::with_path("info").post(menu_find_info))
                    .push(Router::with_path("tree/ignore").post(menu_find_tree_ignore))
                    .push(Router::with_path("save").post(menu_save))
                    .push(Router::with_path("insert").post(menu_insert))
                    .push(Router::with_path("update").post(menu_update))
                    .push(Router::with_path("enable").post(menu_enable))
                    .push(Router::with_path("batch/enable").post(menu_batch_enable))
                    .push(Router::with_path("disable").post(menu_disable))
                    .push(Router::with_path("batch/disable").post(menu_batch_disable))
                    .push(Router::with_path("remove").post(menu_remove))
                    .push(Router::with_path("batch/remove").post(menu_batch_remove))
                    .push(Router::with_path("update/parent").post(menu_update_parent))
                    .push(Router::with_path("index.view").get(menu_view)),
            )
            .push(
                Router::with_path("user")
                    .push(Router::with_path("page").post(user_find_page))
                    .push(Router::with_path("list").post(user_find_list))
                    .push(Router::with_path("info").post(user_find_info))
                    .push(Router::with_path("save").post(user_save))
                    .push(Router::with_path("insert").post(user_insert))
                    .push(Router::with_path("update").post(user_update))
                    .push(Router::with_path("enable").post(user_enable))
                    .push(Router::with_path("batch/enable").post(user_batch_enable))
                    .push(Router::with_path("disable").post(user_disable))
                    .push(Router::with_path("batch/disable").post(user_batch_disable))
                    .push(Router::with_path("remove").post(user_remove))
                    .push(Router::with_path("batch/remove").post(user_batch_remove))
                    .push(Router::with_path("index.view").get(user_view)),
            ),
    );
    router = router.push(action_router);
    return router;
}
