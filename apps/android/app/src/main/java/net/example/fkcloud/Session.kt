package net.example.fkcloud

import java.io.IOException

/**
 * Wraps SecureStore + RmfcClient: validates the server URL policy
 * (HTTPS unless the user explicitly allowed HTTP) and caches the JWT,
 * re-authenticating with the stored credentials when it expires.
 */
class Session(private val store: SecureStore) {

    class ConfigException(message: String) : IOException(message)

    fun serverUrl(): String {
        val raw = store.getString(SecureStore.KEY_SERVER_URL)?.trim()
        if (raw.isNullOrEmpty()) throw ConfigException("no_server")
        val url = if (raw.startsWith("http://") || raw.startsWith("https://")) raw else "https://$raw"
        if (url.startsWith("http://") && !store.getBoolean(SecureStore.KEY_ALLOW_HTTP)) {
            throw ConfigException("http_forbidden")
        }
        return url
    }

    fun client(): RmfcClient = RmfcClient(serverUrl())

    /** Fresh login, stores token with a conservative 23h lifetime (server issues 24h). */
    fun login(email: String, password: String): String {
        val token = client().login(email, password)
        store.putSecret(SecureStore.KEY_TOKEN, token)
        store.putLong(
            SecureStore.KEY_TOKEN_EXPIRY,
            System.currentTimeMillis() + 23L * 60 * 60 * 1000
        )
        return token
    }

    /** Returns a valid token, re-logging in with stored credentials if needed. */
    fun ensureToken(): String {
        val cached = store.getSecret(SecureStore.KEY_TOKEN)
        val expiry = store.getLong(SecureStore.KEY_TOKEN_EXPIRY)
        if (!cached.isNullOrEmpty() && System.currentTimeMillis() < expiry) {
            return cached
        }
        val email = store.getString(SecureStore.KEY_EMAIL)
        val password = store.getSecret(SecureStore.KEY_PASSWORD)
        if (email.isNullOrEmpty() || password.isNullOrEmpty()) {
            throw ConfigException("no_credentials")
        }
        return login(email, password)
    }

    /** Retries `block` once after a re-login when the server answers 401. */
    fun <T> withAuth(block: (client: RmfcClient, token: String) -> T): T {
        val client = client()
        return try {
            block(client, ensureToken())
        } catch (e: ApiException) {
            if (e.code != 401) throw e
            store.putSecret(SecureStore.KEY_TOKEN, null)
            block(client, ensureToken())
        }
    }
}
