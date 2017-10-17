%module NativeBindings;

%{
#include "backend.h"
%}

%include "stdint.i"
%include "csharp/arrays_csharp.i"

// We can use C# naming convention:
%rename(CreateAccount)   create_account;
%rename(CreateAccount2)  create_account_2;
%rename(GetAppId)        get_app_id;
%rename(GetAppInfo)      get_app_info;
%rename(GetAppKey)       get_app_key;
%rename(GetAppName)      get_app_name;
%rename(RandomKeys)      random_keys;
%rename(RandomNumbers)   random_numbers;
%rename(RegisterApp)     register_app;
%rename(VerifyKeys)      verify_keys;
%rename(VerifySignature) verify_signature;

%include "backend.h"
