package net.example.fkcloud

import android.content.Intent
import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import net.example.fkcloud.databinding.ActivityFolderPickerBinding
import net.example.fkcloud.databinding.ItemBreadcrumbBinding
import net.example.fkcloud.databinding.ItemDocumentRowBinding
import net.example.fkcloud.databinding.ItemFolderRowBinding
import java.util.Locale

/**
 * Navigable browser over the rmfakecloud document tree: tap a folder to go
 * in, tap a breadcrumb to jump back, tap "Select this folder" to return it
 * as the upload destination. Documents are shown for context only (browsing
 * the tree, not viewing/downloading — out of scope for this app).
 */
class FolderPickerActivity : AppCompatActivity() {

    companion object {
        const val EXTRA_FOLDER_ID = "folder_id"
        const val EXTRA_FOLDER_NAME = "folder_name"
    }

    private lateinit var binding: ActivityFolderPickerBinding
    private lateinit var store: SecureStore
    private val stack = mutableListOf<TreeEntry.Dir>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityFolderPickerBinding.inflate(layoutInflater)
        setContentView(binding.root)
        store = SecureStore(applicationContext)

        binding.buttonCancel.setOnClickListener {
            setResult(RESULT_CANCELED)
            finish()
        }
        binding.buttonSelect.setOnClickListener {
            val current = stack.last()
            setResult(
                RESULT_OK,
                Intent()
                    .putExtra(EXTRA_FOLDER_ID, current.id)
                    .putExtra(EXTRA_FOLDER_NAME, current.name)
            )
            finish()
        }

        binding.buttonSelect.isEnabled = false
        loadTree()
    }

    private fun loadTree() {
        binding.textStatus.text = getString(R.string.folder_picker_loading)
        val rootLabel = getString(R.string.folder_root)
        lifecycleScope.launch {
            val result = withContext(Dispatchers.IO) {
                runCatching {
                    Session(store).withAuth { client, token -> client.getDocumentTree(token, rootLabel) }
                }
            }
            result.fold(
                onSuccess = { root ->
                    binding.textStatus.visibility = View.GONE
                    binding.buttonSelect.isEnabled = true
                    stack.clear()
                    stack.add(root)
                    render()
                },
                onFailure = { e ->
                    binding.textStatus.text = getString(R.string.folder_picker_error, e.message ?: "?")
                }
            )
        }
    }

    private fun render() {
        renderBreadcrumb()
        renderEntries()
        binding.buttonSelect.text = getString(R.string.folder_picker_select)
    }

    private fun renderBreadcrumb() {
        binding.breadcrumbContainer.removeAllViews()
        stack.forEachIndexed { index, dir ->
            if (index > 0) {
                val separator = ItemBreadcrumbBinding.inflate(
                    layoutInflater, binding.breadcrumbContainer, false
                ).root
                separator.text = getString(R.string.folder_picker_chevron)
                separator.isClickable = false
                separator.isFocusable = false
                binding.breadcrumbContainer.addView(separator)
            }
            val crumb = ItemBreadcrumbBinding.inflate(
                layoutInflater, binding.breadcrumbContainer, false
            ).root
            crumb.text = dir.name
            crumb.setOnClickListener {
                while (stack.size > index + 1) stack.removeAt(stack.size - 1)
                render()
            }
            binding.breadcrumbContainer.addView(crumb)
        }
    }

    private fun renderEntries() {
        binding.entriesContainer.removeAllViews()
        val children = stack.last().children
        val dirs = children.filterIsInstance<TreeEntry.Dir>()
        val docs = children.filterIsInstance<TreeEntry.Doc>()

        if (children.isEmpty()) {
            binding.textStatus.visibility = View.VISIBLE
            binding.textStatus.text = getString(R.string.folder_picker_empty)
        } else {
            binding.textStatus.visibility = View.GONE
        }

        for (dir in dirs) {
            val row = ItemFolderRowBinding.inflate(layoutInflater, binding.entriesContainer, false)
            row.textName.text = dir.name
            row.root.setOnClickListener {
                stack.add(dir)
                render()
            }
            binding.entriesContainer.addView(row.root)
        }
        for (doc in docs) {
            val row = ItemDocumentRowBinding.inflate(layoutInflater, binding.entriesContainer, false)
            row.textName.text = doc.name
            row.textSize.text = formatFileSize(doc.sizeBytes)
            binding.entriesContainer.addView(row.root)
        }
    }

    private fun formatFileSize(bytes: Long): String {
        val kb = bytes / 1024.0
        return if (kb < 1024) {
            String.format(Locale.getDefault(), "%.0f Ko", kb)
        } else {
            String.format(Locale.getDefault(), "%.1f Mo", kb / 1024.0)
        }
    }
}
