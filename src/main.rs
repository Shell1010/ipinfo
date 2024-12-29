use clap::Parser;
use colored::*;
use dirs::config_dir;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "Shell1010 <amin.dev03@gmail.com>",
    about = "IP Information Fetcher"
)]
struct Args {
    #[arg(help = "List of IP addresses to fetch information for.")]
    ip_addresses: Vec<String>,

    #[arg(short = 'c', long = "cookie", help = "Set or update the cookie.")]
    cookie: Option<String>,

    #[arg(short = 'f', long = "file", help = "Read addresses from a file.")]
    file: Option<String>,

    #[arg(short, long, help = "Enable verbose output.")]
    verbose: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    cookie: String,
}

fn read_lines_to_vec(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?; 
        lines.push(line);
    }

    Ok(lines)
}

impl Config {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        if config_path.exists() {
            let config_data = fs::read_to_string(config_path)?;
            let config: Config = serde_json::from_str(&config_data)?;
            Ok(config)
        } else {
            Err("Configuration file not found. Set a cookie with -c first.".into())
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        let config_data = serde_json::to_string_pretty(self)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(config_path, config_data)?;
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = config_dir().ok_or("Failed to find config directory.")?;
    path.push("ipinfo");
    path.push("config.json");
    Ok(path)
}

fn prettify_item(item: &Value, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!(
        "{}{}",
        indent_str,
        item.to_string().green()
    );
}

fn prettify_json(key: &str, value: &Value, indent: usize) {
    let indent_str = "  ".repeat(indent);

    match value {
        Value::Object(map) => {
            if key == "tokenDetails" {
                return;
            }

            println!("{}{}:", indent_str, key.bold().blue());
            for (sub_key, sub_value) in map {
                prettify_json(sub_key, sub_value, indent + 1);
            }
        }
        Value::Array(array) => {
            println!("{}{}: [", indent_str, key.bold().blue());
            for item in array {
                prettify_item(item, indent + 1);
            }
            println!("{}]", indent_str);
        }
        _ => {
            println!(
                "{}{}: {}",
                indent_str,
                key.bold().blue(),
                value.to_string().green()
            );
        }
    }
}

fn fetch_ip_info(ip: &str, cookie: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ipinfo.io/account/search";

    if verbose {
        println!(
            "{}",
            format!("Fetching IP information for: {}", ip)
                .bold()
                .yellow()
        );
    }

    let client = Client::new();

    let response = client
        .get(url)
        .header("cookie", cookie)
        .header("host", "ipinfo.io")
        .header("referrer", "https://ipinfo.io/account/search")
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0",
        )
        .header("content-type", "application/json")
        .header("connection", "keep-alive")
        .header("cache-control", "no-cache")
        .header("te", "trailers")
        .query(&[("query", ip)])
        .send()?;

    if response.status().is_success() {
        let data: Value = response.json()?;
        println!(
            "{} {}",
            "✔ Success:".green().bold(),
            format!("Fetched data for IP: {}", ip).cyan()
        );

        if let Some(map) = data.as_object() {
            for (key, value) in map {
                prettify_json(key, value, 0);
            }
        }
    } else {
        println!(
            "{} {}",
            "✘ Error:".red().bold(),
            format!(
                "Failed to fetch data for IP {}. Status: {}",
                ip,
                response.status()
            )
            .yellow()
        );
    }

    println!();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    if let Some(cookie_value) = args.cookie {
        let config = Config {
            cookie: cookie_value,
        };
        config.save()?;
        println!("{}", "Cookie saved successfully!".bold().green());
        return Ok(());
    }

    let config = Config::load()?;
    let mut addresses = vec![];
    addresses.append(&mut args.ip_addresses);
    if let Some(file) = args.file {
        addresses.append(&mut read_lines_to_vec(&file)?);
    }
    if addresses.is_empty() {
        eprintln!(
            "{}",
            "No IP addresses provided. Use: ipinfo <IP> [<IP>...]"
                .bold()
                .red()
        );
        return Ok(());
    } 

    for ip in addresses {
        if fetch_ip_info(&ip, &config.cookie, args.verbose).is_ok() {};
    }

    Ok(())
}
