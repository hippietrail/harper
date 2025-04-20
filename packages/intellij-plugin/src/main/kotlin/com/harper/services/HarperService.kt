package com.harper.services

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import java.io.File
import java.io.IOException
import java.util.concurrent.TimeUnit

@Service
class HarperService(project: Project) {
    private var languageServerProcess: Process? = null
    private val config = project.getService(HarperConfigService::class.java)

    init {
        println("HarperService initialized for project: ${project.name}")
        startLanguageServer()
    }

    fun restartLanguageServer() {
        stopLanguageServer()
        startLanguageServer()
    }

    private fun startLanguageServer() {
        val lsPath = config.languageServerPath
        if (lsPath == null || !File(lsPath).exists()) {
            throw IOException("Language server executable not found at: $lsPath")
        }

        try {
            val processBuilder = ProcessBuilder(lsPath)
            processBuilder.redirectOutput(ProcessBuilder.Redirect.INHERIT)
            processBuilder.redirectError(ProcessBuilder.Redirect.INHERIT)
            
            languageServerProcess = processBuilder.start()
            println("Language server started")
        } catch (e: IOException) {
            throw IOException("Failed to start language server: ${e.message}", e)
        }
    }

    private fun stopLanguageServer() {
        languageServerProcess?.let { process ->
            process.destroy()
            if (!process.waitFor(5, TimeUnit.SECONDS)) {
                process.destroyForcibly()
            }
            println("Language server stopped")
        }
        languageServerProcess = null
    }

    fun sayHello(): String {
        return "Hello from Harper LSP!"
    }
}