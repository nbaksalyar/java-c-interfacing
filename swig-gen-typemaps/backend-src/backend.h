#ifndef _BACKEND_H_
#define _BACKEND_H_

#include <stdbool.h>
#include <stdint.h>

#ifdef __cpulsplus
extern "C" {
#endif

    typedef struct AppInfo {
        char *p_id;
        char *p_name;
        char *p_vendor;
    } AppInfo;

    typedef struct AuthReq {
        AppInfo *p_info;
        bool needs_own_container;
        uint64_t req_id;
    } AuthReq;

    typedef struct AuthResp {
        char *p_msg;
        uint64_t orig_req_id;
    } AuthResp;

    typedef struct FfiResult {
        int32_t error_code;
        char *p_error;
    } FfiResult;

    typedef void(*callback_t)(void*, const FfiResult*, const AuthResp*);
    void backend_on_auth_request(const AuthReq *p_auth_req, void *ctx, callback_t o_cb);

#ifdef __cpulsplus
}
#endif

#endif
