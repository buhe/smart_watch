use std::{thread, time::Duration};

use anyhow::Result;

use esp_idf_hal::gpio::{Gpio26, Output, Gpio22, Unknown, Gpio21};
use esp_idf_svc::http::client::EspHttpClient;
use esp_idf_hal::i2c;

use crate::{time::Time, weather::Weather, cat_play::CatPlay, distance::Distance};

use self::app::App;

pub mod app;
pub struct AppContext {
    pub http: EspHttpClient,
    pub i2c0: Option<i2c::I2C0>,
    pub gpio26 :Option<Gpio26<Output>>,
    pub gpio22: Option<Gpio22<Unknown>>,
    pub gpio21: Option<Gpio21<Unknown>>,
}

pub fn load_app(ctx: &mut AppContext) -> Result<()> {
    let mut apps: Vec<Box<dyn App>> = vec![
        Box::new(Time {r: None, count: None}),
        Box::new(Weather{count: None, cond: 0}),
        Box::new(CatPlay{count: None, cond: 0}),
        Box::new(Distance{count:None, cond: 0, tof: None}),
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