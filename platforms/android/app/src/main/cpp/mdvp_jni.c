#include <jni.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

#define MAX_JOB_BYTES ((size_t)64 * 1024 * 1024)

extern intptr_t mdvp_convert_job(
    const uint8_t *input,
    size_t input_len,
    const uint8_t *content_type,
    size_t content_type_len,
    const uint8_t *title,
    size_t title_len,
    uint32_t mode,
    uint8_t *output,
    size_t output_capacity);

static jstring error_string(JNIEnv *env, const char *message) {
    return (*env)->NewStringUTF(env, message);
}

static size_t utf16_utf8_size(const jchar *value, jsize length) {
    size_t size = 0;
    for (jsize i = 0; i < length; i++) {
        uint32_t codepoint = value[i];
        if (codepoint >= 0xD800 && codepoint <= 0xDBFF
                && i + 1 < length
                && value[i + 1] >= 0xDC00 && value[i + 1] <= 0xDFFF) {
            codepoint = 0x10000 + ((codepoint - 0xD800) << 10)
                    + (value[++i] - 0xDC00);
        } else if (codepoint >= 0xD800 && codepoint <= 0xDFFF) {
            codepoint = 0xFFFD;
        }
        size += codepoint <= 0x7F ? 1 : (codepoint <= 0x7FF ? 2 : (codepoint <= 0xFFFF ? 3 : 4));
    }
    return size;
}

static size_t write_utf8(uint32_t codepoint, char *output) {
    if (codepoint <= 0x7F) {
        output[0] = (char)codepoint;
        return 1;
    }
    if (codepoint <= 0x7FF) {
        output[0] = (char)(0xC0 | (codepoint >> 6));
        output[1] = (char)(0x80 | (codepoint & 0x3F));
        return 2;
    }
    if (codepoint <= 0xFFFF) {
        output[0] = (char)(0xE0 | (codepoint >> 12));
        output[1] = (char)(0x80 | ((codepoint >> 6) & 0x3F));
        output[2] = (char)(0x80 | (codepoint & 0x3F));
        return 3;
    }
    output[0] = (char)(0xF0 | (codepoint >> 18));
    output[1] = (char)(0x80 | ((codepoint >> 12) & 0x3F));
    output[2] = (char)(0x80 | ((codepoint >> 6) & 0x3F));
    output[3] = (char)(0x80 | (codepoint & 0x3F));
    return 4;
}

static char *utf16_to_utf8(const jchar *value, jsize length, size_t *output_len) {
    size_t capacity = utf16_utf8_size(value, length);
    char *output = (char *)malloc(capacity + 1);
    if (output == NULL) return NULL;
    size_t offset = 0;
    for (jsize i = 0; i < length; i++) {
        uint32_t codepoint = value[i];
        if (codepoint >= 0xD800 && codepoint <= 0xDBFF
                && i + 1 < length
                && value[i + 1] >= 0xDC00 && value[i + 1] <= 0xDFFF) {
            codepoint = 0x10000 + ((codepoint - 0xD800) << 10)
                    + (value[++i] - 0xDC00);
        } else if (codepoint >= 0xD800 && codepoint <= 0xDFFF) {
            codepoint = 0xFFFD;
        }
        offset += write_utf8(codepoint, output + offset);
    }
    output[offset] = '\0';
    *output_len = offset;
    return output;
}

static uint32_t read_utf8_codepoint(const uint8_t *value, size_t length, size_t *offset) {
    uint8_t first = value[(*offset)++];
    if (first < 0x80) return first;
    if ((first & 0xE0) == 0xC0 && *offset < length) {
        uint32_t codepoint = ((uint32_t)(first & 0x1F) << 6)
                | (uint32_t)(value[(*offset)++] & 0x3F);
        return codepoint >= 0x80 ? codepoint : 0xFFFD;
    }
    if ((first & 0xF0) == 0xE0 && *offset + 1 < length) {
        uint32_t codepoint = ((uint32_t)(first & 0x0F) << 12)
                | ((uint32_t)(value[(*offset)++] & 0x3F) << 6)
                | (uint32_t)(value[(*offset)++] & 0x3F);
        return codepoint >= 0x800 && !(codepoint >= 0xD800 && codepoint <= 0xDFFF)
                ? codepoint : 0xFFFD;
    }
    if ((first & 0xF8) == 0xF0 && *offset + 2 < length) {
        uint32_t codepoint = ((uint32_t)(first & 0x07) << 18)
                | ((uint32_t)(value[(*offset)++] & 0x3F) << 12)
                | ((uint32_t)(value[(*offset)++] & 0x3F) << 6)
                | (uint32_t)(value[(*offset)++] & 0x3F);
        return codepoint >= 0x10000 && codepoint <= 0x10FFFF ? codepoint : 0xFFFD;
    }
    return 0xFFFD;
}

