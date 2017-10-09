#ifndef BACKEND_H_
#define BACKEND_H_

#ifdef __cplusplus
extern "C" {
#endif

    void backend_with_string(void *ctx, void(*o_cb)(void *ctx, const char *result));
    void backend_with_string_async(void *ctx, void(*o_cb)(void *ctx, const char *result));

#ifdef __cplusplus
}
#endif

#endif
