# join_str

Simple proc macro to do simple string interpolation as an array join

```rust
use join_str::jstr;

fn main () {
  let actor_name = "GameROMPlayer";
  assert_eq!(
    String::from("Actor/ActorLink/GameROMPlayer.bxml"),
    jstr!("Actor/ActorLink/{actor_name}.bxml")
  );
}
```

If your interpolated code contains quotation marks, pass a raw string to the macro:

```rust
use join_str::jstr;
use std::collections::HashMap;

fn main () {
  let mut actors: HashMap<String, String> = HashMap::new();
  actors.insert(String::from("GameROMPlayer"), String::from("Link"));
  assert_eq!(
    String::from("The GameROMPlayer actor is Link"),
    jstr!(r#"The GameROMPlayer actor is {actors["GameROMPlayer"]}"#)
  );
}
```
