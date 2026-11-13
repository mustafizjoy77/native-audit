use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use jni::objects::{JClass, JString};
use jni::sys::{jchar, jint};
use jni::JNIEnv;

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "system" fn Java_com_github_shadowsocks_bg_Tun2proxy_run(
    mut env: JNIEnv,
    _class: JClass,
    cli_args: JString,
    tun_mtu: jchar,
) -> jint {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("Tun2proxyNative"),
    );

    let cli_str: String = match env.get_string(&cli_args) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };

    log::info!("tun2proxy starting with args: {}, mtu: {}", cli_str, tun_mtu);
    IS_RUNNING.store(true, Ordering::SeqCst);

    let args: Vec<String> = cli_str.split_whitespace().map(|s| s.to_string()).collect();
    let result = tun2proxy::run_with_args(&args, tun_mtu as u16);

    IS_RUNNING.store(false, Ordering::SeqCst);
    match result {
        Ok(_) => 0,
        Err(e) => {
            log::error!("tun2proxy exited with error: {:?}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_shadowsocks_bg_Tun2proxy_stop(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    log::info!("tun2proxy stopping requested");
    IS_RUNNING.store(false, Ordering::SeqCst);
    tun2proxy::stop();
    0
}
