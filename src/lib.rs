// Copyright 2021, Console Ltd https://console.dev
// SPDX-License-Identifier: AGPL-3.0-or-later

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use log::info;
use mailchimp::{Lists, MailchimpApi};
use rocket::config::{Config, Environment};
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use std::env;

#[get("/get_subscribers", format = "json")]
fn get_subscribers() -> JsonValue {
    // Create API client
    let api_key = env::var("LAMSUBS_MAILCHIMP_APIKEY")
        .expect("LAMSUBS_MAILCHIMP_APIKEY not set");
    let api = MailchimpApi::new(&api_key);

    // Query the specific list
    let lists = Lists::new(api);
    let list_id = env::var("LAMSUBS_MAILCHIMP_LIST_ID")
        .expect("LAMSUBS_MAILCHIMP_LIST_ID not set");
    let r_list = lists.get_list_info(&list_id, HashMap::new());

    match r_list {
        Ok(list) => {
            // Get the stats
            let stats = list.stats.as_ref().expect("No stats returned");

            info!("Raw stats: {:?}", stats);

            // The number of active members in the list
            if let Some(member_count) = stats.member_count {
                json!({
                   "frames": [{
                        "icon": "i29438",
                        "text": member_count
                    }]
                })
            } else {
                json!({
                    "frames": [{
                        "icon": "i619",
                        "text": "No stats"
                    }]
                })
            }
        }
        Err(e) => {
            // Log errors
            let error = format!("Error getting Mailchimp list info: {:?}", e);
            json!({
                "frames": [{
                    "icon": "i619",
                    "text": error
                }]
            })
        }
    }
}

pub fn rocket() -> rocket::Rocket {
    // Define Rocket routes
    let routes = routes![get_subscribers,];

    // Pick up custom port setting for Azure
    // https://docs.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Clinux#create-and-build-your-function
    let port: u16 = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    // Creating a custom config for each environment seems to be the only way
    // to set a custom port on Rocket
    // https://api.rocket.rs/v0.4/rocket/config/struct.ConfigBuilder.html#example-2
    let config;
    if env::var("LAMSUBS_PRODUCTION").is_ok() {
        config = Config::build(Environment::Production)
            .port(port)
            .log_level(rocket::config::LoggingLevel::Normal)
            .unwrap();
    } else {
        config = Config::build(Environment::Development)
            .address("127.0.0.1")
            .port(port)
            .log_level(rocket::config::LoggingLevel::Debug)
            .unwrap();
    }

    rocket::custom(config).mount("/api", routes)
}
