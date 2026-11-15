package io.github.danielfalcaodf.gamepadwheel

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview

/** Phase 0: an empty screen that only proves the toolchain and CI work. */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent { MaterialTheme { Surface { Banner() } } }
    }
}

fun bannerText(appName: String): String = "$appName: phase 0"

@Composable
fun Banner() {
    Text(bannerText(stringResource(R.string.app_name)))
}

@Preview
@Composable
private fun BannerPreview() {
    MaterialTheme { Banner() }
}
