#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::{ffi::{c_char, c_void}, mem::MaybeUninit};

use super::api::{
    REFrameworkFieldHandle, REFrameworkMethodHandle, REFrameworkPluginInitializeParam, REFrameworkPropertyHandle,
    REFrameworkSDKData, REFrameworkTDB, REFrameworkTDBHandle, REFrameworkTDBTypeDefinition,
    REFrameworkTypeDefinitionHandle, REFrameworkVMObjType,
};

#[repr(C, packed)]
pub union InvokeRetUnion {
    bytes: [u8; 128],
    byte: u8,
    word: u16,
    dword: u32,
    f: f32,
    qword: u64,
    d: f64,
    ptr: *mut std::ffi::c_void,
}

#[repr(C, packed)]
pub struct InvokeRet {
    union_data: InvokeRetUnion,
    exception_thrown: bool,
}

impl Default for InvokeRet {
    fn default() -> Self {
        InvokeRet {
            union_data: unsafe { MaybeUninit::zeroed().assume_init() },
            exception_thrown: false,
        }
    }
}

pub struct API {
    m_param: *const REFrameworkPluginInitializeParam,
    m_sdk: *const REFrameworkSDKData,
    m_lua_mtx: std::sync::Mutex<()>,
}

impl API {
    pub unsafe fn new(param: *const REFrameworkPluginInitializeParam) -> Option<Self> {
        if param.is_null() {
            return None;
        }

        let sdk = (*param).sdk;
        Some(API {
            m_param: param,
            m_sdk: sdk,
            m_lua_mtx: std::sync::Mutex::new(()),
        })
    }

    #[inline]
    pub fn param(&self) -> *const REFrameworkPluginInitializeParam {
        self.m_param
    }

    #[inline]
    pub fn sdk(&self) -> *const REFrameworkSDKData {
        self.m_sdk
    }

    #[inline]
    pub unsafe fn tdb(&self) -> TDB {
        let get_tdb_fn = (*(*self.sdk()).functions).get_tdb;
        let handle = get_tdb_fn();
        TDB::new(handle)
    }
}

pub struct TDB {
    handle: REFrameworkTDBHandle,
}

impl TDB {
    pub fn new(handle: REFrameworkTDBHandle) -> TDB {
        TDB { handle }
    }

    #[inline]
    fn get_ptr(&self) -> *const REFrameworkTDB {
        unsafe { std::mem::transmute(self.handle) }
    }

    pub unsafe fn get_num_types(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_types;
        func(self.handle)
    }

    pub unsafe fn get_num_methods(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_methods;
        func(self.handle)
    }

    pub unsafe fn get_num_fields(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_fields;
        func(self.handle)
    }

    pub unsafe fn get_num_properties(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_properties;
        func(self.handle)
    }

    pub unsafe fn get_strings_size(&self) -> u32 {
        let func = (*self.get_ptr()).get_strings_size;
        func(self.handle)
    }

    pub unsafe fn get_raw_data_size(&self) -> u32 {
        let func = (*self.get_ptr()).get_raw_data_size;
        func(self.handle)
    }

    pub unsafe fn get_string_database(&self) -> *const c_char {
        let func = (*self.get_ptr()).get_string_database;
        func(self.handle)
    }

    pub unsafe fn get_raw_database(&self) -> *mut u8 {
        let func = (*self.get_ptr()).get_raw_database;
        func(self.handle)
    }

    pub unsafe fn get_type(&self, index: u32) -> REFrameworkTypeDefinitionHandle {
        let func = (*self.get_ptr()).get_type;
        func(self.handle, index)
    }

    pub unsafe fn find_type(&self, name: *const c_char) -> REFrameworkTypeDefinitionHandle {
        let func = (*self.get_ptr()).find_type;
        func(self.handle, name)
    }

    pub unsafe fn find_type_by_fqn(&self, fqn: u32) -> REFrameworkTypeDefinitionHandle {
        let func = (*self.get_ptr()).find_type_by_fqn;
        func(self.handle, fqn)
    }

    pub unsafe fn get_method(&self, index: u32) -> REFrameworkMethodHandle {
        let func = (*self.get_ptr()).get_method;
        func(self.handle, index)
    }

    pub unsafe fn find_method(&self, type_name: *const c_char, name: *const c_char) -> REFrameworkMethodHandle {
        let func = (*self.get_ptr()).find_method;
        func(self.handle, type_name, name)
    }

    pub unsafe fn get_field(&self, index: u32) -> REFrameworkFieldHandle {
        let func = (*self.get_ptr()).get_field;
        func(self.handle, index)
    }

    pub unsafe fn find_field(&self, type_name: *const c_char, name: *const c_char) -> REFrameworkFieldHandle {
        let func = (*self.get_ptr()).find_field;
        func(self.handle, type_name, name)
    }

    pub unsafe fn get_property(&self, index: u32) -> REFrameworkPropertyHandle {
        let func = (*self.get_ptr()).get_property;
        func(self.handle, index)
    }
}

pub struct TypeDefinition {
    handle: REFrameworkTypeDefinitionHandle,
}

impl TypeDefinition {
    pub fn new(handle: REFrameworkTypeDefinitionHandle) -> TypeDefinition {
        TypeDefinition { handle }
    }

    #[inline]
    fn get_ptr(&self) -> *const REFrameworkTDBTypeDefinition {
        unsafe { std::mem::transmute(self.handle) }
    }

    pub unsafe fn get_index(&self) -> u32 {
        let func = (*self.get_ptr()).get_index;
        func(self.handle)
    }

    pub unsafe fn get_size(&self) -> u32 {
        let func = (*self.get_ptr()).get_size;
        func(self.handle)
    }

