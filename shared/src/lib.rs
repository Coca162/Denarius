#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use reqwest::Client;

pub mod error;
pub mod money;
pub mod routes;
pub mod types;

pub struct AekosiaAPI {
    pub client: Client,
    pub website_url: String,
    eco_balance: String,
    eco_payment: String,
    eco_print: String,
    person_register: String,
    person_get: String,
    person_get_discord: String
}

const UUID: usize = 32;
const I64: usize = 20;
const U64: usize = 19;

impl AekosiaAPI {
    #[allow(clippy::must_use_candidate)]
    pub fn new(client: Client, website_url: &str) -> AekosiaAPI {
        let website_url = website_url.trim_end_matches('/').to_string();

        AekosiaAPI {
            client,
            eco_balance: format_with_query(&website_url, "/eco/balance/", UUID),
            eco_payment: format!("{website_url}/eco/payment"),
            eco_print: format_with_query(&website_url, "/eco/print/", UUID + 1 + I64),
            person_register: format_with_query(&website_url, "/person/register/", I64),
            person_get: format_with_query(&website_url, "/person/", UUID),
            person_get_discord: format_with_query(&website_url, "/person/from_discord/", U64),

            website_url,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_test() -> AekosiaAPI {
        AekosiaAPI::new(Client::new(), "http://127.0.0.1:8080")
    }
}

fn format_with_query(website_url: &str, path: &'static str, query_type: usize) -> String {
    let mut string = String::with_capacity(website_url.len() + path.len() + query_type);
    string.push_str(website_url);
    string.push_str(path);
    string
}
