/// Shell scripts for various Android system operations
pub struct AndroidScripts;

impl AndroidScripts {
    pub const LIST_DEVICES: &'static str = "devices -l";

    pub const DISK_USAGE: &'static str = "df /data";

    pub const LIST_APPS: &'static str = "pm list packages --show-versioncode --user 0";

    pub fn get_prop(prop: &str) -> String {
        format!("getprop {}", prop)
    }

    pub fn content_query(uri: &str, projection: &str) -> String {
        format!("content query --uri {} --projection {}", uri, projection)
    }

    pub fn find_files(roots: &[String]) -> String {
        let roots_str = roots.join(" ");
        format!("find {} -type f -exec stat -c '%n|%s|%Y' {{}} + 2>/dev/null", roots_str)
    }

    pub fn query_mediastore(media_type: &str) -> String {
        let uri = match media_type {
            "video" => "content://media/external/video/media",
            _ => "content://media/external/images/media",
        };
        // data: path, _size: size, date_modified: time, mime_type, width, height, datetaken: creation time, latitude, longitude
        let projection = "_data:_size:date_modified:mime_type:width:height:datetaken:latitude:longitude";
        format!("content query --uri {} --projection {}", uri, projection)
    }

    pub fn cat_file(path: &str) -> String {
        format!("cat \"{}\"", path)
    }
}
