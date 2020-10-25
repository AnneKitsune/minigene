#[macro_export]
macro_rules! event_reader_res {
    ($name:ident, $ev_type:ty) => {
        pub struct $name(pub ReaderId<$ev_type>);
    };
}
