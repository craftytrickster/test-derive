extern crate schema;
#[macro_use]
extern crate schema_derive;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use schema::SchemaObject;

#[derive(Debug, SchemaObject, Serialize, Deserialize)]
enum Varying {
    Item1(FrenchToast),
    Item2(Waffles)
}


#[derive(Debug, SchemaObject, Serialize, Deserialize)]
struct FrenchToast {
    has_cinnamon: bool,
    quantity: u8
}

#[derive(Debug, SchemaObject, Serialize, Deserialize)]
struct Waffles {
    nooks: u8,
}

fn main() {
    let bytes = b"{  \"Item1\": {  \"has_cinnamon\": true, \"quantity\": 49 } }";
    let toast = Varying::deserialize_me(bytes, true);
    println!("Check out my toast! {:?}", toast);

    let waf_bytes = b"<Item2 nooks=\"10\"></Item2>";
    let waffle = Varying::deserialize_me(waf_bytes, false);
    println!("Check out my waffle! {:?}", waffle);

}
