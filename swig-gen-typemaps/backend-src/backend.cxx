#include "backend.h"

#include <iostream>
#include <sstream>
#include <thread>
#include <vector>

using std::cout;
using std::endl;

FfiResult ok() {
    return FfiResult {
        .error_code = 0,
        .error = (char*) "OK"
    };
}

template<typename F>
void run(const char* name, F body) {
    cout << "- C: " << name << "(): Start" << endl;

    auto thread = std::thread([=]() {
        cout << "- C: " << name << "(): New thread. Calling the callback..." << endl;
        body();
        cout << "- C: " << name << "(): Finished calling the callback. Exiting thread..." << endl;
    });

    thread.detach();
}

void register_app(const AppInfo* app_info, void* ctx, cb_t o_cb)
{
    run("register_app", [=]() {
        auto result = ok();
        o_cb(ctx, &result);
    });
}

void get_app_id(const AppInfo* app_info, void* ctx, cb_i32_t o_cb)
{
    run("get_app_id", [=]() {
        auto result = ok();
        o_cb(ctx, &result, app_info->id);
    });
}

void get_app_name(const AppInfo* app_info, void* ctx, cb_string_t o_cb)
{
    run("get_app_name", [=]() {
        auto result = ok();
        o_cb(ctx, &result, app_info->name);
    });
}

void get_app_key(const AppInfo* app_info, void* ctx, cb_Key_t o_cb)
{
    run("get_app_key", [=]() {
        auto result = ok();
        o_cb(ctx, &result, &app_info->key);
    });
}

void random_numbers(void* ctx, cb_i32_array_t o_cb)
{
    run("random_numbers", [=]() {
        auto result = ok();
        std::vector<int32_t> numbers = { 1, 1, 2, 3, 5, 8, 13, 21 };
        o_cb(ctx, &result, &numbers[0], numbers.size());
    });
}

void random_keys(void* ctx, cb_Key_array_t o_cb)
{
    run("random_keys", [=]() {
        auto result = ok();

        size_t count = 5;

        std::vector<Key> keys;
        keys.reserve(count);

        for (auto i = 0; i < count; ++i) {
            auto byte = (int8_t) i;
            keys.push_back(Key {{ byte, byte, byte, byte, byte, byte, byte, byte }});
        }

        o_cb(ctx, &result, &keys[0], keys.size());
    });
}

void get_app_info(const AppInfo* app_info, void* ctx, cb_i32_string_Key_t o_cb)
{
    run("get_app_info", [=]() {
        auto result = ok();
        o_cb(ctx, &result, app_info->id, app_info->name, &app_info->key);
    });
}

void create_account(const char*  locator,
                    const char*  password,
                    void*        ctx,
                    cb_AppInfo_t o_connect_cb,
                    cb_t         o_disconnect_cb)
{
    std::string name(locator);
    name.append(":");
    name.append(password);

    run("create_account", [=]() {
        auto result = ok();
        auto app_info = AppInfo {
            .id = 5678,
            .name = const_cast<char*>(name.c_str()),
            .key = Key {{ 0, 4, 6, 8, 9, 10, 12, 14 }}
        };

        cout << "- C: create_account(): calling connect callback..." << endl;
        o_connect_cb(ctx, &result, &app_info);
        cout << "- C: create_account(): calling disconnect callback..." << endl;
        o_disconnect_cb(ctx, &result);
    });
}
