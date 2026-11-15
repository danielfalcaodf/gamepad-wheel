plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
}

android {
    namespace = "io.github.danielfalcaodf.gamepadwheel"
    compileSdk = 37

    defaultConfig {
        applicationId = "io.github.danielfalcaodf.gamepadwheel"
        minSdk = 26
        targetSdk = 37
        versionCode = 1
        versionName = "0.1.0"
    }

    buildFeatures {
        compose = true
    }

    lint {
        warningsAsErrors = true
        // lifecycle-runtime ships lint checks built for another lint version; not ours to fix.
        disable += "ObsoleteLintCustomCheck"
    }
}

dependencies {
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.activity.compose)
    debugImplementation(libs.androidx.compose.ui.tooling)
    testImplementation(libs.junit)
}
