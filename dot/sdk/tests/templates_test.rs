//! Tests for the templates module
//!
//! This tests the Template trait and all template implementations

use polkadot_cookbook_sdk::templates::*;

#[test]
fn test_all_templates_return_non_empty() {
    // ReadmeTemplate
    let readme = ReadmeTemplate::new("test-recipe");
    let content = readme.generate();
    assert!(
        !content.is_empty(),
        "ReadmeTemplate should return non-empty content"
    );
    assert!(content.len() > 100, "ReadmeTemplate content too short");

    // TestTemplate
    let test = TestTemplate::new("test-recipe");
    let content = test.generate();
    assert!(
        !content.is_empty(),
        "TestTemplate should return non-empty content"
    );
    assert!(content.len() > 50, "TestTemplate content too short");
}

#[test]
fn test_templates_contain_expected_markers() {
    // Each template should contain specific markers that identify them

    let readme = ReadmeTemplate::new("my-test").generate();
    assert!(readme.contains("# My Test"));
    assert!(readme.contains("Prerequisites"));

    let test = TestTemplate::new("my-test").generate();
    assert!(test.contains("describe"));
    assert!(test.contains("@polkadot/api"));
}

#[test]
fn test_template_trait_implemented() {
    // Verify all templates implement the Template trait
    fn assert_template<T: Template>(_t: &T) {}

    assert_template(&ReadmeTemplate::new("test"));
    assert_template(&TestTemplate::new("test"));
}

#[test]
fn test_readme_template_with_different_slugs() {
    let slugs = vec![
        "simple",
        "multi-word-slug",
        "with-numbers-123",
        "very-long-slug-name-for-testing",
    ];

    for slug in slugs {
        let readme = ReadmeTemplate::new(slug);
        let content = readme.generate();

        // Should start with heading
        assert!(content.starts_with("# "));

        // Should contain slug in path
        assert!(content.contains(&format!("cd recipes/{}", slug)));

        // Should not be empty
        assert!(content.len() > 100);
    }
}

#[test]
fn test_test_template_with_different_slugs() {
    let slugs = vec!["test-a", "test-b", "my-recipe"];

    for slug in slugs {
        let test = TestTemplate::new(slug);
        let content = test.generate();

        // Should contain the slug in test description
        assert!(content.contains(slug));

        // Should contain test structure
        assert!(content.contains("describe"));
        assert!(content.contains("it("));
    }
}

#[test]
fn test_template_consistency() {
    // Generate same template multiple times, should get same output
    let slug = "consistency-test";

    let readme1 = ReadmeTemplate::new(slug).generate();
    let readme2 = ReadmeTemplate::new(slug).generate();
    assert_eq!(readme1, readme2);

    let test1 = TestTemplate::new(slug).generate();
    let test2 = TestTemplate::new(slug).generate();
    assert_eq!(test1, test2);
}
