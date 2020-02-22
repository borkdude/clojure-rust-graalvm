package borkdude.clojure.rust;

public class ClojureRust {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native String hello(String input);

    // The rest is just regular ol' Java!
    public static String helloFromRust() {
        System.loadLibrary("mylib"); // native-image doesn't support loading
                                     // libraries from static initializer blocks
        String output = hello("from Rust via Java");
        return output;
    }
}
