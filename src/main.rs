use std::sync::Arc;

use anyhow::bail;
use anyhow::Result;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::prelude::*;
// use embedded_graphics::image::{Image, ImageRawLE};
use embedded_graphics::text::*;
// use embedded_hal::digital::v2::OutputPin;
// use epd_waveshare::{graphics::VarDisplay, prelude::*};

use embedded_hal::digital::v2::OutputPin;
use embedded_svc::utils::anyerror::AnyError;
// // use embedded_svc::ping::Ping;
// use embedded_svc::utils::anyerror::*;
use embedded_svc::wifi::Wifi;
use embedded_svc::{wifi::{Configuration, ClientConfiguration, Status, ClientStatus, ClientConnectionStatus, ClientIpStatus, ApStatus}};
use esp_idf_hal::gpio::Gpio16;
use esp_idf_hal::gpio::Gpio18;
use esp_idf_hal::gpio::Gpio19;
use esp_idf_hal::gpio::Gpio21;
use esp_idf_hal::gpio::Gpio23;
use esp_idf_hal::gpio::Gpio5;
use esp_idf_hal::gpio::Output;
use esp_idf_hal::gpio::Unknown;
use esp_idf_hal::spi::Master;
use esp_idf_hal::spi::SPI2;
use esp_idf_svc::{netif::EspNetifStack, sysloop::EspSysLoopStack, nvs::EspDefaultNvs, wifi::EspWifi};
// use esp_idf_sys as _;
// use log::info; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_svc::http::client::EspHttpClient;
use esp_idf_hal::i2c::{self};
// use embedded_hal::digital::v2::OutputPin;

use esp_idf_hal::delay;
use esp_idf_hal::gpio::{self};
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::{self};

use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use load::AppContext;
use load::load_app;


use st7789;
use st7789::ST7789;
// use target::Target;

mod load;
mod time;
mod weather;
mod cat_play;
mod distance;

mod target;


const SSID: &str = "Xiaomi_85FE";
const PASS: &str = "aa11aa041212";


fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let _wifi = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;
    let client = EspHttpClient::new_default()?;
     let mut display = lcd(
        pins.gpio4,
        pins.gpio16,//ao
        pins.gpio23,
        peripherals.spi2,
        pins.gpio18,//scl
        pins.gpio19,//sda
        pins.gpio5,
    )?;
    display.clear(Rgb565::BLACK.into()).unwrap();
    // init context
    let mut ctx = AppContext{
        http: client,
        gpio26: Some(pins.gpio26.into_output().unwrap()),
        gpio22: Some(pins.gpio22),
        gpio21: Some(pins.gpio21),
        i2c0: Some(peripherals.i2c0),
        // targets: None,
    };
    // load app
    load_app(&mut ctx,&mut display)
}

fn lcd(
    backlight: gpio::Gpio4<gpio::Unknown>,
    dc: gpio::Gpio16<gpio::Unknown>,
    rst: gpio::Gpio23<gpio::Unknown>,
    spi: spi::SPI2,
    sclk: gpio::Gpio18<gpio::Unknown>,
    sdo: gpio::Gpio19<gpio::Unknown>,
    cs: gpio::Gpio5<gpio::Unknown>,
) -> Result<ST7789<SPIInterfaceNoCS<Master<SPI2, Gpio18<Unknown>, Gpio19<Unknown>, Gpio21<Unknown>, Gpio5<Unknown>>, Gpio16<Output>>, Gpio23<Output>>>{
    println!("About to initialize the ST7789 LED driver");

    let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());

    let mut backlight = backlight.into_output()?;
    backlight.set_high()?;

    let di = SPIInterfaceNoCS::new(
        spi::Master::<spi::SPI2, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo,
                sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
                cs: Some(cs),
            },
            config,
        )?,
        dc.into_output()?,
    );

    let mut display:  ST7789<SPIInterfaceNoCS<Master<SPI2, Gpio18<Unknown>, Gpio19<Unknown>, Gpio21<Unknown>, Gpio5<Unknown>>, Gpio16<Output>>, Gpio23<Output>> = st7789::ST7789::new(
        di,
        rst.into_output()?,
        320,
        240,
    );

    AnyError::<st7789::Error<_>>::wrap(|| {
        display.init(&mut delay::Ets)?;
        display.set_orientation(st7789::Orientation::Landscape)
    })?;
    Ok(display)
}


