#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("public"))
        .run(([0, 0, 0, 0], 8000))
        .await;
}
