class Frontend {
    private native void BackendWithStringJNI(String cb_name);
    public void backend_with_string_cb(String result) {
        System.out.println("- Got in the Java callback: " + result);
    }
    public void backend_with_string() {
        BackendWithStringJNI("backend_with_string_cb");
    }

    private native void BackendWithStringAsyncJNI(String cb_name);
    public void backend_with_string_async_cb(String result) {
        System.out.println("- Got in the Java async callback: " + result);
    }
    public void backend_with_string_async() {
        BackendWithStringAsyncJNI("backend_with_string_async_cb");
    }

    public static void main(String args[]) {
        Frontend obj = new Frontend();

        System.out.println("- Calling from Java into native code which will callback into us, " +
                           "synchronously...");
        obj.backend_with_string();
        System.out.println("- Control returned back in Java");

        System.out.println("-");

        System.out.println("- Calling from Java into async native code which will callback into " +
                           "us, asynchronously...");
        obj.backend_with_string_async();
        System.out.println("- Control returned back in Java");
        try {
            Thread.sleep(5000);
        } catch (Exception e) {}
    }

    static {
        System.loadLibrary("frontend");
    }
}
