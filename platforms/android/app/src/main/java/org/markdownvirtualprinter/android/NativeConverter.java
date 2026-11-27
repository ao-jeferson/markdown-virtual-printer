package org.markdownvirtualprinter.android;

final class NativeConverter {
    static {
        System.loadLibrary("printer_core");
        System.loadLibrary("mdvp_jni");
    }

    private NativeConverter() {}

    static native String convertPdf(byte[] input, String title, boolean keepFormatting);
}
