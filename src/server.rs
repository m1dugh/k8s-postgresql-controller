use rocket::{get, routes, config::Sig};
use std::{str::FromStr, collections::{hash_map::RandomState, HashSet}};
use crate::error::Error;

use rocket::config::Config;

#[get("/health")]
async fn health() { }

pub async fn start_server(address: &str, port: u16) -> Result<(), Error> {

    println!("rocket start");
    let ip_addr = match std::net::Ipv4Addr::from_str(address) {
        Err(_) => return Err(Error::Default("invalid ip address".to_string())),
        Ok(v) => v.into(),
    };

    println!("found {ip_addr:?}");

    let mut passed_signals =
        HashSet::<Sig, RandomState>::new();
    passed_signals.insert(Sig::Term);
    passed_signals.insert(Sig::Hup);

    let config = Config {
        port,
        address: ip_addr,
        shutdown: rocket::config::Shutdown {
            ctrlc: false,
            signals: passed_signals,
            ..Default::default()
        },
        ..Config::default()
    };

    rocket::custom(config)
        .mount("/", routes![health])
        .launch()
        .await?;

    Ok(())
}
