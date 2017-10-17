#include <jni.h>
#include <string.h>

#include <cassert>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "backend.h"

static JavaVM* jvm = nullptr;

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

jobject new_java_object(JNIEnv* env, jclass klass) {
    auto constructor = env->GetMethodID(klass, "<init>", "()V");
    assert(constructor);

    auto output = env->NewObject(klass, constructor);
    assert(output);

    return output;
}

template<typename> const char* type_name();

// int
// -----------------------------------------------------------------------------
template<> const char* type_name<int32_t>() { return "int"; }

jint to_java(JNIEnv*, int32_t input) {
    return (jint) input;
}

// char*
// -----------------------------------------------------------------------------
template<> const char* type_name<const char*>() { return "String"; }

jstring to_java(JNIEnv* env, const char* input) {
    return env->NewStringUTF(input);
}

void from_java(JNIEnv* env, jstring input, char*& output) {
    auto chars = env->GetStringUTFChars(input, nullptr);
    output = strdup(chars);
    env->ReleaseStringUTFChars(input, chars);
}

// array of objects / structs
// -----------------------------------------------------------------------------
template<typename T>
jobjectArray to_java(JNIEnv* env, std::pair<const T*, size_t> input) {
    auto elementClass = env->FindClass(type_name<T>());
    assert(elementClass);

    auto array = env->NewObjectArray(input.second, elementClass, 0);
    assert(array);

    for (auto i = 0; i < input.second; ++i) {
        env->SetObjectArrayElement(array, i, to_java(env, input.first + i));
    }

    return array;
}

template<typename T>
void from_java(JNIEnv* env, jobjectArray input, std::vector<T>& output) {
    output.resize(env->GetArrayLength(input));

    for (auto i = 0; i < output.size(); ++i) {
        auto element = env->GetObjectArrayElement(input, i);
        from_java(env, element, output[i]);
    }
}


// array of byte
// -----------------------------------------------------------------------------
void from_java(JNIEnv* env, jbyteArray input, std::vector<uint8_t>& output) {
    output.resize(env->GetArrayLength(input));
    env->GetByteArrayRegion(input, 0, output.size(), (jbyte*) &output[0]);
}

// array of int
// -----------------------------------------------------------------------------
jintArray to_java(JNIEnv* env, std::pair<const int32_t*, size_t> input) {
    auto output = env->NewIntArray(input.second);
    env->SetIntArrayRegion(output, 0, input.second, input.first);
    return output;
}

// FfiResult
// -----------------------------------------------------------------------------
template<> const char* type_name<FfiResult>() { return "FfiResult"; }

jobject to_java(JNIEnv* env, const FfiResult* input) {
    auto klass = env->FindClass("FfiResult");
    assert(klass);

    auto output = new_java_object(env, klass);

    auto f_errorCode = env->GetFieldID(klass, "errorCode", "I");
    assert(f_errorCode);
    env->SetIntField(output, f_errorCode, input->error_code);

    auto f_error = env->GetFieldID(klass, "error", "Ljava/lang/String;");
    assert(f_error);
    env->SetObjectField(output, f_error, to_java(env, input->error));

    return output;
}

// Key
// -----------------------------------------------------------------------------
template<> const char* type_name<Key>() { return "Key"; }

void from_java(JNIEnv* env, jobject input, Key& output) {
    auto klass = env->GetObjectClass(input);
    assert(klass);

    auto f_bytes = env->GetFieldID(klass, "bytes", "[B");
    assert(f_bytes);
    auto j_bytes = (jbyteArray) env->GetObjectField(input, f_bytes);

    env->GetByteArrayRegion(j_bytes, 0, 8, (jbyte*) &output.bytes);
}

jobject to_java(JNIEnv* env, const Key* input) {
    auto klass = env->FindClass("Key");
    assert(klass);

    auto output = new_java_object(env, klass);

    auto f_bytes = env->GetFieldID(klass, "bytes", "[B");
    assert(f_bytes);

    auto j_bytes = env->NewByteArray(8);
    assert(j_bytes);

    env->SetByteArrayRegion(j_bytes, 0, 8, (jbyte*) input->bytes);
    env->SetObjectField(output, f_bytes, j_bytes);

    return output;
}

