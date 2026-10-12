use super::locate::candidates;
use std::path::Path;

#[test]
fn searches_the_secrets_mount_before_project_files_without_duplicates() {
    let paths = candidates();
    let mount = paths
        .iter()
        .position(|path| path == Path::new("/vault/secrets/rustyroad.toml"))
        .expect("the secrets mount should be searched");
    let cwd = std::env::current_dir().unwrap();
    let local = paths
        .iter()
        .position(|path| path.parent() == Some(cwd.as_path()))
        .expect("the working directory should be searched");

    assert!(mount < local);
    for (index, path) in paths.iter().enumerate() {
        assert!(
            !paths[..index].contains(path),
            "{} is listed twice",
            path.display()
        );
    }
}
