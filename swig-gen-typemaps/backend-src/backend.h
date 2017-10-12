#ifndef _BACKEND_H_
#define _BACKEND_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cpulsplus
extern "C" {
#endif
    typedef struct Key {
        int8_t bytes[8];
    } Key;

    typedef struct AppInfo {
        int32_t id;
        char* name;
        Key key;
    } AppInfo;

    typedef struct FfiResult {
        int32_t error_code;
        char* error;
    } FfiResult;

    typedef void(*cb_t)(void*, const FfiResult*);
    typedef void(*cb_i32_t)(void*, const FfiResult*, int32_t);
    typedef void(*cb_string_t)(void*, const FfiResult*, const char*);
    typedef void(*cb_i32_array_t)(void*, const FfiResult*, const int32_t*, size_t);
    typedef void(*cb_Key_t)(void*, const FfiResult*, const Key*);
    typedef void(*cb_Key_array_t)(void*, const FfiResult*, const Key*, size_t);
    typedef void(*cb_i32_string_Key_t)(void*, const FfiResult*, int32_t, const char*, const Key*);
    typedef void(*cb_AppInfo_t)(void*, const FfiResult*, const AppInfo*);

    // One callback with 0 params
    void register_app(const AppInfo* app_info, void* ctx, cb_t o_cb);
    // One callback with one primitive (int) param
    void get_app_id(const AppInfo* app_info, void* ctx, cb_i32_t o_cb);
    // One callback with one string param
    void get_app_name(const AppInfo* app_info, void* ctx, cb_string_t o_cb);
    // One callback with native struct param
    void get_app_key(const AppInfo* app_info, void* ctx, cb_Key_t o_cb);
    // One callback with array or ints param
    void random_numbers(void* ctx, cb_i32_array_t o_cb);
    // One callback with array of native structs param
    void random_keys(void* ctx, cb_Key_array_t o_cb);
    // One callback with multiple arguments
    void get_app_info(const AppInfo* app_info, void* ctx, cb_i32_string_Key_t o_cb);
    // Multiple callbacks
    void create_account(const char*  locator,
                        const char*  password,
                        void*        ctx,
                        cb_AppInfo_t o_connect_cb,
                        cb_t         o_disconnect_cb);

    // Input array of primitive type
    void verify_signature(const uint8_t* ptr, size_t len, void* ctx, cb_t o_cb);
    // Input array of native structs
    void verify_keys(const Key* ptr, size_t len, void* ctx, cb_t o_cb);

#ifdef __cpulsplus
}
#endif

#endif
