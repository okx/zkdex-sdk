pub mod java_bridge {
    use jni::objects::*;
    use jni::JNIEnv;
    use jni::sys::{jboolean, jstring};
    use crate::{hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_transfer, hash_withdraw, sign_limit_order, sign_liquidate, sign_signed_oracle_price, sign_transfer, sign_withdraw, verify_signature};


    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_verifySignature<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, sig_r: JString<'local>, sig_s: JString<'local>, pub_key: JString<'local>, msg: JString<'local>) -> jboolean {
        let sig_r: String = env.get_string(&sig_r).expect("Couldn't get java sig_r").into();
        let sig_s: String = env.get_string(&sig_s).expect("Couldn't get java sig_rs").into();
        let pub_key: String = env.get_string(&pub_key).expect("Couldn't get java pub_key").into();
        let msg: String = env.get_string(&msg).expect("Couldn't get java msg").into();
        let result = verify_signature(&sig_r, &sig_s, &pub_key, &msg).expect("Couldn't get verify_signature result");
        jboolean::from(result)
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signWithdraw<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, asset_id: JString<'local>, pri_key: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        let assert_id: String = env.get_string(&asset_id).expect("Couldn't get java asset_id").into();
        let pri_key: String = env.get_string(&pri_key).expect("Coludn't get java pri_key").into();
        match sign_withdraw(&json, &assert_id, &pri_key) {
            Ok(ret) => {
                let output = env.new_string(serde_json::to_string(&ret).unwrap()).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signTransfer<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, pri_key: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        let pri_key: String = env.get_string(&pri_key).expect("Coludn't get java pri_key").into();
        match sign_transfer(&json, &pri_key) {
            Ok(ret) => {
                let output = env.new_string(serde_json::to_string(&ret).unwrap()).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signLimitOrder<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, pri_key: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        let pri_key: String = env.get_string(&pri_key).expect("Coludn't get java pri_key").into();
        match sign_limit_order(&json, &pri_key) {
            Ok(ret) => {
                let output = env.new_string(serde_json::to_string(&ret).unwrap()).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signLiquidate<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, pri_key: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        let pri_key: String = env.get_string(&pri_key).expect("Coludn't get java pri_key").into();
        match sign_liquidate(&json, &pri_key) {
            Ok(ret) => {
                let output = env.new_string(serde_json::to_string(&ret).unwrap()).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signSignedOraclePrice<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, pri_key: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        let pri_key: String = env.get_string(&pri_key).expect("Coludn't get java pri_key").into();
        match sign_signed_oracle_price(&json, &pri_key) {
            Ok(ret) => {
                let output = env.new_string(serde_json::to_string(&ret).unwrap()).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashWithdraw<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, asset_id: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        let assert_id: String = env.get_string(&asset_id).expect("Couldn't get java asset_id").into();
        match hash_withdraw(&json, &assert_id) {
            Ok(ret) => {
                let output = env.new_string(ret).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashTransfer<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>, pri_key: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        match hash_transfer(&json) {
            Ok(ret) => {
                let output = env.new_string(ret).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashLimitOrder<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        match hash_limit_order(&json) {
            Ok(ret) => {
                let output = env.new_string(ret).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashLiquidate<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        match hash_liquidate(&json) {
            Ok(ret) => {
                let output = env.new_string(ret).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashSignedOraclePrice<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, json: JString<'local>) -> jstring {
        let json: String = env.get_string(&json).expect("Couldn't get java json").into();
        match hash_signed_oracle_price(&json) {
            Ok(ret) => {
                let output = env.new_string(ret).expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(e) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{e:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }
}