fn _draw_profile<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
    D::Color: From<Rgb565>,
{
    display.clear(Rgb565::BLACK.into())?;

    Text::new(
        "hello!!!",
        Point::new(10, 10),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
    )
    .draw(display)?;

    println!("LED rendering profile done");

    Ok(())
}
// fn _waveshare_epd_hello_world(
//     spi: spi::SPI2,
//     sclk: gpio::Gpio13<gpio::Unknown>,
//     sdo: gpio::Gpio14<gpio::Unknown>,
//     cs: gpio::Gpio15<gpio::Unknown>,
//     busy_in: gpio::Gpio25<gpio::Unknown>,
//     dc: gpio::Gpio27<gpio::Unknown>,
//     rst: gpio::Gpio26<gpio::Unknown>,
// ) -> Result<()> {
//     println!("About to initialize Waveshare 1.54 e-paper display");
//     let cs = cs.into_output().unwrap();
//     let busy_in = busy_in.into_input().unwrap();
//     let dc = dc.into_output().unwrap();
//     let rst = rst.into_output().unwrap();

//     let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());

//     let mut my_spi = spi::Master::<spi::SPI2, _, _, _, _>::new(
//         spi,
//         spi::Pins {
//             sclk: sclk,
//             sdo: sdo,
//             sdi: Option::<gpio::Gpio12<gpio::Unknown>>::None,
//             cs: Option::<gpio::Gpio15<gpio::Unknown>>::None,
//         },
//         config,
//     )
//     .unwrap();
//     // Setup EPD
//     let mut epd = Epd1in54::new(&mut my_spi, cs, busy_in, dc, rst, &mut delay::Ets).unwrap();
//     // Use display graphics from embedded-graphics
//     let mut buffer =
//         vec![DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH as usize / 8 * HEIGHT as usize];
//     let mut display = VarDisplay::new(WIDTH, HEIGHT, &mut buffer);

//     let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

//     // Create a text at position (20, 30) and draw it using the previously defined style
//     Text::new("Hello Rust!.......", Point::new(0, 0), style).draw(&mut display)?;

//     // Display updated frame
//     epd.update_frame(&mut my_spi, &display.buffer(), &mut delay::Ets)?;
//     epd.display_frame(&mut my_spi, &mut delay::Ets)?;
//     println!("setup epd");
//     Ok(())
// }


fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    // info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        // info!(
        //     "Found configured access point {} on channel {}",
        //     SSID, ours.channel
        // );
        Some(ours.channel)
    } else {
        // info!(
        //     "Configured access point {} not found during scanning, will go with unknown channel",
        //     SSID
        // );
        None
    };

    // wifi.set_configuration(&Configuration::Mixed(
    //     ClientConfiguration {
    //         ssid: SSID.into(),
    //         password: PASS.into(),
    //         channel,
    //         ..Default::default()
    //     },
    //     AccessPointConfiguration {
    //         ssid: "aptest".into(),
    //         channel: channel.unwrap_or(1),
    //         ..Default::default()
    //     },
    // ))?;


    wifi.set_configuration(&Configuration::Client(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        }
    ))?;

    // info!("Wifi configuration set, about to get status");

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_ip_settings))),
        ApStatus::Stopped
    ) = status
    {
        // println!("Wifi connected");

        // ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

fn _vl53l0x_hello_world(
    i2c: i2c::I2C0,
    // _rst: gpio::Gpio16<gpio::Unknown>,
    scl: gpio::Gpio22<gpio::Unknown>,
    sda: gpio::Gpio21<gpio::Unknown>,
) -> Result<()> {
    println!("About to initialize a generic SSD1306 I2C LED driver");

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(1.MHz().into());

    let mut tof = vl53l0x::VL53L0x::new(i2c::Master::<i2c::I2C0, _, _>::new(
        i2c,
        i2c::MasterPins { sda, scl },
        config,
    )?).unwrap();

     println!("ready to set meas budget");
    tof.set_measurement_timing_budget(200000).expect("timbudg");
    println!("meas budget set; start cont");
    tof.start_continuous(0).expect("start cont");
    // let mut delay = delay::Ets;
   

    // loop {
        match tof.read_range_continuous_millimeters_blocking() {
            Ok(meas) => {
                println!("vl: millis {}", meas);
            }
            Err(e) => {
                println!("Err meas: {:?}\r\n", e);
            }
        };
    // }
    Ok(())
}

// fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
//     // info!("About to do some pings for {:?}", ip_settings);

//     let ping_summary =
//         ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
//     if ping_summary.transmitted != ping_summary.received {
//         bail!(
//             "Pinging gateway {} resulted in timeouts",
//             ip_settings.subnet.gateway
//         );
//     }

//     // info!("Pinging done");

//     Ok(())
// }