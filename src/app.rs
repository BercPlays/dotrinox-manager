use std::fs::File;
use std::io::Write;
use std::{error, fs};

use reqwest::blocking::Client;
use serde_json::{from_str, to_string_pretty};

use crate::files::{config_file_path, project_directory, read_file};
use crate::panel_api_wrapper::{OAuth2TokenRequestPayload, PanelAPI, PanelWrapper};
use crate::panel_types::{ConfigJson, Perms, Users};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Actions(pub Vec<&'static str>);

#[derive(Debug)]
pub enum Menus {
    ActionPage = 1,
    VerifyUserPage = 2,
    ErrorPage = 3,
    BanPage = 4,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub selected_action: usize,
    pub actions: Actions,
    pub current_menu: Menus,
    pub panel_wrapper: PanelWrapper,
    pub users: Users,
    pub new_users: Users,
    pub user_vec: Vec<String>,
    pub new_user_vec: Vec<String>,

    pub verified_users: Users,
    pub verified_user_vec: Vec<String>,

    pub error_string: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            selected_action: 0,
            actions: Actions(vec![
                "Verify a user",
                "Ban a user",
                "Maintnence mode (take away permissions from users) (Not finished)",
                "Open config folder",
            ]),
            current_menu: Menus::ActionPage,
            panel_wrapper: PanelWrapper::new(OAuth2TokenRequestPayload::default()),
            users: Users::default(),
            new_users: Users::default(),
            user_vec: Vec::new(),
            new_user_vec: Vec::new(),
            error_string: String::from(""),
            verified_user_vec: Vec::default(),
            verified_users: Users::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_action(&mut self) {
        if let Some(res) = self.selected_action.checked_add(1) {
            self.selected_action = res;
        }
    }

    pub fn previous_action(&mut self) {
        if let Some(res) = self.selected_action.checked_sub(1) {
            self.selected_action = res;
        }
    }

    pub fn select_action(&mut self) {
        let index = &self.selected_action;
        match index {
            0 => {
                let json = from_str::<ConfigJson>(read_file(config_file_path()).as_str()).unwrap();
                self.current_menu = Menus::VerifyUserPage;
                match self.panel_wrapper.users() {
                    Ok(users) => {
                        self.users = users;

                        self.new_user_vec = vec![];
                        self.user_vec = vec![];
                        self.verified_user_vec = vec![];
                        self.verified_users.users = vec![];
                        self.new_users.users = vec![];

                        for user in &self.users.users {
                            self.user_vec
                                .push(format!("{} ({})", user.username, user.email))
                        }
                        for user in &self.users.users {
                            if !json.verify_data.contains(&user.id) {
                                self.new_users.users.push(user.clone())
                            }
                        }
                        for user in &self.users.users {
                            if json.verify_data.contains(&user.id) {
                                self.verified_users.users.push(user.clone())
                            }
                        }
                        for user in &self.verified_users.users {
                            self.verified_user_vec
                                .push(format!("{} ({})", user.username, user.email))
                        }
                        for user in &self.new_users.users {
                            self.new_user_vec
                                .push(format!("{} ({})", user.username, user.email))
                        }
                    }
                    Err(err) => {
                        self.current_menu = Menus::ErrorPage;
                        self.error_string = err.to_string();
                    }
                };
                self.selected_action = 0;
            }
            1 => {
                let json = from_str::<ConfigJson>(read_file(config_file_path()).as_str()).unwrap();
                self.current_menu = Menus::BanPage;
                match self.panel_wrapper.users() {
                    Ok(users) => {
                        self.users = users;

                        self.new_user_vec = vec![];
                        self.user_vec = vec![];
                        self.verified_user_vec = vec![];
                        self.verified_users.users = vec![];
                        self.new_users.users = vec![];

                        for user in &self.users.users {
                            self.user_vec
                                .push(format!("{} ({})", user.username, user.email))
                        }
                        for user in &self.users.users {
                            if !json.verify_data.contains(&user.id) {
                                self.new_users.users.push(user.clone())
                            }
                        }
                        for user in &self.users.users {
                            if json.verify_data.contains(&user.id) {
                                self.verified_users.users.push(user.clone())
                            }
                        }
                        for user in &self.verified_users.users {
                            self.verified_user_vec
                                .push(format!("{} ({})", user.username, user.email))
                        }
                        for user in &self.new_users.users {
                            self.new_user_vec
                                .push(format!("{} ({})", user.username, user.email))
                        }
                    }
                    Err(err) => {
                        self.current_menu = Menus::ErrorPage;
                        self.error_string = err.to_string();
                    }
                };
                self.selected_action = 0;
            }

            3 => match opener::open(project_directory()) {
                Ok(ok) => (),
                Err(err) => panic!("CANT OPEN FOLDER"),
            },
            _ => todo!(),
        }
    }

    pub fn select_user(&mut self, index: usize) {
        match self.current_menu {
            Menus::VerifyUserPage => {
                let mut json =
                    from_str::<ConfigJson>(read_file(config_file_path()).as_str()).unwrap();
                self.panel_wrapper
                    .set_perms(self.new_users.users[index].id, json.user_perms)
                    .unwrap();
                let mut json1 =
                    from_str::<ConfigJson>(read_file(config_file_path()).as_str()).unwrap();
                json1.verify_data.push(self.new_users.users[index].id);
                fs::write(
                    config_file_path(),
                    to_string_pretty::<ConfigJson>(&json1).unwrap().as_bytes(),
                );
                self.current_menu = Menus::ActionPage;
            }
            Menus::BanPage => {
                let mut json =
                    from_str::<ConfigJson>(read_file(config_file_path()).as_str()).unwrap();

                self.panel_wrapper
                    .set_perms(self.verified_users.users[index].id, Perms::default())
                    .unwrap();
                json.verify_data.remove(index);
                fs::write(
                    config_file_path(),
                    to_string_pretty::<ConfigJson>(&json).unwrap().as_bytes(),
                );

                self.current_menu = Menus::ActionPage;
            }
            _ => todo!(),
        }
    }
}
