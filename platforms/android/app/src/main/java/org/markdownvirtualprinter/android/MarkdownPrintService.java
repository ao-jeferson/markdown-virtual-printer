package org.markdownvirtualprinter.android;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.os.ParcelFileDescriptor;
import android.print.PrintAttributes;
import android.print.PrintJobInfo;
import android.print.PrinterCapabilitiesInfo;
import android.print.PrinterId;
import android.print.PrinterInfo;
import android.printservice.PrintJob;
import android.printservice.PrintService;
import android.printservice.PrinterDiscoverySession;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.security.MessageDigest;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.Set;

public final class MarkdownPrintService extends PrintService {
    static final String ACTION_SAVE_FINISHED =
            "org.markdownvirtualprinter.android.ACTION_SAVE_FINISHED";
    static final String EXTRA_JOB_ID = "job_id";
    static final String EXTRA_SAVE_SUCCESS = "save_success";

    private static final String CHANNEL_ID = "markdown_print_jobs";
    private static final int MAX_JOB_BYTES = 64 * 1024 * 1024;
    private static final String PREFERENCES = "printer_preferences";

    private final ExecutorService worker = Executors.newSingleThreadExecutor();
    private final Handler mainHandler = new Handler(Looper.getMainLooper());
    private final Object jobLock = new Object();
    private final Map<String, PrintJob> activeJobs = new ConcurrentHashMap<>();
    private final Map<String, Integer> notificationIds = new ConcurrentHashMap<>();
    private final Set<String> processingJobs = ConcurrentHashMap.newKeySet();
    private final Set<String> cancelledJobs = ConcurrentHashMap.newKeySet();

