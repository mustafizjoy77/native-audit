package com.github.shadowsocks.bg;

/**
 * JNI bridge for the official Rust tun2proxy native engine (libtun2proxy.so).
 * Exported native symbols: Java_com_github_shadowsocks_bg_Tun2proxy_run and _stop.
 */
public final class Tun2proxy {
    private static volatile boolean sLoaded = false;
    private static volatile String sLoadError = null;

    static {
        try {
            System.loadLibrary("tun2proxy");
            sLoaded = true;
        } catch (Throwable t) {
            sLoaded = false;
            sLoadError = t.getMessage();
        }
    }

    private Tun2proxy() {
    }

    public static boolean isNativeLoaded() {
        return sLoaded;
    }

    public static String getLoadError() {
        return sLoadError;
    }

    public static native int run(String cliArgs, char tunMtu);

    public static native int stop();
}
