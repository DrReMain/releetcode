plugins {
    kotlin("jvm") version "1.9.22"
    id("me.champeau.jmh") version "0.7.2"
}

group = "com.releetcode"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
    implementation(kotlin("stdlib"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(8)
}

jmh {
    benchmarkMode.set(listOf("avgt"))
    timeUnit.set("ns")
    warmupIterations.set(1)
    iterations.set(3)
    fork.set(1)
}
