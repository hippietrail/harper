package com.harper

import com.intellij.openapi.fileTypes.FileTypeFactory
import com.intellij.openapi.fileTypes.FileTypeManager
import com.intellij.openapi.fileTypes.FileType
import com.intellij.openapi.fileTypes.FileTypeRegistry
import com.intellij.openapi.vfs.VirtualFile

class HarperFileTypeFactory : FileTypeFactory() {
    override fun createFileTypes(registry: FileTypeRegistry) {
        // Register our file types
        registry.registerFileType(HarperFileType, "*.harper")
    }
}

object HarperFileType : FileType {
    override fun getName(): String = "HARPER"
    override fun getDescription(): String = "Harper configuration file"
    override fun getDefaultExtension(): String = "harper"
    override fun getIcon() = null // TODO: Add icon
    override fun isBinary(): Boolean = false
    override fun isReadOnly(): Boolean = false
    override fun getCharset(file: VirtualFile, content: ByteArray): String = "UTF-8"
}