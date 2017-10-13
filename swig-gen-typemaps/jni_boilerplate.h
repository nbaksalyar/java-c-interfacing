#pragma once

#include <iostream>
#include <string>
#include <cassert>
#include <jni.h>

#include "backend.h"

static JavaVM* jvm = nullptr;

// This is called when `loadLibrary` is called on the Java side.
extern "C"
jint JNI_OnLoad(JavaVM* vm, void* reserved) {
    jvm = vm;
    // TODO: not sure about this version.
    return JNI_VERSION_1_4;
}

// Wrap the C struct in the Java wrapper.
template<typename T>
jobject wrap(JNIEnv* env, const char* class_name, const T* input) {
    jclass klass = env->FindClass(class_name);
    assert(klass);

    // Here we exploit the fact that all SWIG generated struct have constructor
    // which takes a `long` (pointer to the C struct) and a `boolean` (whether the C
    // struct is owner or not).
    jmethodID constructor = env->GetMethodID(klass, "<init>", "(JZ)V");
    assert(constructor);

    jobject output = env->NewObject(klass, constructor, (jlong) input, JNI_FALSE);
    assert(output);

    return output;
}

// Helper
template<typename... F>
void call_cb(void* ctx,
             const FfiResult* result,
             const char* cb_class,
             const char* cb_method,
             const char* args_sig = "",
             F... wrap_args)
{
    JNIEnv* env = nullptr;
    jvm->AttachCurrentThreadAsDaemon((void**) &env, nullptr);

    jobject obj = (jobject) ctx;

    const jclass callbackClass = env->FindClass(cb_class);
    assert(callbackClass);

    std::string sig("(LFfiResult;");
    sig.append(args_sig);
    sig.append(")V");

    const jmethodID method = env->GetMethodID(callbackClass, cb_method, sig.c_str());
    assert(method);

    // TODO: handle exceptions thrown from inside the callback.

    env->CallVoidMethod(obj,
                        method,
                        wrap(env, "FfiResult", result),
                        wrap_args(env)...);

    env->DeleteGlobalRef(obj);
}

// Helper
template<typename T>
void call_cb_object(void* ctx, const FfiResult* result, const char* name, const T* arg) {
    call_cb(ctx, result, "Callback1", "call", "Ljava/lang/Object;", [=](auto env) {
        return wrap(env, name, arg);
    });
}

// Helper
template<typename T>
void call_cb_object_array(void* ctx, const FfiResult* result, const char* name, const T* ptr, size_t len) {
    call_cb(ctx, result, "Callback1", "call", "Ljava/lang/Object;", [=](auto env) {
        jclass elementClass = env->FindClass(name);
        assert(elementClass);

        auto array = env->NewObjectArray(len, elementClass, 0);
        assert(array);

        for (auto i = 0; i < len; ++i) {
            env->SetObjectArrayElement(array, i, wrap(env, name, ptr + i));
        }

        return array;
    });
}



void call_cb_void(void* ctx, const FfiResult* result) {
    call_cb(ctx, result, "Callback0", "call");
}

void call_cb_i32(void* ctx, const FfiResult* result, int32_t arg) {
    call_cb(ctx, result, "CallbackInt", "call", "I", [=](auto env) {
        return (jint) arg;
    });
}

void call_cb_string(void* ctx, const FfiResult* result, const char* arg) {
    call_cb(ctx, result, "Callback1", "call", "Ljava/lang/Object;", [=](auto env) {
        return env->NewStringUTF(arg);
    });
}

void call_cb_Key(void* ctx, const FfiResult* result, const Key* arg) {
    call_cb_object(ctx, result, "Key", arg);
}

void call_cb_i32_array(void* ctx, const FfiResult* result, const int32_t* ptr, size_t len) {
    call_cb(ctx, result, "Callback1", "call", "Ljava/lang/Object;", [=](auto env) {
        auto array = env->NewIntArray(len);
        env->SetIntArrayRegion(array, 0, len, ptr);
        return array;
    });
}

void call_cb_Key_array(void* ctx, const FfiResult* result, const Key* ptr, size_t len) {
    call_cb_object_array(ctx, result, "Key", ptr, len);
}

void call_cb_i32_string_Key(void* ctx,
                            const FfiResult* result,
                            int32_t arg0,
                            const char* arg1,
                            const Key* arg2)
{
    call_cb(
        ctx, result, "CallbackIntStringKey", "call", "ILjava/lang/String;LKey;",
        [=](auto env) { return (jint) arg0; },
        [=](auto env) { return env->NewStringUTF(arg1); },
        [=](auto env) { return wrap(env, "Key", arg2); }
    );
}

void call_cb_CreateAccountEvent(void* ctx, const FfiResult* result, const CreateAccountEvent* arg) {
    call_cb_object(ctx, result, "CreateAccountEvent", arg);
}

void call_create_account_connect_cb(void* ctx, const FfiResult* result, const AppInfo* app_info) {
    call_cb(ctx, result, "CreateAccountHandler", "onConnect", "LAppInfo;",
            [=](auto env) { return wrap(env, "AppInfo", app_info); });
}

void call_create_account_disconnect_cb(void* ctx, const FfiResult* result) {
    call_cb(ctx, result, "CreateAccountHandler", "onDisconnect");
}

template<typename T>
void copy_object_array(const char* class_name, JNIEnv* env, jobjectArray array, T*& o_ptr, size_t& o_len) {
    jclass klass = env->FindClass(class_name);
    assert(klass);

    std::string sig("(L");
    sig.append(class_name);
    sig.append(";)J");

    jmethodID method = env->GetStaticMethodID(klass, "getCPtr", sig.c_str());
    assert(method);

    size_t len = env->GetArrayLength(array);

    auto temp = new T[len];

    for (size_t i = 0; i < len; ++i) {
        auto j_object = env->GetObjectArrayElement(array, i);
        auto c_object = (T*) env->CallStaticLongMethod(klass, method, j_object);

        temp[i] = *c_object;
    }

    o_ptr = temp;
    o_len = len;
}
