// way of storing information, error handling and reading from file
use std::{collections::HashMap, error::Error, fs};

// turning read file into hashmap of users
use serde_json;

// user struct
use crate::user::User;

pub struct App {
    // general app states
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub user_to_delete: u32,
    pub user_to_delete_str: String,
    
    // list of currently saved users
    pub user_list: HashMap<u32, User>,

    // user input to save to user 
    pub username: String,
    pub email: String,
    pub admin: bool,
    pub uuid: u32,

    // file path for importing json files
    pub file_path_input: String,

    // last recorded error
    pub error: String,

    // current crate version
    pub version: String,


}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
    LoadingFromFile,
    Error,
    DeleteUser,
}

pub enum CurrentlyEditing {
    Username,
    Email,
    Admin,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main, 
            currently_editing: None,
            user_to_delete: 1000,
            user_to_delete_str: String::new(),
            
            user_list: HashMap::new(),

            username: String::new(),
            email: String::new(),
            admin: false,
            uuid: 1000,

            file_path_input: String::new(),

            version: env!("CARGO_PKG_VERSION").to_string(),

            error: String::new()
        }
    }
    
    pub fn json_to_hashmap(&mut self) -> Result<HashMap<u32, User>, Box<dyn Error>>{
        let json_string = read_json_from_file(&self.file_path_input)?;

        let lookup: HashMap<u32, User> = serde_json::from_str(&json_string)?;

        self.uuid = 1000;

        for _ in &lookup {
            self.uuid += 1
        }

        Ok(lookup)
    }

    pub fn save_user(&mut self) {
        self.user_list
            .insert(
                self.uuid.clone(),
                User {
                active: true,
                username: self.username.clone(),
                email: self.email.clone(),
                uuid: self.uuid.clone(),
                admin: self.admin.clone()
            },
        );

        self.username = String::new();
        self.email = String::new();
        self.admin = false;
        self.uuid += 1;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Username => self.currently_editing = Some(CurrentlyEditing::Email),
                CurrentlyEditing::Email => self.currently_editing = Some(CurrentlyEditing::Admin),
                CurrentlyEditing::Admin => self.currently_editing = Some(CurrentlyEditing::Username)
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Username)
        }
    }
    
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string_pretty(&self.user_list)?;
        println!("{}", output);
        Ok(())
    }

    pub fn toggle_admin(&mut self) {
        if self.admin {
            self.admin = false
        } else if !self.admin {
            self.admin = true
        }
    }

    pub fn delete_user(&mut self) {
        self.user_list.insert(
            self.user_to_delete,
            User { 
                active: false, 
                username: String::new(), 
                email: String::new(), 
                uuid: self.user_to_delete, 
                admin: false 
            }
        );
    }
}

fn read_json_from_file(file: &str) -> Result<String, Box<dyn Error>> {
    let string = fs::read_to_string(file)?;
    Ok(string)
}