static jstring utf8_to_jstring(JNIEnv *env, const uint8_t *value, size_t length) {
    jchar *utf16 = (jchar *)malloc((length + 1) * sizeof(jchar));
    if (utf16 == NULL) return NULL;
    size_t offset = 0;
    jsize count = 0;
    while (offset < length) {
        uint32_t codepoint = read_utf8_codepoint(value, length, &offset);
        if (codepoint <= 0xFFFF) {
            utf16[count++] = (jchar)codepoint;
        } else {
            codepoint -= 0x10000;
            utf16[count++] = (jchar)(0xD800 | (codepoint >> 10));
            utf16[count++] = (jchar)(0xDC00 | (codepoint & 0x3FF));
        }
    }
    jstring result = (*env)->NewString(env, utf16, count);
    free(utf16);
    return result;
}

JNIEXPORT jstring JNICALL
Java_org_markdownvirtualprinter_android_NativeConverter_convertPdf(
        JNIEnv *env, jclass klass, jbyteArray input, jstring title, jboolean keep_formatting) {
    (void)klass;
    if (input == NULL || title == NULL) return error_string(env, "ERROR: missing print job data");

    jsize input_length = (*env)->GetArrayLength(env, input);
    if (input_length < 0 || (size_t)input_length > MAX_JOB_BYTES) {
        return error_string(env, "ERROR: print job is larger than 64 MiB");
    }
    jbyte *input_bytes = (*env)->GetByteArrayElements(env, input, NULL);
    if (input_bytes == NULL) return error_string(env, "ERROR: could not read print job");
    jsize title_length = (*env)->GetStringLength(env, title);
    const jchar *title_chars = (*env)->GetStringChars(env, title, NULL);
    if (title_chars == NULL) {
        (*env)->ReleaseByteArrayElements(env, input, input_bytes, JNI_ABORT);
        return error_string(env, "ERROR: could not read print job title");
    }
    size_t title_bytes_length = 0;
    char *title_bytes = utf16_to_utf8(title_chars, title_length, &title_bytes_length);
    (*env)->ReleaseStringChars(env, title, title_chars);
    if (title_bytes == NULL) {
        (*env)->ReleaseByteArrayElements(env, input, input_bytes, JNI_ABORT);
        return error_string(env, "ERROR: not enough memory to convert print job");
    }

    static const uint8_t content_type[] = "application/pdf";
    uint32_t mode = keep_formatting == JNI_TRUE ? 0 : 1;
    intptr_t required = mdvp_convert_job(
            (const uint8_t *)input_bytes, (size_t)input_length,
            content_type, sizeof(content_type) - 1,
            (const uint8_t *)title_bytes, title_bytes_length,
            mode, NULL, 0);
    if (required < 0 || (size_t)required > MAX_JOB_BYTES) {
        free(title_bytes);
        (*env)->ReleaseByteArrayElements(env, input, input_bytes, JNI_ABORT);
        return error_string(env, "ERROR: Rust core could not convert this print job");
    }

    uint8_t *output = (uint8_t *)malloc((size_t)required + 1);
    if (output == NULL) {
        free(title_bytes);
        (*env)->ReleaseByteArrayElements(env, input, input_bytes, JNI_ABORT);
        return error_string(env, "ERROR: not enough memory to convert print job");
    }
    intptr_t written = mdvp_convert_job(
            (const uint8_t *)input_bytes, (size_t)input_length,
            content_type, sizeof(content_type) - 1,
            (const uint8_t *)title_bytes, title_bytes_length,
            mode, output, (size_t)required + 1);

    free(title_bytes);
    (*env)->ReleaseByteArrayElements(env, input, input_bytes, JNI_ABORT);
    if (written < 0 || written != required) {
        free(output);
        return error_string(env, "ERROR: Rust core could not finish converting this print job");
    }
    jstring result = utf8_to_jstring(env, output, (size_t)written);
    free(output);
    if (result == NULL) return error_string(env, "ERROR: not enough memory to return Markdown");
    return result;
}
