package io.github.danielfalcaodf.gamepadwheel

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import io.github.danielfalcaodf.gamepadwheel.ui.theme.GwTheme
import io.github.danielfalcaodf.gamepadwheel.ui.theme.Showcase

/** Phase 0: shows the theme's component catalogue. No product screen exists yet. */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent { GwTheme { Showcase() } }
    }
}
