extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod array;
pub mod data_source;

#[derive(Debug)]
pub struct Bridges {
    pub assignment_number: u32,
    pub user_name: String,
    pub api_key: String,
    server: Server,
    general_fields: GeneralFields,
    data_structure: serde_json::Value,
    subassignment_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeneralFields {
    title: String,
    map_overlay: bool,
    coord_system_type: String,
}

#[derive(Clone, Debug)]
pub enum Server {
    Live,
    Clone,
    Local,
    None,
}

impl Bridges {
    pub fn new(assignment_number: u32, user_name: &str, api_key: &str) -> Bridges {
        Bridges::new_from_strings(
            assignment_number,
            String::from(user_name),
            String::from(api_key),
        )
    }

    pub fn new_from_strings(assignment_number: u32, user_name: String, api_key: String) -> Bridges {
        Bridges {
            assignment_number,
            user_name,
            api_key,
            server: Server::Live,
            general_fields: GeneralFields {
                title: String::from(""),
                map_overlay: false,
                coord_system_type: String::from(""),
            },
            data_structure: serde_json::Value::default(),
            subassignment_index: 0,
        }
    }

    pub fn set_server(&mut self, server: Server) {
        self.server = server;
    }

    pub fn set_server_from_str(&mut self, server: &str) -> Result<Server, &'static str> {
        let server: Server = match server {
            "live" => Server::Live,
            "clone" => Server::Clone,
            "local" => Server::Local,
            _ => Server::None,
        };

        match server {
            Server::None => Err("Invalid server use: live, clone, or local"),
            _ => {
                self.set_server(server.clone());
                Ok(server)
            }
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.general_fields.title = title;
    }

    pub fn set_map_overlay(&mut self, map_overlay: bool) {
        self.general_fields.map_overlay = map_overlay;
    }

    pub fn update_data_structure(&mut self, data_structure: impl serde::ser::Serialize) {
        let json: serde_json::Value = match serde_json::to_value(&data_structure) {
            Ok(s) => s,
            Err(error) => panic!(
                "There was a problem serialzing Bridges data_structure: {}",
                error
            ),
        };
        self.data_structure = json;
    }

    pub fn visualize(&mut self) {
        use std::fmt::Write;

        let url = match self.server {
            Server::Live => "http://bridges-cs.herokuapp.com",
            Server::Clone => "http://bridges-clone.herokuapp.com",
            Server::Local => "http://127.0.0.1:3000",
            Server::None => "",
        };

        let mut uri = String::from(url);
        let subassignment_index_leading_zero = match self.subassignment_index {
            0..=9 => "0",
            _ => "",
        };

        write!(
            &mut uri,
            "/assignments/{}.{}{}?apikey={}&username={}",
            self.assignment_number,
            subassignment_index_leading_zero,
            self.subassignment_index,
            self.api_key,
            self.user_name
        );

        println!("{}", uri);

        let mut json: serde_json::Value = match serde_json::to_value(&self.general_fields) {
            Ok(s) => s,
            Err(error) => panic!("There was a problem serialzing Bridges fields: {}", error),
        };

        merge(&mut json, self.data_structure.clone());
        let client = reqwest::Client::new();
        let resp = match client.post(uri.as_str()).json(&json).send() {
            Ok(resp) => resp,
            Err(error) => panic!("There was a problem sending the request: {}", error),
        };

        println!("{:?}", json);
        self.subassignment_index += 1;

        use reqwest::StatusCode;
        match resp.status() {
            StatusCode::OK => println!(
                "\nCheck Your Visualization at the following link:\n\n{}/assignments/{}/{}\n\n",
                url, self.assignment_number, self.user_name
            ),
            _ => println!(
                "There was a problem sending the visualization representation to the server. StatusCode:{} \n",
                resp.status()
            ),
        };
    }
}

/*
 *  This function takes two deserialized JSON Values and adds the second Value's fields to the
 *  first's
 */
use serde_json::Value;
fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_vis_post() {
        use super::*;
        use std::env;

        let user_name = match env::var("BRIDGES_USER_NAME") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_USER_NAME: {:?}", error),
        };
        let api_key = match env::var("BRIDGES_API_KEY") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_API_KEY: {:?}", error),
        };

        let mut my_bridges = Bridges::new_from_strings(1, user_name, api_key);
        let my_array = array::Array::<i32>::new();
        my_bridges.set_data_structure(&my_array);
        my_bridges.visualize();
    }
}
