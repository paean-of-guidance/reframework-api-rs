#[macro_export]
macro_rules! invoke_method {
    ($method:ident, $this:ident) => {{
        let mut args: Vec<*mut c_void> = Vec::new();
        $method.invoke($this as *mut c_void, &mut args)
    }};
    ($method:ident, $this:ident, $($arg:expr),*) => {{
        let mut args: Vec<*mut c_void> = Vec::new();

        $(
            let arg_ptr: *mut c_void = $arg as *mut c_void;
            args.push(arg_ptr);
        )*

        let result = $method.invoke($this as *mut c_void, &mut args);

        result
    }};
}
