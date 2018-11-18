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
    pub assignment_number: i32,
    pub user_name: String,
    pub api_key: String,
    server: Server,
    general_fields: GeneralFields,
    data_structure: DataStructure,
}

#[derive(Debug)]
pub enum Server {
    Live,
    Clone,
    Local,
}

#[derive(Debug)]
pub enum DataStructure {
    Array,
    None,
}

impl Bridges {
    pub fn new_with_strings(assignment_number: i32, user_name: String, api_key: String) -> Bridges {
        Bridges {
            assignment_number,
            user_name,
            api_key,
            server: Server::Live,
            general_fields: GeneralFields {
                visual: String::from(""),
                title: String::from(""),
                map_overlay: false,
                coord_system_type: String::from(""),
            },
            data_structure: DataStructure::None,
        }
    }
    pub fn new(assignment_number: i32, user_name: &str, api_key: &str) -> Bridges {
        Bridges::new_with_strings(
            assignment_number,
            String::from(user_name),
            String::from(api_key),
        )
    }
    pub fn set_server(&mut self, server: Server) {
        self.server = server;
    }

    pub fn visualize(&self) {}
}

#[derive(Serialize, Deserialize, Debug)]
struct GeneralFields {
    visual: String,
    title: String,
    map_overlay: bool,
    coord_system_type: String,
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
        use std::fmt::Write;

        let ds = GeneralFields {
            visual: String::from("Array"),
            title: String::from("bridges-rust test case"),
            map_overlay: false,
            coord_system_type: String::from("cartesian"),
        };

        let dims = Array {
            dims: vec![0, 0, 0],
            nodes: vec![],
        };

        let user_name = match env::var("BRIDGES_USER_NAME") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_USER_NAME: {:?}", error),
        };
        let api_key = match env::var("BRIDGES_API_KEY") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_API_KEY: {:?}", error),
        };

        let assignment = "1.0";
        let mut uri = String::from("http://bridges-clone.herokuapp.com/assignments/");
        write!(
            &mut uri,
            "{}?apikey={}&username={}",
            assignment, api_key, user_name
        );

        let mut json_ds: serde_json::Value = match serde_json::to_value(&ds) {
            Ok(s) => s,
            Err(error) => panic!("There was a problem serialzing data structure: {}", error),
        };
        let json_arr: serde_json::Value = match serde_json::to_value(&dims) {
            Ok(s) => s,
            Err(error) => panic!("There was a problem serialzing data structure: {}", error),
        };
        println!("{:?}", json_ds);
        merge(&mut json_ds, json_arr);
        println!("{:?}", json_ds);

        let client = reqwest::Client::new();
        let resp = client.post(uri.as_str()).json(&json_ds).send();
        let resp: reqwest::Response = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("There was a problem sending the request: {}", error),
        };

        use reqwest::StatusCode;
        assert_eq!(resp.status(), StatusCode::OK);

        println!("{:?}", resp);
    }
}
