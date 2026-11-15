package io.github.danielfalcaodf.gamepadwheel.ui.theme

import java.io.File
import java.lang.reflect.Modifier
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

/** Gradle runs unit tests with the module directory (`mobile/app`) as the working directory. */
class TokensTest {
    /** The flat JSON as `key -> value text`; one regex is enough, so no JSON library. */
    private val json: Map<String, String> =
        Regex(""""([a-z0-9_.]+)":\s*(?:"([^"]*)"|([0-9.]+))""")
            .findAll(File("../../docs/design/tokens.json").readText())
            .associate { it.groupValues[1] to it.groupValues[2].ifEmpty { it.groupValues[3] } }

    /** `COLOR_TEXT_2` -> `color_text_2`, with colours as `#RRGGBB` and numbers as written. */
    private val code: Map<String, String> =
        Tokens::class.java.declaredFields
            // Skip the object's INSTANCE and the `$stable` field the Compose compiler adds.
            .filter { Modifier.isStatic(it.modifiers) && it.name.all { c -> c.isUpperCase() || c.isDigit() || c == '_' } && it.name != "INSTANCE" }
            .associate { field ->
                val value = field.get(null)
                field.name.lowercase() to
                    if (value is Long) "#%06X".format(value and 0xFFFFFF) else value.toString()
            }

    @Test
    fun tokensMatchTheJsonSourceOfTruth() {
        assertTrue("tokens.json was not parsed", json.size > 100)
        assertEquals(json.mapKeys { it.key.replace('.', '_') }, code)
    }

    @Test
    fun everyTextStyleHasAFontFile() {
        val weightNames = mapOf("400" to "regular", "500" to "medium", "600" to "semibold", "700" to "bold")
        json.keys.filter { it.startsWith("text.") && it.endsWith(".family") }.forEach { key ->
            val family = json.getValue(key).lowercase().replace(' ', '_')
            val weight = weightNames.getValue(json.getValue(key.replace(".family", ".weight")))
            assertTrue("$key: no $family $weight", File("src/main/res/font/${family}_$weight.ttf").exists())
        }
    }
}
