using System;
using System.Runtime.InteropServices;

[StructLayout(LayoutKind.Sequential)]
public struct FfiResult {
    [MarshalAs(UnmanagedType.I4)]
    public int errorCode;

    [MarshalAs(UnmanagedType.LPStr)]
    public String error;
}

[StructLayout(LayoutKind.Sequential)]
public struct Key {
    [MarshalAs(UnmanagedType.ByValArray, SizeConst = 8)]
    public byte[] bytes;
}

[StructLayout(LayoutKind.Sequential)]
public struct AppInfo {
    [MarshalAs(UnmanagedType.I4)]
    public int id;

    [MarshalAs(UnmanagedType.LPStr)]
    public String name;

    public Key key;
}


public class NativeBindings {
    private delegate void Callback0(IntPtr ctx, ref FfiResult result);
    private delegate void Callback1<T>(IntPtr ctx, ref FfiResult result, T arg);

    public static void RegisterApp(AppInfo appInfo, Action<FfiResult> cb) {
        var ctx = GCHandle.ToIntPtr(GCHandle.Alloc(cb));
        register_app(ref appInfo, ctx, new Callback0(Call0));
    }

    public static void GetAppId(AppInfo appInfo, Action<FfiResult, int> cb) {
        var ctx = GCHandle.ToIntPtr(GCHandle.Alloc(cb));
        get_app_id(ref appInfo, ctx, new Callback1<int>(Call1<int>));
    }

    public static void GetAppName(AppInfo appInfo, Action<FfiResult, String> cb) {
        var ctx = GCHandle.ToIntPtr(GCHandle.Alloc(cb));
        get_app_name(ref appInfo, ctx, new Callback1<String>(Call1<String>));
    }

    // ---------------------

    private static void Call0(IntPtr ctx, ref FfiResult result) {
        var handle = GCHandle.FromIntPtr(ctx);
        var cb = (Action<FfiResult>) handle.Target;
        cb(result);
        handle.Free();
    }

    private static void Call1<T>(IntPtr ctx, ref FfiResult result, T arg) {
        var handle = GCHandle.FromIntPtr(ctx);
        var cb = (Action<FfiResult, T>) handle.Target;
        cb(result, arg);
        handle.Free();
    }

    [DllImport("backend")]
    private static extern void register_app(ref AppInfo appInfo, IntPtr ctx, Callback0 o_cb);

    [DllImport("backend")]
    private static extern void get_app_id(ref AppInfo appInfo, IntPtr ctx, Callback1<int> o_cb);

    [DllImport("backend")]
    private static extern void get_app_name(ref AppInfo appInfo, IntPtr ctx, Callback1<String> o_cb);
}
