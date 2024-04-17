#[cfg(feature = "java")]
pub mod java_bridge {
    use crate::unified::{
        unified_hash_liquidate, unified_hash_oracle_price, unified_hash_perpetual_trade,
        unified_hash_spot_trade, unified_hash_transfer, unified_hash_withdrawal,
        unified_sign_liquidate, unified_sign_oracle_price, unified_sign_perpetual_trade,
        unified_sign_spot_trade, unified_sign_transfer, unified_sign_withdrawal,
    };
    use crate::zkw::JubjubSignature;
    use crate::{
        hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_spot_limit_order,
        hash_spot_transfer, hash_spot_withdrawal, hash_transfer, hash_withdraw, is_on_curve,
        l1_sign, private_key_from_seed, private_key_to_pubkey_xy, pub_key_to_xy, sign,
        sign_limit_order, sign_liquidate, sign_signed_oracle_price, sign_spot_limit_order,
        sign_spot_transfer, sign_spot_withdrawal, sign_transfer, sign_withdraw, verify_signature,
    };
    use jni::objects::*;
    use jni::sys::{jboolean, jstring};
    use jni::JNIEnv;
    use serde::Serialize;
    use std::panic;

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_verifySignature<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        sig_r: JString<'local>,
        sig_s: JString<'local>,
        pub_key_x: JString<'local>,
        pub_key_y: JString<'local>,
        msg: JString<'local>,
    ) -> jboolean {
        let sig_r = env.get_string(&sig_r);
        let sig_s = env.get_string(&sig_s);
        let pub_key_x = env.get_string(&pub_key_x);
        let pub_key_y = env.get_string(&pub_key_y);
        let msg = env.get_string(&msg);
        let result = panic::catch_unwind(|| {
            let sig_r: String = sig_r.expect("Couldn't get java sig_r").into();
            let sig_s: String = sig_s.expect("Couldn't get java sig_s").into();
            let pub_key_x: String = pub_key_x.expect("Couldn't get java pub_key_x").into();
            let pub_key_y: String = pub_key_y.expect("Couldn't get java pub_key_x").into();
            let msg: String = msg.expect("Couldn't get java msg").into();
            let result = verify_signature(&sig_r, &sig_s, &pub_key_x, &pub_key_y, &msg)
                .expect("Couldn't get verify_signature result");
            jboolean::from(result)
        });

        match result {
            Ok(r) => r,
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                jboolean::from(false)
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signWithdraw<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_withdraw(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signTransfer<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_transfer(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signLimitOrder<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_limit_order(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signLiquidate<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_liquidate(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signSignedOraclePrice<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_signed_oracle_price(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashWithdraw<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_withdraw(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_hashTransfer<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_transfer(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_hashLimitOrder<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_limit_order(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_hashLiquidate<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_liquidate(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_hashSignedOraclePrice<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_signed_oracle_price(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_sign<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        private_key: JString<'local>,
        msg: JString<'local>,
    ) -> jstring {
        let private_key = env.get_string(&private_key);
        let msg = env.get_string(&msg);
        match panic::catch_unwind(|| {
            let msg: String = msg.expect("Couldn't get java msg").into();
            let private_key: String = private_key.expect("Couldn't get java json").into();
            sign(&private_key, &msg).expect("Couldn't sign msg")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
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
    pub extern "system" fn Java_com_okx_ZKDEX_ethSign<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        private_key: JString<'local>,
        msg: JString<'local>,
    ) -> jstring {
        let private_key = env.get_string(&private_key);
        let msg = env.get_string(&msg);
        match panic::catch_unwind(|| {
            let msg: String = msg.expect("Couldn't get java msg").into();
            let private_key: String = private_key.expect("Couldn't get java json").into();
            l1_sign(&msg, &private_key).expect("Couldn't sign msg")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
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
    pub extern "system" fn Java_com_okx_ZKDEX_privateKeyFromSeed<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        seed: JString<'local>,
    ) -> jstring {
        let seed = env.get_string(&seed);

        match panic::catch_unwind(|| {
            let seed: String = seed.expect("Couldn't get java seed").into();
            private_key_from_seed(seed.as_bytes()).expect("Couldn't derive private key from seed")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_isOnCurve<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        x: JString<'local>,
        y: JString<'local>,
    ) -> jboolean {
        let x = env.get_string(&x);
        let y = env.get_string(&y);
        match panic::catch_unwind(|| {
            let x: String = x.expect("Couldn't get java string x").into();
            let y: String = y.expect("Couldn't get java string x").into();
            is_on_curve(&x, &y).expect("Couldn't get verify xy is on curve")
        }) {
            Ok(ret) => jboolean::from(ret),
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                jboolean::from(false)
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_privateKeyToPublicKeyXY<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        private_key: JString<'local>,
    ) -> jstring {
        let private_key = env.get_string(&private_key);

        match panic::catch_unwind(|| {
            let private_key: String = private_key
                .expect("Couldn't get java private key string")
                .into();
            private_key_to_pubkey_xy(&private_key)
                .expect("Couldn't convert private key to public key xy")
        }) {
            Ok(ret) => {
                #[derive(Serialize)]
                struct XY {
                    x: String,
                    y: String,
                }
                let xy = XY { x: ret.0, y: ret.1 };
                let output = env
                    .new_string(serde_json::to_string(&xy).unwrap())
                    .expect("Couldn't create java string!");
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
    pub extern "system" fn Java_com_okx_ZKDEX_publicKeyToXY<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        public_key: JString<'local>,
    ) -> jstring {
        let public_key = env.get_string(&public_key);

        match panic::catch_unwind(|| {
            let public_key: String = public_key.expect("Couldn't get java string").into();
            pub_key_to_xy(&public_key).expect("Couldn't convert public key to xy")
        }) {
            Ok(ret) => {
                #[derive(Serialize)]
                struct XY {
                    x: String,
                    y: String,
                }
                let xy = XY { x: ret.0, y: ret.1 };
                let output = env
                    .new_string(serde_json::to_string(&xy).unwrap())
                    .expect("Couldn't create java string!");
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
    pub extern "system" fn Java_com_okx_ZKDEX_signSpotWithdrawal<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_spot_withdrawal(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }
    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signSpotTransfer<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_spot_transfer(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_signSpotLimitOrder<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            sign_spot_limit_order(&json, &pri_key).expect("Couldn get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_hashSpotWithdrawal<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_spot_withdrawal(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_hashSpotTransfer<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_spot_transfer(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_hashSpotLimitOrder<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            hash_spot_limit_order(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedSignWithdrawal<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            unified_sign_withdrawal(&json, &pri_key).expect("Couldn't get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedHashWithdrawal<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            unified_hash_withdrawal(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedSignTransfer<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            unified_sign_transfer(&json, &pri_key).expect("Couldn't get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedHashTransfer<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            unified_hash_transfer(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedSignSpotTrade<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key_a: JString<'local>,
        pri_key_b: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key_a = env.get_string(&pri_key_a);
        let pri_key_b = env.get_string(&pri_key_b);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key_a: String = pri_key_a.expect("Coludn't get java pri_key a").into();
            let pri_key_b: String = pri_key_b.expect("Coludn't get java pri_key b").into();
            unified_sign_spot_trade(&json, &pri_key_a, &pri_key_b)
                .expect("Couldn't get jubjubSignature")
        }) {
            Ok(ret) => {
                #[derive(Serialize)]
                struct ComposeSignature {
                    signature_a: JubjubSignature,
                    signature_b: JubjubSignature,
                }
                let c_sig = ComposeSignature {
                    signature_a: ret.0,
                    signature_b: ret.1,
                };
                let output = env
                    .new_string(serde_json::to_string(&c_sig).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedHashSpotTrade<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            unified_hash_spot_trade(&json).expect("Couldn't get hash")
        }) {
            Ok(ret) => {
                #[derive(Serialize)]
                struct ComposeHash {
                    hash_a: String,
                    hash_b: String,
                }
                let c_h = ComposeHash {
                    hash_a: ret.0,
                    hash_b: ret.1,
                };
                let output = env
                    .new_string(serde_json::to_string(&c_h).unwrap())
                    .expect("Couldn't create java string!");
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
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedSignPerpetualTrade<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key_a: JString<'local>,
        pri_key_b: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key_a = env.get_string(&pri_key_a);
        let pri_key_b = env.get_string(&pri_key_b);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key_a: String = pri_key_a.expect("Coludn't get java pri_key a").into();
            let pri_key_b: String = pri_key_b.expect("Coludn't get java pri_key b").into();
            unified_sign_perpetual_trade(&json, &pri_key_a, &pri_key_b)
                .expect("Couldn't get jubjubSignature")
        }) {
            Ok(ret) => {
                #[derive(Serialize)]
                struct ComposeSignature {
                    signature_a: JubjubSignature,
                    signature_b: JubjubSignature,
                }
                let c_sig = ComposeSignature {
                    signature_a: ret.0,
                    signature_b: ret.1,
                };
                let output = env
                    .new_string(serde_json::to_string(&c_sig).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedHashPerpetualTrade<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            unified_hash_perpetual_trade(&json).expect("Couldn't get hash")
        }) {
            Ok(ret) => {
                #[derive(Serialize)]
                struct ComposeHash {
                    hash_a: String,
                    hash_b: String,
                }
                let c_h = ComposeHash {
                    hash_a: ret.0,
                    hash_b: ret.1,
                };
                let output = env
                    .new_string(serde_json::to_string(&c_h).unwrap())
                    .expect("Couldn't create java string!");
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
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedSignOraclePrice<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            unified_sign_oracle_price(&json, &pri_key).expect("Couldn't get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedHashOraclePrice<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            unified_hash_oracle_price(&json).expect("Couldn't get hash")
        }) {
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
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedSignLiquidate<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
        pri_key: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        let pri_key = env.get_string(&pri_key);

        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            let pri_key: String = pri_key.expect("Coludn't get java pri_key").into();
            unified_sign_liquidate(&json, &pri_key).expect("Couldn't get jubjubSignature")
        }) {
            Ok(ret) => {
                let output = env
                    .new_string(serde_json::to_string(&ret).unwrap())
                    .expect("Couldn't create java string!");
                output.into_raw()
            }
            Err(err) => {
                env.exception_clear().expect("clear");
                env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                    .expect("throw");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_okx_ZKDEX_unifiedHashLiquidate<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        json: JString<'local>,
    ) -> jstring {
        let json = env.get_string(&json);
        match panic::catch_unwind(|| {
            let json: String = json.expect("Couldn't get java json").into();
            unified_hash_liquidate(&json).expect("Couldn't get hash")
        }) {
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
