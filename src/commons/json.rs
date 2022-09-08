use json::{JsonValue, parse};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

pub struct JsonCommons {}


impl JsonCommons {

    pub fn new() -> JsonCommons {
        return JsonCommons {};
    }

    pub fn read_str(&self, content: &str) -> JsonValue {
        return parse(content).expect("Failed to parse content to JSON");
    }

    pub fn read_file(&self, file_path: &str) -> JsonValue {
        let mut file = File::open(file_path).expect(format!("Failed to open {}", file_path).as_str());
        let mut content = String::new();
        file.read_to_string(&mut content).expect(format!("Failed to read content from {}", file_path).as_str());
        return self.read_str(content.as_str())
    }

    pub fn get_path(&self, path: &str, content: JsonValue) -> Option<JsonValue> {
        let mut keys = path.split(".")
            .map(|key| key.to_string())
            .collect::<VecDeque<String>>();

        if keys.is_empty() {
            return None;
        }

        let key = keys.pop_front().unwrap();

        if content.has_key(&key) && keys.len() > 0 {
            return if content[&key].is_array() {
                let parse_index = keys.pop_front().unwrap().parse::<usize>();

                match parse_index {
                    Ok(index) => {
                        let array = self.parse_to_vec(content[&key].to_owned());

                        match array.get(index).cloned() {
                            Some(object) => {
                                let last_key = format!("{}.{}", &key, &index);
                                return if keys.len() > 0 {
                                    self.get_path(&self.gen_path(path.to_string(), last_key), object)
                                } else {
                                    return Some(object)
                                }
                            }
                            None => None
                        }
                    },
                    Err(_) => None
                }
            } else {
                self.get_path(&self.gen_path(path.to_string(), key.to_owned()), content[&key].to_owned())
            }
        }

        return if content.has_key(&key) {
            Some(content.to_owned()[&key].take())
        } else {
            None
        };
    }

    pub fn path_exists(&self, path: &str, content: JsonValue) -> bool {
        return self.get_path(&path, content.to_owned()).is_some();
    }

    pub fn parse_to_vec(&self, content: JsonValue) -> VecDeque<JsonValue> {
        let mut vec = VecDeque::<JsonValue>::new();
        if content.is_array() && content.members().len() > 0 {
            for member in content.members() {
                if !member.is_empty() {
                    vec.push_back(member.to_owned());
                }
            }
        }
        return vec;
    }

    fn gen_path(&self, path: String, last_key: String) -> String {
        return path.replace(format!("{}.", last_key).as_str(), "");
    }
}