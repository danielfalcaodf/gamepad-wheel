package io.github.danielfalcaodf.gamepadwheel

import org.junit.Assert.assertEquals
import org.junit.Test

class BannerTest {
    @Test
    fun bannerNamesTheApp() {
        assertEquals("gamepad-wheel: phase 0", bannerText("gamepad-wheel"))
    }
}
