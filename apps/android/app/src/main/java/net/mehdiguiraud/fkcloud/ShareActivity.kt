package net.example.fkcloud

import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.OpenableColumns
import android.view.View
import android.widget.Toast
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import net.example.fkcloud.databinding.ActivityShareBinding
import java.util.Locale

class ShareActivity : AppCompatActivity() {

    companion object {
        private const val AUTO_CLOSE_DELAY_MS = 1400L
    }

    private lateinit var binding: ActivityShareBinding
    private lateinit var store: SecureStore
    private var uris: List<Uri> = emptyList()

    private var selectedFolderId: String = "root"
    private lateinit var selectedFolderName: String

    private val pickFolder = registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
        if (result.resultCode == RESULT_OK) {
            val id = result.data?.getStringExtra(FolderPickerActivity.EXTRA_FOLDER_ID)
            val name = result.data?.getStringExtra(FolderPickerActivity.EXTRA_FOLDER_NAME)
            if (id != null && name != null) {
                selectedFolderId = id
                selectedFolderName = name
                binding.textSelectedFolder.text = name
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityShareBinding.inflate(layoutInflater)
        setContentView(binding.root)
        store = SecureStore(applicationContext)

        selectedFolderName = getString(R.string.folder_root)
        binding.textSelectedFolder.text = selectedFolderName

        uris = extractUris(intent)
        if (uris.isEmpty()) {
            Toast.makeText(this, R.string.share_nothing, Toast.LENGTH_LONG).show()
            finish()
            return
        }

        val names = uris.map { displayName(it) }
        val unsupported = names.filterNot { supportedExtension(it) }
        binding.textFiles.text = uris.joinToString("\n") { uri ->
            val name = displayName(uri)
            val size = fileSize(uri)
            if (size != null) "$name\n${formatFileSize(size)}" else name
        }
        if (unsupported.isNotEmpty()) {
            binding.textWarning.visibility = View.VISIBLE
            binding.textWarning.text =
                getString(R.string.share_unsupported_warning, unsupported.joinToString(", "))
        }

        binding.buttonUpload.setOnClickListener { upload() }
        binding.buttonCancel.setOnClickListener { finish() }
        binding.buttonChangeFolder.setOnClickListener {
            pickFolder.launch(Intent(this, FolderPickerActivity::class.java))
        }

        checkSession()
    }

    /** Validates the stored config/session early so misconfiguration surfaces
     * immediately, instead of only when the user taps Send. */
    private fun checkSession() {
        lifecycleScope.launch {
            val result = withContext(Dispatchers.IO) {
                runCatching { Session(store).ensureToken() }
            }
            result.onFailure { e -> failConfig(e) }
        }
    }

    private fun extractUris(intent: Intent): List<Uri> = when (intent.action) {
        Intent.ACTION_SEND -> listOfNotNull(intent.getParcelable(Intent.EXTRA_STREAM))
        Intent.ACTION_SEND_MULTIPLE ->
            intent.getParcelableList(Intent.EXTRA_STREAM)
        else -> emptyList()
    }

    @Suppress("DEPRECATION")
    private fun Intent.getParcelable(key: String): Uri? =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU)
            getParcelableExtra(key, Uri::class.java)
        else
            getParcelableExtra(key)

