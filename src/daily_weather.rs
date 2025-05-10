//! 获取全国每日天气

use csv::Reader;
use qweather_sdk::api::APIResponse;
use qweather_sdk::api::weather::WeatherDailyForecastResponse;
use qweather_sdk::client::QWeatherClient;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use tokio::{task, time};

#[allow(dead_code)]
#[derive(Debug)]
struct City {
    location_id: String,
    location_name_en: String,
    location_name_zh: String,
    iso_3166_1: String,
    country_region_en: String,
    country_region_zh: String,
    adm1_name_en: String,
    adm1_name_zh: String,
    adm2_name_en: String,
    adm2_name_zh: String,
    timezone: String,
    latitude: f64,
    longitude: f64,
    ad_code: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct WeatherResult {
    city_info: City,
    api_result: WeatherDailyForecastResponse,
}

async fn city_reader() -> Vec<City> {
    // Read CSV in blocking task to avoid blocking async runtime
    let cities = task::spawn_blocking(|| {
        let mut rdr = Reader::from_path("assets/filtered_cities.csv").unwrap();
        let mut cities = Vec::new();
        for result in rdr.records() {
            let record = result.unwrap();
            cities.push(City {
                location_id: record.get(0).unwrap().to_string(),
                location_name_en: record.get(1).unwrap().to_string(),
                location_name_zh: record.get(2).unwrap().to_string(),
                iso_3166_1: record.get(3).unwrap().to_string(),
                country_region_en: record.get(4).unwrap().to_string(),
                country_region_zh: record.get(5).unwrap().to_string(),
                adm1_name_en: record.get(6).unwrap().to_string(),
                adm1_name_zh: record.get(7).unwrap().to_string(),
                adm2_name_en: record.get(8).unwrap().to_string(),
                adm2_name_zh: record.get(9).unwrap().to_string(),
                timezone: record.get(10).unwrap().to_string(),
                latitude: record.get(11).unwrap().parse().unwrap(),
                longitude: record.get(12).unwrap().parse().unwrap(),
                ad_code: record.get(13).unwrap().to_string(),
            });
        }
        cities
    })
    .await
    .unwrap();
    cities
}
async fn data_getter(client: QWeatherClient, cities: Vec<City>) -> Vec<WeatherResult> {
    // 并发任务准备：共享客户端并收集句柄
    let client = Arc::new(client);
    let mut handles = Vec::new();
    // 按间隔启动查询任务
    for city in cities.into_iter().take(3) {
        time::sleep(Duration::from_secs(1)).await;
        let client = client.clone();
        // 具体的代码
        let handle = task::spawn(async move {
            let result = client
                .weather_daily_forecast(city.location_id.as_str(), 30)
                .await
                .unwrap();
            (city, result)
        });
        handles.push(handle);
    }
    // 收集并处理所有任务结果
    let mut weather_results = Vec::new();
    for handle in handles {
        let (city, result) = handle.await.unwrap();
        match result {
            APIResponse::Success(value) => weather_results.push(WeatherResult {
                city_info: city,
                api_result: value,
            }),
            APIResponse::Error(err) => {
                println!("Error: {}", err);
                exit(1)
            }
        }
    }
    weather_results
}

#[allow(unused_variables)]
pub async fn get_daily_weather(client: QWeatherClient) {
    // Read cities from CSV
    let cities = city_reader().await;
    let data = data_getter(client, cities).await;
    println!("{:?}", data);
}
