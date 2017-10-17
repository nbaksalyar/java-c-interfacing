
public class NativeBindings {
    static {
        System.loadLibrary("frontend");
    }

    public static native void registerApp(AppInfo app, Callback cb);
    public static native void getAppId(AppInfo app, Callback_int cb);
    public static native void getAppName(AppInfo app, Callback_String cb);
    public static native void getAppKey(AppInfo app, Callback_Key cb);
    public static native void randomNumbers(Callback_array_int cb);
    public static native void randomKeys(Callback_array_Key cb);
    public static native void getAppInfo(AppInfo app, Callback_int_String_Key cb);

    public static native void createAccount(String locator,
                                            String password,
                                            Callback_AppInfo connectCb,
                                            Callback disconnectCb);

    public static native void verifySignature(byte[] data, Callback cb);
    public static native void verifyKeys(Key[] data, Callback cb);
}
