use crate::client::AdbClient;
use crate::parsers::communication_parser::CommunicationParser;
use crate::parsers::contact_parser::ContactParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{CallLog, Contact, DeviceId, Sms};

#[derive(Clone)]
pub struct AdbDataRepository {
    client: AdbClient,
}

impl AdbDataRepository {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    fn safe_content_query(
        &self,
        device_id: &DeviceId,
        uri: &str,
        projection: &str,
    ) -> Result<String> {
        let script = AndroidScripts::content_query(uri, projection);
        self.execute_query(device_id, uri, &script)
    }

    fn safe_content_query_ordered(
        &self,
        device_id: &DeviceId,
        uri: &str,
        projection: &str,
        sort: &str,
    ) -> Result<String> {
        let script = AndroidScripts::content_query_ordered(uri, projection, sort);
        self.execute_query(device_id, uri, &script)
    }

    fn execute_query(
        &self,
        device_id: &DeviceId,
        uri: &str,
        script: &str,
    ) -> Result<String> {
        let output = self.client.shell(&device_id.0, script);
        match output {
            Ok(out) => {
                if out.contains("Permission denied") || out.contains("Error") {
                    tracing::warn!("ADB query warning for {}: {}", uri, out.trim());
                    Ok(String::new())
                } else {
                    Ok(out)
                }
            }
            Err(e) => {
                tracing::error!("ADB query failed for {}: {}", uri, e);
                Ok(String::new())
            }
        }
    }

    pub fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> {
        let projection = "contact_id:display_name:mimetype:account_name:data1:data2:data3:data4:data5:data6:data7:data8:data9:data10";
        let output =
            self.safe_content_query(device_id, "content://com.android.contacts/data", projection)?;
        Ok(ContactParser::parse(device_id, &output))
    }

    pub fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        let output = self.safe_content_query_ordered(
            device_id,
            "content://sms",
            "address:body:date:type",
            "date DESC",
        )?;
        Ok(CommunicationParser::parse_sms(&output))
    }

    pub fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        let output = self.safe_content_query_ordered(
            device_id,
            "content://call_log/calls",
            "number:date:duration:type:name:geocoded_location",
            "date DESC",
        )?;
        Ok(CommunicationParser::parse_call_logs(&output))
    }
}
