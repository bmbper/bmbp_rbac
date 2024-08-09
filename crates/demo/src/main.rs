use bmbp_lib_ui::build_bmbp_ui_lib_router;
use bmbp_rbac::build_bmbp_rbac_router;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let host = "0.0.0.0:9001";
    tracing::info!("启动初始化服务,监听地址:{}......", host);
    let acceptor = TcpListener::new(host).bind().await;
    let mut router = Router::new();
    router = router.push(build_bmbp_ui_lib_router());
    router = router.push(build_bmbp_rbac_router());
    Server::new(acceptor).serve(router).await;
}
