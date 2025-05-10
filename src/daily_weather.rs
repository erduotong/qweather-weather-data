use qweather_sdk::client::QWeatherClient;

/// 获取全国城市的每日天气

pub async fn get_daily_weather(client: QWeatherClient) {
    let response = client.weather_now("101010100").await.unwrap();
    println!("{:#?}", response);
}
