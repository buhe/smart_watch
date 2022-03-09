use esp_idf_svc::http::client::EspHttpClient;

pub mod app;
pub struct AppContext {
    http: EspHttpClient,
}