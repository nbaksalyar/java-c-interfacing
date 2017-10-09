#ifndef _JNI_UTILS_H
#define _JNI_UTILS_H

#include <jni.h>

typedef struct JniContext {
    JNIEnv    *env;
    jobject   obj;
    jmethodID cb_method_id;
} JniContext;

typedef struct JniContextAsync {
    JavaVM    *vm;
    jobject   obj;
    jmethodID cb_method_id;
} JniContextAsync;

#endif
