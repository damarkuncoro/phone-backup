package com.phonebackup.agent

import android.content.Context
import android.net.Uri
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.websocket.*
import io.ktor.websocket.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import org.json.JSONObject

sealed class AgentConnectionState {
    object Idle : AgentConnectionState()
    object Connecting : AgentConnectionState()
    data class Connected(val host: String, val port: Int) : AgentConnectionState()
    data class Syncing(val message: String, val progress: Int) : AgentConnectionState()
    data class Error(val reason: String) : AgentConnectionState()
}

class AgentNetworkClient(private val context: Context) {

    private val extractor = AndroidDataExtractor(context)
    private val client = HttpClient(OkHttp) {
        install(WebSockets)
    }

    private val _connectionState = MutableStateFlow<AgentConnectionState>(AgentConnectionState.Idle)
    val connectionState: StateFlow<AgentConnectionState> = _connectionState

    suspend fun connectWithQrUri(qrUriString: String) = withContext(Dispatchers.IO) {
        try {
            _connectionState.value = AgentConnectionState.Connecting
            val uri = Uri.parse(qrUriString)
            val ip = uri.getQueryParameter("ip") ?: "127.0.0.1"
            val port = uri.getQueryParameter("port")?.toIntOrNull() ?: 3030
            val token = uri.getQueryParameter("token") ?: ""

            client.webSocket(host = ip, port = port, path = "/agent") {
                _connectionState.value = AgentConnectionState.Connected(ip, port)

                // 1. Send Handshake
                val handshake = extractor.getDeviceMetrics().apply {
                    put("token", token)
                }
                send(Frame.Text(JSONObject().apply {
                    put("type", "handshake")
                    put("payload", handshake)
                }.toString()))

                // 2. Listen for desktop requests
                for (frame in incoming) {
                    if (frame is Frame.Text) {
                        handleCommand(frame.readText(), this)
                    }
                }
            }
        } catch (e: Exception) {
            _connectionState.value = AgentConnectionState.Error(e.message ?: "Connection failed")
        }
    }

    private suspend fun handleCommand(rawJson: String, session: DefaultWebSocketSession) {
        val req = JSONObject(rawJson)
        when (req.optString("action")) {
            "get_contacts" -> {
                val contacts = extractor.extractContacts()
                session.send(Frame.Text(JSONObject().apply {
                    put("type", "contacts_response")
                    put("data", contacts)
                }.toString()))
            }
            "get_sms" -> {
                val sms = extractor.extractSms()
                session.send(Frame.Text(JSONObject().apply {
                    put("type", "sms_response")
                    put("data", sms)
                }.toString()))
            }
            "get_call_logs" -> {
                val logs = extractor.extractCallLogs()
                session.send(Frame.Text(JSONObject().apply {
                    put("type", "call_logs_response")
                    put("data", logs)
                }.toString()))
            }
        }
    }
}
