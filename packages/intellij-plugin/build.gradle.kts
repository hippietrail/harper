plugins {
    id("org.jetbrains.intellij") version "1.15.0"
    kotlin("jvm") version "1.9.0"
}

group = "com.harper"
version = "0.29.1"  // Match VS Code version

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    testImplementation("org.jetbrains.intellij.plugins:testFramework:1.1")
}

intellij {
    version.set("2023.2")
    type.set("IC") // IntelliJ Community
    plugins.set(listOf("org.intellij.plugins.markdown"))
}

tasks {
    buildSearchableOptions {
        enabled = false
    }
    
    patchPluginXml {
        sinceBuild.set("232")
        untilBuild.set("232.*")
    }
    
    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }
    
    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }
}