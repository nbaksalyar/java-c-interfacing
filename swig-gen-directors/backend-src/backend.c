#include "backend.h"

#include <malloc.h>
#include <pthread.h>
#include <stdio.h>

typedef struct ThreadData {
    pthread_t thread_id;
    const AuthReq *p_auth_req;
    void *ctx;
    void(*o_cb)(void *ctx, const FfiResult *p_result, const AuthResp *p_auth_resp);
} ThreadData;

static void* backend_on_auth_request_routine(void *arg) {
    printf("- In a new thread backend_on_auth_request_routine() inside C code\n");

    ThreadData *args = (ThreadData*)arg;

    printf("- Calling the function passed to us by Java...\n");

    const char *fmt = "Granted authorisation to App with ID: %s, Name: %s and vendor: %s. "
                      "Asked for exclusive App container: %d.";

    const AppInfo *p_info = args->p_auth_req->p_info;

    int len = snprintf(
                  0,
                  0,
                  fmt,
                  p_info->p_id,
                  p_info->p_name,
                  p_info->p_vendor,
                  args->p_auth_req->needs_own_container);

    if(len > 0) {
        FfiResult result = { .error_code = 0, .p_error = (char*)"OK" };

        char *p_msg = (char*)malloc(sizeof(char) * (len + 1));

        sprintf(
            p_msg,
            fmt,
            p_info->p_id,
            p_info->p_name,
            p_info->p_vendor,
            args->p_auth_req->needs_own_container);

        AuthResp auth_resp = { .p_msg = p_msg, .orig_req_id = args->p_auth_req->req_id };

        args->o_cb(args->ctx, &result, &auth_resp);

        free(p_msg);
    } else {
        FfiResult result = {
            .error_code = -2,
            .p_error = (char*)"ERROR: Could not construct AuthResponse"
        };
        args->o_cb(args->ctx, &result, 0);
    }

    printf("- Finished invoking the given callback. Now back in C code."
           " Exiting thread...\n");

    free(args);

    return 0;
}

void backend_on_auth_request(
    const AuthReq *p_auth_req,
    void *ctx,
    void(*o_cb)(void *ctx, const FfiResult *p_result, const AuthResp *p_auth_resp)
) {
    printf("- In function backend_on_auth_request() inside C code\n");

    ThreadData *args = (ThreadData*)malloc(sizeof(ThreadData));
    args->p_auth_req = p_auth_req;
    args->ctx = ctx;
    args->o_cb = o_cb;

    if (pthread_create(&args->thread_id, 0, backend_on_auth_request_routine, args)) {
        printf("- ERROR: Could not create a thread !!\n");
        FfiResult result = {
            .error_code = -1,
            .p_error = (char*)"ERROR: Could not create thread"
        };
        o_cb(args->ctx, &result, 0);

        free(args);
    }
}
