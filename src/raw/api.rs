#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::{c_char, c_void};
pub type wchar_t = i16;

use macros::declare_reframework_handle;

pub const REFRAMEWORK_PLUGIN_VERSION_MAJOR: i32 = 1;
pub const REFRAMEWORK_PLUGIN_VERSION_MINOR: i32 = 10;
pub const REFRAMEWORK_PLUGIN_VERSION_PATCH: i32 = 0;

pub const REFRAMEWORK_RENDERER_D3D11: i32 = 0;
pub const REFRAMEWORK_RENDERER_D3D12: i32 = 1;

pub const REFRAMEWORK_ERROR_UNKNOWN: i32 = -1;
pub const REFRAMEWORK_ERROR_NONE: i32 = 0;
pub const REFRAMEWORK_ERROR_OUT_TOO_SMALL: i32 = 1;
pub const REFRAMEWORK_ERROR_EXCEPTION: i32 = 2;
pub const REFRAMEWORK_ERROR_IN_ARGS_SIZE_MISMATCH: i32 = 3;

pub const REFRAMEWORK_HOOK_CALL_ORIGINAL: i32 = 0;
pub const REFRAMEWORK_HOOK_SKIP_ORIGINAL: i32 = 1;

pub type REFrameworkResult = i32;

pub type lua_State = *mut c_void;

pub type REFInitializedCb = unsafe extern "C" fn();
pub type REFLuaStateCreatedCb = unsafe extern "C" fn(*mut lua_State);
pub type REFLuaStateDestroyedCb = unsafe extern "C" fn(*mut lua_State);
pub type REFOnPresentCb = unsafe extern "C" fn();
pub type REFOnPreApplicationEntryCb = unsafe extern "C" fn();
pub type REFOnPostApplicationEntryCb = unsafe extern "C" fn();
pub type REFOnDeviceResetCb = unsafe extern "C" fn();
pub type REFOnMessageCb = unsafe extern "C" fn(*mut c_void, u32, u64, i64) -> bool;

#[repr(C)]
pub struct REFImGuiFrameCbData {
    pub context: *mut c_void,
    pub malloc_fn: *mut c_void,
    pub free_fn: *mut c_void,
    pub user_data: *mut c_void,
}

pub type REFOnImGuiFrameCb = unsafe extern "C" fn(*mut REFImGuiFrameCbData);
pub type REFOnImGuiDrawUICb = unsafe extern "C" fn(*mut REFImGuiFrameCbData);

pub type REFCreateScriptState = unsafe extern "C" fn() -> *mut lua_State;
pub type REFDeleteScriptState = unsafe extern "C" fn(*mut lua_State);

pub type REFOnInitializeFn = unsafe extern "C" fn(REFInitializedCb) -> bool;
pub type REFOnLuaStateCreatedFn = unsafe extern "C" fn(REFLuaStateCreatedCb) -> bool;
pub type REFOnLuaStateDestroyedFn = unsafe extern "C" fn(REFLuaStateDestroyedCb) -> bool;
pub type REFOnPresentFn = unsafe extern "C" fn(REFOnPresentCb) -> bool;
pub type REFOnPreApplicationEntryFn = unsafe extern "C" fn(*const c_char, REFOnPreApplicationEntryCb) -> bool;
pub type REFOnPostApplicationEntryFn = unsafe extern "C" fn(*const c_char, REFOnPostApplicationEntryCb) -> bool;
pub type REFLuaLockUnlockFn = unsafe extern "C" fn();
pub type REFOnDeviceResetFn = unsafe extern "C" fn(REFOnDeviceResetCb) -> bool;
pub type REFOnMessageFn = unsafe extern "C" fn(REFOnMessageCb) -> bool;

pub type REFOnImGuiFrameFn = unsafe extern "C" fn(REFOnImGuiFrameCb);
pub type REFOnImGuiDrawUIFn = unsafe extern "C" fn(REFOnImGuiDrawUICb);

#[repr(C)]
pub struct REFrameworkPluginVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub game_name: *const c_char,
}

