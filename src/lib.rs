mod view;

use bmbp_abc::tera_add_template;
use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use salvo::Router;

#[derive(RustEmbed)]
#[folder = "static/bmbp_rbac/"]
struct StaticAssets;

pub fn build_router() -> Router {
    let mut router = Router::new();
    router = router
        .push(Router::with_path("/static/bmbp_rbac/<**path>").get(static_embed::<StaticAssets>()));
    router = router.push(
        Router::with_path("rbac")
            .push(
                Router::with_path("app")
                    .push(
                        Router::with_path("group")
                            .push(Router::with_path("index.view").get(view::app_group_view)),
                    )
                    .push(
                        Router::with_path("app")
                            .push(Router::with_path("index.view").get(view::app_app_view)),
                    )
                    .push(
                        Router::with_path("menu")
                            .push(Router::with_path("index.view").get(view::app_menu_view)),
                    ),
            )
            .push(
                Router::with_path("organ")
                    .push(Router::with_path("index.view").get(view::organ_view)),
            )
            .push(
                Router::with_path("user")
                    .push(Router::with_path("index.view").get(view::user_view)),
            )
            .push(
                Router::with_path("role")
                    .push(Router::with_path("index.view").get(view::role_view)),
            ),
    );
    router
}

#[derive(RustEmbed)]
#[folder = "web/templates"]
#[prefix = "bmbp_rbac/"]
pub(crate) struct PageAssets;
pub fn build_template() {
    for file_path in PageAssets::iter() {
        tera_add_template(file_path.as_ref(), PageAssets::get(file_path.as_ref()));
    }
}
