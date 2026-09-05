package com.phonebackup.agent

import android.Manifest
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private lateinit var networkClient: AgentNetworkClient

    private val permissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val allGranted = permissions.entries.all { it.value }
        if (allGranted) {
            startAgentService()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        networkClient = AgentNetworkClient(this)

        setContent {
            val scope = rememberCoroutineScope()
            var isScanning by remember { mutableStateOf(false) }
            val connState by networkClient.connectionState.collectAsState()

            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize(), color = Color(0xFF0F172A)) {
                    if (isScanning) {
                        Box(modifier = Modifier.fillMaxSize()) {
                            QrScannerView { qrCodeUri ->
                                isScanning = false
                                Toast.makeText(this@MainActivity, "Connecting to desktop...", Toast.LENGTH_SHORT).show()
                                scope.launch {
                                    networkClient.connectWithQrUri(qrCodeUri)
                                }
                            }
                            Button(
                                onClick = { isScanning = false },
                                modifier = Modifier.align(Alignment.BottomCenter).padding(24.dp),
                                colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF334155))
                            ) {
                                Text("Cancel Scanner")
                            }
                        }
                    } else {
                        AgentHomeScreen(
                            connectionState = connState,
                            onStartPairing = {
                                requestPermissionsAndStart()
                                isScanning = true
                            }
                        )
                    }
                }
            }
        }
    }

    private fun requestPermissionsAndStart() {
        val perms = mutableListOf(
            Manifest.permission.READ_CONTACTS,
            Manifest.permission.READ_SMS,
            Manifest.permission.READ_CALL_LOG,
            Manifest.permission.CAMERA
        )
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            perms.add(Manifest.permission.POST_NOTIFICATIONS)
            perms.add(Manifest.permission.READ_MEDIA_IMAGES)
            perms.add(Manifest.permission.READ_MEDIA_VIDEO)
        }
        permissionLauncher.launch(perms.toTypedArray())
    }

    private fun startAgentService() {
        val intent = Intent(this, BackupAgentService::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(intent)
        } else {
            startService(intent)
        }
    }
}

@Composable
fun AgentHomeScreen(connectionState: AgentConnectionState, onStartPairing: () -> Unit) {
    Column(
        modifier = Modifier.fillMaxSize().padding(24.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text("📱 Phone Backup Agent", fontSize = 24.sp, fontWeight = FontWeight.Bold, color = Color.White)
        Spacer(modifier = Modifier.height(8.dp))
        Text("Zero-Debugging Wireless Backup for Android", fontSize = 14.sp, color = Color(0xFF94A3B8))
        Spacer(modifier = Modifier.height(32.dp))

        Card(
            colors = CardDefaults.cardColors(containerColor = Color(0xFF1E293B)),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                val statusText = when (connectionState) {
                    is AgentConnectionState.Idle -> "Status: Standby / Ready to Pair"
                    is AgentConnectionState.Connecting -> "Status: Connecting to Desktop..."
                    is AgentConnectionState.Connected -> "Status: Connected to ${connectionState.host}:${connectionState.port}"
                    is AgentConnectionState.Syncing -> "Status: Syncing (${connectionState.progress}%)"
                    is AgentConnectionState.Error -> "Status: Error - ${connectionState.reason}"
                }
                val statusColor = when (connectionState) {
                    is AgentConnectionState.Connected -> Color(0xFF4ADE80)
                    is AgentConnectionState.Error -> Color(0xFFF87171)
                    else -> Color(0xFF38BDF8)
                }
                Text(statusText, color = statusColor, fontWeight = FontWeight.SemiBold)
                Spacer(modifier = Modifier.height(4.dp))
                Text("Wi-Fi Direct P2P & mTLS Encrypted", fontSize = 12.sp, color = Color(0xFF64748B))
            }
        }

        Spacer(modifier = Modifier.height(32.dp))

        Button(
            onClick = onStartPairing,
            colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF4F46E5)),
            modifier = Modifier.fillMaxWidth().height(50.dp)
        ) {
            Text("⚡ Scan Desktop QR Code to Pair", fontSize = 16.sp, fontWeight = FontWeight.Bold)
        }
    }
}
