# json-commons
A set of JSON common tools in Rust

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
let json = jsonc.read_str(path);

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


*__Attention__: The dotted path does not work in lists*


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
      "model": "Gurgel Itaipu E400",
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
