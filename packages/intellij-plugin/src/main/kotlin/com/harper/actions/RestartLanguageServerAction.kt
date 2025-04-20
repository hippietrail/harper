package com.harper.actions

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.Messages
import com.harper.services.HarperService

class RestartLanguageServerAction : AnAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        
        val service = project.getService(HarperService::class.java)
        if (service == null) {
            Messages.showErrorDialog(project, "Harper service is not available", "Error")
            return
        }
        
        try {
            service.restartLanguageServer()
            Messages.showInfoMessage(project, "Language server restarted successfully", "Success")
        } catch (e: Exception) {
            Messages.showErrorDialog(project, "Failed to restart language server: ${e.message}", "Error")
        }
    }

    override fun update(e: AnActionEvent) {
        val project = e.project
        e.presentation.isEnabled = project != null
    }
}