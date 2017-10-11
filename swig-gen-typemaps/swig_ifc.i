%module NativeBindings;

%{
#include "backend.h"
#include "jni_boilerplate.h"
%}

// Handling of exact integer types (int32_t, ...)
%include "stdint.i"
// Handling of arrays
%include "java/arrays_java.i"

// Callback (const FfiResult*)
%typemap(jni)    (void* ctx, cb_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_t o_cb) "Callback0";
%typemap(jtype)  (void* ctx, cb_t o_cb) "Callback0";
%typemap(javain) (void* ctx, cb_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_void;
}

// Callback (const FfiResult*, int32_t)
%typemap(jni)    (void* ctx, cb_i32_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_i32_t o_cb) "CallbackInt";
%typemap(jtype)  (void* ctx, cb_i32_t o_cb) "CallbackInt";
%typemap(javain) (void* ctx, cb_i32_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_i32_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_i32;
}

// Callback (const FfiResult*, const char*)
%typemap(jni)    (void* ctx, cb_string_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_string_t o_cb) "Callback1<String>";
%typemap(jtype)  (void* ctx, cb_string_t o_cb) "Callback1<String>";
%typemap(javain) (void* ctx, cb_string_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_string_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_string;
}

// Callback (const FfiResult*, const int32_t*, size_t)
%typemap(jni)    (void* ctx, cb_i32_array_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_i32_array_t o_cb) "Callback1<int[]>";
%typemap(jtype)  (void* ctx, cb_i32_array_t o_cb) "Callback1<int[]>";
%typemap(javain) (void* ctx, cb_i32_array_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_i32_array_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_i32_array;
}

// Callback (const FfiResult*, const Key*)
%typemap(jni)    (void* ctx, cb_Key_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_Key_t o_cb) "Callback1<Key>";
%typemap(jtype)  (void* ctx, cb_Key_t o_cb) "Callback1<Key>";
%typemap(javain) (void* ctx, cb_Key_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_Key_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_Key;
}

// Callback (const FfiResult*, const Key*, size_t)
%typemap(jni)    (void* ctx, cb_Key_array_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_Key_array_t o_cb) "Callback1<Key[]>";
%typemap(jtype)  (void* ctx, cb_Key_array_t o_cb) "Callback1<Key[]>";
%typemap(javain) (void* ctx, cb_Key_array_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_Key_array_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_Key_array;
}

// Callback (const FfiResult*, int32_t, const char*, const Key*)
%typemap(jni)    (void* ctx, cb_i32_string_Key_t o_cb) "jobject";
%typemap(jstype) (void* ctx, cb_i32_string_Key_t o_cb) "CallbackIntStringKey";
%typemap(jtype)  (void* ctx, cb_i32_string_Key_t o_cb) "CallbackIntStringKey";
%typemap(javain) (void* ctx, cb_i32_string_Key_t o_cb) "$javainput";

%typemap(in) (void* ctx, cb_i32_string_Key_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_cb_i32_string_Key;
}

// Two callbacks (const FfiResult*, const AppInfo*)
//               (const FfiResult*)
%typemap(jni)    (void* ctx, cb_AppInfo_t o_connect_cb, cb_t o_disconnect_cb) "jobject";
%typemap(jstype) (void* ctx, cb_AppInfo_t o_connect_cb, cb_t o_disconnect_cb) "CreateAccountHandler";
%typemap(jtype)  (void* ctx, cb_AppInfo_t o_connect_cb, cb_t o_disconnect_cb) "CreateAccountHandler";
%typemap(javain) (void* ctx, cb_AppInfo_t o_connect_cb, cb_t o_disconnect_cb) "$javainput";

%typemap(in) (void* ctx, cb_AppInfo_t o_connect_cb, cb_t o_disconnect_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = call_create_account_connect_cb;
  $3 = call_create_account_disconnect_cb;
}


// Load the DLL automatically
%pragma(java) jniclasscode=%{
  static {
    System.loadLibrary("frontend");
  }
%}

// We can use Java naming convention:
%rename(createAccount) create_account;
%rename(getAppId)      get_app_id;
%rename(getAppInfo)    get_app_info;
%rename(getAppKey)     get_app_key;
%rename(getAppName)    get_app_name;
%rename(randomKeys)    random_keys;
%rename(randomNumbers) random_numbers;
%rename(registerApp)   register_app;

%include "backend.h"
