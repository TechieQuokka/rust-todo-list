use std::env;
use storage::file_storage::FileStorage;
use service::TodoService;
use cli::parse_and_execute;

mod models;
mod storage;
mod service;
mod cli;

fn main() {

    let args: Vec<String> = env::args().collect();

    let storage = match FileStorage::default() {

        Ok(storage) => storage,
        Err(e) => {
            eprintln!("Stroage 초기화 실패: {:?}", e);
            return;
        }
    };

    let mut service = TodoService::new(storage);

    if let Err(e) = parse_and_execute(args, &mut service) {

        eprintln!("error: {:?}", e);
        std::process::exit(1);
    }
}