import java.math.BigInteger;


class Frontend {
    public static void main(String args[]) {
        AppInfo info = new AppInfo();
        info.setP_id("Unique-007");
        info.setP_name("Unique-App");
        info.setP_vendor("Spandan");

        AuthReq req = new AuthReq();
        req.setP_info(info);
        req.setNeeds_own_container(true);
        req.setReq_id(BigInteger.valueOf(9876));

        NativeBindings.backendOnAuthRequest(req, (result, auth_resp) -> {
            System.out.println("In Java: " + auth_resp.getP_msg());
        });

        /* Or without lambdas:
        NativeBindings.backend_on_auth_request(req, new Callback() {
            @Override
            public void call(FfiResult result, AuthResp auth_resp) {
                System.out.println("In Java (anon. class): " + auth_resp.getP_msg());
            }
        });
        */

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