pub type REFPluginRequiredVersionFn = extern "C" fn(*mut REFrameworkPluginVersion);

#[repr(C)]
pub struct REFrameworkPluginFunctions {
    pub on_lua_state_created: REFOnLuaStateCreatedFn,
    pub on_lua_state_destroyed: REFOnLuaStateDestroyedFn,
    pub on_present: REFOnPresentFn,
    pub on_pre_application_entry: REFOnPreApplicationEntryFn,
    pub on_post_application_entry: REFOnPostApplicationEntryFn,
    pub lock_lua: REFLuaLockUnlockFn,
    pub unlock_lua: REFLuaLockUnlockFn,
    pub on_device_reset: REFOnDeviceResetFn,
    pub on_message: REFOnMessageFn,
    pub log_error: extern "C" fn(*const c_char, ...) -> (),
    pub log_warn: extern "C" fn(*const c_char, ...) -> (),
    pub log_info: extern "C" fn(*const c_char, ...) -> (),
    pub is_drawing_ui: extern "C" fn() -> bool,
    pub create_script_state: REFCreateScriptState,
    pub delete_script_state: REFDeleteScriptState,
    pub on_imgui_frame: REFOnImGuiFrameFn,
    pub on_imgui_draw_ui: REFOnImGuiDrawUIFn,
}

#[repr(C)]
pub struct REFrameworkRendererData {
    pub renderer_type: i32,
    pub device: *mut c_void,
    pub swapchain: *mut c_void,
    pub command_queue: *mut c_void,
}

declare_reframework_handle!(REFrameworkTypeDefinitionHandle);
declare_reframework_handle!(REFrameworkMethodHandle);
declare_reframework_handle!(REFrameworkFieldHandle);
declare_reframework_handle!(REFrameworkPropertyHandle);
declare_reframework_handle!(REFrameworkManagedObjectHandle);
declare_reframework_handle!(REFrameworkTDBHandle);
declare_reframework_handle!(REFrameworkHandle);
declare_reframework_handle!(REFrameworkResourceHandle);
declare_reframework_handle!(REFrameworkResourceManagerHandle);
declare_reframework_handle!(REFrameworkVMContextHandle);
declare_reframework_handle!(REFrameworkTypeInfoHandle);
declare_reframework_handle!(REFrameworkReflectionPropertyHandle);
declare_reframework_handle!(REFrameworkReflectionMethodHandle);

pub const REFRAMEWORK_CREATE_INSTANCE_FLAGS_NONE: i32 = 0;
pub const REFRAMEWORK_CREATE_INSTANCE_FLAGS_SIMPLIFY: i32 = 1;

pub const REFRAMEWORK_VM_OBJ_TYPE_NULL: u32 = 0;
pub const REFRAMEWORK_VM_OBJ_TYPE_OBJECT: u32 = 1;
pub const REFRAMEWORK_VM_OBJ_TYPE_ARRAY: u32 = 2;
pub const REFRAMEWORK_VM_OBJ_TYPE_STRING: u32 = 3;
pub const REFRAMEWORK_VM_OBJ_TYPE_DELEGATE: u32 = 4;
pub const REFRAMEWORK_VM_OBJ_TYPE_VALTYPE: u32 = 5;

pub type REFrameworkVMObjType = u32;

pub type REFrameworkInvokeMethod = extern "C" fn(*mut c_void, *mut c_void);
pub type REFrameworkReflectionPropertyMethod = extern "C" fn(
    prop: REFrameworkReflectionPropertyHandle,
    thisptr: REFrameworkManagedObjectHandle,
    out_data: *mut c_void,
) -> *mut c_void;

#[repr(C)]
pub struct REFrameworkTDBTypeDefinition {
    pub get_index: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,
    pub get_size: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,
    pub get_valuetype_size: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,
    pub get_fqn: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,