    pub unsafe fn get_valuetype_size(&self) -> u32 {
        let func = (*self.get_ptr()).get_valuetype_size;
        func(self.handle)
    }

    pub unsafe fn get_fqn(&self) -> u32 {
        let func = (*self.get_ptr()).get_fqn;
        func(self.handle)
    }

    pub unsafe fn get_name(&self) -> *const c_char {
        let func = (*self.get_ptr()).get_name;
        func(self.handle)
    }

    pub unsafe fn get_namespace(&self) -> *const c_char {
        let func = (*self.get_ptr()).get_namespace;
        func(self.handle)
    }

    pub unsafe fn get_full_name(&self, out: *mut c_char, out_size: u32, out_len: *mut u32) -> String {
        todo!();
        // let func = (*self.get_ptr()).get_full_name;
        // func(self.handle, out, out_size, out_len)
    }

    pub unsafe fn has_fieldptr_offset(&self) -> bool {
        let func = (*self.get_ptr()).has_fieldptr_offset;
        func(self.handle)
    }

    pub unsafe fn get_fieldptr_offset(&self) -> i32 {
        let func = (*self.get_ptr()).get_fieldptr_offset;
        func(self.handle)
    }

    pub unsafe fn get_num_methods(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_methods;
        func(self.handle)
    }

    pub unsafe fn get_num_fields(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_fields;
        func(self.handle)
    }

    pub unsafe fn get_num_properties(&self) -> u32 {
        let func = (*self.get_ptr()).get_num_properties;
        func(self.handle)
    }

    pub unsafe fn is_derived_from(&self, other: REFrameworkTypeDefinitionHandle) -> bool {
        let func = (*self.get_ptr()).is_derived_from;
        func(self.handle, other)
    }

    pub unsafe fn is_derived_from_by_name(&self, name: *const c_char) -> bool {
        let func = (*self.get_ptr()).is_derived_from_by_name;
        func(self.handle, name)
    }

    pub unsafe fn is_valuetype(&self) -> bool {
        let func = (*self.get_ptr()).is_valuetype;
        func(self.handle)
    }

    pub unsafe fn is_enum(&self) -> bool {
        let func = (*self.get_ptr()).is_enum;
        func(self.handle)
    }

    pub unsafe fn is_by_ref(&self) -> bool {
        let func = (*self.get_ptr()).is_by_ref;
        func(self.handle)
    }

    pub unsafe fn is_pointer(&self) -> bool {
        let func = (*self.get_ptr()).is_pointer;
        func(self.handle)
    }

    pub unsafe fn is_primitive(&self) -> bool {
        let func = (*self.get_ptr()).is_primitive;
        func(self.handle)
    }

    pub unsafe fn get_vm_obj_type(&self) -> REFrameworkVMObjType {
        let func = (*self.get_ptr()).get_vm_obj_type;
        func(self.handle)
    }

    pub unsafe fn find_method(&self, name: *const c_char) -> REFrameworkMethodHandle {
        let func = (*self.get_ptr()).find_method;
        func(self.handle, name)
    }

    pub unsafe fn find_field(&self, name: *const c_char) -> REFrameworkFieldHandle {
        let func = (*self.get_ptr()).find_field;
        func(self.handle, name)
    }

    pub unsafe fn find_property(&self, name: *const c_char) -> REFrameworkPropertyHandle {
        let func = (*self.get_ptr()).find_property;
        func(self.handle, name)
    }

    // pub unsafe fn get_methods(
    //     &self,
    //     out: *mut REFrameworkMethodHandle,
    //     out_size: u32,
    //     out_count: *mut u32,
    // ) -> Vec<Method> {
    //     let func = (*self.get_ptr()).get_methods;
    //     func(self.handle, out, out_size, out_count)
    // }

    // pub unsafe fn get_fields(
    //     &self,
    //     out: *mut REFrameworkFieldHandle,
    //     out_size: u32,
    //     out_count: *mut u32,
    // ) -> Vec<Field> {
    //     let func = (*self.get_ptr()).get_fields;
    //     func(self.handle, out, out_size, out_count)
    // }

    // pub unsafe fn get_instance(&self) -> *mut c_void {
    //     let func = (*self.get_ptr()).get_instance;
    //     func(self.handle)
    // }

    // pub unsafe fn create_instance_deprecated(&self) -> *mut c_void {
    //     let func = (*self.get_ptr()).create_instance_deprecated;
    //     func(self.handle)
    // }

    // pub unsafe fn create_instance(&self, flags: u32) -> REFrameworkManagedObjectHandle {
    //     let func = (*self.get_ptr()).create_instance;
    //     func(self.handle, flags)
    // }

    // pub unsafe fn get_parent_type(&self) -> REFrameworkTypeDefinitionHandle {
    //     let func = (*self.get_ptr()).get_parent_type;
    //     func(self.handle)
    // }

    // pub unsafe fn get_declaring_type(&self) -> REFrameworkTypeDefinitionHandle {
    //     let func = (*self.get_ptr()).get_declaring_type;
    //     func(self.handle)
    // }

    // pub unsafe fn get_underlying_type(&self) -> REFrameworkTypeDefinitionHandle {
    //     let func = (*self.get_ptr()).get_underlying_type;
    //     func(self.handle)
    // }

    // pub unsafe fn get_type_info(&self) -> REFrameworkTypeInfoHandle {
    //     let func = (*self.get_ptr()).get_type_info;
    //     func(self.handle)
    // }

    // pub unsafe fn get_runtime_type(&self) -> REFrameworkManagedObjectHandle {
    //     let func = (*self.get_ptr()).get_runtime_type;
    //     func(self.handle)
    // }
}

// pub mod member {

// }
