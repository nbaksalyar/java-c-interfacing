#pragma once

#include <assert.h>

static JavaVM* jvm = nullptr;

// This is called when `loadLibrary` is called on the Java side.
extern "C"
jint JNI_OnLoad(JavaVM* vm, void* reserved) {
    jvm = vm;
    // TODO: not sure about this version.
    return JNI_VERSION_1_8;
}

// Convert C struct to its Java counterpart.
jobject convertToJava(JNIEnv* env, const char* class_name, const void* input) {
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

void callback_wrapper(void* ctx, const FfiResult* result, const AuthResp* auth_resp) {
    JNIEnv* env = nullptr;
    jvm->AttachCurrentThread((void**) &env, nullptr);

    jobject obj = (jobject) ctx;

    const jclass callbackClass = env->FindClass("Callback");
    assert(callbackClass);

    const jmethodID method = env->GetMethodID(callbackClass, "call", "(LFfiResult;LAuthResp;)V");
    assert(method);

    jobject j_result = convertToJava(env, "FfiResult", result);
    jobject j_auth_resp = convertToJava(env, "AuthResp", auth_resp);

    // TODO: handle exceptions thrown from inside the callback.

    env->CallVoidMethod(obj, method, j_result, j_auth_resp);
    env->DeleteGlobalRef(obj);

    jvm->DetachCurrentThread();
}