    pub get_name: extern "C" fn(REFrameworkTypeDefinitionHandle) -> *const c_char,
    pub get_namespace: extern "C" fn(REFrameworkTypeDefinitionHandle) -> *const c_char,
    pub get_full_name: extern "C" fn(
        REFrameworkTypeDefinitionHandle,
        out: *mut c_char,
        out_size: u32,
        out_len: *mut u32,
    ) -> REFrameworkResult,

    pub has_fieldptr_offset: extern "C" fn(REFrameworkTypeDefinitionHandle) -> bool,
    pub get_fieldptr_offset: extern "C" fn(REFrameworkTypeDefinitionHandle) -> i32,

    pub get_num_methods: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,
    pub get_num_fields: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,
    pub get_num_properties: extern "C" fn(REFrameworkTypeDefinitionHandle) -> u32,

    pub is_derived_from: extern "C" fn(REFrameworkTypeDefinitionHandle, REFrameworkTypeDefinitionHandle) -> bool,
    pub is_derived_from_by_name: extern "C" fn(REFrameworkTypeDefinitionHandle, *const c_char) -> bool,
    pub is_valuetype: extern "C" fn(REFrameworkTypeDefinitionHandle) -> bool,
    pub is_enum: extern "C" fn(REFrameworkTypeDefinitionHandle) -> bool,
    pub is_by_ref: extern "C" fn(REFrameworkTypeDefinitionHandle) -> bool,
    pub is_pointer: extern "C" fn(REFrameworkTypeDefinitionHandle) -> bool,
    pub is_primitive: extern "C" fn(REFrameworkTypeDefinitionHandle) -> bool,

    pub get_vm_obj_type: extern "C" fn(REFrameworkTypeDefinitionHandle) -> REFrameworkVMObjType,

    /* All lookups are cached on our end */
    /* however, the pointers will always stay the same, */
    /* so you can cache them on your end e.g. with a static var to get a minor speed increase. */
    pub find_method: extern "C" fn(REFrameworkTypeDefinitionHandle, *const c_char) -> REFrameworkMethodHandle,
    pub find_field: extern "C" fn(REFrameworkTypeDefinitionHandle, *const c_char) -> REFrameworkFieldHandle,
    pub find_property: extern "C" fn(REFrameworkTypeDefinitionHandle, *const c_char) -> REFrameworkPropertyHandle,

    /* out_size is the full size, in bytes of the out buffer */
    /* out_len is how many elements were written to the out buffer, not the size of the written data */
    pub get_methods: extern "C" fn(
        REFrameworkTypeDefinitionHandle,
        out: *mut REFrameworkMethodHandle,
        out_size: u32,
        out_count: *mut u32,
    ) -> REFrameworkResult,
    pub get_fields: extern "C" fn(
        REFrameworkTypeDefinitionHandle,
        out: *mut REFrameworkFieldHandle,
        out_size: u32,
        out_count: *mut u32,
    ) -> REFrameworkResult,

    /* get_instance usually only used for native singletons */
    pub get_instance: extern "C" fn(REFrameworkTypeDefinitionHandle) -> *mut c_void,
    pub create_instance_deprecated: extern "C" fn(REFrameworkTypeDefinitionHandle) -> *mut c_void,
    pub create_instance: extern "C" fn(REFrameworkTypeDefinitionHandle, flags: u32) -> REFrameworkManagedObjectHandle,

    pub get_parent_type: extern "C" fn(REFrameworkTypeDefinitionHandle) -> REFrameworkTypeDefinitionHandle,
    pub get_declaring_type: extern "C" fn(REFrameworkTypeDefinitionHandle) -> REFrameworkTypeDefinitionHandle,
    pub get_underlying_type: extern "C" fn(REFrameworkTypeDefinitionHandle) -> REFrameworkTypeDefinitionHandle,

    pub get_type_info: extern "C" fn(REFrameworkTypeDefinitionHandle) -> REFrameworkTypeInfoHandle,
    pub get_runtime_type: extern "C" fn(REFrameworkTypeDefinitionHandle) -> REFrameworkManagedObjectHandle,
}

