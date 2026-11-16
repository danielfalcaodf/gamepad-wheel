package io.github.danielfalcaodf.gamepadwheel.ui.theme

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp

enum class ButtonKind { Primary, Secondary, Ghost }

/** A button in one of the design variants, always at least the 56 dp thumb target tall. */
@Composable
fun GwButton(
    text: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    kind: ButtonKind = ButtonKind.Primary,
    enabled: Boolean = true,
) {
    val (container, content, border) =
        when (kind) {
            ButtonKind.Primary -> Triple(GwColors.accent, GwColors.onAccent, Color.Transparent)
            ButtonKind.Secondary -> Triple(GwColors.surface2, GwColors.text, GwColors.border)
            ButtonKind.Ghost -> Triple(Color.Transparent, GwColors.textLabel, Color.Transparent)
        }
    Button(
        onClick = onClick,
        modifier = modifier.heightIn(min = Tokens.SIZE_MOBILE_TOUCH_TARGET.dp),
        enabled = enabled,
        shape = RoundedCornerShape(Tokens.RADIUS_MOBILE_BUTTON.dp),
        colors =
            ButtonDefaults.buttonColors(
                containerColor = container,
                contentColor = content,
                disabledContainerColor = container.copy(alpha = container.alpha * Tokens.ALPHA_DISABLED),
                disabledContentColor = content.copy(alpha = Tokens.ALPHA_DISABLED),
            ),
        border = BorderStroke(1.dp, border),
        contentPadding = PaddingValues(horizontal = Tokens.SPACE_MOBILE_BUTTON_PADDING_X.dp),
    ) {
        Text(text, style = GwType.button)
    }
}

/** The meaning a status badge carries. [base] is tinted for the background, [content] is the text. */
enum class Tone(val base: Color, val content: Color, val backgroundAlpha: Float) {
    Ok(GwColors.ok, GwColors.ok, Tokens.ALPHA_BADGE_BG),
    Warn(GwColors.warn, GwColors.warn, Tokens.ALPHA_BADGE_BG),
    Err(GwColors.err, GwColors.errText, Tokens.ALPHA_BADGE_BG),
    Info(GwColors.accent, GwColors.accentText, Tokens.ALPHA_BADGE_BG_INFO),
    Off(GwColors.text2, GwColors.neutral, Tokens.ALPHA_BADGE_BG),
}

/** Status badge: tinted pill with a dot and a text, so status never relies on colour alone. */
@Composable
fun StatusBadge(tone: Tone, text: String, modifier: Modifier = Modifier) {
    Row(
        modifier =
            modifier
                .height(Tokens.SIZE_MOBILE_BADGE.dp)
                .background(tone.base.copy(alpha = tone.backgroundAlpha), CircleShape)
                .padding(horizontal = Tokens.SPACE_MOBILE_BADGE_PADDING_X.dp),
        horizontalArrangement = Arrangement.spacedBy(Tokens.SPACE_SM.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(Modifier.size(Tokens.SIZE_MOBILE_BADGE_DOT.dp).background(tone.content, CircleShape))
        Text(text, style = GwType.badge, color = tone.content)
    }
}
