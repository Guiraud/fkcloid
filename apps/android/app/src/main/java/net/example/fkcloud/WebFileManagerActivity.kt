package net.example.fkcloud

import android.net.Uri
import android.content.Intent
import android.os.Bundle
import android.view.View
import android.webkit.WebResourceRequest
import android.webkit.WebResourceError
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.activity.OnBackPressedCallback
import androidx.appcompat.app.AppCompatActivity
import net.example.fkcloud.databinding.ActivityWebFileManagerBinding

/**
 * Embeds the rmfakecloud web UI (its own file manager: browse, rename,
 * move, delete, download) instead of reimplementing it natively. The user
 * logs in through the web app's own form the first time; the WebView's
 * cookie jar keeps the session for later visits.
 */
class WebFileManagerActivity : AppCompatActivity() {

    private lateinit var binding: ActivityWebFileManagerBinding
    private lateinit var allowedHost: String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityWebFileManagerBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val serverUrl = try {
            Session(SecureStore(applicationContext)).serverUrl()
        } catch (e: Session.ConfigException) {
            binding.textError.visibility = View.VISIBLE
            binding.textError.text = when (e.message) {
                "http_forbidden" -> getString(R.string.status_http_forbidden)
                else -> getString(R.string.status_no_server)
            }
            binding.buttonBack.setOnClickListener { finish() }
            return
        }
        allowedHost = Uri.parse(serverUrl).host ?: ""

        binding.buttonBack.setOnClickListener {
            if (binding.webview.canGoBack()) binding.webview.goBack() else finish()
        }
        onBackPressedDispatcher.addCallback(this, object : OnBackPressedCallback(true) {
            override fun handleOnBackPressed() {
                if (binding.webview.canGoBack()) binding.webview.goBack() else finish()
            }
        })

        setUpWebView()
        binding.webview.loadUrl(serverUrl)
    }

    private fun setUpWebView() {
        val webView = binding.webview
        val settings = webView.settings
        settings.javaScriptEnabled = true
        settings.domStorageEnabled = true
        // rmfakecloud's own UI declares a mobile viewport; WebView only
        // honors it with wide-viewport + overview mode enabled. Without
        // these the page renders at a fixed desktop width and gets cropped
        // on the right instead of reflowing to the screen.
        settings.useWideViewPort = true
        settings.loadWithOverviewMode = true
        settings.setSupportZoom(true)
        settings.builtInZoomControls = true
        settings.displayZoomControls = false

        webView.webViewClient = object : WebViewClient() {
            // Keep navigation on the configured server; open anything else
            // (external links) in the system browser instead of in-app.
            override fun shouldOverrideUrlLoading(view: WebView, request: WebResourceRequest): Boolean {
                val host = request.url.host
                if (host == null || host == allowedHost) return false
                startActivity(Intent(Intent.ACTION_VIEW, request.url))
                return true
            }

            override fun onPageStarted(view: WebView, url: String, favicon: android.graphics.Bitmap?) {
                binding.progress.visibility = View.VISIBLE
                binding.textError.visibility = View.GONE
            }

            override fun onPageFinished(view: WebView, url: String) {
                binding.progress.visibility = View.GONE
                // rmfakecloud's own top nav doesn't wrap on narrow screens
                // and gets clipped even with a correct viewport; neither
                // setInitialScale() (overridden by the page's own
                // initial-scale=1) nor zoomBy() (WebView quirk) affect it.
                // CSS zoom on the whole document does.
                view.evaluateJavascript(
                    "document.documentElement.style.zoom = '50%';",
                    null
                )
            }

            override fun onReceivedError(
                view: WebView,
                request: WebResourceRequest,
                error: WebResourceError
            ) {
                if (!request.isForMainFrame) return
                binding.progress.visibility = View.GONE
                binding.webview.visibility = View.GONE
                binding.textError.visibility = View.VISIBLE
                binding.textError.text = getString(
                    R.string.file_manager_load_error,
                    request.url.toString(),
                    error.description ?: "?"
                )
            }
        }
    }
}
