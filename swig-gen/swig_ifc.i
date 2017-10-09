%module(directors="1") NativeBindings;
%feature("director");

%{
#include "backend.h"
#include "stdint.h"
#include <unistd.h>

class BackendOnAuthReqCb {
    public:
        virtual ~BackendOnAuthReqCb() = default;
        virtual void o_cb(const FfiResult *p_result, const AuthResp *p_auth_resp ) = 0;
};

static void backend_on_auth_request_cb(
        void *ctx,
        const FfiResult *p_result,
        const AuthResp *p_auth_resp
) {
    // Only to simulate a delay - no real use
    sleep(2);
    BackendOnAuthReqCb *user_data = (BackendOnAuthReqCb*)ctx;
    user_data->o_cb(p_result, p_auth_resp);
}


void backend_on_auth_request_java(const AuthReq *p_auth_req, BackendOnAuthReqCb *user_data) {
    backend_on_auth_request(p_auth_req, user_data, backend_on_auth_request_cb);
}
%}

// Has to be before the header file inclusion
%ignore backend_on_auth_request;

%include "stdint.i"
%include "backend.h"

class BackendOnAuthReqCb {
    public:
        virtual ~BackendOnAuthReqCb() = default;
        virtual void o_cb(const FfiResult *p_result, const AuthResp *p_auth_resp ) = 0;
};

void backend_on_auth_request_java(const AuthReq *p_auth_req, BackendOnAuthReqCb *user_data);
