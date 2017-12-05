#![allow(non_snake_case)]

extern crate jni;

extern crate ffi_utils;
extern crate safe_app;
extern crate safe_core;

// Extensions for the JNI crate.
#[macro_use]
mod macros;

use ffi_utils::*;
use jni::{JNIEnv, JavaVM};
use jni::objects::{DetachedGlobalRef, GlobalRef, JClass, JObject, JString};
use jni::strings::JNIStr;
use jni::sys::{jbyte, jint, jlong};
use safe_app as ffi;
use safe_app::*;
use safe_core::arrays::*;
use safe_core::ffi::*;
use safe_core::ffi::ipc::req::{AppExchangeInfo, AuthReq, ContainerPermissions, ContainersReq,
                               PermissionSet, ShareMData, ShareMDataReq};
use safe_core::ffi::ipc::resp::{AccessContInfo, AccessContainerEntry, AppAccess, AppKeys,
                                AuthGranted, ContainerInfo, MDataKey, MDataValue, MetadataResponse};
use safe_core::ffi::nfs::File;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::slice;

/// Converts `user_data` back into a Java callback object
unsafe fn convert_cb_from_java(env: &JNIEnv, ctx: *mut c_void) -> GlobalRef {
    DetachedGlobalRef::new(env.get_java_vm().unwrap(), ctx as jni::sys::jobject).attach(env)
}

static mut JVM: Option<JavaVM> = None;

#[no_mangle]
// This is called when `loadLibrary` is called on the Java side.
pub unsafe extern "C" fn JNI_OnLoad(
    vm: *mut jni::sys::JavaVM,
    _reserved: *mut c_void,
) -> jni::sys::jint {
    JVM = Some(JavaVM::from_raw(vm).unwrap());
    jni::sys::JNI_VERSION_1_4
}

// Trait for conversion of rust value to java value.
trait ToJava<'a, T: 'a> {
    fn to_java(&self, env: &'a JNIEnv) -> T;
}

// Trait for conversion of java value to rust value.
trait FromJava<T> {
    fn from_java(env: &JNIEnv, input: T) -> Self;
}

gen_primitive_type_converter!(u8, jbyte);
gen_primitive_type_converter!(i32, jint);
gen_primitive_type_converter!(u32, jint);
gen_primitive_type_converter!(i64, jlong);
gen_primitive_type_converter!(u64, jlong);

gen_byte_array_converter!(i8, 8);
gen_byte_array_converter!(u8, 24);
gen_byte_array_converter!(u8, 32);
gen_byte_array_converter!(u8, 64);

impl<'a> ToJava<'a, bool> for bool {
    fn to_java(&self, _env: &JNIEnv) -> bool {
        *self
    }
}

impl<'a> ToJava<'a, jlong> for usize {
    fn to_java(&self, _env: &JNIEnv) -> jlong {
        *self as jlong
    }
}

impl<'a> FromJava<JString<'a>> for *const c_char {
    fn from_java(env: &JNIEnv, input: JString) -> Self {
        CString::from_java(env, input).into_raw()
    }
}

impl<'a> ToJava<'a, JString<'a>> for *const c_char {
    fn to_java(&self, env: &'a JNIEnv) -> JString<'a> {
        unsafe { env.new_string(JNIStr::from_ptr(*self).to_owned()).unwrap() }
    }
}

impl<'a> FromJava<JString<'a>> for *mut c_char {
    fn from_java(env: &JNIEnv, input: JString) -> Self {
        <*const _>::from_java(env, input) as *mut _
    }
}

impl<'a> ToJava<'a, JString<'a>> for *mut c_char {
    fn to_java(&self, env: &'a JNIEnv) -> JString<'a> {
        (*self as *const _).to_java(env)
    }
}


impl<'a> FromJava<JString<'a>> for CString {
    fn from_java(env: &JNIEnv, input: JString) -> Self {
        let tmp: &CStr = &*env.get_string(input).unwrap();
        tmp.to_owned()
    }
}

// TODO: implement this for all primitive types (consider defining a `PrimitiveType`
// trait and implement it for all rust types that correspond to primitive java types)
impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [i32] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_int_array(self.len() as jni::sys::jsize).unwrap();
        env.set_int_array_region(output, 0, self).unwrap();
        JObject::from(output as jni::sys::jobject)
    }
}

impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [u8] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_byte_array(self.len() as jni::sys::jsize).unwrap();
        env.set_byte_array_region(output, 0, unsafe {
            slice::from_raw_parts(self.as_ptr() as *const i8, self.len())
        }).unwrap();
        JObject::from(output as jni::sys::jobject)
    }
}

impl<'a> FromJava<JObject<'a>> for Vec<u8> {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        let input = input.into_inner() as jni::sys::jbyteArray;
        env.convert_byte_array(input).unwrap()
    }
}

impl<'a> ToJava<'a, JObject<'a>> for FfiResult {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("FfiResult", "()V", &[]).unwrap();

        env.set_field(
            output,
            "errorCode",
            "I",
            self.error_code.to_java(env).into(),
        ).unwrap();

        let error: JObject = self.description.to_java(&env).into();
        env.set_field(output, "error", "Ljava/lang/String;", error.into())
            .unwrap();

        output
    }
}

impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [MDataKey] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output =
            env.new_object_array(self.len() as jni::sys::jsize, "MDataKey", JObject::null())
                .unwrap();

        JObject::from(output as jni::sys::jobject)
    }
}

impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [MDataValue] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output =
            env.new_object_array(self.len() as jni::sys::jsize, "MDataValue", JObject::null())
                .unwrap();

        JObject::from(output as jni::sys::jobject)
    }
}

impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [UserPermissionSet] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object_array(
            self.len() as jni::sys::jsize,
            "UserPermissionSet",
            JObject::null(),
        ).unwrap();

        JObject::from(output as jni::sys::jobject)
    }
}

impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [ContainerPermissions] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object_array(
            self.len() as jni::sys::jsize,
            "ContainerPermissions",
            JObject::null(),
        ).unwrap();

        JObject::from(output as jni::sys::jobject)
    }
}

include!("jni.rs");
