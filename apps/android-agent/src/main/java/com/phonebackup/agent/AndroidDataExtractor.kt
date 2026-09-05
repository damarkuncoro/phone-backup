package com.phonebackup.agent

import android.content.Context
import android.database.Cursor
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.os.StatFs
import android.provider.CallLog
import android.provider.ContactsContract
import android.provider.Telephony
import org.json.JSONArray
import org.json.JSONObject

class AndroidDataExtractor(private val context: Context) {

    fun getDeviceMetrics(): JSONObject {
        val stat = StatFs(Environment.getDataDirectory().path)
        val totalBytes = stat.totalBytes
        val freeBytes = stat.availableBytes
        val usedBytes = totalBytes - freeBytes

        return JSONObject().apply {
            put("device_id", "${Build.MANUFACTURER}_${Build.MODEL}_${Build.SERIAL.take(6)}")
            put("manufacturer", Build.MANUFACTURER)
            put("model", Build.MODEL)
            put("android_version", "Android ${Build.VERSION.RELEASE} (API ${Build.VERSION.SDK_INT})")
            put("storage_total_bytes", totalBytes)
            put("storage_used_bytes", usedBytes)
            put("battery_percent", 90)
            put("temperature_c", 33.5)
        }
    }

    fun extractContacts(): JSONArray {
        val array = JSONArray()
        val resolver = context.contentResolver
        val cursor: Cursor? = resolver.query(
            ContactsContract.CommonDataKinds.Phone.CONTENT_URI,
            arrayOf(
                ContactsContract.CommonDataKinds.Phone.CONTACT_ID,
                ContactsContract.CommonDataKinds.Phone.DISPLAY_NAME,
                ContactsContract.CommonDataKinds.Phone.NUMBER
            ),
            null, null, ContactsContract.CommonDataKinds.Phone.DISPLAY_NAME + " ASC"
        )

        cursor?.use {
            val idCol = it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.CONTACT_ID)
            val nameCol = it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.DISPLAY_NAME)
            val numCol = it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.NUMBER)

            while (it.moveToNext()) {
                val obj = JSONObject().apply {
                    put("id", it.getString(idCol) ?: "")
                    put("name", it.getString(nameCol) ?: "Unknown")
                    put("phone", it.getString(numCol) ?: "")
                }
                array.put(obj)
            }
        }
        return array
    }

    fun extractSms(): JSONArray {
        val array = JSONArray()
        val cursor: Cursor? = context.contentResolver.query(
            Telephony.Sms.CONTENT_URI,
            arrayOf(Telephony.Sms._ID, Telephony.Sms.ADDRESS, Telephony.Sms.BODY, Telephony.Sms.DATE, Telephony.Sms.TYPE),
            null, null, Telephony.Sms.DATE + " DESC"
        )

        cursor?.use {
            val idCol = it.getColumnIndex(Telephony.Sms._ID)
            val addrCol = it.getColumnIndex(Telephony.Sms.ADDRESS)
            val bodyCol = it.getColumnIndex(Telephony.Sms.BODY)
            val dateCol = it.getColumnIndex(Telephony.Sms.DATE)
            val typeCol = it.getColumnIndex(Telephony.Sms.TYPE)

            while (it.moveToNext()) {
                val obj = JSONObject().apply {
                    put("id", it.getString(idCol) ?: "")
                    put("address", it.getString(addrCol) ?: "")
                    put("body", it.getString(bodyCol) ?: "")
                    put("timestamp_ms", it.getLong(dateCol))
                    put("type", if (it.getInt(typeCol) == 1) "inbox" else "sent")
                }
                array.put(obj)
            }
        }
        return array
    }

    fun extractCallLogs(): JSONArray {
        val array = JSONArray()
        val cursor: Cursor? = context.contentResolver.query(
            CallLog.Calls.CONTENT_URI,
            arrayOf(CallLog.Calls._ID, CallLog.Calls.NUMBER, CallLog.Calls.DATE, CallLog.Calls.DURATION, CallLog.Calls.TYPE),
            null, null, CallLog.Calls.DATE + " DESC"
        )

        cursor?.use {
            val idCol = it.getColumnIndex(CallLog.Calls._ID)
            val numCol = it.getColumnIndex(CallLog.Calls.NUMBER)
            val dateCol = it.getColumnIndex(CallLog.Calls.DATE)
            val durCol = it.getColumnIndex(CallLog.Calls.DURATION)
            val typeCol = it.getColumnIndex(CallLog.Calls.TYPE)

            while (it.moveToNext()) {
                val obj = JSONObject().apply {
                    put("id", it.getString(idCol) ?: "")
                    put("number", it.getString(numCol) ?: "")
                    put("timestamp_ms", it.getLong(dateCol))
                    put("duration_sec", it.getLong(durCol))
                    put("type", when (it.getInt(typeCol)) {
                        CallLog.Calls.INCOMING_TYPE -> "incoming"
                        CallLog.Calls.OUTGOING_TYPE -> "outgoing"
                        CallLog.Calls.MISSED_TYPE -> "missed"
                        else -> "other"
                    })
                }
                array.put(obj)
            }
        }
        return array
    }
}
