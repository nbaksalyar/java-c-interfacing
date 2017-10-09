#include "frontend.h"
#include "backend.h"
#include "jni_utils.h"

#include <malloc.h>

static void backend_with_string_jni_cb(void *user_data, const char *result) {
    JniContext ctx = *(JniContext*)user_data;

    jstring java_result = (*ctx.env)->NewStringUTF(ctx.env, result);

    (*ctx.env)->ExceptionClear(ctx.env);
    (*ctx.env)->CallVoidMethod(ctx.env, ctx.obj, ctx.cb_method_id, java_result);
}

JNIEXPORT void JNICALL Java_Frontend_BackendWithStringJNI(
        JNIEnv *env,
        jobject obj,
        jstring cb_name
) {
    // Reason I take this from Java itself is Java can pass us a new function and we wouldn't care
    // in the jni layer about it.
    const char *cb_name_cstr = (*env)->GetStringUTFChars(env, cb_name, 0);

    jclass obj_class = (*env)->GetObjectClass(env, obj);

    // To know the string for the function parameters (last arg below), construct an equivalent
    // dummy native function in Java and run `javah` on it. It'll generate the `.h` file with a doc
    // comment specifying the string (protocal) representing the function parameters.
    jmethodID cb_method_id = (*env)->GetMethodID(
            env,
            obj_class,
            cb_name_cstr,
            "(Ljava/lang/String;)V");

    (*env)->ReleaseStringUTFChars(env, cb_name, cb_name_cstr);

    if(!cb_method_id) {
        printf("ERROR!! Could not get the callback method's id!\n");
        return;
    }

    // Since the backend invokes the given callback synchronously, we don't need to heap allocate
    // this, else we would have to if the backend was async/non-blocking.
    JniContext jni_ctx;
    jni_ctx.env = env;
    jni_ctx.obj = obj;
    jni_ctx.cb_method_id = cb_method_id;

    backend_with_string(&jni_ctx, backend_with_string_jni_cb);
}


// ======================================================================================
// ======================================================================================
// ======================================================================================


static void backend_with_string_async_jni_cb(void *user_data, const char *result) {
    JniContextAsync *ctx = (JniContextAsync*)user_data;

    JNIEnv *new_env;
    (*ctx->vm)->AttachCurrentThread(ctx->vm, (void**)&new_env, 0);

    jstring java_result = (*new_env)->NewStringUTF(new_env, result);

    (*new_env)->ExceptionClear(new_env);
    (*new_env)->CallVoidMethod(new_env, ctx->obj, ctx->cb_method_id, java_result);

    (*new_env)->DeleteGlobalRef(new_env, ctx->obj);

    // After JVM is detached, env etc will no longer be valid; so perform everything that uses them
    // before this, like DeleteGlobalRef() etc., else there will be seg-fault
    (*ctx->vm)->DetachCurrentThread(ctx->vm);

    free(ctx);
}

JNIEXPORT void JNICALL Java_Frontend_BackendWithStringAsyncJNI(
        JNIEnv *env,
        jobject obj,
        jstring cb_name
) {
    const char *cb_name_cstr = (*env)->GetStringUTFChars(env, cb_name, 0);

    jclass obj_class = (*env)->GetObjectClass(env, obj);

    jmethodID cb_method_id = (*env)->GetMethodID(
            env,
            obj_class,
            cb_name_cstr,
            "(Ljava/lang/String;)V");

    (*env)->ReleaseStringUTFChars(env, cb_name, cb_name_cstr);

    if(!cb_method_id) {
        printf("ERROR!! Could not get the callback method's id!\n");
        return;
    }

    JniContextAsync *jni_ctx = (JniContextAsync*)malloc(sizeof(JniContextAsync));
    // On things that persist or don't persist between threads, have a look at:
    // https://stackoverflow.com/a/13417898/1060004
    (*env)->GetJavaVM(env, &jni_ctx->vm);
    jni_ctx->obj = (*env)->NewGlobalRef(env, obj);
    jni_ctx->cb_method_id = cb_method_id;

    backend_with_string_async(jni_ctx, backend_with_string_async_jni_cb);
}
