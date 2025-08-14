package com.genymobile.gnirehtet;

import android.content.Context;
import android.util.Log;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Locale;

public class Logger {

    private static FileWriter writer;

    public static void init(Context ctx) {

        try {
            File root = ctx.getExternalFilesDir("logs");
            File logFile = new File(root, "gnirehtet_client.log");
            writer = new FileWriter(logFile, true);
        } catch (IOException e) {
            Log.e("gnirehtet", "logger init fail", e);
        }
    }

    public static void close() {
        if (writer != null) {
            try {
                writer.close();
                writer = null;
            } catch (IOException e) {
                Log.e("gnirehtet", "logger close fail", e);
            }
        }
    }


    public static void d(String tag, String message) {
        Log.d(tag, message);
        writeToFile("D", tag, message, null);
    }

    public static void i(String tag, String message) {
        Log.i(tag, message);
        writeToFile("I", tag, message, null);
    }

    public static void w(String tag, String message) {
        Log.w(tag, message);
        writeToFile("W", tag, message, null);
    }

    public static void w(String tag, String message, Throwable e) {
        Log.w(tag, message);
        writeToFile("W", tag, message, e);
    }

    public static void e(String tag, String message, Throwable e) {
        Log.e(tag, message, e);
        writeToFile("E", tag, message, e);
    }

    public static void e(String tag, String message) {
        Log.e(tag, message);
        writeToFile("E", tag, message, null);
    }

    private static void writeToFile(String level, String tag, String message, Throwable throwable) {
        if (writer == null)
            return;

        try {
            String timestamp = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.CHINA)
                                        .format(new Date());
            String logEntry = String.format("%s %s/%s: %s\n", timestamp, level, tag, message);
            writer.write(logEntry);

            if (throwable != null) {
                StringWriter sw = new StringWriter();
                PrintWriter pw = new PrintWriter(sw);
                throwable.printStackTrace(pw);
                writer.write(sw.toString());
                writer.write("\n");
                pw.close();
                sw.close();
            }

            writer.flush();
        } catch (IOException e) {
            Log.e("gnirehtet", "Failed to write to log file", e);
        }
    }
}
