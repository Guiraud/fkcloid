package net.example.fkcloud

import android.content.Intent
import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import net.example.fkcloud.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    companion object {
        private const val DEFAULT_SERVER_URL = "https://rm-cloud.example.invalid"
    }

    private lateinit var binding: ActivityMainBinding
    private lateinit var store: SecureStore

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        store = SecureStore(applicationContext)

        // MaterialCardView's app:cardBackgroundColor ignores our custom colorSurface
        // (Material3's tonal elevation overlay wins); set it programmatically to
        // guarantee the charte's cream surface.
        binding.cardShareHint.setCardBackgroundColor(
            ContextCompat.getColor(this, R.color.registre_surface)
        )

        binding.inputServer.setText(
            store.getString(SecureStore.KEY_SERVER_URL) ?: DEFAULT_SERVER_URL
        )
        binding.inputEmail.setText(store.getString(SecureStore.KEY_EMAIL) ?: "")
        binding.inputPassword.setText(store.getSecret(SecureStore.KEY_PASSWORD) ?: "")
        binding.checkAllowHttp.isChecked = store.getBoolean(SecureStore.KEY_ALLOW_HTTP)

        binding.buttonSave.setOnClickListener { saveAndTest() }
        binding.buttonFileManager.setOnClickListener {
            startActivity(Intent(this, WebFileManagerActivity::class.java))
        }
    }

    private fun saveAndTest() {
        val server = binding.inputServer.text.toString().trim()
        val email = binding.inputEmail.text.toString().trim()
        val password = binding.inputPassword.text.toString()
        val allowHttp = binding.checkAllowHttp.isChecked

        if (server.isEmpty() || email.isEmpty() || password.isEmpty()) {
            binding.textStatus.text = getString(R.string.status_fill_all_fields)
            return
        }

        store.putString(SecureStore.KEY_SERVER_URL, server)
        store.putString(SecureStore.KEY_EMAIL, email)
        store.putSecret(SecureStore.KEY_PASSWORD, password)
        store.putBoolean(SecureStore.KEY_ALLOW_HTTP, allowHttp)
        store.putSecret(SecureStore.KEY_TOKEN, null)

        binding.buttonSave.isEnabled = false
        binding.textStatus.text = getString(R.string.status_connecting)
        binding.cardShareHint.visibility = View.GONE

        lifecycleScope.launch {
            val result = withContext(Dispatchers.IO) {
                runCatching { Session(store).login(email, password) }
            }
            binding.buttonSave.isEnabled = true
            result.fold(
                onSuccess = {
                    binding.textStatus.text = getString(R.string.status_connected)
                    binding.cardShareHint.visibility = View.VISIBLE
                },
                onFailure = { e ->
                    binding.textStatus.text = when {
                        e is Session.ConfigException && e.message == "http_forbidden" ->
                            getString(R.string.status_http_forbidden)
                        e is ApiException && e.code == 401 ->
                            getString(R.string.status_bad_credentials)
                        else -> getString(R.string.status_error, e.message ?: "?")
                    }
                }
            )
        }
    }
}