    private final BroadcastReceiver saveReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            String jobId = intent.getStringExtra(EXTRA_JOB_ID);
            if (jobId == null) return;
            PrintJob job = activeJobs.remove(jobId);
            cancelNotification(jobId);
            deletePendingFile(jobId);
            if (job == null) return;
            if (intent.getBooleanExtra(EXTRA_SAVE_SUCCESS, false)) {
                job.complete();
            } else {
                job.fail(getString(R.string.save_cancelled));
            }
        }
    };

    @Override
    public void onCreate() {
        super.onCreate();
        createNotificationChannel();
        IntentFilter filter = new IntentFilter(ACTION_SAVE_FINISHED);
        if (Build.VERSION.SDK_INT >= 33) {
            registerReceiver(saveReceiver, filter, Context.RECEIVER_NOT_EXPORTED);
        } else {
            registerReceiver(saveReceiver, filter);
        }
    }

    @Override
    protected void onConnected() {
        super.onConnected();
        List<PrintJob> jobs = getActivePrintJobs();
        if (jobs == null) return;
        for (PrintJob job : jobs) resumeActiveJob(job);
    }

    @Override
    protected PrinterDiscoverySession onCreatePrinterDiscoverySession() {
        return new MarkdownDiscoverySession();
    }

    @Override
    protected void onPrintJobQueued(PrintJob job) {
        PrintJobInfo info = job.getInfo();
        String jobId = job.getId().toString();
        String title = safeTitle(info.getLabel());
        ParcelFileDescriptor document;
        try {
            document = job.getDocument().getData();
        } catch (RuntimeException error) {
            job.fail(getString(R.string.job_failed));
            return;
        }
        if (document == null) {
            job.fail(getString(R.string.job_failed));
            return;
        }

        if (!job.start()) {
            try {
                document.close();
            } catch (IOException ignored) {
                // The job is no longer active, so there is nothing else to report.
            }
            return;
        }
        boolean keepFormatting = getSharedPreferences(PREFERENCES, MODE_PRIVATE)
                .getBoolean("keep_formatting", true);
        activeJobs.put(jobId, job);
        processingJobs.add(jobId);
        worker.execute(() -> convertAndNotify(job, jobId, title, document, keepFormatting));
    }

    @Override
    protected void onRequestCancelPrintJob(PrintJob job) {
        String jobId = job.getId().toString();
        synchronized (jobLock) {
            if (processingJobs.contains(jobId)) cancelledJobs.add(jobId);
            activeJobs.remove(jobId);
            cancelNotification(jobId);
            deletePendingFile(jobId);
            job.cancel();
        }
    }

    @Override
    public void onDestroy() {
        unregisterReceiver(saveReceiver);
        worker.shutdownNow();
        super.onDestroy();
    }

    private void convertAndNotify(
            PrintJob job, String jobId, String title,
            ParcelFileDescriptor document, boolean keepFormatting) {
        try (InputStream input = new ParcelFileDescriptor.AutoCloseInputStream(document)) {
            byte[] pdf = readBounded(input);
            String markdown = NativeConverter.convertPdf(pdf, title, keepFormatting);
            if (markdown == null || markdown.startsWith("ERROR:")) {
                String error = markdown == null ? "" : markdown.substring("ERROR:".length()).trim();
                throw new IOException(error.contains("larger than 64 MiB")
                        ? getString(R.string.job_too_large)
                        : getString(R.string.job_failed));
            }
            if (cancelledJobs.contains(jobId)) return;
            File pending = pendingFile(jobId);
            File parent = pending.getParentFile();
            if (parent == null || (!parent.exists() && !parent.mkdirs())) {
                throw new IOException(getString(R.string.job_failed));
            }
            try (FileOutputStream output = new FileOutputStream(pending)) {
                output.write(markdown.getBytes(java.nio.charset.StandardCharsets.UTF_8));
            }
            if (cancelledJobs.contains(jobId)) {
                deletePendingFile(jobId);
                return;
            }
            postSaveNotification(jobId, title);
        } catch (Exception error) {
            deletePendingFile(jobId);
            String reason = getString(R.string.job_failed);
            if (getString(R.string.notifications_required).equals(error.getMessage())) {
                reason = getString(R.string.notifications_required);
            } else if (getString(R.string.job_too_large).equals(error.getMessage())) {
                reason = getString(R.string.job_too_large);
            }
            String failureReason = reason;
            mainHandler.post(() -> {
                activeJobs.remove(jobId);
                if (!job.isCancelled()) job.fail(failureReason);
            });
        } finally {
            processingJobs.remove(jobId);
            cancelledJobs.remove(jobId);
        }
    }

    private void resumeActiveJob(PrintJob job) {
        String jobId = job.getId().toString();
        activeJobs.put(jobId, job);
        if (job.isQueued() && !job.start()) return;
        if (!job.isStarted() || !processingJobs.add(jobId)) return;

        String title = safeTitle(job.getInfo().getLabel());
        File pending = pendingFile(jobId);
        if (pending.isFile()) {
            try {
                postSaveNotification(jobId, title);
            } catch (IOException error) {
                activeJobs.remove(jobId);
                job.fail(getString(R.string.notifications_required));
            } finally {
                processingJobs.remove(jobId);
            }
            return;
        }

        ParcelFileDescriptor document;
        try {
            document = job.getDocument().getData();
        } catch (RuntimeException error) {
            processingJobs.remove(jobId);
            activeJobs.remove(jobId);
            job.fail(getString(R.string.job_failed));
            return;
        }
        if (document == null) {
            processingJobs.remove(jobId);
            activeJobs.remove(jobId);
            job.fail(getString(R.string.job_failed));
            return;
        }
        boolean keepFormatting = getSharedPreferences(PREFERENCES, MODE_PRIVATE)
                .getBoolean("keep_formatting", true);
        worker.execute(() -> convertAndNotify(job, jobId, title, document, keepFormatting));
    }

    private byte[] readBounded(InputStream input) throws IOException {
        ByteArrayOutputStream result = new ByteArrayOutputStream();
        byte[] buffer = new byte[32 * 1024];
        int total = 0;
        int count;
        while ((count = input.read(buffer)) != -1) {
            total += count;
            if (total > MAX_JOB_BYTES) throw new IOException(getString(R.string.job_too_large));
            result.write(buffer, 0, count);
        }
        return result.toByteArray();
    }

    private void postSaveNotification(String jobId, String title) throws IOException {
        NotificationManager manager = getSystemService(NotificationManager.class);
        if (manager == null || !manager.areNotificationsEnabled()) {
            throw new IOException(getString(R.string.notifications_required));
        }
        Intent openSave = new Intent(this, MarkdownSaveActivity.class)
                .putExtra(EXTRA_JOB_ID, jobId)
                .putExtra(Intent.EXTRA_TITLE, title);
        PendingIntent pendingIntent = PendingIntent.getActivity(
                this, jobId.hashCode(), openSave,
                PendingIntent.FLAG_UPDATE_CURRENT | PendingIntent.FLAG_IMMUTABLE);
        int notificationId = jobId.hashCode();
        Notification notification = new Notification.Builder(this, CHANNEL_ID)
                .setSmallIcon(android.R.drawable.ic_menu_save)
                .setContentTitle(getString(R.string.notification_title))
                .setContentText(getString(R.string.notification_body))
                .setContentIntent(pendingIntent)
                .setAutoCancel(true)
                .build();
        synchronized (jobLock) {
            if (cancelledJobs.contains(jobId)) return;
            notificationIds.put(jobId, notificationId);
            manager.notify(notificationId, notification);
        }
    }

    private void createNotificationChannel() {
        if (Build.VERSION.SDK_INT < 26) return;
        NotificationChannel channel = new NotificationChannel(
                CHANNEL_ID, getString(R.string.app_name), NotificationManager.IMPORTANCE_DEFAULT);
        channel.setDescription(getString(R.string.notification_body));
        NotificationManager manager = getSystemService(NotificationManager.class);
        if (manager != null) manager.createNotificationChannel(channel);
    }

    private void cancelNotification(String jobId) {
        Integer id = notificationIds.remove(jobId);
        if (id != null) getSystemService(NotificationManager.class).cancel(id);
    }

    private void deletePendingFile(String jobId) {
        pendingFile(jobId).delete();
    }

    static File pendingFile(Context context, String jobId) {
        return new File(new File(context.getCacheDir(), "pending-markdown"), fileKey(jobId) + ".md");
    }

    private File pendingFile(String jobId) {
        return pendingFile(this, jobId);
    }

    private static String fileKey(String jobId) {
        try {
            byte[] digest = MessageDigest.getInstance("SHA-256")
                    .digest(jobId.getBytes(java.nio.charset.StandardCharsets.UTF_8));
            StringBuilder value = new StringBuilder(digest.length * 2);
            for (byte b : digest) value.append(String.format(java.util.Locale.ROOT, "%02x", b));
            return value.toString();
        } catch (Exception error) {
            throw new IllegalStateException("SHA-256 unavailable", error);
        }
    }

    private String safeTitle(CharSequence source) {
        String value = source == null ? "Print job" : source.toString().trim();
        value = value.replaceAll("[\\\\/:*?\"<>|\\p{Cntrl}]", "_").trim();
        if (value.isEmpty()) value = "Print job";
        if (value.toLowerCase(java.util.Locale.ROOT).endsWith(".md")) {
            value = value.substring(0, value.length() - 3);
        }
        return value;
    }

    private final class MarkdownDiscoverySession extends PrinterDiscoverySession {
        private final PrinterId printerId = generatePrinterId("markdown-virtual-printer");
        private PrinterInfo printer;

        @Override
        public void onStartPrinterDiscovery(List<PrinterId> priorityList) {
            PrinterCapabilitiesInfo capabilities = new PrinterCapabilitiesInfo.Builder(printerId)
                    .addMediaSize(PrintAttributes.MediaSize.ISO_A4, true)
                    .addResolution(new PrintAttributes.Resolution("mdvp-300", "300 dpi", 300, 300), true)
                    .setColorModes(PrintAttributes.COLOR_MODE_MONOCHROME,
                            PrintAttributes.COLOR_MODE_MONOCHROME)
                    .setDuplexModes(PrintAttributes.DUPLEX_MODE_NONE,
                            PrintAttributes.DUPLEX_MODE_NONE)
                    .build();
            printer = new PrinterInfo.Builder(
                    printerId, getString(R.string.app_name), PrinterInfo.STATUS_IDLE)
                    .setCapabilities(capabilities)
                    .build();
            addPrinters(Collections.singletonList(printer));
        }

        @Override
        public void onStopPrinterDiscovery() {}

        @Override
        public void onValidatePrinters(List<PrinterId> printerIds) {
            if (printer != null && printerIds.contains(printerId)) {
                addPrinters(Collections.singletonList(printer));
            }
        }

        @Override
        public void onStartPrinterStateTracking(PrinterId printerId) {
            if (printer != null && this.printerId.equals(printerId)) {
                addPrinters(Collections.singletonList(printer));
            }
        }

        @Override
        public void onStopPrinterStateTracking(PrinterId printerId) {}

        @Override
        public void onDestroy() {
            printer = null;
        }
    }
}
