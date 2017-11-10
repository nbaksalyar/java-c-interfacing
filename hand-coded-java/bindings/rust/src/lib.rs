#![allow(non_snake_case)]

extern crate jni;

// Extensions for the JNI crate.
mod jni_ext;

use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::strings::JNIStr;
use jni_ext::{GlobalRefExt, JAVA_VM_INIT, JavaVM};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::slice;

mod backend {
    #![allow(non_upper_case_globals, non_camel_case_types, unused)]
    include!(concat!(env!("OUT_DIR"), "/backend.rs"));
}

// Trait for conversion of rust value to java value.
trait ToJava<'a, T: 'a> {
    fn to_java(&self, env: &'a JNIEnv) -> T;
}

// Trait for conversion of java value to rust value.
trait FromJava<T> {
    fn from_java(env: &JNIEnv, input: T) -> Self;
}


impl FromJava<jni::sys::jint> for i32 {
    fn from_java(_env: &JNIEnv, input: jni::sys::jint) -> Self {
        input as Self
    }
}

impl<'a> ToJava<'a, jni::sys::jint> for i32 {
    fn to_java(&self, _env: &JNIEnv) -> jni::sys::jint {
        *self as jni::sys::jint
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

// TODO: this would have to be explicitly implemented for array of all types and
// sizes we need. Consider using macro.
impl<'a> FromJava<JObject<'a>> for [i8; 8] {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        let input = input.into_inner() as jni::sys::jbyteArray;
        let mut output = [0; 8];
        env.get_byte_array_region(input, 0, &mut output).unwrap();

        output
    }
}

impl<'a> ToJava<'a, JObject<'a>> for [i8; 8] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_byte_array(self.len() as jni::sys::jsize).unwrap();
        env.set_byte_array_region(output, 0, self).unwrap();
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

impl<'a> ToJava<'a, JObject<'a>> for backend::FfiResult {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("FfiResult", "()V", &[]).unwrap();

        env.set_field(
            output,
            "errorCode",
            "I",
            self.error_code.to_java(env).into(),
        ).unwrap();

        let error: JObject = self.error.to_java(&env).into();
        env.set_field(output, "error", "Ljava/lang/String;", error.into())
            .unwrap();

        output
    }
}

impl<'a> FromJava<JObject<'a>> for backend::Key {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        let bytes = env.get_field(input, "bytes", "[B").unwrap().l().unwrap();
        let bytes = <[i8; 8]>::from_java(env, bytes);

        backend::Key { bytes }
    }
}

impl<'a> ToJava<'a, JObject<'a>> for backend::Key {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("Key", "()V", &[]).unwrap();

        let bytes = self.bytes.to_java(env);
        env.set_field(output, "bytes", "[B", bytes.into()).unwrap();

        output
    }
}

impl<'a, 'b> ToJava<'a, JObject<'a>> for &'b [backend::Key] {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object_array(self.len() as jni::sys::jsize, "Key", JObject::null())
            .unwrap();

        for (index, item) in self.iter().enumerate() {
            env.set_object_array_element(output, index as jni::sys::jsize, item.to_java(env))
                .unwrap();
        }

        JObject::from(output as jni::sys::jobject)
    }
}

impl<'a> FromJava<JObject<'a>> for Vec<backend::Key> {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        let input = input.into_inner() as jni::sys::jobjectArray;
        let len = env.get_array_length(input).unwrap() as usize;

        let mut output = Vec::with_capacity(len);

        for index in 0..len {
            let item = env.get_object_array_element(input, index as jni::sys::jsize)
                .unwrap();
            output.push(backend::Key::from_java(&env, item));
        }

        output
    }
}

impl<'a> FromJava<JObject<'a>> for backend::AppInfo {
    fn from_java(env: &JNIEnv, input: JObject) -> Self {
        let id = env.get_field(input, "id", "I").unwrap().i().unwrap() as i32;

        let name = env.get_field(input, "name", "Ljava/lang/String;")
            .unwrap()
            .l()
            .unwrap()
            .into();
        let name = <*mut _>::from_java(env, name);

        let key = env.get_field(input, "key", "LKey;").unwrap().l().unwrap();
        let key = backend::Key::from_java(env, key);

        backend::AppInfo {
            id,
            name: name,
            key,
        }
    }
}

