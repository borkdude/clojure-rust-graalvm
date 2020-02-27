// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

use heim::memory as memory;
use heim::units::information;
use futures::executor;

async fn get_mem(unit: String) -> u64 {

    let mem = memory::memory().await.unwrap();
    match &unit[..] {
        "byte" => mem.free().get::<information::byte>(),
        "megabyte" => mem.free().get::<information::megabyte>(),
        "gigabyte" => mem.free().get::<information::gigabyte>(),
        _ => panic!("Unrecognized unit: {}", unit)
    }
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_borkdude_clojure_rust_ClojureRust_getFreeMemoryRust(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                             _class: JClass,
                                             unit: JString)
                                             -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let unit: String =
        env.get_string(unit).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.

    let free = executor::block_on(get_mem(unit));

    let output = env.new_string(format!("{}", free))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}
