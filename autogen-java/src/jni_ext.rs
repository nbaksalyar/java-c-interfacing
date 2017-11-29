use jni;
use jni::JNIEnv;
use jni::objects::GlobalRef;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub trait GlobalRefExt {
    unsafe fn from_raw_ptr(env: &JNIEnv, ptr: *mut c_void) -> Self;
    fn into_raw_ptr(self) -> *mut c_void;
}

impl GlobalRefExt for GlobalRef {
    unsafe fn from_raw_ptr(env: &JNIEnv, ptr: *mut c_void) -> Self {
        Self::new(env.get_native_interface(), ptr as jni::sys::jobject)
    }

    fn into_raw_ptr(self) -> *mut c_void {
        let ptr = self.as_obj().into_inner() as *mut c_void;
        // Prevent the destructor from releasing the global reference.
        mem::forget(self);
        ptr
    }
}

pub struct JavaVM(*mut jni::sys::JavaVM);

unsafe impl Send for JavaVM {}

impl JavaVM {
    pub fn from_raw(ptr: *mut jni::sys::JavaVM) -> Self {
        JavaVM(ptr)
    }

    // TODO: better error handling
    pub unsafe fn attach_current_thread_as_daemon(&self) -> jni::errors::Result<JNIEnv> {
        let mut env_ptr = ptr::null_mut();
        let fn_ptr = (**self.0).AttachCurrentThreadAsDaemon.unwrap();

        fn_ptr(self.0, &mut env_ptr, ptr::null_mut());

        JNIEnv::from_raw(env_ptr as *mut jni::sys::JNIEnv)
    }
}

pub const JAVA_VM_INIT: JavaVM = JavaVM(0 as *mut _);
