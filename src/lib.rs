extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod array;
pub mod data_source;
pub mod element;
mod link;
pub mod linked_list;

pub trait CloneDefault: Clone + Default {}
impl<T> CloneDefault for T where T: Clone + Default {}

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

#[derive(Debug)]
pub struct Bridges {
    pub assignment_number: u32,
    pub user_name: String,
    pub api_key: String,
    server: Server,
    general_fields: GeneralFields,
    data_structure: serde_json::Value,
    subassignment_index: u32,
    show_json: bool,
}

impl Bridges {
    /// Constructor for Bridges struct
    /// # Example
    /// ```
    /// use bridges::Bridges;
    /// let mut my_bridges = Bridges::new(1, "user_name", "api_key");
    ///
    /// ```
    pub fn new(assignment_number: u32, user_name: &str, api_key: &str) -> Self {
        Self::new_from_strings(
            assignment_number,
            String::from(user_name),
            String::from(api_key),
        )
    }

    /// Same as other construct except with `String`s in place of `str`
    pub fn new_from_strings(assignment_number: u32, user_name: String, api_key: String) -> Self {
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
            show_json: false,
        }
    }

    pub fn set_server(&mut self, server: Server) {
        self.server = server;
    }

    /// Sets server with str instead of using Server enum, options: `"live"`, `"clone"`, `"local"`
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

    pub fn set_show_json(&mut self, show_json: bool) {
        self.show_json = show_json;
    }

    /// Serializes data structure and stores it on Bridges struct
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

    /// Attempts to post visualization to a BRIDGES server
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

        let mut json: serde_json::Value = match serde_json::to_value(&self.general_fields) {
            Ok(s) => s,
            Err(error) => panic!("There was a problem serialzing Bridges fields: {}", error),
        };

        merge(&mut json, self.data_structure.clone());
        match json {
            serde_json::Value::Null => panic!(
                "There was a problem adding the data_structure to the JSON, data_structure: {:?}",
                self.data_structure
            ),
            _ => (),
        }

        let client = reqwest::Client::new();
        let resp = match client.post(uri.as_str()).json(&json).send() {
            Ok(resp) => resp,
            Err(error) => panic!("There was a problem sending the request: {}", error),
        };

        if self.show_json {
            println!("{:?}", json);
        }
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

    /// Wrapper function that calls `update_data_structure` and `visualize`
    pub fn update_and_visualize(&mut self, data_structure: impl serde::ser::Serialize) {
        self.update_data_structure(data_structure);
        self.visualize();
    }
}

// exposes struct functions for ease of access
/// Function that exposes the constructor for Bridges struct
/// # Example
/// ```
/// let mut my_bridges = bridges::new(1, "user_name", "api_key");
///
/// ```
pub fn new(assignment_number: u32, user_name: &str, api_key: &str) -> Bridges {
    Bridges::new_from_strings(
        assignment_number,
        String::from(user_name),
        String::from(api_key),
    )
}
pub fn new_from_strings(assignment_number: u32, user_name: String, api_key: String) -> Bridges {
    Bridges::new_from_strings(assignment_number, user_name, api_key)
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
    fn test_arr_post() {
        use super::*;
        use array::Array;
        use element::Element;
        use std::env;

        let user_name = match env::var("BRIDGES_USER_NAME") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_USER_NAME: {:?}", error),
        };
        let api_key = match env::var("BRIDGES_API_KEY") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_API_KEY: {:?}", error),
        };

        let mut my_bridges = new_from_strings(1, user_name, api_key);
        let mut my_array: Array<i32> = array::new();
        my_array.dims = vec![5, 0, 0];
        for item in 0..5 {
            let mut my_element: Element<i32> = element::new(item.clone());
            my_element.color = vec![63.75 * item as f32, 0.0, 0.0, 1.0];
            my_element.name = item.to_string();
            my_array.nodes.push(my_element);
        }
        my_bridges.set_server(Server::Clone);
        my_bridges.update_and_visualize(&my_array);
    }

    #[test]
    fn test_ll_post() {
        use super::*;
        use linked_list::ListType;
        use std::env;

        let user_name = match env::var("BRIDGES_USER_NAME") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_USER_NAME: {:?}", error),
        };
        let api_key = match env::var("BRIDGES_API_KEY") {
            Ok(var) => var,
            Err(error) => panic!("There was a problem reading BRIDGES_API_KEY: {:?}", error),
        };

        let mut my_bridges = new_from_strings(2, user_name, api_key);
        my_bridges.set_show_json(true);
        my_bridges.set_server(Server::Clone);
        build_list_and_vis(&mut my_bridges, ListType::Single);
        build_list_and_vis(&mut my_bridges, ListType::Double);
        build_list_and_vis(&mut my_bridges, ListType::CircleSingle);
        build_list_and_vis(&mut my_bridges, ListType::CircleDouble);
    }

    pub fn build_list_and_vis(
        my_bridges: &mut super::Bridges,
        list_type: super::linked_list::ListType,
    ) {
        use super::*;
        use element::Element;
        use linked_list::LinkedList;

        let mut my_list: LinkedList<i32> = linked_list::new();
        my_list.set_list_type(list_type);
        for item in 0..5 {
            let mut my_element: Element<i32> = element::new(item);
            my_element.color = vec![63.75 * item as f32, 0.0, 0.0, 1.0];
            my_element.name = item.to_string();
            my_list.append(my_element);
        }

        my_bridges.update_and_visualize(&my_list);
    }
}
