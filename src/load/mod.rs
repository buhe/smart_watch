use esp_idf_svc::http::client::EspHttpClient;

use crate::time::Time;

use self::app::App;

pub mod app;
pub struct AppContext {
    http: EspHttpClient,
}

pub fn loadApp(ctx: &AppContext) {
    let apps = vec![
        Time {},
    ];
    for a in apps.iter() {
         a.init(ctx);
    }
       
}