#[repr(C)]
pub struct REFrameworkMethodParameter {
    pub name: *const c_char,
    pub t: REFrameworkTypeDefinitionHandle,
    pub reserved: u64,
}

#[repr(C)]
pub struct REFrameworkTDBMethod {
    /* make sure out size is at least size of InvokeRet */
    /* each arg is always 8 bytes, even if it's something like a byte */
    pub invoke: extern "C" fn(
        REFrameworkMethodHandle,
        thisptr: *mut c_void,
        in_args: *mut *mut c_void,
        in_args_size: u32,
        out: *mut c_void,
        out_size: u32,
    ) -> REFrameworkResult,
    pub get_function: extern "C" fn(REFrameworkMethodHandle) -> *mut c_void,
    pub get_name: extern "C" fn(REFrameworkMethodHandle) -> *const c_char,
    pub get_declaring_type: extern "C" fn(REFrameworkMethodHandle) -> REFrameworkTypeDefinitionHandle,
    pub get_return_type: extern "C" fn(REFrameworkMethodHandle) -> REFrameworkTypeDefinitionHandle,

    pub get_num_params: extern "C" fn(REFrameworkMethodHandle) -> u32,

    /* out_size is the full size, in bytes of the out buffer */
    /* out_count is how many elements were written to the out buffer, not the size of the written data */
    pub get_params: extern "C" fn(
        REFrameworkMethodHandle,
        out: *mut REFrameworkMethodParameter,
        out_size: u32,
        out_len: *mut u32,
    ) -> REFrameworkResult,

    pub get_index: extern "C" fn(REFrameworkMethodHandle) -> u32,
    pub get_virtual_index: extern "C" fn(REFrameworkMethodHandle) -> i32,
    pub is_static: extern "C" fn(REFrameworkMethodHandle) -> bool,
    pub get_flags: extern "C" fn(REFrameworkMethodHandle) -> u16,
    pub get_impl_flags: extern "C" fn(REFrameworkMethodHandle) -> u16,
    pub get_invoke_id: extern "C" fn(REFrameworkMethodHandle) -> u32,
}

#[repr(C)]
pub struct REFrameworkTDBField {
    pub get_name: extern "C" fn(REFrameworkFieldHandle) -> *const c_char,

    pub get_declaring_type: extern "C" fn(REFrameworkFieldHandle) -> REFrameworkTypeDefinitionHandle,
    pub get_type: extern "C" fn(REFrameworkFieldHandle) -> REFrameworkTypeDefinitionHandle,

    pub get_offset_from_base: extern "C" fn(REFrameworkFieldHandle) -> u32,
    pub get_offset_from_fieldptr: extern "C" fn(REFrameworkFieldHandle) -> u32,
    pub get_flags: extern "C" fn(REFrameworkFieldHandle) -> u32,

    pub is_static: extern "C" fn(REFrameworkFieldHandle) -> bool,
    pub is_literal: extern "C" fn(REFrameworkFieldHandle) -> bool,

    pub get_init_data: extern "C" fn(REFrameworkFieldHandle) -> *mut c_void,
    pub get_data_raw: extern "C" fn(REFrameworkFieldHandle, obj: *mut c_void, is_value_type: bool) -> *mut c_void,

    pub get_index: extern "C" fn(REFrameworkFieldHandle) -> u32,
}

#[repr(C)]
pub struct REFrameworkTDBProperty;

#[repr(C)]
pub struct REFrameworkTDB {
    pub get_num_types: extern "C" fn(REFrameworkTDBHandle) -> u32,
    pub get_num_methods: extern "C" fn(REFrameworkTDBHandle) -> u32,
    pub get_num_fields: extern "C" fn(REFrameworkTDBHandle) -> u32,
    pub get_num_properties: extern "C" fn(REFrameworkTDBHandle) -> u32,
    pub get_strings_size: extern "C" fn(REFrameworkTDBHandle) -> u32,
    pub get_raw_data_size: extern "C" fn(REFrameworkTDBHandle) -> u32,
    pub get_string_database: extern "C" fn(REFrameworkTDBHandle) -> *const c_char,
    pub get_raw_database: extern "C" fn(REFrameworkTDBHandle) -> *mut u8,

