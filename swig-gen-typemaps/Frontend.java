import java.math.BigInteger;
import java.util.Arrays;

class Frontend {
    public static void main(String args[]) {
        AppInfo app = new AppInfo();
        app.setId(1234);
        app.setName("Unique-App");

        {
            Key key = new Key();
            key.setBytes(new byte[] { 1, 2, 3, 5, 7, 11, 13, 17 });
            app.setKey(key);
        }

        NativeBindings.registerApp(app, (result) -> {
            System.out.println("- Java: registerApp()");
        });

        // ---

        NativeBindings.getAppId(app, (result, res) -> {
            System.out.println("- Java: getAppId(): " + res);
        });

        // ---

        NativeBindings.getAppName(app, (result, res) -> {
            System.out.println("- Java: getAppName(): " + res);
        });

        // ---

        NativeBindings.getAppKey(app, (result, res) -> {
            System.out.println("- Java: getAppKey(): " + Arrays.toString(res.getBytes()));
        });

        // ---

        NativeBindings.randomNumbers((result, res) -> {
            System.out.println("- Java: randomNumbers(): " + Arrays.toString(res));
        });

        // ---

        NativeBindings.randomKeys((result, res) -> {
            System.out.println("- Java: randomKeys():");
            for (int i = 0; i < res.length; ++i) {
                System.out.println("    " + i + ": " + Arrays.toString(res[i].getBytes()));
            }
        });

        // ---

        NativeBindings.getAppInfo(app, (result, id, name, key) -> {
            System.out.println("- Java: getAppInfo(): { id: " + id
                               + ", name: " + name
                               + ", key: " + Arrays.toString(key.getBytes())
                               + " }");
        });

        // ---

        NativeBindings.createAccount("locator", "password",
            new CreateAccountHandler() {
                @Override
                public void onConnect(FfiResult result, AppInfo app_info) {
                    System.out.println(
                          "- Java: createAccount() [connect]: { id: " + app_info.getId()
                        + ", name: " + app_info.getName()
                        + ", key: " + Arrays.toString(app_info.getKey().getBytes())
                        + " }"
                    );
                }

                @Override
                public void onDisconnect(FfiResult result) {
                    System.out.println("- Java: createAccount() [disconnect]");
                }
            }
        );

        // ---

        // TODO: this currently doesnt' work correctly - the callback gets called
        // only once.
        NativeBindings.createAccount2("locator2", "password2", (result, event) -> {
            switch (event.getType()) {
                case NativeBindings.CREATE_ACCOUNT_CONNECT:
                    AppInfo app_info = event.getConnected().getApp_info();

                    System.out.println(
                          "- Java: createAccount2() [connect]: { id: " + app_info.getId()
                        + ", name: " + app_info.getName()
                        + ", key: " + Arrays.toString(app_info.getKey().getBytes())
                        + " }"
                    );

                    break;
                case NativeBindings.CREATE_ACCOUNT_DISCONNECT:
                    System.out.println("- Java: createAccount2() [disconnect]");
                    break;
            }
        });

        // ---

        byte[] data1 = new byte[] { 0, 0, 0, 0, 0, 0, 0, 0 };
        byte[] data2 = new byte[] { 1, 1, 1, 2, 1, 1, 2, 1 };

        NativeBindings.verifySignature(data1, (result) -> {
            System.out.println("- Java: verifySignature(): " + result.getError());
        });

        NativeBindings.verifySignature(data2, (result) -> {
            System.out.println("- Java: verifySignature(): " + result.getError());
        });

        // ---

        Key key0 = new Key();
        key0.setBytes(new byte[] { 0, 0, 0, 0, 0, 0, 0, 0 });

        Key key1 = new Key();
        key1.setBytes(new byte[] { 1, 1, 1, 1, 1, 1, 1, 1 });

        Key key2 = new Key();
        key2.setBytes(new byte[] { 2, 2, 2, 2, 2, 2, 2, 2 });

        Key[] keys = new Key[] { key0, key1, key2 };

        NativeBindings.verifyKeys(keys, (result) -> {
            System.out.println("- Java: verifyKeys()");
        });

        try { Thread.sleep(10000); } catch(Exception e) {}
        System.out.println("- Java: Exiting Frontend");
    }
}
