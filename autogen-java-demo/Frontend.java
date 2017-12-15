import net.maidsafe.safe_app.*;
import java.util.Arrays;

class Frontend {
    public static void main(String args[]) {
	NativeBindings.appExeFileStem(
				      (result, filename) -> {
					  System.out.println("- Java: " + filename);
				      }
				     );

	NativeBindings.appUnregistered(new byte[] {}, () -> {
		System.out.println("Java - disconnected");
	    }, (result, app) -> {
		System.out.println("Java - got the app (" + app + ")");		
	    });

        try { Thread.sleep(5000); } catch(InterruptedException e) {}
        System.out.println("- Java: Exiting Frontend");
    }
}
