public interface CreateAccountHandler {
    public void onConnect(FfiResult result, AppInfo app_info);
    public void onDisconnect(FfiResult result);
}
