use std::sync::Arc;
use std::{thread, time::Duration};
use anyhow::bail;
use anyhow::Result;

use embedded_graphics::prelude::*;
use embedded_graphics::image::{Image, ImageRawLE};
use embedded_graphics::text::*;

// use embedded_svc::ping::Ping;
use embedded_svc::utils::anyerror::*;
use embedded_svc::wifi::Wifi;
use embedded_svc::{wifi::{Configuration, ClientConfiguration, Status, ClientStatus, ClientConnectionStatus, ClientIpStatus, ApStatus}};
use esp_idf_svc::{netif::EspNetifStack, sysloop::EspSysLoopStack, nvs::EspDefaultNvs, wifi::EspWifi};
// use esp_idf_sys as _;
// use log::info; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_svc::http::client::EspHttpClient;

use embedded_hal::digital::v2::OutputPin;

use esp_idf_hal::delay;
use esp_idf_hal::gpio::{self, Gpio18, Unknown, Gpio19, Gpio21, Gpio5, Gpio16, Output, Gpio23};
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::{self, Master, SPI2};

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::*;

use profile::Profile;
use st7789::ST7789;

const SSID: &str = env!("SSID");
const PASS: &str = env!("PASS");

// mod github;
mod bilibili;
pub mod profile;

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
    let mut display = lcd(
        pins.gpio4,
        pins.gpio16,//ao
        pins.gpio23,
        peripherals.spi2,
        pins.gpio18,
        pins.gpio19,//sda
        pins.gpio5,
    )?;
    let mut i = 0;
    loop {
        // println!("...start...");
        let mut client;
        let res = EspHttpClient::new_default();
        match res {
            Ok(c) => client = c,
            Err(_) => continue,
        }
        let mut res = vec![];
        // res.push(github::init(&mut client)?);
        res.push(bilibili::init(&mut client)?);
        
        for p in  res {
            println!("{:?}", &p);
            AnyError::<st7789::Error<_>>::wrap(|| { draw_profile(&mut display, &p) })?;
        }
        drop(client);
        // drop(wifi);
        i = i + 1;
        println!("...{}...", i);
        
        // println!("...end...");
        thread::sleep(Duration::from_millis(20000));
    }
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


fn draw_profile<D>(display: &mut D, p: &Profile) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
    D::Color: From<Rgb565>,
{
    display.clear(Rgb565::BLACK.into())?;
    let data =  include_bytes!("../images/tv.raw");
    // println!("data : {:?}", data);
    let raw_image = ImageRawLE::<Rgb565>::new(data, 64);
    Image::new(
        &raw_image, 
        Point::new(16, (display.bounding_box().size.height - 32) as i32 / 2))
    .draw(display)?;

    Text::new(
        format!("nick name: {}\nfolloers: {}\nfollowing: {}" ,&p.display ,&p.followers, &p.followings).as_str(),
        Point::new(100, (display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
    )
    .draw(display)?;

    println!("LED rendering profile done");

    Ok(())
}

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