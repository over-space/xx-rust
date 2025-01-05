use tracing::info;

fn init(){
    tracing_subscriber::fmt::init();
}

fn main() {
    init();
    info!("start exector...");
}
