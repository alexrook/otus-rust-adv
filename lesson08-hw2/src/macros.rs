use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
        ( $( $key:expr => $val:expr ),* $(,)? ) => {{
            let mut m = HashMap::new();
            $(
                m.insert($key, $val);
            )*
            m
        }};
    }

pub fn demo() {
    let m = hashmap! { "a" => 1, "b" => 2, "w"=> 24 };
    println!("{m:?}");
}
