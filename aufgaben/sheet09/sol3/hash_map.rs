
macro_rules! hash_map {
    ( $($k:expr => $v:expr ,)* ) => {{
        let mut hash_map = ::std::collections::HashMap::new();
        $( hash_map.insert($k, $v); )*
        hash_map
    }};
    ( $($k:expr => $v:expr ),* ) => { hash_map!($($k => $v ,)*) };
}


fn main() {
    let ages = hash_map!{ "Sabine" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
}
