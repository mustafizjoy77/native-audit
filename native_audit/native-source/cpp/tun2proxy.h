#ifndef TUN2PROXY_H
#define TUN2PROXY_H

#include <jni.h>

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT jint JNICALL Java_com_github_shadowsocks_bg_Tun2proxy_run(JNIEnv *, jclass, jstring, jchar);
JNIEXPORT jint JNICALL Java_com_github_shadowsocks_bg_Tun2proxy_stop(JNIEnv *, jclass);

#ifdef __cplusplus
}
#endif

#endif // TUN2PROXY_H
