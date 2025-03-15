#[macro_export]
macro_rules! static_ref {
    ($name:ident) => {
        &*&raw const $name
    };
}

#[macro_export]
macro_rules! static_mut {
    ($name:ident) => {
        &mut *&raw mut $name
    };
}