    pub get_type: extern "C" fn(REFrameworkTDBHandle, index: u32) -> REFrameworkTypeDefinitionHandle,
    pub find_type: extern "C" fn(REFrameworkTDBHandle, name: *const c_char) -> REFrameworkTypeDefinitionHandle,
    pub find_type_by_fqn: extern "C" fn(REFrameworkTDBHandle, fqn: u32) -> REFrameworkTypeDefinitionHandle,
    pub get_method: extern "C" fn(REFrameworkTDBHandle, index: u32) -> REFrameworkMethodHandle,
    pub find_method:
        extern "C" fn(REFrameworkTDBHandle, type_name: *const c_char, name: *const c_char) -> REFrameworkMethodHandle,
    pub get_field: extern "C" fn(REFrameworkTDBHandle, index: u32) -> REFrameworkFieldHandle,
    pub find_field:
        extern "C" fn(REFrameworkTDBHandle, type_name: *const c_char, name: *const c_char) -> REFrameworkFieldHandle,
    pub get_property: extern "C" fn(REFrameworkTDBHandle, index: u32) -> REFrameworkPropertyHandle,
}

#[repr(C)]
pub struct REFrameworkManagedObject {
    pub add_ref: extern "C" fn(REFrameworkManagedObjectHandle),
    pub release: extern "C" fn(REFrameworkManagedObjectHandle),
    pub get_type_definition: extern "C" fn(REFrameworkManagedObjectHandle) -> REFrameworkTypeDefinitionHandle,
    pub is_managed_object: extern "C" fn(*mut c_void) -> bool,
    pub get_ref_count: extern "C" fn(REFrameworkManagedObjectHandle) -> u32,
    pub get_size: extern "C" fn(REFrameworkManagedObjectHandle) -> u32,
    pub get_vm_obj_type: extern "C" fn(REFrameworkManagedObjectHandle) -> u32,
    pub get_type_info: extern "C" fn(REFrameworkManagedObjectHandle) -> REFrameworkTypeInfoHandle,
    pub get_reflection_properties: extern "C" fn(REFrameworkManagedObjectHandle) -> *mut c_void,
    pub get_reflection_property_descriptor:
        extern "C" fn(REFrameworkManagedObjectHandle, name: *const c_char) -> REFrameworkReflectionPropertyHandle,
    pub get_reflection_method_descriptor:
        extern "C" fn(REFrameworkManagedObjectHandle, name: *const c_char) -> REFrameworkReflectionMethodHandle,
}

#[repr(C)]
pub struct REFrameworkNativeSingleton {
    pub instance: *mut c_void,
    /// t is not guaranteed to be non-null so we pass the name along too
    pub t: REFrameworkTypeDefinitionHandle,
    pub type_info: REFrameworkTypeInfoHandle,
    pub name: *const c_char,
}

#[repr(C)]
pub struct REFrameworkManagedSingleton {
    pub instance: REFrameworkManagedObjectHandle,
    pub t: REFrameworkTypeDefinitionHandle,
    pub type_info: REFrameworkTypeInfoHandle,
}

#[repr(C)]
pub struct REFrameworkResourceManager {
    pub create_resource: extern "C" fn(
        REFrameworkResourceManagerHandle,
        type_name: *const c_char,
        name: *const c_char,
    ) -> REFrameworkResourceHandle,
    pub create_userdata: extern "C" fn(
        REFrameworkResourceManagerHandle,
        type_name: *const c_char,
        name: *const c_char,
    ) -> REFrameworkManagedObjectHandle,
}

#[repr(C)]
pub struct REFrameworkResource {
    pub add_ref: extern "C" fn(REFrameworkResourceHandle),
    pub release: extern "C" fn(REFrameworkResourceHandle),
    pub create_holder:
        extern "C" fn(REFrameworkResourceHandle, type_name: *const c_char) -> REFrameworkManagedObjectHandle,
}

