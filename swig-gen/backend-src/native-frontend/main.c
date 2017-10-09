#include "backend.h"

#include <inttypes.h>
#include <malloc.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

typedef struct Context {
    pthread_cond_t *p_cond;
    pthread_mutex_t *p_mutex;
    bool done;

    FfiResult result;
    AuthResp auth_resp;
} Context;

void callback(void *ctx, const FfiResult *p_result, const AuthResp *p_auth_resp) {
    Context *p_context = ctx;

    sleep(1);

    printf("Signalling to the main thread to exit...\n");

    pthread_mutex_lock(p_context->p_mutex);

    p_context->result.p_error = malloc(sizeof(char) * (strlen(p_result->p_error) + 1));
    strcpy(p_context->result.p_error, p_result->p_error);
    p_context->result.error_code = p_result->error_code;

    if(p_auth_resp) {
        p_context->auth_resp.p_msg = malloc(sizeof(char) * (strlen(p_auth_resp->p_msg) + 1));
        strcpy(p_context->auth_resp.p_msg, p_auth_resp->p_msg);
        p_context->auth_resp.orig_req_id = p_auth_resp->orig_req_id;
    }

    p_context->done = true;

    pthread_cond_signal(p_context->p_cond);
    pthread_mutex_unlock(p_context->p_mutex);
}

int main() {
    AppInfo app_info = { .p_id = "App-ID-0", .p_name = "MyApp", .p_vendor = "Spandan" };
    AuthReq auth_req = { .p_info = &app_info, .needs_own_container = true, .req_id = 999 };

    pthread_mutex_t mutex;
    pthread_mutex_init(&mutex, 0);

    pthread_cond_t cond;
    pthread_cond_init(&cond, 0);

    Context context = { .p_cond = &cond, .p_mutex = &mutex, .done = false };

    backend_on_auth_reqest(&auth_req, &context, callback);
    printf("Back in main(). Waiting for results...\n");

    pthread_mutex_lock(&mutex);
    while(!context.done) {
        pthread_cond_wait(&cond, &mutex);
    }
    pthread_mutex_unlock(&mutex);

    pthread_cond_destroy(&cond);
    pthread_mutex_destroy(&mutex);

    if(context.result.error_code) {
        printf("Unsuccessful. %s\n", context.result.p_error);
    } else {
        printf("Result: %s\n", context.result.p_error);
        printf("Got AuthResp msg corresponding to request id [%" PRIu64 "]: %s\n",
               context.auth_resp.orig_req_id,
               context.auth_resp.p_msg);
    }

    free(context.auth_resp.p_msg);
    free(context.result.p_error);

    printf("Exiting main()\n");
}
