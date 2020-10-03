#[test]
fn test_changelog_mentions_version() {
    version_sync::assert_contains_regex!("CHANGELOG.md", r"^## \[{version}] - \d{4}-\d{2}-\d{2}");
}
