mod daily_weather;

use crate::daily_weather::get_daily_weather;
use dotenvy::dotenv;
use qweather_sdk::client::{ClientConfig, QWeatherClient};
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::process::exit;

type WeatherTask =
    Box<dyn FnOnce(QWeatherClient) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // 用逗号分割成两个string的vec
    let ids = env::var("QWEATHER_IDS").unwrap();
    let keys = env::var("QWEATHER_KEYS").unwrap();
    let id_vec = ids.split(',').collect::<Vec<&str>>();
    let keys_vec = keys.split(',').collect::<Vec<&str>>();
    // 路径长度必须一样
    if id_vec.len() != keys_vec.len() {
        println!("ID与KEY长度不匹配");
        exit(1);
    }
    println!("{:?}\n{:?}", id_vec, keys_vec);
    let mut clients = Vec::new();
    for (id, key) in id_vec.iter().zip(keys_vec.iter()) {
        let client_config = ClientConfig::new(id, key);
        let client = QWeatherClient::with_config(client_config);
        clients.push(client);
    }
    let mut tasks: Vec<WeatherTask> = Vec::new();
    tasks.push(Box::new(|client| Box::pin(get_daily_weather(client))));
    let mut futures = Vec::new();
    for (i, task) in tasks.into_iter().enumerate() {
        if i < clients.len() {
            let client = clients.remove(0);
            let future = task(client);
            futures.push(future);
        } else {
            println!("客户端数量不足，无法执行所有任务");
            exit(1);
        }
    }
    futures::future::join_all(futures).await;
}
