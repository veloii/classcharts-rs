#[macro_export]
macro_rules! new_params {
    ($($key:expr, $value:expr),*) => {
        {
            let mut body = url::form_urlencoded::Serializer::new(String::new());
            $(
                body.append_pair($key, $value);
            )*
            body.finish()
        }
    };
}