/// NOT a type definition
#[repr(C)]
pub struct REFrameworkTypeInfo {
    pub get_name: extern "C" fn(REFrameworkTypeInfoHandle) -> *const c_char,
    pub get_type_definition: extern "C" fn(REFrameworkTypeInfoHandle) -> REFrameworkTypeDefinitionHandle,
    pub is_clr_type: extern "C" fn(REFrameworkTypeInfoHandle) -> bool,
    pub is_singleton: extern "C" fn(REFrameworkTypeInfoHandle) -> bool,
    pub get_singleton_instance: extern "C" fn(REFrameworkTypeInfoHandle) -> *mut c_void,
    pub create_instance: extern "C" fn(REFrameworkTypeInfoHandle) -> *mut c_void,
    pub get_reflection_properties: extern "C" fn(REFrameworkTypeInfoHandle) -> *mut c_void,
    pub get_reflection_property_descriptor:
        extern "C" fn(REFrameworkTypeInfoHandle, name: *const c_char) -> REFrameworkReflectionPropertyHandle,
    pub get_reflection_method_descriptor:
        extern "C" fn(REFrameworkTypeInfoHandle, name: *const c_char) -> REFrameworkReflectionMethodHandle,
    pub get_deserializer_fn: extern "C" fn(REFrameworkTypeInfoHandle) -> *mut c_void,
    pub get_parent: extern "C" fn(REFrameworkTypeInfoHandle) -> REFrameworkTypeInfoHandle,
    pub get_crc: extern "C" fn(REFrameworkTypeInfoHandle) -> u32,
}

#[repr(C)]
pub struct REFrameworkVMContext {
    pub has_exception: extern "C" fn(REFrameworkVMContextHandle) -> bool,
    pub unhandled_exception: extern "C" fn(REFrameworkVMContextHandle),
    pub local_frame_gc: extern "C" fn(REFrameworkVMContextHandle),
    pub cleanup_after_exception: extern "C" fn(REFrameworkVMContextHandle, old_reference_count: i32),
}

/// NOT a TDB method
#[repr(C)]
pub struct REFrameworkReflectionMethod {
    pub get_function: extern "C" fn(REFrameworkReflectionMethodHandle) -> REFrameworkInvokeMethod,
}

/// NOT a TDB property
#[repr(C)]
pub struct REFrameworkReflectionProperty {
    pub get_getter: extern "C" fn(REFrameworkReflectionPropertyHandle) -> REFrameworkReflectionPropertyMethod,
    pub is_static: extern "C" fn(REFrameworkReflectionPropertyHandle) -> bool,
    pub get_size: extern "C" fn(REFrameworkReflectionPropertyHandle) -> u32,
}

pub type REFPreHookFn = extern "C" fn(
    argc: i32,
    argv: *mut *mut c_void,
    arg_tys: *mut REFrameworkTypeDefinitionHandle,
    ret_addr: u64,
) -> i32;
pub type REFPostHookFn =
    extern "C" fn(ret_val: *mut *mut c_void, ret_ty: REFrameworkTypeDefinitionHandle, ret_addr: u64);

#[repr(C)]
pub struct REFrameworkSDKFunctions {
    pub get_tdb: extern "C" fn() -> REFrameworkTDBHandle,
    pub get_resource_manager: extern "C" fn() -> REFrameworkResourceManagerHandle,
    pub get_vm_context: extern "C" fn() -> REFrameworkVMContextHandle, // per-thread context

    pub typeof_: extern "C" fn(type_name: *const c_char) -> REFrameworkManagedObjectHandle, // System.Type
    pub get_managed_singleton: extern "C" fn(type_name: *const c_char) -> REFrameworkManagedObjectHandle,
    pub get_native_singleton: extern "C" fn(type_name: *const c_char) -> *mut c_void,

