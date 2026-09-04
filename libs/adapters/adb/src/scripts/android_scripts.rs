/// Shell scripts for various Android system operations
pub struct AndroidScripts;

impl AndroidScripts {
    pub const LIST_DEVICES: &'static str = "devices -l";

    pub const DISK_USAGE: &'static str = "df /data";

    pub const LIST_APPS: &'static str = "pm list packages --show-versioncode --user 0";

    pub const BATTERY_STATUS: &'static str = "dumpsys battery";

    pub fn get_prop(prop: &str) -> String {
        format!("getprop {}", prop)
    }

    pub fn content_query(uri: &str, projection: &str) -> String {
        format!("content query --uri {} --projection {}", uri, projection)
    }

    pub fn content_query_ordered(uri: &str, projection: &str, sort_order: &str) -> String {
        format!("content query --uri {} --projection {} --sort \"{}\"", uri, projection, sort_order)
    }

    pub fn find_files(roots: &[String]) -> String {
        let roots_str = roots
            .iter()
            .map(|r| format!("\"{}\"", r.trim()))
            .collect::<Vec<_>>()
            .join(" ");
        format!("find {} -type f ! -path '*/.trash/*' ! -path '*/cache/*' ! -path '*/.cache/*' -exec stat -c '%n|%s|%Y' {{}} + 2>/dev/null", roots_str)
    }

    pub fn query_mediastore(media_type: &str) -> String {
        let uri = match media_type {
            "video" => "content://media/external/video/media",
            "audio" => "content://media/external/audio/media",
            "file" => "content://media/external/file",
            _ => "content://media/external/images/media",
        };
        // data: path, _size: size, date_modified: time, mime_type, width, height, datetaken: creation time, latitude, longitude
        let projection =
            "_data:_size:date_modified:mime_type:width:height:datetaken:latitude:longitude";
        format!("content query --uri {} --projection {}", uri, projection)
    }

    pub fn cat_file(path: &str) -> String {
        format!("cat \"{}\"", path)
    }

    pub fn list_dir(path: &str) -> String {
        let clean_path = path.trim_end_matches('/');
        let base = if clean_path.is_empty() {
            ""
        } else {
            clean_path
        };
        format!(
            "ls -a1 \"{}\" | while read line; do [ \"$line\" = \".\" ] || [ \"$line\" = \"..\" ] || stat -L -c \"%n|%s|%Y|%F\" \"{}/$line\" 2>/dev/null; done",
            path, base
        )
    }

    pub fn rm_rf(path: &str) -> String {
        format!("rm -rf \"{}\"", path)
    }

    pub fn mv(old: &str, new: &str) -> String {
        format!("mv \"{}\" \"{}\"", old, new)
    }

    pub fn cp_r(source: &str, target: &str) -> String {
        format!("cp -r \"{}\" \"{}\"", source, target)
    }

    pub fn sha256sum(path: &str) -> String {
        format!("sha256sum \"{}\" 2>/dev/null", path)
    }
}
