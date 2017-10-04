#ifndef _JNI_UTILS_H
#define _JNI_UTILS_H

typedef struct JniContext {
    JNIEnv    *env;
    jobject   obj;
    jmethodID cb_method_id;
} JniContext;

#endif
