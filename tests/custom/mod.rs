use crate::helpers::defaults;
use std::path::Path;

const STORAGE: &[(&str, &str)] = &[(
    defaults::paths::resources::stuttgart_regbez::DIR,
    defaults::paths::resources::stuttgart_regbez::URL,
)];

#[test]
pub fn are_files_complete() {
    let osmgraphing_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for (dir, url) in STORAGE {
        let path = osmgraphing_dir.join(dir);
        if !path.exists() {
            panic!(
                "You specified the feature 'custom', but a needed custom resource does not exist.\n\
                \n\
                +------------------------------------------------------------------------------+\n\
                Missing '{}'\n\
                \n\
                You may download it with\n\
                \n\
                wget -O '{}.tar.xz' '{}'\n\
                \n\
                and extract it with\n\
                \n\
                tar --extract --file '{}.tar.xz' --directory '{}'\n\
                +------------------------------------------------------------------------------+\n\
                \n",
                dir,
                dir,
                url,
                dir,
                defaults::paths::resources::DIR
            );
        }
    }
}
