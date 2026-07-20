package net.example.fkcloud

import android.content.ContentResolver
import android.net.Uri
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.MultipartBody
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody
import okhttp3.RequestBody.Companion.toRequestBody
import okio.BufferedSink
import okio.source
import org.json.JSONArray
import org.json.JSONObject
import java.io.IOException
import java.util.concurrent.TimeUnit

/** A node in the rmfakecloud document tree: a folder or a document. */
sealed class TreeEntry {
    data class Dir(val id: String, val name: String, val children: List<TreeEntry>) : TreeEntry()
    data class Doc(val name: String, val type: String, val sizeBytes: Long) : TreeEntry()
}

class ApiException(val code: Int, message: String) : IOException(message)

/**
 * Minimal client for the rmfakecloud web UI API (/ui/api).
 * Auth: POST /ui/api/login returns a JWT (plain text body), then
 * Authorization: Bearer <token> on subsequent calls.
 */
class RmfcClient(private val baseUrl: String) {

    private val client = OkHttpClient.Builder()
        .connectTimeout(20, TimeUnit.SECONDS)
        .readTimeout(60, TimeUnit.SECONDS)
        .writeTimeout(0, TimeUnit.MILLISECONDS) // large uploads: no write timeout
        .build()

    private fun url(path: String): String = baseUrl.trimEnd('/') + path

    /** Returns the JWT session token. */
    fun login(email: String, password: String): String {
        val body = JSONObject()
            .put("email", email)
            .put("password", password)
            .toString()
            .toRequestBody("application/json".toMediaType())
        val request = Request.Builder().url(url("/ui/api/login")).post(body).build()
        client.newCall(request).execute().use { response ->
            if (!response.isSuccessful) {
                throw ApiException(response.code, "Login failed: HTTP ${response.code}")
            }
            val token = response.body?.string()?.trim()
            if (token.isNullOrEmpty()) throw ApiException(0, "Login failed: empty token")
            return token
        }
    }

    /**
     * Full document tree from GET /ui/api/documents, wrapped in a synthetic
     * root directory whose id is "root" (the API's own top-level parent id).
     */
    fun getDocumentTree(token: String, rootLabel: String): TreeEntry.Dir {
        val request = Request.Builder()
            .url(url("/ui/api/documents"))
            .header("Authorization", "Bearer $token")
            .get()
            .build()
        client.newCall(request).execute().use { response ->
            if (!response.isSuccessful) {
                throw ApiException(response.code, "Cannot list documents: HTTP ${response.code}")
            }
            val json = JSONObject(response.body?.string() ?: "{}")
            return TreeEntry.Dir("root", rootLabel, parseEntries(json.optJSONArray("Entries")))
        }
    }

    private fun parseEntries(entries: JSONArray?): List<TreeEntry> {
        entries ?: return emptyList()
        val result = mutableListOf<TreeEntry>()
        for (i in 0 until entries.length()) {
            val entry = entries.optJSONObject(i) ?: continue
            result.add(
                if (entry.optBoolean("isFolder")) {
                    TreeEntry.Dir(
                        entry.getString("id"),
                        entry.optString("name", "?"),
                        parseEntries(entry.optJSONArray("children"))
                    )
                } else {
                    TreeEntry.Doc(
                        entry.optString("name", "?"),
                        entry.optString("type", ""),
                        entry.optLong("size", 0L)
                    )
                }
            )
        }
        return result
    }

    /**
     * Uploads one document via POST /ui/api/documents/upload.
     * @param parentId folder id, or "root" for the top level.
     */
    fun upload(
        token: String,
        parentId: String,
        displayName: String,
        mimeType: String,
        resolver: ContentResolver,
        uri: Uri,
    ) {
        val fileBody = object : RequestBody() {
            override fun contentType() = mimeType.toMediaType()
            override fun writeTo(sink: BufferedSink) {
                resolver.openInputStream(uri)?.use { input ->
                    sink.writeAll(input.source())
                } ?: throw IOException("Cannot open $displayName")
            }
        }
        val multipart = MultipartBody.Builder()
            .setType(MultipartBody.FORM)
            .addFormDataPart("parent", parentId)
            .addFormDataPart("file", displayName, fileBody)
            .build()
        val request = Request.Builder()
            .url(url("/ui/api/documents/upload"))
            .header("Authorization", "Bearer $token")
            .post(multipart)
            .build()
        client.newCall(request).execute().use { response ->
            when {
                response.isSuccessful -> return
                response.code == 409 ->
                    throw ApiException(409, "Document already exists: $displayName")
                else ->
                    throw ApiException(response.code, "Upload failed (HTTP ${response.code}): $displayName")
            }
        }
    }
}
