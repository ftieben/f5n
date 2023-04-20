use f5n_skills::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}