use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();

    spacegame_lib::main();
}
