use {tokio::process::Command, warp::Filter};

#[tokio::main]
async fn main() {
    let fortune = warp::path("fortune").and_then(fortune);

    warp::serve(fortune).run(([127, 0, 0, 1], 8082)).await;
}

async fn fortune() -> Result<impl warp::Reply, std::convert::Infallible> {
    let output = Command::new("/usr/games/fortune")
        .output()
        .await
        .expect("error running command");
    let parsed = std::str::from_utf8(&output.stdout)
        .expect("error parsing output")
        .to_string();
    Ok(parsed)
}
