# clojure-rust-graalvm-native

An example of calling Rust from a Clojure program, which is then compiled with GraalVM to a native binary.

This repo is an adapted example of what is described in the README of the Rust
[jni](https://docs.rs/jni/0.14.0/jni/) library.

In `clojure/src-java` there is a Java static method which calls a Rust function
via JNI. We call this static method from Clojure.

## Build

To build this project, first build the Rust library.

``` shell
$ cd rust
$ cargo build --release
```

This will create a `libmydylib.so` (Linux) or `libmydylib.dylib` (macOS) in `target/release`.
Copy this file to the `clojure` directory:

``` shell
$ cp target/release/libmylib.dylib ../clojure
```

Now we will compile the Clojure project into an uberjar:

``` shell
$ cd ../clojure
$ lein uberjar
```

Let's test it:

``` shell
$ java -jar target/clj-rust-0.1.0-SNAPSHOT-standalone.jar
Hello from Clojure
Hello, from Rust via Java!
```

Now the final part. We will compile the uberjar to a native using GraalVM:

``` shell
$ /Users/borkdude/Downloads/graalvm-ce-java8-19.3.1/Contents/Home/bin/native-image \
  --initialize-at-build-time --no-server --no-fallback \
  -jar target/clj-rust-0.1.0-SNAPSHOT-standalone.jar -H:Name=clojure-rust
```

Does it work?

``` shell
$ time ./clojure-rust
Hello from Clojure
Hello, from Rust via Java!
./clojure-rust  0.00s user 0.00s system 48% cpu 0.018 total
```

Yes, and it's fast :-)

## License

Copyright © 2019 Michiel Borkent

Distributed under the EPL License, same as Clojure. See LICENSE.