// AppInfo
// -----------------------------------------------------------------------------
void from_java(JNIEnv* env, jobject input, AppInfo& output) {
    auto klass = env->GetObjectClass(input);
    assert(klass);

    auto f_id = env->GetFieldID(klass, "id", "I");
    assert(f_id);
    output.id = env->GetIntField(input, f_id);

    auto f_name = env->GetFieldID(klass, "name", "Ljava/lang/String;");
    assert(f_name);
    auto j_name = (jstring) env->GetObjectField(input, f_name);
    from_java(env, j_name, output.name);

    auto f_key = env->GetFieldID(klass, "key", "LKey;");
    assert(f_key);
    auto j_key = env->GetObjectField(input, f_key);
    from_java(env, j_key, output.key);
}

jobject to_java(JNIEnv* env, const AppInfo* input) {
    auto klass = env->FindClass("AppInfo");
    assert(klass);

    auto output = new_java_object(env, klass);

    auto f_id = env->GetFieldID(klass, "id", "I");
    assert(f_id);
    env->SetIntField(output, f_id, to_java(env, input->id));

    auto f_name = env->GetFieldID(klass, "name", "Ljava/lang/String;");
    assert(f_name);
    env->SetObjectField(output, f_name, to_java(env, input->name));

    auto f_key = env->GetFieldID(klass, "key", "LKey;");
    assert(f_key);
    env->SetObjectField(output, f_key, to_java(env, &input->key));

    return output;
}

// -----------------------------------------------------------------------------

template<typename... T>
void call_impl(const char* cb_class_name, const char* signature, void* ctx, const FfiResult* result, T... args) {
    JNIEnv* env = nullptr;
    jvm->AttachCurrentThreadAsDaemon((void**) &env, nullptr);

    auto cb = (jobject) ctx;

    auto cb_class = env->FindClass(cb_class_name);
    assert(cb_class);

    auto method = env->GetMethodID(cb_class, "call", signature);
    assert(method);

    // TODO: handle exceptions thrown from inside the callback.

    env->CallVoidMethod(cb, method, to_java(env, result), to_java(env, args)...);
    env->DeleteGlobalRef(cb);
}

void call(void* ctx, const FfiResult* result) {
    call_impl("Callback", "(LFfiResult;)V", ctx, result);
}

void call_int(void* ctx, const FfiResult* result, int32_t arg) {
    call_impl("Callback_int", "(LFfiResult;I)V", ctx, result, arg);
}

void call_array_int(void* ctx, const FfiResult* result, const int32_t* ptr, size_t len) {
    call_impl("Callback_array_int",
              "(LFfiResult;[I)V",
              ctx,
              result,
              std::make_pair(ptr, len));
}

void call_String(void* ctx, const FfiResult* result, const char* arg) {
    call_impl("Callback_String",
              "(LFfiResult;Ljava/lang/String;)V",
              ctx,
              result,
              arg);
}

void call_Key(void* ctx, const FfiResult* result, const Key* arg) {
    call_impl("Callback_Key", "(LFfiResult;LKey;)V", ctx, result, arg);
}

void call_array_Key(void* ctx, const FfiResult* result, const Key* ptr, size_t len) {
    call_impl("Callback_array_Key", "(LFfiResult;[LKey;)V", ctx, result, std::make_pair(ptr, len));
}

void call_int_String_Key(void* ctx, const FfiResult* result, int32_t arg0, const char* arg1, const Key* arg2) {
    call_impl("Callback_int_String_Key",
              "(LFfiResult;ILjava/lang/String;LKey;)V",
              ctx,
              result,
              arg0,
              arg1,
              arg2);
}

// Helper to call callback of function that take multiple callbacks.
template<typename... Ts>
void call_multi_impl(const char* cb_class_name,
                     const char* signature,
                     size_t index,
                     size_t count,
                     void* ctx,
                     const FfiResult* result,
                     Ts... args)
{
    JNIEnv* env = nullptr;
    jvm->AttachCurrentThreadAsDaemon((void**) &env, nullptr);

    auto cbs = (jobject*) ctx;

    auto cb_class = env->FindClass(cb_class_name);
    assert(cb_class);

    auto method = env->GetMethodID(cb_class, "call", signature);
    assert(method);

    // TODO: handle exceptions thrown from inside the callback.

    env->CallVoidMethod(cbs[index], method, to_java(env, result), to_java(env, args)...);
    env->DeleteGlobalRef(cbs[index]);
    cbs[index] = nullptr;

    // If all callbacks were already called, we can delete the context to prevent
    // leaking memory.
    for (auto i = 0; i < count; ++i) {
        if (cbs[i] != nullptr) {
            return;
        }
    }

    delete cbs;
}

