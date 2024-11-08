use basic_logger::Logger;

use crate::raw::api::*;

mod basic_logger;
#[cfg(feature = "logging")]
mod logger;
mod wrapper;

pub use wrapper::*;

pub const REFRAMEWORK_PLUGIN_VERSION_MAJOR: i32 = 1;
pub const REFRAMEWORK_PLUGIN_VERSION_MINOR: i32 = 10;
pub const REFRAMEWORK_PLUGIN_VERSION_PATCH: i32 = 0;

static mut INSTANCE: Option<RefAPI> = None;
#[cfg(feature = "logging")]
static mut LOGGER: Option<logger::RefLogger> = None;

/// API wrapper for the ReFramework API in Rust
pub struct RefAPI {
    param: *const REFrameworkPluginInitializeParam,
    sdk: *const REFrameworkSDKData,
    lua_mtx: std::sync::Mutex<()>,
}

unsafe impl Send for RefAPI {}
unsafe impl Sync for RefAPI {}

impl RefAPI {
    /// Creates a new API instance from param
    unsafe fn new(param: *const REFrameworkPluginInitializeParam) -> Option<Self> {
        if param.is_null() {
            return None;
        }

        let sdk = (*param).sdk;
        Some(RefAPI {
            param,
            sdk,
            lua_mtx: std::sync::Mutex::new(()),
        })
    }

    /// Initialize the API instance.
    ///
    /// After initialized, use [RefAPI::instance] to get the instance.
    pub unsafe fn initialize(param: *const REFrameworkPluginInitializeParam) -> Option<&'static RefAPI> {
        INSTANCE = RefAPI::new(param);

        Self::instance()
    }

    /// Gets the current API instance.
    pub fn instance() -> Option<&'static RefAPI> {
        unsafe { INSTANCE.as_ref() }
    }

    #[cfg(feature = "logging")]
    /// Initializes for `log` crate.
    ///
    /// # Safety
    ///
    /// Ensure RefAPI is initialized.
    pub unsafe fn init_log(prefix: &str, max_level: log::LevelFilter) {
        use logger::RefLogger;

        if Self::instance().is_none() {
            return;
        }

        let logger = RefLogger::new(prefix);
        LOGGER = Some(logger);

        let _ = log::set_logger(LOGGER.as_ref().unwrap());
        log::set_max_level(max_level);
    }

    #[inline]
    pub fn param_ptr(&self) -> *const REFrameworkPluginInitializeParam {
        self.param
    }

    #[inline]
    pub fn sdk_ptr(&self) -> *const REFrameworkSDKData {
        self.sdk
    }

    pub fn param(&self) -> &REFrameworkPluginInitializeParam {
        unsafe { &*self.param }
    }

    pub fn sdk(&self) -> &REFrameworkSDKData {
        unsafe { &*self.sdk }
    }

    pub fn log(&self) -> Logger {
        Logger::new(self)
    }

    pub fn tdb(&self) -> &REFrameworkTDB {
        unsafe {
            let tdb_ptr: *const __REFrameworkTDBHandle = (self.sdk().functions().get_tdb)();
            let tdb_: *const REFrameworkTDB = std::mem::transmute(tdb_ptr);
            &*tdb_
        }
    }
}
