using System;
using System.Threading;
using System.Runtime.InteropServices;

public class Frontend {
    public static void Main() {
        var key = new Key();
        key.bytes = new byte[] { 1, 2, 3, 5, 7, 11, 13, 17 };

        var app = new AppInfo();
        app.id = 1234;
        app.name = "Unique-App";
        app.key = key;

        NativeBindings.RegisterApp(app, (result) => {
            Console.WriteLine("- C#: RegisterApp(): " + result.error);
        });

        // ---

        NativeBindings.GetAppId(app, (result, res) => {
            Console.WriteLine("- C#: GetAppId(): " + result.error + ": " + res);
        });

        // ---

        NativeBindings.GetAppName(app, (result, res) => {
            Console.WriteLine("- C#: GetAppName(): " + result.error + ": " + res);
        });

        Thread.Sleep(5000);
    }
}
