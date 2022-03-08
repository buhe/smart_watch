use esp_idf_svc::http::client::EspHttpClient;

pub struct AppContext {
    http: EspHttpClient,
}