impl<'a> ToJava<'a, JObject<'a>> for backend::AppInfo {
    fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
        let output = env.new_object("AppInfo", "()V", &[]).unwrap();

        env.set_field(output, "id", "I", self.id.to_java(&env).into())
            .unwrap();

        let name: JObject = self.name.to_java(&env).into();
        env.set_field(output, "name", "Ljava/lang/String;", name.into())
            .unwrap();

        env.set_field(output, "key", "LKey;", self.key.to_java(&env).into())
            .unwrap();

        output
    }
}

unsafe extern "C" fn call(ctx: *mut c_void, result: *const backend::FfiResult) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);

    env.call_method(cb.as_obj(), "call", "(LFfiResult;)V", &[result.into()])
        .unwrap();
}

unsafe extern "C" fn call_int(ctx: *mut c_void, result: *const backend::FfiResult, arg: i32) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);
    let arg = arg.to_java(&env);

    env.call_method(
        cb.as_obj(),
        "call",
        "(LFfiResult;I)V",
        &[result.into(), arg.into()],
    ).unwrap();
}

unsafe extern "C" fn call_String(
    ctx: *mut c_void,
    result: *const backend::FfiResult,
    arg: *const c_char,
) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);
    let arg: JObject = arg.to_java(&env).into();

    env.call_method(
        cb.as_obj(),
        "call",
        "(LFfiResult;Ljava/lang/String;)V",
        &[result.into(), arg.into()],
    ).unwrap();
}

unsafe extern "C" fn call_Key(
    ctx: *mut c_void,
    result: *const backend::FfiResult,
    arg: *const backend::Key,
) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);
    let arg = (*arg).to_java(&env);

    env.call_method(
        cb.as_obj(),
        "call",
        "(LFfiResult;LKey;)V",
        &[result.into(), arg.into()],
    ).unwrap();
}

unsafe extern "C" fn call_array_int(
    ctx: *mut c_void,
    result: *const backend::FfiResult,
    arg0: *const i32,
    arg1: usize,
) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);
    let arg = slice::from_raw_parts(arg0, arg1).to_java(&env);

    env.call_method(
        cb.as_obj(),
        "call",
        "(LFfiResult;[I)V",
        &[result.into(), arg.into()],
    ).unwrap();
}

unsafe extern "C" fn call_array_Key(
    ctx: *mut c_void,
    result: *const backend::FfiResult,
    arg0: *const backend::Key,
    arg1: usize,
) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);
    let arg = slice::from_raw_parts(arg0, arg1).to_java(&env);

    env.call_method(
        cb.as_obj(),
        "call",
        "(LFfiResult;[LKey;)V",
        &[result.into(), arg.into()],
    ).unwrap();
}

unsafe extern "C" fn call_int_String_Key(
    ctx: *mut c_void,
    result: *const backend::FfiResult,
    arg0: i32,
    arg1: *const c_char,
    arg2: *const backend::Key,
) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let cb = GlobalRef::from_raw_ptr(&env, ctx);
    let result = (*result).to_java(&env);
    let arg0 = arg0.to_java(&env);
    let arg1: JObject = arg1.to_java(&env).into();
    let arg2 = (*arg2).to_java(&env);

    env.call_method(
        cb.as_obj(),
        "call",
        "(LFfiResult;ILjava/lang/String;LKey;)V",
        &[result.into(), arg0.into(), arg1.into(), arg2.into()],
    ).unwrap();
}

unsafe extern "C" fn call_createAccount_0(
    ctx: *mut c_void,
    result: *const backend::FfiResult,
    arg: *const backend::AppInfo,
) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let mut cbs = Box::from_raw(ctx as *mut [Option<GlobalRef>; 2]);
    let result = (*result).to_java(&env);
    let arg = (*arg).to_java(&env);

    if let Some(cb) = cbs[0].take() {
        env.call_method(
            cb.as_obj(),
            "call",
            "(LFfiResult;LAppInfo;)V",
            &[result.into(), arg.into()],
        ).unwrap();
    }

    // Prevent the context to be destroyed unless all callbacks have been called.
    if cbs.iter().any(|cb| cb.is_some()) {
        mem::forget(cbs);
    }
}

