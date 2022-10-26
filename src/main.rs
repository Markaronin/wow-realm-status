use clap::Parser;
use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Response {
    data: ResponseData,
}
#[derive(Deserialize, Debug)]
struct ResponseData {
    #[serde(alias = "Realms")]
    realms: Vec<ResponseDataRealm>,
}
#[derive(Deserialize, Debug)]
struct ResponseDataRealm {
    name: String,
    online: bool,
}

fn get_realm_status(client: &Client, realm_name: &str) -> ResponseDataRealm {
    let response = client.post("https://worldofwarcraft.com/graphql")
        .header("content-type", "application/json")
        .body("{\"operationName\":\"GetInitialRealmStatusData\",\"variables\":{\"input\":{\"compoundRegionGameVersionSlug\":\"us\"}},\"extensions\":{\"persistedQuery\":{\"version\":1,\"sha256Hash\":\"9c7cc66367037fda3007b7f592201c2610edb2c9a9292975cd131a37bbe61930\"}}}")
        .send()
        .unwrap();

    let parsed_response: Response = response.json().unwrap();

    parsed_response
        .data
        .realms
        .into_iter()
        .find(|realm| realm.name == realm_name)
        .expect("Couldn't find your realm in the list of realms")
}

fn watch_realm_status(realm_name: &str) {
    let client = Client::new();

    loop {
        let realm = get_realm_status(&client, realm_name);

        let current_time = chrono::Local::now().format("%H:%M:%S");
        let blurb = if realm.online {
            format!("online").green()
        } else {
            format!("offline").red()
        };
        println!("{current_time} The realm Bleeding Hollow is {blurb}");

        std::thread::sleep(Duration::from_secs(5));

        if realm.online {
            break;
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    realm: String,
}

fn main() {
    let args = Args::parse();

    watch_realm_status(&args.realm);
}
