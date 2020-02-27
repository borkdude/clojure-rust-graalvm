package borkdude.clojure.rust;

public class ClojureRust {
    // This declares that the static `getFreeMemory` method will be provided
    // a native library.
    private static native String getFreeMemoryRust(String unit);

    // The rest is just regular ol' Java!
    public static String getFreeMemory(String unit) {
        System.loadLibrary("mylib"); // native-image doesn't support loading
                                     // libraries from static initializer blocks
        String output = getFreeMemoryRust(unit);
        return output;
    }
}
