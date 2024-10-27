use clap::Parser;
use dotenvy::dotenv;
use regex::Regex;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::error::Error;

#[derive(Parser)]
#[command(version, about, author)]
pub struct Args {
    /// Github Username to fetch activity for
    #[arg(value_parser = validate_username)]
    user_name: String,

    /// Number of Followers
    #[arg(short, long)]
    followers: bool,
}

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Payload {
    commits: Option<Vec<Commit>>,
    ref_type: Option<String>,
    action: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct Commit {}

#[derive(Deserialize, Debug)]
struct Event {
    repo: Repo,
    r#type: EventType,
    payload: Payload,
}

#[derive(Deserialize, Debug)]
enum EventType {
    PullRequestEvent,
    PushEvent,
    CreateEvent,
    WatchEvent,
    ForkEvent,
    IssuesEvent,
}

fn validate_username(user_name: &str) -> Result<String, String> {
    let gh_username_regex = Regex::new(r"^[a-zA-Z0-9_-]{1,39}$").unwrap();

    if gh_username_regex.is_match(user_name) {
        return Ok(user_name.to_string());
    } else {
        return Err(format!("{} is an invalid GitHub username", user_name));
    }
}

fn load_env_variable(key: &str) -> Result<String, env::VarError> {
    env::var(key)
}

fn fetch(
    token: String,
    user_name: &str,
    resource: Option<String>,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let url = match resource {
        Some(resource) => format!("https://api.github.com/users/{}/{}", user_name, resource),
        None => format!("https://api.github.com/users/{}", user_name),
    };

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

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

const API_KEY_ENV: &str = "TOKEN";

fn main() -> Result<(), Box<dyn Error>> {
    // Load the .env File
    dotenv().ok();

    let api_key = load_env_variable(API_KEY_ENV)
        .map_err(|e| format!("Error fetching {}: {}", API_KEY_ENV, e))?;

    let Args {
        user_name,
        followers,
    } = Args::parse();

    if followers {
        let response = fetch(api_key, &user_name, None).expect("te");

        let json: Value = response.json()?;

        println!("{}'s Followers Number: {}", user_name, json["followers"]);
        return Ok(());
    }

    let response = fetch(api_key, &user_name, Some("events".to_string())).expect("te");

    let events: Vec<Event> = response.json()?;

    for event in events {
        match event.r#type {
            EventType::PullRequestEvent => {
                println!(" - Opened a pull request in {}", event.repo.name);
            }
            EventType::PushEvent => {
                if event.payload.clone().commits.is_some() {
                    println!(
                        " - Pushed {} commits to {}.",
                        event.payload.clone().commits.unwrap().len(),
                        event.repo.name
                    );
                }
            }
            EventType::CreateEvent => {
                let r#type = event.payload.clone().ref_type.unwrap();

                if r#type == "repository" {
                    println!(" - Created a repository named {}", event.repo.name);
                } else {
                    println!(" - Created a {} in {}", r#type, event.repo.name);
                }
            }
            EventType::WatchEvent => {
                println!(" - Starred {}", event.repo.name);
            }
            EventType::ForkEvent => {
                println!(" - Forked {}", event.repo.name);
            }
            EventType::IssuesEvent => {
                let action = event.payload.action.unwrap();
                let capitalized_action = capitalize_first_letter(&action);

                println!(" - {} an issue in {}", capitalized_action, event.repo.name);
            }
        }
    }
    Ok(())
}
