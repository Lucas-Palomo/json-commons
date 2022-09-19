use std::collections::vec_deque::VecDeque;
use json::{JsonValue, parse};
use std::fs::File;
use std::io::Read;
use bson::{Array, Bson, Document};

pub struct JsonCommons {}


impl JsonCommons {

    /// Creates a new instance of `JsonCommons`
    /// # Example
    /// ```rust
    /// use json_commons::JsonCommons;
    ///
    /// let jsonc = JsonCommons::new();
    /// ```
    pub fn new() -> JsonCommons {
        return JsonCommons {};
    }

    /// Read a `&str` and parse to a `JsonValue`.
    /// # Panics
    /// Panics if content cannot be parsed to a `JsonValue`.
    /// # Example
    /// ```rust
    /// let content = "{\"car\": {\"model\": \"Gurgel Itaipu E400\", \"color\": \"red\"}}";
    ///
    /// let json = jsonc.read_str(content);
    /// ```
    pub fn read_str(&self, content: &str) -> JsonValue {
        return parse(content).expect("Failed to parse content to JSON");
    }

    /// Open and parse all file content to a `JsonValue`
    /// # Panics
    /// - If occurs an error to open or read file
    /// - If content cannot be parsed to a `JsonValue`
    /// # Example
    /// ```rust
    /// let path = "/user/docs/myfile.json";
    /// let json = jsonc.read_file(path);
    /// ```
    pub fn read_file(&self, file_path: &str) -> JsonValue {
        let mut file = File::open(file_path).expect(format!("Failed to open {}", file_path).as_str());
        let mut content = String::new();
        file.read_to_string(&mut content).expect(format!("Failed to read content from {}", file_path).as_str());
        return self.read_str(content.as_str())
    }

    /// You can get any dotted path as an Optional
    ///
    /// If path is not found then return is `None`
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

    /// Check if a dotted path exists
    pub fn path_exists(&self, path: &str, content: JsonValue) -> bool {
        return self.get_path(&path, content.to_owned()).is_some();
    }

    /// Parse a list of `JsonValue` to a `VecDeque<JsonValue>`
    ///
    /// # Examples
    ///
    /// Consider the following content
    ///
    /// ```json
    /// {
    ///   "cars": [
    ///     {
    ///      "model": "Gurgel Itaipu E400",
    ///       "color": "red"
    ///     },
    ///     {
    ///       "model": "Gurgel X15",
    ///       "color": "brown"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// You can Parse the key **__cars__** to a vec
    ///
    /// ```rust
    /// let json = jsonc.read_str(content);
    /// let cars = jsonc.get_path("cars", json).unwrap();
    ///
    /// jsonc.parse_to_vec(cars); // The output is a vec of JsonValue, with len 2
    /// ```
    pub fn parse_to_vec(&self, content: JsonValue) -> VecDeque<JsonValue> {
        let mut vec = VecDeque::<JsonValue>::new();
        if content.is_array() && content.members().len() > 0 {
            for member in content.members() {
                vec.push_back(member.to_owned());
            }
        }
        return vec;
    }

    /// Serialize a `JsonValue` into a `String`.
    ///
    /// Consider the following content
    /// ```json
    /// {
    ///     "car": {
    ///         "model": "Gurgel Itaipu E4000",
    ///         "color": "red"
    ///     }
    /// }
    /// ```
    /// Output is equivalent to Javascript **__JSON.stringify__**
    /// ```rust
    /// let serialized = jsonc.serialize(myjson);
    /// assert_eq!("{\"car\":{\"model\":\"Gurgel Itaipu E400\",\"color\":\"red\"}}", serialized);
    /// ```
    pub fn serialize(&self, json: JsonValue) -> String {
        return format!("{:#}", format!("{}", json.to_string()));
    }

    /// Parse a `Document` into a `JsonValue`
    pub fn json_from_document(&self, doc: Document) -> JsonValue {
        return self.read_str(format!("{:#}", format!("{}", doc.to_string())).as_str())
    }

    /// Parse a `Bson` into a `JsonValue`
    pub fn json_from_bson(&self, bson: Bson) -> JsonValue {
        return self.read_str(format!("{:#}", format!("{}", bson.to_string())).as_str())
    }

    /// Parse a Hex String into a `JsonValue`
    pub fn json_from_bson_hex(&self, bson_hex: String) -> JsonValue {
        let bytes = hex::decode(bson_hex).expect("Failed to decode hex");
        let doc = Document::from_reader(&mut bytes.as_slice()).expect("Failed to read bytes");
        return self.json_from_document(doc);
    }

    /// Parse a `JsonValue` into a  `Document`
    pub fn parse_to_document(&self, json: JsonValue) -> Document {
        let mut document = Document::new();

        for (key, value) in json.entries() {
            document.insert(key, self.bson_value(value.to_owned()));
        }

        return document
    }

    /// Parse a `JsonValue` into a `Bson`
    pub fn parse_to_bson(&self, json: JsonValue) -> Bson {
        return Bson::from(self.parse_to_document(json.to_owned()))
    }

    /// Serialize a `JsonValue` into a Hex `String`.
    ///
    /// Consider the following content
    /// ```json
    /// {
    ///     "car": {
    ///         "model": "Gurgel Itaipu E4000",
    ///         "color": "red"
    ///     }
    /// }
    /// ```
    /// Output is a encoded Hex BSON
    /// ```rust
    /// let bson_hex = jsonc.serialize_to_bson_hex(myjson);
    /// assert_eq!("3c000000036361720032000000026d6f64656c001300000047757267656c2049746169707520453430300002636f6c6f720004000000726564000000", bson_hex);
    /// ```
    pub fn serialize_to_bson_hex(&self, json: JsonValue) -> String {
        let document = self.parse_to_document(json.to_owned());
        let mut bytes: Vec<u8> = vec![];
        document.to_writer(&mut bytes).expect("Failed to write bytes");
        return hex::encode(bytes.as_slice());
    }

    // Private Functions
    fn gen_path(&self, path: String, last_key: String) -> String {
        return path.replace(format!("{}.", last_key).as_str(), "");
    }

    fn bson_value(&self, value: JsonValue) -> Bson {

        if value.is_null() {
            return Bson::Null
        }

        if value.is_string() {
            return Bson::from(value.to_owned().as_str().unwrap());
        }

        if value.is_boolean() {
            return Bson::from(value.to_owned().as_bool().unwrap());
        }

        if value.is_number() {
            let number = value.to_owned().take().as_number().unwrap().to_string();
            let float = number.parse::<f64>();

            match float {
                Ok(float_value) => {
                    return if float_value.fract() != 0.0 {
                        Bson::from(float_value)
                    } else {
                        Bson::from(float_value as i64)
                    }
                },
                Err(e) => panic!("{}", e)
            }
        }

        if value.is_object() {
            return Bson::from(self.parse_to_document(value.to_owned()));
        }

        if value.is_array() {
            let mut array = Array::new();

            for entry in self.parse_to_vec(value.to_owned()) {
                array.push(self.bson_value(entry.to_owned()));
            }

            return Bson::from(array);
        }

        return Bson::Null;
    }
}