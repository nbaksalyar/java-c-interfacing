%module NativeBindings;

%{
#include "backend.h"
#include "jni_boilerplate.h"
%}

// Handling of exact integer types (int32_t, ...)
%include "stdint.i"
// Handling of arrays
%include "java/arrays_java.i"

%define CALLBACK(signature, java_type)
  %typemap(jni)    (void* ctx, cb_ ## signature ## _t o_cb) "jobject";
  %typemap(jstype) (void* ctx, cb_ ## signature ## _t o_cb) "java_type";
  %typemap(jtype)  (void* ctx, cb_ ## signature ## _t o_cb) "java_type";
  %typemap(javain) (void* ctx, cb_ ## signature ## _t o_cb) "$javainput";

  %typemap(in) (void* ctx, cb_ ## signature ## _t o_cb) {
    $1 = jenv->NewGlobalRef($input);
    jenv->DeleteLocalRef($input);

    $2 = call_cb_ ## signature;
  }
%enddef

// Single callback
CALLBACK(void,           Callback0)
CALLBACK(string,         Callback1<String>)
CALLBACK(i32,            CallbackInt)
CALLBACK(i32_array,      Callback1<int[]>)
CALLBACK(Key,            Callback1<Key>)
CALLBACK(Key_array,      Callback1<Key[]>)
CALLBACK(i32_string_Key, CallbackIntStringKey)

// Two callbacks
%typemap(jni)    (void* ctx, cb_AppInfo_t o_connect_cb, cb_void_t o_disconnect_cb) "jobject";
%typemap(jstype) (void* ctx, cb_AppInfo_t o_connect_cb, cb_void_t o_disconnect_cb) "CreateAccountHandler";
%typemap(jtype)  (void* ctx, cb_AppInfo_t o_connect_cb, cb_void_t o_disconnect_cb) "CreateAccountHandler";
%typemap(javain) (void* ctx, cb_AppInfo_t o_connect_cb, cb_void_t o_disconnect_cb) "$javainput";
%typemap(in)     (void* ctx, cb_AppInfo_t o_connect_cb, cb_void_t o_disconnect_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_create_account_connect_cb;
  $3 = call_create_account_disconnect_cb;
}

// Array of uint8_t
%apply(char *STRING, size_t LENGTH) { (const uint8_t* ptr, size_t len) };

// Array of Key
%typemap(jni)     (const Key* ptr, size_t len) "jobjectArray"
%typemap(jtype)   (const Key* ptr, size_t len) "Key[]"
%typemap(jstype)  (const Key* ptr, size_t len) "Key[]"
%typemap(javain)  (const Key* ptr, size_t len) "$javainput"

%typemap(in)      (const Key* ptr, size_t len) {
  copy_object_array<Key>("Key", jenv, $input, $1, $2);
}

%typemap(freearg) (const Key* ptr, size_t len) {
  delete $1;
}

// Load the DLL automatically
%pragma(java) jniclasscode=%{
  static {
    System.loadLibrary("frontend");
  }
%}

// We can use Java naming convention:
%rename(createAccount)   create_account;
%rename(getAppId)        get_app_id;
%rename(getAppInfo)      get_app_info;
%rename(getAppKey)       get_app_key;
%rename(getAppName)      get_app_name;
%rename(randomKeys)      random_keys;
%rename(randomNumbers)   random_numbers;
%rename(registerApp)     register_app;
%rename(verifyKeys)      verify_keys;
%rename(verifySignature) verify_signature;

%include "backend.h"
