use std::ffi::CString;

use super::RefAPI;

pub struct Logger<'a> {
    api: &'a RefAPI,
}

impl<'a> Logger<'a> {
    pub fn new(api: &'a RefAPI) -> Self {
        Self { api }
    }

    pub fn info(&self, message: &str) {
        let c_msg = CString::new(message).unwrap_or_default();

        let func = self.api.param().functions().log_info;
        func(c_msg.as_ptr());
    }

    pub fn warn(&self, message: &str) {
        let c_msg = CString::new(message).unwrap_or_default();

        let func = self.api.param().functions().log_warn;
        func(c_msg.as_ptr());
    }

    pub fn error(&self, message: &str) {
        let c_msg = CString::new(message).unwrap_or_default();

        let func = self.api.param().functions().log_error;
        func(c_msg.as_ptr());
    }
}
