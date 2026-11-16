package io.github.danielfalcaodf.gamepadwheel.ui.theme

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Shapes
import androidx.compose.material3.Typography
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp
import io.github.danielfalcaodf.gamepadwheel.R

/** The token colours as Compose [Color]s. Status colours are not part of the Material scheme. */
object GwColors {
    val bg = Color(Tokens.COLOR_BG)
    val surface = Color(Tokens.COLOR_SURFACE)
    val surface2 = Color(Tokens.COLOR_SURFACE_2)
    val border = Color(Tokens.COLOR_BORDER)
    val text = Color(Tokens.COLOR_TEXT)
    val text2 = Color(Tokens.COLOR_TEXT_2)
    val accent = Color(Tokens.COLOR_ACCENT)
    val ok = Color(Tokens.COLOR_OK)
    val warn = Color(Tokens.COLOR_WARN)
    val err = Color(Tokens.COLOR_ERR)
    val neutral = Color(Tokens.COLOR_NEUTRAL)
    val cardBorder = Color(Tokens.COLOR_CARD_BORDER)
    val selected = Color(Tokens.COLOR_SELECTED)
    val textLabel = Color(Tokens.COLOR_TEXT_LABEL)
    val accentText = Color(Tokens.COLOR_ACCENT_TEXT)
    val errText = Color(Tokens.COLOR_ERR_TEXT)
    val onAccent = Color(Tokens.COLOR_ON_ACCENT)
    val darkScreen = Color(Tokens.COLOR_DARK_SCREEN)
    val darkScreenText = Color(Tokens.COLOR_DARK_SCREEN_TEXT)
}

private val fontFamilies =
    mapOf(
        "Inter" to
            FontFamily(
                Font(R.font.inter_regular, FontWeight.W400),
                Font(R.font.inter_medium, FontWeight.W500),
                Font(R.font.inter_semibold, FontWeight.W600),
            ),
        "Space Grotesk" to
            FontFamily(
                Font(R.font.space_grotesk_semibold, FontWeight.W600),
                Font(R.font.space_grotesk_bold, FontWeight.W700),
            ),
        "JetBrains Mono" to
            FontFamily(
                Font(R.font.jetbrains_mono_regular, FontWeight.W400),
                Font(R.font.jetbrains_mono_semibold, FontWeight.W600),
            ),
    )

private fun style(family: String, weight: Int, size: Int) =
    TextStyle(
        fontFamily = fontFamilies.getValue(family),
        fontWeight = FontWeight(weight),
        fontSize = size.sp,
    )

/** The text styles of the design, by their token name. */
object GwType {
    val brand = style(Tokens.TEXT_BRAND_FAMILY, Tokens.TEXT_BRAND_WEIGHT, Tokens.TEXT_BRAND_SIZE)
    val screenTitle =
        style(
            Tokens.TEXT_SCREEN_TITLE_FAMILY,
            Tokens.TEXT_SCREEN_TITLE_WEIGHT,
            Tokens.TEXT_SCREEN_TITLE_SIZE,
        )
    val cardTitle =
        style(
            Tokens.TEXT_CARD_TITLE_FAMILY,
            Tokens.TEXT_CARD_TITLE_WEIGHT,
            Tokens.TEXT_CARD_TITLE_SIZE,
        )
    val body = style(Tokens.TEXT_BODY_FAMILY, Tokens.TEXT_BODY_WEIGHT, Tokens.TEXT_BODY_SIZE)
    val label = style(Tokens.TEXT_LABEL_FAMILY, Tokens.TEXT_LABEL_WEIGHT, Tokens.TEXT_LABEL_SIZE)
    val help = style(Tokens.TEXT_HELP_FAMILY, Tokens.TEXT_HELP_WEIGHT, Tokens.TEXT_HELP_SIZE)
    val caption =
        style(Tokens.TEXT_CAPTION_FAMILY, Tokens.TEXT_CAPTION_WEIGHT, Tokens.TEXT_CAPTION_SIZE)
            .copy(letterSpacing = 0.08.em)
    val button =
        style(Tokens.TEXT_BUTTON_FAMILY, Tokens.TEXT_BUTTON_WEIGHT, Tokens.TEXT_SIZE_MOBILE_BUTTON)
    val badge =
        style(Tokens.TEXT_BADGE_FAMILY, Tokens.TEXT_BADGE_WEIGHT, Tokens.TEXT_SIZE_MOBILE_BADGE)
    val mono = style(Tokens.TEXT_MONO_FAMILY, Tokens.TEXT_MONO_WEIGHT, Tokens.TEXT_MONO_SIZE)
    val number =
        style(Tokens.TEXT_NUMBER_FAMILY, Tokens.TEXT_NUMBER_WEIGHT, Tokens.TEXT_NUMBER_SIZE)
    val numberLarge =
        style(
            Tokens.TEXT_NUMBER_LARGE_FAMILY,
            Tokens.TEXT_NUMBER_LARGE_WEIGHT,
            Tokens.TEXT_NUMBER_LARGE_SIZE,
        )
}

// One accent only: every Material role that would otherwise default to a second hue is mapped
// to a neutral, so stock Material components cannot introduce a colour the design lacks.
private val colorScheme =
    darkColorScheme(
        primary = GwColors.accent,
        onPrimary = GwColors.onAccent,
        primaryContainer = GwColors.selected,
        onPrimaryContainer = GwColors.text,
        secondary = GwColors.neutral,
        onSecondary = GwColors.onAccent,
        secondaryContainer = GwColors.selected,
        onSecondaryContainer = GwColors.text,
        tertiary = GwColors.neutral,
        onTertiary = GwColors.onAccent,
        tertiaryContainer = GwColors.selected,
        onTertiaryContainer = GwColors.text,
        background = GwColors.bg,
        onBackground = GwColors.text,
        surface = GwColors.bg,
        onSurface = GwColors.text,
        surfaceVariant = GwColors.surface2,
        onSurfaceVariant = GwColors.text2,
        surfaceTint = GwColors.surface,
        surfaceContainerLowest = GwColors.bg,
        surfaceContainerLow = GwColors.surface,
        surfaceContainer = GwColors.surface,
        surfaceContainerHigh = GwColors.surface2,
        surfaceContainerHighest = GwColors.surface2,
        error = GwColors.err,
        onError = GwColors.onAccent,
        outline = GwColors.border,
        outlineVariant = GwColors.cardBorder,
    )

private val typography =
    Typography(
        headlineMedium = GwType.screenTitle,
        titleLarge = GwType.screenTitle.copy(fontSize = Tokens.TEXT_SIZE_MOBILE_TITLE.sp),
        titleMedium = GwType.cardTitle,
        bodyLarge = GwType.body,
        bodyMedium = GwType.body,
        bodySmall = GwType.help,
        labelLarge = GwType.button,
        labelMedium = GwType.label,
        labelSmall = GwType.caption,
    )

private val shapes =
    Shapes(
        extraSmall = RoundedCornerShape(Tokens.RADIUS_SEGMENT.dp),
        small = RoundedCornerShape(Tokens.RADIUS_CONTROL.dp),
        medium = RoundedCornerShape(Tokens.RADIUS_CARD.dp),
        large = RoundedCornerShape(Tokens.RADIUS_MOBILE_TILE.dp),
        extraLarge = RoundedCornerShape(Tokens.RADIUS_MODAL.dp),
    )

/** The app theme. Dark only: the design has no light theme and uses no dynamic colour. */
@Composable
fun GwTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = colorScheme,
        typography = typography,
        shapes = shapes,
        content = content,
    )
}