    @Suppress("DEPRECATION")
    private fun Intent.getParcelableList(key: String): List<Uri> =
        (if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU)
            getParcelableArrayListExtra(key, Uri::class.java)
        else
            getParcelableArrayListExtra(key)) ?: emptyList()

    private fun displayName(uri: Uri): String {
        var name: String? = null
        try {
            contentResolver.query(uri, arrayOf(OpenableColumns.DISPLAY_NAME), null, null, null)
                ?.use { cursor ->
                    if (cursor.moveToFirst()) {
                        val index = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
                        if (index >= 0) name = cursor.getString(index)
                    }
                }
        } catch (_: Exception) {
            // Provider may refuse metadata queries; the stream grant is enough.
        }
        var result = name ?: uri.lastPathSegment ?: "document"
        // Some providers return an opaque id: derive the extension from the MIME type.
        if (!result.contains('.')) {
            val mime = try { contentResolver.getType(uri) } catch (_: Exception) { null }
                ?: intent.type
            when (mime) {
                "application/pdf" -> result += ".pdf"
                "application/epub+zip" -> result += ".epub"
            }
        }
        return result
    }

    private fun fileSize(uri: Uri): Long? {
        var size: Long? = null
        try {
            contentResolver.query(uri, arrayOf(OpenableColumns.SIZE), null, null, null)?.use { cursor ->
                if (cursor.moveToFirst()) {
                    val index = cursor.getColumnIndex(OpenableColumns.SIZE)
                    if (index >= 0 && !cursor.isNull(index)) size = cursor.getLong(index)
                }
            }
        } catch (_: Exception) {
            // Size is a display nicety; upload proceeds without it.
        }
        return size
    }

    private fun formatFileSize(bytes: Long): String {
        val kb = bytes / 1024.0
        return if (kb < 1024) {
            String.format(Locale.getDefault(), "%.0f Ko", kb)
        } else {
            String.format(Locale.getDefault(), "%.1f Mo", kb / 1024.0)
        }
    }

    private fun supportedExtension(name: String): Boolean {
        val lower = name.lowercase()
        return lower.endsWith(".pdf") || lower.endsWith(".epub")
    }

    private fun mimeFor(name: String): String = when {
        name.lowercase().endsWith(".pdf") -> "application/pdf"
        name.lowercase().endsWith(".epub") -> "application/epub+zip"
        else -> "application/octet-stream"
    }

    private fun upload() {
        val parent = selectedFolderId
        binding.buttonUpload.isEnabled = false
        binding.progress.visibility = View.VISIBLE
        binding.progress.max = uris.size
        binding.progress.progress = 0

        lifecycleScope.launch {
            val errors = mutableListOf<String>()
            withContext(Dispatchers.IO) {
                val session = Session(store)
                uris.forEachIndexed { index, uri ->
                    val name = displayName(uri)
                    runCatching {
                        session.withAuth { client, token ->
                            client.upload(token, parent, name, mimeFor(name), contentResolver, uri)
                        }
                    }.onFailure { e -> errors.add(e.message ?: name) }
                    withContext(Dispatchers.Main) { binding.progress.progress = index + 1 }
                }
            }
            binding.progress.visibility = View.GONE
            if (errors.isEmpty()) {
                binding.groupContent.visibility = View.GONE
                binding.groupSuccess.visibility = View.VISIBLE
                binding.textSuccess.text = getString(R.string.share_success, selectedFolderName)
                binding.buttonClose.setOnClickListener { finish() }
                launch {
                    delay(AUTO_CLOSE_DELAY_MS)
                    finish()
                }
            } else {
                binding.buttonUpload.isEnabled = true
                binding.textWarning.visibility = View.VISIBLE
                binding.textWarning.text =
                    getString(R.string.share_errors, errors.joinToString("\n"))
            }
        }
    }

    private fun failConfig(e: Throwable) {
        val message = when {
            e is Session.ConfigException && e.message == "no_server" ->
                getString(R.string.status_no_server)
            e is Session.ConfigException && e.message == "no_credentials" ->
                getString(R.string.status_no_credentials)
            e is Session.ConfigException && e.message == "http_forbidden" ->
                getString(R.string.status_http_forbidden)
            e is ApiException && e.code == 401 ->
                getString(R.string.status_bad_credentials)
            else -> getString(R.string.status_error, e.message ?: "?")
        }
        binding.textWarning.visibility = View.VISIBLE
        binding.textWarning.text = message
        if (e is Session.ConfigException) {
            // Open settings so the user can fix the configuration.
            startActivity(Intent(this, MainActivity::class.java))
        }
    }
}
