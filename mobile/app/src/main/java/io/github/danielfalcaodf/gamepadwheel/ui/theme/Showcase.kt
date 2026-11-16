package io.github.danielfalcaodf.gamepadwheel.ui.theme

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.github.danielfalcaodf.gamepadwheel.R

/**
 * Component catalogue, to check the theme against the design canvas. Not a product screen:
 * phase 0 ships the visual foundation only.
 */
@Composable
fun Showcase() {
    Surface(Modifier.fillMaxSize()) {
        Column(
            modifier =
                Modifier
                    .safeDrawingPadding()
                    .verticalScroll(rememberScrollState())
                    .padding(Tokens.SPACE_XXL.dp),
            verticalArrangement = Arrangement.spacedBy(Tokens.SPACE_XL.dp),
        ) {
            Text(stringResource(R.string.app_name), style = MaterialTheme.typography.headlineMedium)
            Text(
                stringResource(R.string.showcase_intro),
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )

            Section(R.string.showcase_colors)
            FlowRow(
                horizontalArrangement = Arrangement.spacedBy(Tokens.SPACE_LG.dp),
                verticalArrangement = Arrangement.spacedBy(Tokens.SPACE_LG.dp),
            ) {
                swatches.forEach { (name, color) ->
                    Column(verticalArrangement = Arrangement.spacedBy(Tokens.SPACE_XS.dp)) {
                        Box(
                            Modifier
                                .size(width = 96.dp, height = 48.dp)
                                .background(color, MaterialTheme.shapes.small)
                                .border(
                                    BorderStroke(1.dp, GwColors.cardBorder),
                                    MaterialTheme.shapes.small,
                                ),
                        )
                        Text(name, style = GwType.mono, color = GwColors.text2)
                    }
                }
            }

            Section(R.string.showcase_type)
            textStyles.forEach { (name, style) ->
                Text(stringResource(R.string.showcase_sample), style = style)
                Text(name, style = GwType.mono, color = GwColors.text2)
            }

            Section(R.string.showcase_buttons)
            FlowRow(horizontalArrangement = Arrangement.spacedBy(Tokens.SPACE_LG.dp)) {
                GwButton(stringResource(R.string.showcase_button_primary), onClick = {})
                GwButton(
                    stringResource(R.string.showcase_button_secondary),
                    onClick = {},
                    kind = ButtonKind.Secondary,
                )
                GwButton(
                    stringResource(R.string.showcase_button_ghost),
                    onClick = {},
                    kind = ButtonKind.Ghost,
                )
                GwButton(
                    stringResource(R.string.showcase_button_disabled),
                    onClick = {},
                    enabled = false,
                )
            }

            Section(R.string.showcase_badges)
            FlowRow(
                horizontalArrangement = Arrangement.spacedBy(Tokens.SPACE_MD.dp),
                verticalArrangement = Arrangement.spacedBy(Tokens.SPACE_MD.dp),
            ) {
                StatusBadge(Tone.Off, stringResource(R.string.showcase_badge_off))
                StatusBadge(Tone.Info, stringResource(R.string.showcase_badge_info))
                StatusBadge(Tone.Ok, stringResource(R.string.showcase_badge_ok))
                StatusBadge(Tone.Warn, stringResource(R.string.showcase_badge_warn))
                StatusBadge(Tone.Err, stringResource(R.string.showcase_badge_err))
            }
        }
    }
}

@Composable
private fun Section(title: Int) {
    Text(
        stringResource(title),
        style = MaterialTheme.typography.titleMedium,
        modifier = Modifier.padding(top = Tokens.SPACE_XL.dp),
    )
}

private val swatches =
    listOf(
        "bg" to GwColors.bg,
        "surface" to GwColors.surface,
        "surface_2" to GwColors.surface2,
        "border" to GwColors.border,
        "text" to GwColors.text,
        "text_2" to GwColors.text2,
        "accent" to GwColors.accent,
        "ok" to GwColors.ok,
        "warn" to GwColors.warn,
        "err" to GwColors.err,
        "neutral" to GwColors.neutral,
        "dark_screen" to GwColors.darkScreen,
    )

private val textStyles: List<Pair<String, TextStyle>> =
    listOf(
        "brand" to GwType.brand,
        "screen_title" to GwType.screenTitle,
        "card_title" to GwType.cardTitle,
        "body" to GwType.body,
        "label" to GwType.label,
        "help" to GwType.help,
        "caption" to GwType.caption,
        "button" to GwType.button,
        "mono" to GwType.mono,
        "number" to GwType.number,
        "number_large" to GwType.numberLarge,
    )

@Preview(widthDp = 844, heightDp = 390)
@Composable
private fun ShowcasePreview() {
    GwTheme { Showcase() }
}