void call_createAccount_0(void* ctx, const FfiResult* result, const AppInfo* arg) {
    call_multi_impl("Callback_AppInfo", "(LFfiResult;LAppInfo;)V", 0, 2, ctx, result, arg);
}

void call_createAccount_1(void* ctx, const FfiResult* result) {
    call_multi_impl("Callback", "(LFfiResult;)V", 1, 2, ctx, result);
}

// -----------------------------------------------------------------------------
// Wrappers
// -----------------------------------------------------------------------------

extern "C" {

// This is called when `loadLibrary` is called on the Java side.
jint JNI_OnLoad(JavaVM* vm, void* reserved) {
    jvm = vm;
    // TODO: not sure about this version.
    return JNI_VERSION_1_4;
}

void Java_NativeBindings_registerApp(JNIEnv* env, jclass klass, jobject j_app_info, jobject cb) {
    AppInfo app_info;
    from_java(env, j_app_info, app_info);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    register_app(&app_info, ctx, call);

    // TODO: clean up app_info.
}

void Java_NativeBindings_getAppId(JNIEnv* env, jclass klass, jobject j_app_info, jobject cb) {
    AppInfo app_info;
    from_java(env, j_app_info, app_info);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    get_app_id(&app_info, ctx, call_int);
}

void Java_NativeBindings_getAppName(JNIEnv* env, jclass klass, jobject j_app_info, jobject cb) {
    AppInfo app_info;
    from_java(env, j_app_info, app_info);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    get_app_name(&app_info, ctx, call_String);
}

void Java_NativeBindings_getAppKey(JNIEnv* env, jclass klass, jobject j_app_info, jobject cb) {
    AppInfo app_info;
    from_java(env, j_app_info, app_info);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    get_app_key(&app_info, ctx, call_Key);
}

void Java_NativeBindings_randomNumbers(JNIEnv* env, jclass klass, jobject cb) {
    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    random_numbers(ctx, call_array_int);
}

void Java_NativeBindings_randomKeys(JNIEnv* env, jclass klass, jobject cb) {
    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    random_keys(ctx, call_array_Key);
}

void Java_NativeBindings_getAppInfo(JNIEnv* env, jclass klass, jobject j_app_info, jobject cb) {
    AppInfo app_info;
    from_java(env, j_app_info, app_info);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    get_app_info(&app_info, ctx, call_int_String_Key);
}

void Java_NativeBindings_createAccount(JNIEnv* env,
                                       jclass klass,
                                       jstring j_locator,
                                       jstring j_password,
                                       jobject connect_cb,
                                       jobject disconnect_cb)
{
    char* locator;
    from_java(env, j_locator, locator);

    char* password;
    from_java(env, j_password, password);

    auto cbs = new jobject[2];
    cbs[0] = env->NewGlobalRef(connect_cb);
    cbs[1] = env->NewGlobalRef(disconnect_cb);

    env->DeleteLocalRef(connect_cb);
    env->DeleteLocalRef(disconnect_cb);

    create_account(locator,
                   password,
                   (void*) cbs,
                   call_createAccount_0,
                   call_createAccount_1);

    free(locator);
    free(password);
}

void Java_NativeBindings_verifySignature(JNIEnv* env, jclass klass, jbyteArray j_data, jobject cb) {
    // TODO: instead of copying the data from the java array, we can "borrow" it
    // and then release it at the end - potentially avoiding the copy.

    std::vector<uint8_t> data;
    from_java(env, j_data, data);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    verify_signature(&data[0], data.size(), ctx, call);
}

void Java_NativeBindings_verifyKeys(JNIEnv* env, jclass klass, jobjectArray j_data, jobject cb) {
    std::vector<Key> data;
    from_java(env, j_data, data);

    auto ctx = (void*) env->NewGlobalRef(cb);
    env->DeleteLocalRef(cb);

    verify_keys(&data[0], data.size(), ctx, call);
}

} // extern "C"
