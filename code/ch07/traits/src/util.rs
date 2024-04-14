#[allow(dead_code)]
pub fn get_type<T>(a: &T) -> &'static str {
    get_type_imp(a)
}

pub fn get_type_imp<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[macro_export]
macro_rules! tuple_len {
    ($($name:ident),*) => {
        {
            let mut count = 0;
            $(let _ = stringify!($name); count += 1;)*
            count
        }
    };
}

#[macro_export]
macro_rules! tuple_value_by_index {
    ($tuple:expr, $index:ident) => {{
        let tuple = $tuple;
        let index = $index;

        let len = tuple_len!(tuple);

        if index < len {
            match index {
                // $(
                //     index => Some(0),
                // )*
                _ => None,
            }
        } else {
            None
        }
    }};
}