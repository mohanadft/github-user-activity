use clap::Parser;
use dotenvy::dotenv;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::{env, error::Error};

#[derive(Parser)]
#[command(version, about, author)]
pub struct Args {
    /// Github User
    #[arg()]
    user_name: String,
}

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Payload {
    commits: Option<Vec<Commit>>,
}

#[derive(Deserialize, Debug, Clone)]
struct Commit {}

#[derive(Deserialize, Debug)]
struct Event {
    repo: Repo,
    r#type: EventType,
    payload: Option<Payload>,
}

#[derive(Deserialize, Debug)]
enum EventType {
    PullRequestEvent,
    PushEvent,
    CreateEvent,
    WatchEvent,
    ForkEvent,
    StarEvent,
}

fn load_env_variable(key: &str) -> Result<String, env::VarError> {
    env::var(key)
}

fn fetch(token: String, user_name: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/users/{}/events", user_name);

    let mut headers = HeaderMap::new();

    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );

    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );

    headers.insert(USER_AGENT, HeaderValue::from_static("Gh-User-Activity"));

    Ok(client
        .get(url)
        .headers(headers.clone())
        .bearer_auth(token)
        .send()?)
}

const API_KEY_ENV: &str = "TOKEN";

fn main() -> Result<(), Box<dyn Error>> {
    // Load the .env File
    dotenv().ok();

    let api_key = load_env_variable(API_KEY_ENV)
        .map_err(|e| format!("Error fetching {}: {}", API_KEY_ENV, e))?;

    let Args { user_name } = Args::parse();
    let json = fetch(api_key, user_name).expect("te").text();

    let actions = serde_json::from_str::<Vec<Event>>(&json.expect("tet")).unwrap();

    for action in actions {
        match action.r#type {
            EventType::PullRequestEvent => {
                println!(" - Opened a pull request in {}", action.repo.name);
            }
            EventType::PushEvent => {
                if action.payload.is_some() && action.payload.clone().unwrap().commits.is_some() {
                    println!(
                        " - Pushed {} commits to {}.",
                        action.payload.clone().unwrap().commits.unwrap().len(),
                        action.repo.name
                    );
                }
            }
            EventType::CreateEvent => {}
            EventType::WatchEvent => {
                println!(" - Starred {}", action.repo.name);
            }
            EventType::ForkEvent => {
                println!(" - Forked {}", action.repo.name);
            }
            _ => {
                eprintln!("Unhandled Type of Event");
            }
        }
    }
    Ok(())
}
