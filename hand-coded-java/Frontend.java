import java.util.Arrays;

class Frontend {
    public static void main(String args[]) {
        Key appKey = new Key();
        appKey.bytes = new byte[] { 1, 2, 3, 5, 7, 11, 13, 17 };

        AppInfo app = new AppInfo();
        app.id = 1234;
        app.name = "Unique-App";
        app.key = appKey;

        NativeBindings.registerApp(app, (result) -> {
            System.out.println("- Java: registerApp() [" + result.error + "]");
        });

        // ---

        NativeBindings.getAppId(app, (result, arg) -> {
            System.out.println("- Java: getAppId(): " + arg);
        });

        // ---

        NativeBindings.getAppName(app, (result, arg) -> {
            System.out.println("- Java: getAppName(): " + arg);
        });

        // ---

        NativeBindings.getAppKey(app, (result, arg) -> {
            System.out.println("- Java: getAppKey(): " + Arrays.toString(arg.bytes));
        });

        // ---

        NativeBindings.randomNumbers((result, arg) -> {
            System.out.println("- Java: randomNumbers(): " + Arrays.toString(arg));
        });

        // ---

        NativeBindings.randomKeys((result, arg) -> {
            System.out.println("- Java: randomKeys():");
            for (int i = 0; i < arg.length; ++i) {
                System.out.println("    " + i + ": " + Arrays.toString(arg[i].bytes));
            }
        });

        // ---

        NativeBindings.getAppInfo(app, (result, id, name, key) -> {
            System.out.println("- Java: getAppInfo(): { id: " + id
                               + ", name: " + name
                               + ", key: " + Arrays.toString(key.bytes)
                               + " }");
        });

        // ---

        NativeBindings.createAccount("my_locator", "my_password",
            (result, app_info) -> {
                System.out.println(
                      "- Java: createAccount() [connect]: { id: " + app_info.id
                    + ", name: " + app_info.name
                    + ", key: " + Arrays.toString(app_info.key.bytes)
                    + " }"
                );
            },
            (result) -> {
                System.out.println("- Java: createAccount() [disconnect]");
            }
        );

        // ---

        byte[] data1 = new byte[] { 0, 0, 0, 0, 0, 0, 0, 0 };
        byte[] data2 = new byte[] { 1, 1, 1, 2, 1, 1, 2, 1 };

        NativeBindings.verifySignature(data1, (result) -> {
            System.out.println("- Java: verifySignature(): " + result.error);
        });

        NativeBindings.verifySignature(data2, (result) -> {
            System.out.println("- Java: verifySignature(): " + result.error);
        });

        // ---

        Key key0 = new Key();
        key0.bytes = new byte[] { 0, 0, 0, 0, 0, 0, 0, 0 };

        Key key1 = new Key();
        key1.bytes = new byte[] { 1, 1, 1, 1, 1, 1, 1, 1 };

        Key key2 = new Key();
        key2.bytes = new byte[] { 2, 2, 2, 2, 2, 2, 2, 2 };

        Key[] keys = new Key[] { key0, key1, key2 };

        NativeBindings.verifyKeys(keys, (result) -> {
            System.out.println("- Java: verifyKeys()");
        });

        try { Thread.sleep(5000); } catch(InterruptedException e) {}
        System.out.println("- Java: Exiting Frontend");
    }
}