    /* out_size is the full size, in bytes of the out buffer */
    /* out_count is how many elements were written to the out buffer, not the size of the written data */
    pub get_managed_singletons:
        extern "C" fn(out: *mut REFrameworkManagedSingleton, out_size: u32, out_count: *mut u32) -> REFrameworkResult,
    pub get_native_singletons:
        extern "C" fn(out: *mut REFrameworkNativeSingleton, out_size: u32, out_count: *mut u32) -> REFrameworkResult,

    pub create_managed_string: extern "C" fn(str: *const wchar_t) -> REFrameworkManagedObjectHandle,
    pub create_managed_string_normal: extern "C" fn(str: *const c_char) -> REFrameworkManagedObjectHandle,

    pub add_hook: extern "C" fn(REFrameworkMethodHandle, REFPreHookFn, REFPostHookFn, ignore_jmp: bool) -> u32,
    pub remove_hook: extern "C" fn(REFrameworkMethodHandle, u32),

    pub allocate: extern "C" fn(size: u64) -> *mut c_void,
    pub deallocate: extern "C" fn(*mut c_void),
}

/// these are NOT pointers to the actual objects
/// they are interfaces with functions that take handles to the objects
/// the functions, however, can return the actual objects
#[repr(C)]
pub struct REFrameworkSDKData {
    functions: *const REFrameworkSDKFunctions,
    tdb: *const REFrameworkTDB,
    type_definition: *const REFrameworkTDBTypeDefinition,
    method: *const REFrameworkTDBMethod,
    field: *const REFrameworkTDBField,
    property: *const REFrameworkTDBProperty,
    managed_object: *const REFrameworkManagedObject,
    resource_manager: *const REFrameworkResourceManager,
    resource: *const REFrameworkResource,
    type_info: *const REFrameworkTypeInfo, // NOT a type definition
    vm_context: *const REFrameworkVMContext,
    reflection_method: *const REFrameworkReflectionMethod, // NOT a TDB method
    reflection_property: *const REFrameworkReflectionProperty, // NOT a TDB property
}

impl REFrameworkSDKData {
    pub fn functions(&self) -> &REFrameworkSDKFunctions {
        unsafe { &*self.functions }
    }

    pub fn tdb(&self) -> &REFrameworkTDB {
        unsafe { &*self.tdb }
    }

    pub fn type_definition(&self) -> &REFrameworkTDBTypeDefinition {
        unsafe { &*self.type_definition }
    }

    pub fn method(&self) -> &REFrameworkTDBMethod {
        unsafe { &*self.method }
    }

    pub fn field(&self) -> &REFrameworkTDBField {
        unsafe { &*self.field }
    }

    pub fn property(&self) -> &REFrameworkTDBProperty {
        unsafe { &*self.property }
    }

    pub fn managed_object(&self) -> &REFrameworkManagedObject {
        unsafe { &*self.managed_object }
    }

    pub fn resource_manager(&self) -> &REFrameworkResourceManager {
        unsafe { &*self.resource_manager }
    }

    pub fn resource(&self) -> &REFrameworkResource {
        unsafe { &*self.resource }
    }

    pub fn type_info(&self) -> &REFrameworkTypeInfo {
        unsafe { &*self.type_info }
    }

    pub fn vm_context(&self) -> &REFrameworkVMContext {
        unsafe { &*self.vm_context }
    }

    pub fn reflection_method(&self) -> &REFrameworkReflectionMethod {
        unsafe { &*self.reflection_method }
    }

    pub fn reflection_property(&self) -> &REFrameworkReflectionProperty {
        unsafe { &*self.reflection_property }
    }
}

#[repr(C)]
pub struct REFrameworkPluginInitializeParam {
    pub reframework_module: *mut c_void,
    pub version: *const REFrameworkPluginVersion,
    pub functions: *const REFrameworkPluginFunctions,
    pub renderer_data: *const REFrameworkRendererData,
    pub sdk: *const REFrameworkSDKData,
}

impl REFrameworkPluginInitializeParam {
    pub fn functions(&self) -> &REFrameworkPluginFunctions {
        unsafe { &*self.functions }
    }
}

pub type REFPluginInitializeFn = extern "C" fn(*const REFrameworkPluginInitializeParam) -> bool;
