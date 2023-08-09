pub mod java_bridge {
    use jni::objects::*;
    use jni::JNIEnv;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_okx_RustJNI_init(env: JNIEnv, _class: JClass) {
        println!("rust-java-demo inited");
    }
}