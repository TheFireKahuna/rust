use std::path::PathBuf;

use tempfile::TempDir;

use crate::{BuildStamp, Config, Flags};

#[test]
fn test_smart_stamp_observes_dirty_tree_in_dry_run() {
    // Each bootstrap invocation has its own command cache.
    let hash = |path: &std::path::Path| {
        let context = crate::utils::tests::TestCtx::new();
        let config = context.config("build").create_config();
        let build = crate::Build::new(config);
        let builder = crate::core::builder::Builder::new(&build);
        assert!(builder.config.dry_run());
        super::generate_smart_stamp_hash(&builder, path, "revision")
    };
    let repository = TempDir::new().unwrap();
    let git = |args: &[&str]| {
        assert!(std::process::Command::new("git")
            .arg("-C").arg(repository.path()).args(args).status().unwrap().success());
    };
    git(&["init", "-q"]);
    let file = repository.path().join("tracked");
    std::fs::write(&file, "before\n").unwrap();
    git(&["add", "tracked"]);
    let before = hash(repository.path());
    std::fs::write(&file, "after\n").unwrap();
    let after = hash(repository.path());
    assert_ne!(before, after);
    assert_eq!(after, hash(repository.path()));
    let untracked = repository.path().join("new.cpp");
    std::fs::write(&untracked, "before").unwrap();
    let new_before = hash(repository.path());
    std::fs::write(&untracked, "after").unwrap();
    assert_ne!(new_before, hash(repository.path()));
}

#[test]
#[should_panic(expected = "prefix can not start or end with '.'")]
fn test_with_invalid_prefix() {
    let dir = TempDir::new().unwrap();
    BuildStamp::new(dir.path()).with_prefix(".invalid");
}

#[test]
#[should_panic(expected = "prefix can not start or end with '.'")]
fn test_with_invalid_prefix2() {
    let dir = TempDir::new().unwrap();
    BuildStamp::new(dir.path()).with_prefix("invalid.");
}

#[test]
fn test_is_up_to_date() {
    let dir = TempDir::new().unwrap();

    let mut build_stamp = BuildStamp::new(dir.path()).add_stamp("v1.0.0");
    build_stamp.write().unwrap();

    assert!(
        build_stamp.is_up_to_date(),
        "Expected stamp file to be up-to-date, but contents do not match the expected value."
    );

    build_stamp.stamp = "dummy value".to_owned();
    assert!(
        !build_stamp.is_up_to_date(),
        "Stamp should no longer be up-to-date as we changed its content right above."
    );

    build_stamp.remove().unwrap();
}

#[test]
fn test_with_prefix() {
    let dir = TempDir::new().unwrap();

    let stamp = BuildStamp::new(dir.path()).add_stamp("v1.0.0");
    assert_eq!(stamp.path.file_name().unwrap(), ".stamp");

    let stamp = stamp.with_prefix("test");
    let expected_filename = ".test-stamp";
    assert_eq!(stamp.path.file_name().unwrap(), expected_filename);

    let stamp = stamp.with_prefix("extra-prefix");
    let expected_filename = ".extra-prefix-test-stamp";
    assert_eq!(stamp.path.file_name().unwrap(), expected_filename);
}
