use dotenv::dotenv;

#[actix_rt::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    spacegame_lib::main().await;
}
