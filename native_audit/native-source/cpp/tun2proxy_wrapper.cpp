#include <jni.h>
#include <string>
#include <android/log.h>

#define TAG "Tun2proxyJNI"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, TAG, __VA_ARGS__)

extern "C" {

JNIEXPORT jint JNICALL
Java_com_github_shadowsocks_bg_Tun2proxy_run(JNIEnv *env, jclass clazz, jstring cliArgs, jchar tunMtu);

JNIEXPORT jint JNICALL
Java_com_github_shadowsocks_bg_Tun2proxy_stop(JNIEnv *env, jclass clazz);

}
