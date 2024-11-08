use super::*;

pub struct RefAPIMethod<'a> {
    api: &'a RefAPI,
    inner: *const REFrameworkTDBMethod,
}

impl<'a> RefAPIMethod<'a> {
    pub fn new(api: &'a RefAPI, inner: *const REFrameworkTDBMethod) -> Self {
        Self { api, inner }
    }

    pub fn inner(&self) -> &REFrameworkTDBMethod {
        unsafe { &*self.inner }
    }

    pub fn into_inner(self) -> *const REFrameworkTDBMethod {
        self.inner
    }

    pub fn add_hook(&self, pre_fn: REFPreHookFn, post_fn: REFPostHookFn, ignore_jmp: bool) -> u32 {
        unsafe {
            let func = self.api.sdk().functions().add_hook;
            func(std::mem::transmute(self.inner), pre_fn, post_fn, ignore_jmp)
        }
    }
}

pub struct RefAPITDB<'a> {
    api: &'a RefAPI,
    inner: *const REFrameworkTDB,
}

impl<'a> RefAPITDB<'a> {
    pub fn find_type(&self, name: &str) -> Option<REFrameworkTypeDefinitionHandle> {
        todo!()
    }
}
