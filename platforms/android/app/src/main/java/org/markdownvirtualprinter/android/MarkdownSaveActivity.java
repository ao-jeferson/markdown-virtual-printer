package org.markdownvirtualprinter.android;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.widget.Toast;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.io.OutputStream;

public final class MarkdownSaveActivity extends Activity {
    private static final int CREATE_MARKDOWN_FILE = 1501;
    private String jobId;
    private String title;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        jobId = getIntent().getStringExtra(MarkdownPrintService.EXTRA_JOB_ID);
        title = getIntent().getStringExtra(Intent.EXTRA_TITLE);
        if (jobId == null) {
            finish();
            return;
        }
        File pending = MarkdownPrintService.pendingFile(this, jobId);
        if (!pending.isFile()) {
            notifyService(false);
            Toast.makeText(this, R.string.save_failed, Toast.LENGTH_LONG).show();
            finish();
            return;
        }
        if (savedInstanceState == null) launchDocumentPicker();
    }

    private void launchDocumentPicker() {
        String fileName = title == null || title.trim().isEmpty() ? "Print job" : title.trim();
        if (!fileName.toLowerCase(java.util.Locale.ROOT).endsWith(".md")) fileName += ".md";
        Intent create = new Intent(Intent.ACTION_CREATE_DOCUMENT)
                .addCategory(Intent.CATEGORY_OPENABLE)
                .setType("text/markdown")
                .putExtra(Intent.EXTRA_TITLE, fileName);
        startActivityForResult(create, CREATE_MARKDOWN_FILE);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode != CREATE_MARKDOWN_FILE) return;
        if (resultCode != RESULT_OK || data == null || data.getData() == null) {
            notifyService(false);
            finish();
            return;
        }
        if (writeMarkdown(data.getData())) {
            notifyService(true);
        } else {
            Toast.makeText(this, R.string.save_failed, Toast.LENGTH_LONG).show();
            notifyService(false);
        }
        finish();
    }

    private boolean writeMarkdown(Uri destination) {
        File source = MarkdownPrintService.pendingFile(this, jobId);
        try (InputStream input = new FileInputStream(source);
             OutputStream output = getContentResolver().openOutputStream(destination)) {
            if (input == null || output == null) return false;
            byte[] buffer = new byte[32 * 1024];
            int count;
            while ((count = input.read(buffer)) != -1) output.write(buffer, 0, count);
            output.flush();
            return true;
        } catch (Exception error) {
            return false;
        }
    }

    private void notifyService(boolean success) {
        if (jobId == null) return;
        Intent finished = new Intent(MarkdownPrintService.ACTION_SAVE_FINISHED)
                .setPackage(getPackageName())
                .putExtra(MarkdownPrintService.EXTRA_JOB_ID, jobId)
                .putExtra(MarkdownPrintService.EXTRA_SAVE_SUCCESS, success);
        sendBroadcast(finished);
    }
}
