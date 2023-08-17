package {{pkg_name}}

import android.os.Bundle
import android.app.NativeActivity
import android.content.Intent
import android.content.Context
import android.content.pm.PackageManager
import android.app.Activity
import android.app.Fragment
import android.view.View
import android.view.ViewGroup
import android.view.ViewGroup.LayoutParams
import android.widget.FrameLayout
import androidx.annotation.CallSuper
import androidx.core.app.ActivityCompat
import androidx.annotation.Keep

class MainActivity: NativeActivity() {

    private var containerLayout: ViewGroup? = null

    companion object {
        const val CONTENT_VIEW_ID = 10101010

        init {
            // This is necessary when any of the following happens:
            //     - GameActivity derived class calls to the native code before calling
            //       the super.onCreate() function.
            System.loadLibrary("game")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val frame = FrameLayout(this)
        frame.setId(CONTENT_VIEW_ID)
        setContentView(frame, LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT))

        if (savedInstanceState === null) {
            containerLayout = FrameLayout(this)
            containerLayout?.setLayoutParams(
                ViewGroup.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT
                )
            )
        }
    }
}
