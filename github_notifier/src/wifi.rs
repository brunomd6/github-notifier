extern crate alloc;

use alloc::string::ToString;
use embassy_executor::Spawner;
use embassy_net::{Config, DhcpConfig, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::{peripherals::WIFI, rng::Rng};
use esp_println::println;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{Config as WifiConfig, ControllerConfig};

const SSID: &str = "Ambush";
const PASSWORD: &str = "ambushisVERYcool";

macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        CELL.uninit().write($val)
    }};
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface<'static>>) {
    runner.run().await;
}

pub async fn init_wifi(wifi: WIFI<'static>, rng: Rng, spawner: &Spawner) -> Stack<'static> {
    let station_config = WifiConfig::Station(
        StationConfig::default()
            .with_ssid(SSID)
            .with_password(PASSWORD.to_string()),
    );

    let (controller, interfaces) = esp_radio::wifi::new(
        wifi,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .unwrap();

    spawner.spawn(wifi_task(controller).expect("failed to create wifi task"));

    let seed = ((rng.random() as u64) << 32) | rng.random() as u64;

    let (stack, runner) = embassy_net::new(
        interfaces.station,
        Config::dhcpv4(DhcpConfig::default()),
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    spawner.spawn(net_task(runner).expect("failed to create task"));

    println!("Waiting for IP...");

    while stack.config_v4().is_none() {
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Connected!");

    stack
}

#[embassy_executor::task]
async fn wifi_task(mut controller: esp_radio::wifi::WifiController<'static>) {
    loop {
        println!("Connecting to Wi-Fi...");

        match controller.connect_async().await {
            Ok(_) => {
                println!("Connected!");

                // Wait until the connection is lost.
                controller.wait_for_disconnect_async().await.unwrap();

                println!("Disconnected");
            }

            Err(e) => {
                println!("Connection failed: {:?}", e);
            }
        }

        Timer::after(Duration::from_secs(5)).await;
    }
}
