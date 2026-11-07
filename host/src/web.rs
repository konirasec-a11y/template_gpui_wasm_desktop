use ntex::{SharedCfg, web};
use ntex_files::Files;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let dist_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("host crate must live in the workspace root")
        .join("public/dist");

    web::HttpServer::new(async move |_| {
        web::App::new().service(Files::new("/", dist_dir.clone()).index_file("index.html"))
    })
    .bind("127.0.0.1:3000", SharedCfg::new("hello-world"))?
    .run()
    .await
}
