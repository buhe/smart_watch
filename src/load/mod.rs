use std::{thread, time::Duration};

use anyhow::Result;

use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::http::client::EspHttpClient;

use crate::{time::Time, weather::Weather, cat_play::CatPlay};

use self::app::App;

pub mod app;
pub struct AppContext<'a> {
    pub http: EspHttpClient,
    pub peripherals :&'a Peripherals,
}

pub fn load_app(ctx: &mut AppContext) -> Result<()> {
    let mut apps: Vec<Box<dyn App>> = vec![
        Box::new(Time {r: None, count: None}),
        Box::new(Weather{count: None}),
        Box::new(CatPlay{}),
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