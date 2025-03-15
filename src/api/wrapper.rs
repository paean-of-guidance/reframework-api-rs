#![allow(clippy::missing_transmute_annotations, clippy::not_unsafe_ptr_arg_deref)]

use std::{
    ffi::{c_void, CString},
    ptr::addr_of_mut,
};

use super::*;

pub struct RefAPISdk<'a> {
    api: &'a RefAPI,
}

impl<'a> RefAPISdk<'a> {
    pub fn new(api: &'a RefAPI) -> Self {
        Self { api }
    }

    pub fn get_managed_singleton(&self, name: &str) -> Option<REFrameworkManagedObjectHandle> {
        let c_name = CString::new(name).unwrap();

        let func = self.api.sdk_raw().functions().get_managed_singleton;

        let singleton = unsafe { func(c_name.as_ptr()) };
        if singleton.is_null() {
            None
        } else {
            Some(singleton)
        }
    }

    pub fn get_native_singleton(&self, name: &str) -> Option<*mut c_void> {
        let c_name = CString::new(name).unwrap();

        let func = self.api.sdk_raw().functions().get_native_singleton;

        let singleton = unsafe { func(c_name.as_ptr()) };
        if singleton.is_null() {
            None
        } else {
            Some(singleton)
        }
    }

    pub fn create_managed_string(&self, value: &str) -> Option<REFrameworkManagedObjectHandle> {
        let value_w = value.encode_utf16().chain(Some(0)).collect::<Vec<_>>();

        let create_managed_string_func = self.api.sdk_raw().functions().create_managed_string;

        let managed_string = unsafe { create_managed_string_func(value_w.as_ptr() as *const i16) };
        if managed_string.is_null() {
            None
        } else {
            Some(managed_string)
        }
    }
}

pub struct RefAPIParam<'a> {
    api: &'a RefAPI,
}

impl<'a> RefAPIParam<'a> {
    pub fn new(api: &'a RefAPI) -> Self {
        Self { api }
    }

    pub fn on_lua_state_created(&self, callback: REFLuaStateCreatedCb) -> bool {
        let result = (self.api.param_raw().functions().on_lua_state_created)(callback);

        result
    }

    pub fn on_lua_state_destroyed(&self, callback: REFLuaStateDestroyedCb) -> bool {
        let result = (self.api.param_raw().functions().on_lua_state_destroyed)(callback);

        result
    }
}

pub struct RefAPITDB<'a> {
    api: &'a RefAPI,
    inner: *const REFrameworkTDB,
}

impl<'a> RefAPITDB<'a> {
    pub fn new(api: &'a RefAPI, inner: REFrameworkTDBHandle) -> Self {
        Self {
            api,
            inner: unsafe { std::mem::transmute(inner) },
        }
    }

    pub fn inner(&self) -> &REFrameworkTDB {
        unsafe { &*self.inner }
    }

    fn inner_handle(&self) -> REFrameworkTDBHandle {
        unsafe { std::mem::transmute(self.inner) }
    }

    pub fn find_type(&self, name: &str) -> Option<RefAPITypeDefinition> {
        let c_name = CString::new(name).unwrap_or_default();

        let find_type_func = self.api.sdk_raw().tdb().find_type;
        let result = find_type_func(self.inner_handle(), c_name.as_ptr() as _);

        if result.is_null() {
            None
        } else {
            Some(RefAPITypeDefinition::new(self.api, result))
        }
    }

    pub fn find_method(&self, type_name: &str, name: &str) -> Option<RefAPIMethod> {
        let c_type_name = CString::new(type_name).unwrap_or_default();
        let c_name = CString::new(name).unwrap_or_default();

        let find_method_func = self.api.sdk_raw().tdb().find_method;
        let result = find_method_func(self.inner_handle(), c_type_name.as_ptr() as _, c_name.as_ptr() as _);

        if result.is_null() {
            None
        } else {
            Some(RefAPIMethod::new(self.api, result))
        }
    }
}

#[repr(C, packed)]
pub union InvokeRetUnion {
    pub bytes: [u8; 128],
    pub byte: u8,
    pub word: u16,
    pub dword: u32,
    pub f: f32,
    pub qword: u64,
    pub d: f64,
    pub ptr: *mut std::ffi::c_void,
}

#[repr(C, packed)]
pub struct InvokeRet {
    pub data: InvokeRetUnion,
    pub exception_thrown: bool,
}

