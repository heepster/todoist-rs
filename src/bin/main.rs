extern crate todoist;
extern crate clap;
extern crate preferences;

use std::io::{self, BufRead, Write};

use preferences::{AppInfo, Preferences, PreferencesError};
use clap::{App};

const APP_INFO: AppInfo = AppInfo{name: "todoist", author: "Ian Shehadeh <IanShehadeh2020@gmail.com>"};

fn query_api_key() -> Result<String, PreferencesError> {
    let stdout = io::stdout();
    stdout.lock().write_fmt(format_args!(
        "Please enter your Todoist API key.\n\r>> "
    )).unwrap();
    stdout.lock().flush().unwrap();
    
    let stdin = io::stdin();
    let line = stdin.lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
    line.save(&APP_INFO, "todoist/user/api_key")?;
    String::load(&APP_INFO, "todoist/user/api_key")
}

fn main() {
    let _ = App::new(APP_INFO.name)
                        .author(APP_INFO.author)
                        .version("0.1.0")
                        .about("Simple CLI for todoist")
                        .get_matches();

    let api_key =
        match String::load(&APP_INFO, "todoist/user/api_key") {
            Ok(v) => v,
            Err(_) => query_api_key().unwrap(),
        };
    let mut client = todoist::Client::new(&api_key);
}