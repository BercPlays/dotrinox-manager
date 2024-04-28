use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    pub page: u8,
    pub page_size: u16,
    pub max_size: u16,
    pub total: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: u16,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Users {
    pub users: Vec<User>,
    pub paging: Paging,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Perms {
    #[serde(default)]
    admin: bool,
    #[serde(default)]
    create_servers: bool,
    #[serde(default)]
    delete_servers: bool,
    #[serde(default)]
    deploy_nodes: bool,
    #[serde(default)]
    edit_nodes: bool,
    #[serde(default)]
    edit_server_admin: bool,
    #[serde(default)]
    edit_server_data: bool,
    #[serde(default)]
    edit_server_users: bool,
    #[serde(default)]
    edit_templates: bool,
    #[serde(default)]
    edit_users: bool,
    #[serde(default)]
    email: String,
    #[serde(default)]
    install_server: bool,
    #[serde(default)]
    panel_settings: bool,
    #[serde(default)]
    put_servers_files: bool,
    #[serde(default)]
    send_server_console: bool,
    #[serde(default)]
    server_identifier: String,
    #[serde(default)]
    sftp_server: bool,
    #[serde(default)]
    start_server: bool,
    #[serde(default)]
    stop_server: bool,
    #[serde(default)]
    username: String,
    #[serde(default)]
    view_nodes: bool,
    #[serde(default)]
    view_server_console: bool,
    #[serde(default)]
    view_server_files: bool,
    #[serde(default)]
    view_server_stats: bool,
    #[serde(default)]
    view_servers: bool,
    #[serde(default)]
    view_templates: bool,
    #[serde(default)]
    view_users: bool,
}

#[derive(Deserialize, Serialize, Default)]
pub struct ConfigJson {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
    pub user_perms: Perms,
    pub verify_data: Vec<u16>,
}

impl ConfigJson {
    fn new(
        client_id: String,
        client_secret: String,
        domain: String,
        perms: Perms,
        verify_data: Vec<u16>,
    ) -> Self {
        Self {
            client_id: client_id,
            client_secret: client_secret,
            domain: domain,
            user_perms: perms,
            verify_data: verify_data,
        }
    }

    fn default() -> Self {
        Self {
            client_id: String::from("your_client_id"),
            client_secret: String::from("your_client_secret"),
            domain: String::from("https://your_endpoint.com"),
            user_perms: Perms::default(),
            verify_data: vec![],
        }
    }
}
