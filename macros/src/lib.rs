#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($name:expr => $val:expr) => {
        {
            let mut temp_map = ::std::collections::HashMap::new();
            temp_map.insert($name, $val);
            temp_map
        }
    };
    ( $( $name:expr => $val:expr ),+ $(,)?) => {
        {
            let mut temp_map = ::std::collections::HashMap::new();
            $(
                temp_map.insert($name, $val);
            )*
            temp_map
        }
    };
}
