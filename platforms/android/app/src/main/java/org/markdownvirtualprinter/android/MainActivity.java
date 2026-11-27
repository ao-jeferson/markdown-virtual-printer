package org.markdownvirtualprinter.android;

import android.Manifest;
import android.app.Activity;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.graphics.Typeface;
import android.os.Build;
import android.os.Bundle;
import android.provider.Settings;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.Switch;
import android.widget.TextView;

public final class MainActivity extends Activity {
    private static final int NOTIFICATION_PERMISSION_REQUEST = 1201;
    private static final String PREFERENCES = "printer_preferences";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        buildScreen();
        requestNotificationPermissionIfNeeded();
    }

    private void buildScreen() {
        int pad = (int) (24 * getResources().getDisplayMetrics().density);
        LinearLayout content = new LinearLayout(this);
        content.setOrientation(LinearLayout.VERTICAL);
        content.setPadding(pad, pad, pad, pad);

        TextView title = new TextView(this);
        title.setText(R.string.home_title);
        title.setTextSize(26);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        content.addView(title, matchWrap());

        TextView instructions = new TextView(this);
        instructions.setText(R.string.home_instructions);
        instructions.setTextSize(16);
        instructions.setPadding(0, pad / 2, 0, pad);
        content.addView(instructions, matchWrap());

        Switch formatting = new Switch(this);
        formatting.setText(R.string.keep_formatting);
        formatting.setChecked(getSharedPreferences(PREFERENCES, MODE_PRIVATE)
                .getBoolean("keep_formatting", true));
        content.addView(formatting, matchWrap());

        TextView summary = new TextView(this);
        summary.setText(R.string.formatting_summary);
        summary.setTextSize(14);
        summary.setPadding(0, 0, 0, pad);
        content.addView(summary, matchWrap());

        formatting.setOnCheckedChangeListener((button, checked) ->
                getSharedPreferences(PREFERENCES, MODE_PRIVATE).edit()
                        .putBoolean("keep_formatting", checked).apply());

        Button settings = new Button(this);
        settings.setText(R.string.enable_printer);
        settings.setOnClickListener(view -> {
            Intent intent = new Intent(Settings.ACTION_PRINT_SETTINGS);
            if (intent.resolveActivity(getPackageManager()) != null) {
                startActivity(intent);
            } else {
                startActivity(new Intent(Settings.ACTION_SETTINGS));
            }
        });
        content.addView(settings, matchWrap());
        setContentView(content);
    }

    private void requestNotificationPermissionIfNeeded() {
        if (Build.VERSION.SDK_INT >= 33
                && checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS)
                != PackageManager.PERMISSION_GRANTED) {
            requestPermissions(new String[]{Manifest.permission.POST_NOTIFICATIONS},
                    NOTIFICATION_PERMISSION_REQUEST);
        }
    }

    private LinearLayout.LayoutParams matchWrap() {
        return new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT);
    }
}
