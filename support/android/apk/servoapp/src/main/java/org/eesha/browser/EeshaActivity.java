package org.eesha.browser;

import android.app.NativeActivity;
import android.os.Bundle;

public class EeshaActivity extends NativeActivity {
    static {
        System.loadLibrary("eesha");
    }
}
