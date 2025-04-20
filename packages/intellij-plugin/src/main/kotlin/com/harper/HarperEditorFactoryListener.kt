package com.harper

import com.intellij.openapi.editor.EditorFactory
import com.intellij.openapi.editor.EditorFactoryListener
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.editor.event.EditorFactoryEvent

class HarperEditorFactoryListener : EditorFactoryListener {
    override fun editorCreated(event: EditorFactoryEvent) {
        val editor = event.editor
        println("Editor created: ${editor.document.textLength} characters")
    }

    override fun editorReleased(event: EditorFactoryEvent) {
        val editor = event.editor
        println("Editor released: ${editor.document.textLength} characters")
    }

    companion object {
        fun register() {
            EditorFactory.getInstance().addListener(HarperEditorFactoryListener())
        }
    }
}