impl Default for InvokeRet {
    fn default() -> Self {
        // let data = unsafe { MaybeUninit::zeroed().assume_init() };
        let data = InvokeRetUnion { bytes: [0; 128] };
        Self {
            data,
            exception_thrown: false,
        }
    }
}

impl std::fmt::Debug for InvokeRet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InvokeRet")
            .field("data(ptr)", unsafe {
                &format!("{:p}", &*(self.data.ptr as *const usize))
            })
            .field("exception_thrown", &self.exception_thrown)
            .finish()
    }
}

pub struct RefAPIMethod<'a> {
    api: &'a RefAPI,
    inner: *const REFrameworkTDBMethod,
}

impl<'a> RefAPIMethod<'a> {
    pub fn new(api: &'a RefAPI, inner: REFrameworkMethodHandle) -> Self {
        Self {
            api,
            inner: unsafe { std::mem::transmute(inner) },
        }
    }

    pub fn inner(&self) -> &REFrameworkTDBMethod {
        unsafe { &*self.inner }
    }

    fn inner_handle(&self) -> REFrameworkMethodHandle {
        unsafe { std::mem::transmute(self.inner) }
    }

    pub fn add_hook(&self, pre_fn: Option<REFPreHookFn>, post_fn: Option<REFPostHookFn>, ignore_jmp: bool) -> u32 {
        let add_hook_func = self.api.sdk_raw().functions().add_hook;

        let pre_fn_ptr: REFPreHookFn = if let Some(pre_fn) = pre_fn {
            pre_fn
        } else {
            void_pre_hook_fn
        };
        let post_fn_ptr: REFPostHookFn = if let Some(post_fn) = post_fn {
            post_fn
        } else {
            void_post_hook_fn
        };

        add_hook_func(self.inner_handle(), pre_fn_ptr, post_fn_ptr, ignore_jmp)
    }

    pub fn invoke(&self, obj: *mut c_void, args: &mut [*mut c_void]) -> InvokeRet {
        let mut out = InvokeRet::default();
        unsafe {
            (self.api.sdk_raw().method().invoke)(
                self.inner_handle(),
                obj,
                args.as_mut_ptr(),
                std::mem::size_of_val(args) as u32,
                addr_of_mut!(out) as *mut c_void,
                size_of::<InvokeRet>() as u32,
            );
        }

        out
    }
}

extern "C" fn void_pre_hook_fn(
    _: i32,
    _: *mut *mut std::ffi::c_void,
    _: *mut REFrameworkTypeDefinitionHandle,
    _: u64,
) -> i32 {
    REFRAMEWORK_HOOK_CALL_ORIGINAL
}

extern "C" fn void_post_hook_fn(_: *mut *mut std::ffi::c_void, _: REFrameworkTypeDefinitionHandle, _: u64) {}

pub struct RefAPITypeDefinition<'a> {
    api: &'a RefAPI,
    inner: *const REFrameworkTDBTypeDefinition,
}

impl<'a> RefAPITypeDefinition<'a> {
    pub fn new(api: &'a RefAPI, inner: REFrameworkTypeDefinitionHandle) -> Self {
        Self {
            api,
            inner: unsafe { std::mem::transmute(inner) },
        }
    }

    pub fn inner(&self) -> &REFrameworkTDBTypeDefinition {
        unsafe { &*self.inner }
    }

    pub fn inner_handle(&self) -> REFrameworkTypeDefinitionHandle {
        unsafe { std::mem::transmute(self.inner) }
    }
}

/// A mutex-like wrapper around the Lua VM.
pub struct RefAPILua<'a, T> {
    api: &'a RefAPI,
    inner: T,
}

impl<'a, T> RefAPILua<'a, T> {
    pub fn new(api: &'a RefAPI, inner: T) -> Self {
        Self { api, inner }
    }

    pub fn lock(&self) -> RefAPILuaLock<T> {
        RefAPILuaLock::new(self.api, &self.inner)
    }
}

/// Lua VM mutex lock guard.
pub struct RefAPILuaLock<'a, 't, T> {
    api: &'a RefAPI,
    inner: &'t T,
}

impl<'a, 't, T> RefAPILuaLock<'a, 't, T> {
    fn new(api: &'a RefAPI, inner: &'t T) -> Self {
        (api.param_raw().functions().lock_lua)();

        Self { api, inner }
    }
}

impl<'a, 't, T> Drop for RefAPILuaLock<'a, 't, T> {
    fn drop(&mut self) {
        (self.api.param_raw().functions().unlock_lua)();
    }
}

impl<'a, 't, T> std::ops::Deref for RefAPILuaLock<'a, 't, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
