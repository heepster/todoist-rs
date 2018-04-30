extern crate serde;
extern crate reqwest;
extern crate uuid;
extern crate chrono;
extern crate serde_json;
extern crate erased_serde;

pub mod command;
pub mod resource;

mod types;
#[cfg(test)] mod tests;

pub use types::*;
use std::collections::HashMap;

#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub enum Resource {
    Label(resource::Label),
    Project(resource::Project),
    Item(resource::Item),
    Note(resource::Note),
    Filter(resource::Filter),
    User(resource::User),
    Collaborator(resource::Collaborator),
    LiveNotification(resource::LiveNotification),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceType {
    #[serde(rename = "all")]
    All,

    #[serde(rename = "labels")]
    Labels,

    #[serde(rename = "projects")]
    Projects,

    #[serde(rename = "items")]
    Items,

    #[serde(rename = "notes")]
    Notes,

    #[serde(rename = "filters")]
    Filters,

    #[serde(rename = "reminders")]
    Reminders,

    #[serde(rename = "location")]
    Locations, // TODO

    #[serde(rename = "user")]
    User,

    #[serde(rename = "live_notifications")]
    LiveNotifications,

    #[serde(rename = "collaborators")]
    Collaborators,

    #[serde(rename = "notification_settings")]
    NotificationSettings, // TODO
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ActionErrorObject {
    pub error_code : isize,
    pub error : String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionStatus {
    Ok(String),
    Error(ActionErrorObject),
} 

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct CommandResponse {
    pub sync_status      : HashMap<uuid::Uuid, ActionStatus>,
    pub temp_id_mappings : HashMap<uuid::Uuid, ID>,
}



#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SyncResponse {
    pub sync_token : String,
    pub full_sync : bool,
    pub items : Option<Vec<resource::Item>>,
    pub labels : Option<Vec<resource::Label>>,
    pub projects : Option<Vec<resource::Project>>,
    pub collaborators : Option<Vec<resource::Collaborator>>,
    pub notes : Option<Vec<resource::Note>>,
    pub filters : Option<Vec<resource::Filter>>,
    pub live_notifications : Option<Vec<resource::LiveNotification>>,
    pub reminders : Option<Vec<resource::Reminder>>,
    pub user : Option<resource::User>,
}

/// Client to make request to the todoist API
pub struct Client {
    token: String,
    client: reqwest::Client,
    last_sync: String,
}

impl Client {

    /// Create a new with todoist API client with auth token `tok`
    pub fn new(tok: &str) -> Client {
        Client::new_with_sync(tok, "*")
    }
    
    /// create a new client with a sync token
    pub fn new_with_sync(tok: &str, sync_tok: &str) -> Client {
        Client {
            client: reqwest::Client::new(),
            token: String::from(tok),
            last_sync: String::from(sync_tok),

        }
    }

    /// Request resources from todoist
    pub fn sync(&mut self, what: &[ResourceType]) -> Result<SyncResponse, types::Error> {
        let res : SyncResponse = self.client.post("http://todoist.com/api/v7/sync")
            .form(&[("token", self.token.clone()), 
                    ("sync_token", self.last_sync.clone()),
                    ("resource_types", serde_json::to_string(what)?)])
            .send()?
            .json()?;

        self.last_sync = res.sync_token.clone();
        Ok(res)
    }

    /// Update a user's resources
    pub fn send(&mut self, cmd: &[&command::Command]) -> Result<CommandResponse, types::Error> {
        println!("{}", serde_json::to_string(cmd)?);
        let res : CommandResponse = self.client.post("http://todoist.com/api/v7/sync")
            .form(&[("token", self.token.clone()), 
                    ("commands", serde_json::to_string(cmd)?)])
            .send()?
            .json()?;

        Ok(res)
    }
}