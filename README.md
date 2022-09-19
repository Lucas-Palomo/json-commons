# json-commons
A set of JSON common tools in Rust.



> ## Disclaimer
> This package is a just wrapper for ***[json](https://docs.rs/json/0.12.4/json/)*** and ***[bson](https://docs.rs/bson/2.4.0/bson/)*** packages.



## Features

* read_str

  * return a `json::JsonValue` from str 

* read_file

  * return a `json::JsonValue` from file

* path_exists

  * return a `bool` if dotted path exists

* get_path

  * return an `Option<json::JsonValue>`

* parse_to_vec

  * return a `json::JsonValue` array as `Vec<json::JsonValue>`

* serialize

  * return a JSON string, like a Javascript "JSON.stingify"

* serialize_to_bson_hex

  * return a Hex String that represents BSON

* json_from_bson_hex

  * return a `json::JsonValue` from a Hex String

* parse_to_document

  * return a `bson::Document` from a `json::JsonValue`

* json_from_document

  * return a `json::JsonValue` from a `bson::Document`

* parse_to_bson

  * return a `bson::Bson` from a `json::JsonValue`

* json_from_bson

  * return a `json::JsonValue` from a `bson::Bson`

    

---

## Usage

```rust
use json_commons::JsonCommons;


fn main() {
    let jsonc = JsonCommons::new();
}
```

---

### Parsing Features

Parsing features allows to read a JSON content but in case of failure a panic error occurs

#### Reading a JSON String

```rust
let content = "{\"car\": {\"model\": \"Gurgel Itaipu E400\", \"color\": \"red\"}}";
let json = jsonc.read_str(content);
```

#### Reading a JSON File

```rust
let path = "/user/docs/myfile.json";
let json = jsonc.read_file(path);
```

---

### Dotted Paths Features

The following examples uses this json content

```json
{
  "car": {
    "model": "Gurgel Itaipu E400",
    "color": "red"
  }
}
```

#### Path Exists

You can check if the path exists

```rust
let json = jsonc.read_str(content);

jsonc.path_exists("car", json); // The output is True
jsonc.path_exists("car.model", json); // The output is True

jsonc.path_exists("car.name", json); // The output is False
jsonc.path_exists("person", json); // The output is False

```

#### Get Path

You can get any path as an optional

```rust
let json = jsonc.read_str(content);

jsonc.get_path("car", json); // The output is Ok, containing a JsonValue
jsonc.get_path("car.model", json); // The output is Ok, containing a JsonValue

jsonc.get_path("car.name", json); // The output is None
jsonc.get_path("person", json); // The output is None

```

#### Dotted Paths with Lists

You can get a child element by accessing via index, see this example

```json
{
  "car": {
  	"model": "Gurgel Itaipu E400",
  	"variants": [
      		{
      			"fuel": "gasoline",
      			"color": "yellow"
      		},
      		{
      			"fuel": "eletric",
      			"color": "red"
      		}
      	]
    }
}
```

```rust
let json = jsonc.read_str(content);

jsonc.get_path("car.variants", json); // The output is Ok, containing a JsonValue
jsonc.get_path("car.variants.0", json); // The output is Ok, containing a JsonValue
jsonc.get_path("car.variants.1.fuel", json); // The output is Ok, containing a JsonValue

jsonc.get_path("car.variants.99.fuel", json); // The output is None
jsonc.get_path("car.variants.one", json); // The output is None
```


---

### List Features

Consider the following content

```json
{
  "cars": [
    {
      "model": "Gurgel Itaipu E400",
      "color": "red"
    },
    {
      "model": "Gurgel X15",
      "color": "brown"
    }
  ]
}
```

#### Parse to Vec

You can parse a list to a vec

```rust

let json = jsonc.read_str(content);

let cars = jsonc.get_path("cars", json).unwrap();
jsonc.parse_to_vec(cars); // The output is a vec of JsonValue, with len 2

```

---

### Serializer Features

#### Serialize

This feature is equivalent to Javascript **__JSON.stringify__**

Consider the following content

```json
{
  "car": {
    "model": "Gurgel Itaipu E4000",
    "color": "red"
  }
}
```
Using serialize the output is

```rust
let serialized = jsonc.serialize(myjson);
assert_eq!("{\"car\":{\"model\":\"Gurgel Itaipu E400\",\"color\":\"red\"}}", serialized);
```

#### Serialize to BSON Hex

```rust
let bson_hex = jsonc.serialize_to_bson_hex(myjson);
assert_eq!("3c000000036361720032000000026d6f64656c001300000047757267656c2049746169707520453430300002636f6c6f720004000000726564000000", bson_hex)
```

You can check bson hex using this tool

* [BSON Hex to JSON](http://mcraiha.github.io/tools/BSONhexToJSON/bsonhextojson.html)

---

### Parser Features

* json_from_bson_hex
  * return a `json::JsonValue` from a Hex String
* parse_to_document
  * return a `bson::Document` from a `json::JsonValue`
* json_from_document
  * return a `json::JsonValue` from a `bson::Document`
* parse_to_bson
  * return a `bson::Bson` from a `json::JsonValue`
* json_from_bson
  * return a `json::JsonValue` from a `bson::Bson`

