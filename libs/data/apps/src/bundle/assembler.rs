use crate::model::SplitApkBundle;
use std::collections::HashMap;

pub struct SplitBundleAssembler;

impl SplitBundleAssembler {
    /// Groups individual split APK paths by their package name.
    pub fn group_splits(apk_entries: &[(String, String, u64)]) -> Vec<SplitApkBundle> {
        let mut map: HashMap<String, SplitApkBundle> = HashMap::new();

        for (pkg_name, filename, size) in apk_entries {
            let bundle = map
                .entry(pkg_name.clone())
                .or_insert_with(|| SplitApkBundle::new(pkg_name, 0));

            bundle.add_split(filename, *size);
        }

        map.into_values().collect()
    }

    /// Checks if split bundle is ready for installation (contains base.apk).
    pub fn is_installable(bundle: &SplitApkBundle) -> bool {
        bundle.files.iter().any(|f| f.split_type == crate::model::ApkSplitType::Base)
    }
}
