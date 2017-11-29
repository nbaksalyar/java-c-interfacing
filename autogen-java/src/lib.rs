#![allow(non_snake_case)]

extern crate jni;

extern crate ffi_utils;
extern crate safe_app;
extern crate safe_core;

// Extensions for the JNI crate.
mod jni_ext;
#[macro_use]
mod macros;

use ffi_utils::*;
use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::strings::JNIStr;
use jni::sys::{jint, jlong};
use jni_ext::{GlobalRefExt, JAVA_VM_INIT, JavaVM};
use safe_app as ffi;
use safe_app::*;
use safe_core::arrays::*;
use safe_core::ffi::*;
use safe_core::ffi::ipc::req::{AppExchangeInfo, AuthReq, ContainerPermissions, ContainersReq,
                               PermissionSet, ShareMDataReq};
use safe_core::ffi::ipc::resp::{AccessContInfo, AccessContainerEntry, AppKeys, AuthGranted,
                                MDataKey, MDataValue, MetadataResponse};
use safe_core::ffi::nfs::File;
use std::{ptr, slice};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};

static mut JVM: JavaVM = JAVA_VM_INIT;

// Trait for conversion of rust value to java value.
trait ToJava<'a, T: 'a> {
    fn to_java(&self, env: &'a JNIEnv) -> T;
}

// Trait for conversion of java value to rust value.
trait FromJava<T> {
    fn from_java(env: &JNIEnv, input: T) -> Self;
}


gen_primitive_type_converter!(i32, jint);
gen_primitive_type_converter!(u32, jint);
gen_primitive_type_converter!(u64, jlong);

gen_byte_array_converter!(i8, 8);
gen_byte_array_converter!(u8, 24);
gen_byte_array_converter!(u8, 32);
gen_byte_array_converter!(u8, 64);

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
        let len = env.get_array_length(input).unwrap() as usize;

        let mut output = Vec::new();
        output.resize(len, 0);

        unsafe {
            let slice = mem::transmute(output.as_mut_slice());
            env.get_byte_array_region(input, 0, slice).unwrap();
        }

        output
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

impl<'a> FromJava<JObject<'a>> for MDataInfo {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        MDataInfo {
            name: Default::default(),
            type_tag: 0,
            has_enc_info: false,
            enc_key: Default::default(),
            enc_nonce: Default::default(),
            has_new_enc_info: false,
            new_enc_key: Default::default(),
            new_enc_nonce: Default::default(),
        }
    }
}

impl<'a> ToJava<'a, JObject<'a>> for MDataInfo {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("MDataInfo", "()V", &[]).unwrap();
        output
    }
}

impl<'a> FromJava<JObject<'a>> for File {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        File {
            size: 0,
            created_sec: 0,
            created_nsec: 0,
            modified_sec: 0,
            modified_nsec: 0,
            user_metadata_ptr: ptr::null_mut(),
            user_metadata_len: 0,
            user_metadata_cap: 0,
            data_map_name: Default::default(),
        }
    }
}

impl<'a> ToJava<'a, JObject<'a>> for File {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("File", "()V", &[]).unwrap();
        output
    }
}

impl<'a> FromJava<JObject<'a>> for PermissionSet {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        Default::default()
    }
}

impl<'a> ToJava<'a, JObject<'a>> for PermissionSet {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("PermissionSet", "()V", &[]).unwrap();
        output
    }
}

impl<'a> ToJava<'a, JObject<'a>> for AccountInfo {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("AccountInfo", "()V", &[]).unwrap();
        output
    }
}

impl<'a> ToJava<'a, JObject<'a>> for AuthGranted {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("AuthGranted", "()V", &[]).unwrap();
        output
    }
}

impl<'a> FromJava<JObject<'a>> for AuthGranted {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        AuthGranted {
            app_keys: AppKeys {
                owner_key: Default::default(),
                enc_key: Default::default(),
                sign_pk: Default::default(),
                sign_sk: [0; 64],
                enc_pk: Default::default(),
                enc_sk: [0; 32],
            },
            access_container_info: AccessContInfo {
                id: Default::default(),
                tag: 0,
                nonce: Default::default(),
            },
            access_container_entry: AccessContainerEntry {
                ptr: ptr::null(),
                len: 0,
                cap: 0,
            },
            bootstrap_config_ptr: ptr::null_mut(),
            bootstrap_config_len: 0,
            bootstrap_config_cap: 0,
        }
    }
}

impl<'a> FromJava<JObject<'a>> for AuthReq {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        AuthReq {
            app: AppExchangeInfo {
                id: ptr::null(),
                scope: ptr::null(),
                name: ptr::null(),
                vendor: ptr::null(),
            },
            app_container: false,
            containers: ptr::null(),
            containers_len: 0,
            containers_cap: 0,
        }
    }
}

impl<'a> FromJava<JObject<'a>> for ShareMDataReq {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        ShareMDataReq {
            app: AppExchangeInfo {
                id: ptr::null(),
                scope: ptr::null(),
                name: ptr::null(),
                vendor: ptr::null(),
            },
            mdata: ptr::null(),
            mdata_len: 0,
            mdata_cap: 0,
        }
    }
}

impl<'a> FromJava<JObject<'a>> for ContainersReq {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        ContainersReq {
            app: AppExchangeInfo {
                id: ptr::null(),
                scope: ptr::null(),
                name: ptr::null(),
                vendor: ptr::null(),
            },
            containers: ptr::null(),
            containers_len: 0,
            containers_cap: 0,
        }
    }
}

impl<'a> FromJava<JObject<'a>> for MetadataResponse {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        MetadataResponse {
            name: ptr::null(),
            description: ptr::null(),
            xor_name: Default::default(),
            type_tag: 0,
        }
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
