%module NativeBindings;

%{
#include "backend.h"
#include "jni_boilerplate.h"
%}

%include "stdint.i"

// TODO: figure out a way to not have to specify the argument names

%typemap(jni)    (void* ctx, callback_t o_cb) "jobject";
%typemap(jstype) (void* ctx, callback_t o_cb) "Callback";
%typemap(jtype)  (void* ctx, callback_t o_cb) "Callback";
%typemap(javain) (void* ctx, callback_t o_cb) "$javainput";

%typemap(in) (void* ctx, callback_t o_cb) {
  $1 = jenv->NewGlobalRef($input);
  jenv->DeleteLocalRef($input);

  $2 = callback_wrapper;
}

%pragma(java) jniclasscode=%{
  static {
    System.loadLibrary("frontend");
  }
%}

// We can use Java naming convention:
%rename(backendOnAuthRequest) backend_on_auth_request;

%include "backend.h"
