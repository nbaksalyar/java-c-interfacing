import java.math.BigInteger;

class Frontend extends BackendOnAuthReqCb {
    static {
        System.loadLibrary("frontend");
    }

    public void o_cb(FfiResult result, AuthResp auth_resp) {
        System.out.println("In Java: " + auth_resp.getP_msg());
        swigTakeOwnership();
    }

    public static void main(String args[]) {
        AppInfo info = new AppInfo();
        info.setP_id("Unique-007");
        info.setP_name("Unique-App");
        info.setP_vendor("Spandan");
        
        AuthReq req = new AuthReq();
        req.setP_info(info);
        req.setNeeds_own_container(true);
        req.setReq_id(BigInteger.valueOf(9876));

        Frontend m = new Frontend();
        NativeBindings.backend_on_auth_request_java(req, m);
        m.swigReleaseOwnership();

        System.out.println("Control back in the Java main thread.");

        // Currently C is not deep copying. Ideally after that this should be fine.
        // req.delete();
        // info.delete();

        try { Thread.sleep(3000); } catch(Exception e) {}

        req.delete();
        info.delete();

        System.out.println("Exiting Frontend");
    }
}
