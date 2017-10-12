#include "backend.h"

#include <algorithm>
#include <chrono>
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
    using namespace std::chrono_literals;

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

        std::this_thread::sleep_for(2s);

        cout << "- C: create_account(): calling disconnect callback..." << endl;
        o_disconnect_cb(ctx, &result);
    });
}

void verify_signature(const uint8_t* ptr, size_t len, void* ctx, cb_t o_cb) {
    std::vector<uint8_t> data(ptr, ptr + len);

    run("verify_signature", [=]() {
        bool valid = std::any_of(data.begin(), data.end(), [=](auto e) {
            return e != 0;
        });

        if (valid) {
            auto result = ok();
            o_cb(ctx, &result);
        } else {
            FfiResult result = {
                .error_code = -11,
                .error = (char*) "Invalid signature",
            };
            o_cb(ctx, &result);
        }
    });
}

void verify_keys(const Key* ptr, size_t len, void* ctx, cb_t o_cb) {
    std::vector<Key> keys(ptr, ptr + len);

    run("verify_keys", [=]() {
        for (auto& key : keys) {
            cout << "- C: verify_keys(): [";
            for (auto b : key.bytes) {
                cout << (int) b << ", ";
            }
            cout << "]" << endl;
        }

        auto result = ok();
        o_cb(ctx, &result);
    });
}

