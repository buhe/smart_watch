use std::{thread, time::Duration};

use anyhow::Result;

use embedded_graphics::text::*;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};

use esp_idf_hal::gpio::{Gpio26, Output, Gpio22, Unknown, Gpio21};
use esp_idf_svc::http::client::EspHttpClient;
use esp_idf_hal::i2c;

use crate::{time::Time, weather::Weather, cat_play::CatPlay, distance::Distance, target::Target};

use self::app::App;

pub mod app;
pub struct AppContext {
    pub http: EspHttpClient,
    pub i2c0: Option<i2c::I2C0>,
    pub gpio26 :Option<Gpio26<Output>>,
    pub gpio22: Option<Gpio22<Unknown>>,
    pub gpio21: Option<Gpio21<Unknown>>,
    // pub targets: Option<&'a Vec<Target>>,
}

pub fn load_app<D>(ctx: &mut AppContext, display: &mut D) -> Result<()> 
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
    D::Color: From<Rgb565>,
{
    let mut apps: Vec<Box<dyn App>> = vec![
        Box::new(Time {r: None, count: None}),
        Box::new(Weather{count: None, cond: 0}),
        // Box::new(CatPlay{count: None, cond: 0}),
        // Box::new(Distance{count:None, cond: 0, tof: None, state: crate::distance::State::Stopped}),
    ];
    let mut targets :Vec<Target> = vec![];
    // ctx.targets = Some(&targets);
    for a in apps.iter_mut() {
         a.init(ctx)?;
    }
   
    loop {
        for a in apps.iter_mut() {
            a.run(ctx, &targets)?;
        }
        while !targets.is_empty() {
            let t = targets.pop().unwrap();
        }
        // at render
       Text::new(
                "hello!!!world",
                Point::new(10, 10),
                MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
            )
            .draw(display);
        thread::sleep(Duration::from_millis(20));
    }   
}