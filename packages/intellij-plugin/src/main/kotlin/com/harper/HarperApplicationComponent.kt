package com.harper

import com.intellij.openapi.components.ApplicationComponent
import com.intellij.openapi.project.Project
import com.intellij.openapi.project.ProjectManager
import com.intellij.openapi.project.ProjectManagerListener

class HarperApplicationComponent : ApplicationComponent, ProjectManagerListener {
    override fun initComponent() {
        println("Harper LSP Plugin initialized")
        ProjectManager.getInstance().addProjectManagerListener(this)
    }

    override fun projectOpened(project: Project) {
        println("Project opened: ${project.name}")
    }

    override fun projectClosed(project: Project) {
        println("Project closed: ${project.name}")
    }

    override fun disposeComponent() {
        println("Harper LSP Plugin disposed")
        ProjectManager.getInstance().removeProjectManagerListener(this)
    }

    override fun getComponentName(): String = "HarperApplicationComponent"
}