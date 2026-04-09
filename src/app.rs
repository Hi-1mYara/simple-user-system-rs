// way of storing information, error handling and reading from file
use std::{collections::HashMap, error::Error, fs};

use ratatui::widgets::ListState;
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
    pub user_list: UserList,

    // user input to save to user 
    pub user_info: User,

    // file path for importing json files
    pub file_path_input: String,

    // last recorded error
    pub error: String,

    // current crate version
    pub version: String,
}

pub struct UserList {
    pub user_hash: HashMap<u32, User>,
    pub user_vec: Vec<User>,
    pub list_state: ListState
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

            user_list: UserList::default(),

            user_info: User::default(),

            file_path_input: String::new(),

            version: env!("CARGO_PKG_VERSION").to_string(),

            error: String::new()
        }
    }
    
    pub fn json_to_hashmap(&mut self) -> Result<HashMap<u32, User>, Box<dyn Error>>{
        let json_string = read_json_from_file(&self.file_path_input)?;

        let lookup: HashMap<u32, User> = serde_json::from_str(&json_string)?;

        self.user_info.uuid = 1000;

        for _ in &lookup {
            self.user_info.uuid += 1
        }

        for pairs in &lookup {
            self.user_list.user_vec.push(pairs.1.clone());
        }

        self.user_list.user_vec.sort_by_key(|a| a.uuid);

        Ok(lookup)
    }

    pub fn save_user(&mut self) {
        self.user_list.user_hash
            .insert(self.user_info.uuid.clone(), self.user_info.clone());

        self.user_list.user_vec.push(self.user_info.clone());

        self.user_info.uuid += 1;
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
        let output = serde_json::to_string_pretty(&self.user_list.user_hash)?;
        println!("{}", output);
        Ok(())
    }

    pub fn toggle_admin(&mut self) {
        if self.user_info.admin {
            self.user_info.admin = false
        } else if !self.user_info.admin {
            self.user_info.admin = true
        }
    }

    pub fn delete_user(&mut self) {
        self.user_list.user_hash.insert(
            self.user_to_delete,
            User { 
                active: false, 
                username: String::new(), 
                email: String::new(), 
                uuid: self.user_to_delete, 
                admin: false 
            }
        );

        self.user_list.user_vec.remove((self.user_to_delete - 1000) as usize);
        self.user_list.user_vec
            .push(
                User { 
                    active: false, 
                    username: String::new(), 
                    email: String::new(), 
                    uuid: self.user_to_delete, 
                    admin: false 
                }
            );

        self.user_list.user_vec.sort_by_key(|item| item.uuid);
    }
}

impl Default for UserList {
    fn default() -> Self {
        Self { 
            user_hash: HashMap::new(), 
            user_vec: Vec::new(), 
            list_state: ListState::default().with_selected(None) 
        }
    }
}

fn read_json_from_file(file: &str) -> Result<String, Box<dyn Error>> {
    let string = fs::read_to_string(file)?;
    Ok(string)
}
