class Frontend {
    private native void BackendWithStringJNI(String cb_name);
    public void backend_with_string_cb(String result) {
        System.out.println("- Got in the Java callback: " + result);
    }
    public void backend_with_string() {
        BackendWithStringJNI("backend_with_string_cb");
    }

    public static void main(String args[]) {
        Frontend obj = new Frontend();

        System.out.println("- Calling from Java into native code which will callback into us, " +
                           "synchronously...");
        obj.backend_with_string();
        System.out.println("- Control returned back in Java");
    }

    static {
        System.loadLibrary("frontend");
    }
}
