use std::{thread, time::Duration};

use anyhow::Result;

use esp_idf_svc::http::client::EspHttpClient;

use crate::{time::Time, weather::Weather};

use self::app::App;

pub mod app;
pub struct AppContext {
    pub http: EspHttpClient,
}

pub fn load_app(ctx: &AppContext) -> Result<()> {
    let mut apps: Vec<Box<dyn App>> = vec![
        Box::new(Time {r: None, count: None}),
        Box::new(Weather{}),
    ];
    for a in apps.iter_mut() {
         a.init(ctx)?;
    }
    loop {
        for a in apps.iter_mut() {
            a.run(ctx)?;
        }
        thread::sleep(Duration::from_millis(20));
    }   
}