unsafe extern "C" fn call_createAccount_1(ctx: *mut c_void, result: *const backend::FfiResult) {
    let env = JVM.attach_current_thread_as_daemon().unwrap();

    let mut cbs = Box::from_raw(ctx as *mut [Option<GlobalRef>; 2]);
    let result = (*result).to_java(&env);

    if let Some(cb) = cbs[1].take() {
        env.call_method(cb.as_obj(), "call", "(LFfiResult;)V", &[result.into()])
            .unwrap();
    }

    // Prevent the context to be destroyed unless all callbacks have been called.
    if cbs.iter().any(|cb| cb.is_some()) {
        mem::forget(cbs);
    }
}

static mut JVM: JavaVM = JAVA_VM_INIT;

#[no_mangle]
// This is called when `loadLibrary` is called on the Java side.
pub unsafe extern "C" fn JNI_OnLoad(
    vm: *mut jni::sys::JavaVM,
    _reserved: *mut c_void,
) -> jni::sys::jint {
    JVM = JavaVM::from_raw(vm);
    jni::sys::JNI_VERSION_1_4
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_registerApp(
    env: JNIEnv,
    _class: JClass,
    app_info: JObject,
    cb: JObject,
) {
    let app_info = backend::AppInfo::from_java(&env, app_info);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::register_app(&app_info, ctx, Some(call))
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_getAppId(
    env: JNIEnv,
    _class: JClass,
    app_info: JObject,
    cb: JObject,
) {
    let app_info = backend::AppInfo::from_java(&env, app_info);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::get_app_id(&app_info, ctx, Some(call_int));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_getAppName(
    env: JNIEnv,
    _class: JClass,
    app_info: JObject,
    cb: JObject,
) {
    let app_info = backend::AppInfo::from_java(&env, app_info);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::get_app_name(&app_info, ctx, Some(call_String));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_getAppKey(
    env: JNIEnv,
    _class: JClass,
    app_info: JObject,
    cb: JObject,
) {
    let app_info = backend::AppInfo::from_java(&env, app_info);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::get_app_key(&app_info, ctx, Some(call_Key));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_randomNumbers(
    env: JNIEnv,
    _class: JClass,
    cb: JObject,
) {
    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::random_numbers(ctx, Some(call_array_int));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_randomKeys(
    env: JNIEnv,
    _class: JClass,
    cb: JObject,
) {
    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::random_keys(ctx, Some(call_array_Key));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_getAppInfo(
    env: JNIEnv,
    _class: JClass,
    app_info: JObject,
    cb: JObject,
) {
    let app_info = backend::AppInfo::from_java(&env, app_info);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::get_app_info(&app_info, ctx, Some(call_int_String_Key));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_createAccount(
    env: JNIEnv,
    _class: JClass,
    arg0: JString,
    arg1: JString,
    cb0: JObject,
    cb1: JObject,
) {
    let arg0 = CString::from_java(&env, arg0);
    let arg1 = CString::from_java(&env, arg1);

    let ctx = [
        Some(env.new_global_ref(cb0).unwrap()),
        Some(env.new_global_ref(cb1).unwrap()),
    ];
    let ctx = Box::into_raw(Box::new(ctx)) as *mut c_void;

    env.delete_local_ref(cb0).unwrap();
    env.delete_local_ref(cb1).unwrap();

    backend::create_account(
        arg0.as_ptr(),
        arg1.as_ptr(),
        ctx,
        Some(call_createAccount_0),
        Some(call_createAccount_1),
    );
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_verifySignature(
    env: JNIEnv,
    _class: JClass,
    arg: JObject,
    cb: JObject,
) {
    // TODO: instead of copying the data from the java array, we can "borrow" it
    // and then release it at the end - potentially avoiding the copy.
    let arg = Vec::from_java(&env, arg);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::verify_signature(arg.as_ptr(), arg.len(), ctx, Some(call));
}

#[no_mangle]
pub unsafe extern "system" fn Java_NativeBindings_verifyKeys(
    env: JNIEnv,
    _class: JClass,
    arg: JObject,
    cb: JObject,
) {
    let arg = Vec::from_java(&env, arg);

    let ctx = env.new_global_ref(cb).unwrap().into_raw_ptr();
    env.delete_local_ref(cb).unwrap();

    backend::verify_keys(arg.as_ptr(), arg.len(), ctx, Some(call));
}
