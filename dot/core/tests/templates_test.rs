//! Tests for the templates module
//!
//! This tests the Template trait and all template implementations

use polkadot_cookbook_core::templates::*;

#[test]
fn test_all_templates_return_non_empty() {
    // JustfileTemplate
    let justfile = JustfileTemplate::new();
    let content = justfile.generate();
    assert!(
        !content.is_empty(),
        "JustfileTemplate should return non-empty content"
    );
    assert!(content.len() > 10, "JustfileTemplate content too short");

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

    // RecipeYmlTemplate
    use polkadot_cookbook_core::config::RecipeType;
    let recipe_yml = RecipeYmlTemplate::new(
        "test-recipe",
        "Test Recipe",
        "A test description",
        RecipeType::PolkadotSdk,
        "test-cat",
        true,
    );
    let content = recipe_yml.generate();
    assert!(
        !content.is_empty(),
        "RecipeYmlTemplate should return non-empty content"
    );
    assert!(content.len() > 20, "RecipeYmlTemplate content too short");
}

#[test]
fn test_templates_contain_expected_markers() {
    // Each template should contain specific markers that identify them

    let justfile = JustfileTemplate::new().generate();
    assert!(justfile.contains("default:"));
    assert!(justfile.contains("@just"));

    let readme = ReadmeTemplate::new("my-test").generate();
    assert!(readme.contains("# My Test"));
    assert!(readme.contains("Prerequisites"));

    let test = TestTemplate::new("my-test").generate();
    assert!(test.contains("describe"));
    assert!(test.contains("@polkadot/api"));

    use polkadot_cookbook_core::config::RecipeType;
    let recipe_yml = RecipeYmlTemplate::new(
        "my-test",
        "My Test",
        "Description",
        RecipeType::PolkadotSdk,
        "test-cat",
        true,
    )
    .generate();
    assert!(recipe_yml.contains("slug: my-test"));
    assert!(recipe_yml.contains("name: My Test"));
}

#[test]
fn test_template_trait_implemented() {
    use polkadot_cookbook_core::config::RecipeType;
    // Verify all templates implement the Template trait
    fn assert_template<T: Template>(_t: &T) {}

    assert_template(&JustfileTemplate::new());
    assert_template(&ReadmeTemplate::new("test"));
    assert_template(&TestTemplate::new("test"));
    assert_template(&RecipeYmlTemplate::new(
        "test",
        "Test",
        "Desc",
        RecipeType::PolkadotSdk,
        "cat",
        true,
    ));
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

        // Should contain frontmatter
        assert!(content.contains("---\n"));

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
fn test_recipe_yml_template_variations() {
    use polkadot_cookbook_core::config::RecipeType;
    // Test with different inputs
    let template1 = RecipeYmlTemplate::new(
        "slug-1",
        "Title 1",
        "Desc 1",
        RecipeType::PolkadotSdk,
        "cat1",
        true,
    );
    let content1 = template1.generate();
    assert!(content1.contains("slug-1"));
    assert!(content1.contains("Title 1"));
    assert!(content1.contains("Desc 1"));

    let template2 = RecipeYmlTemplate::new(
        "complex-slug",
        "Complex Title",
        "A longer description with multiple words",
        RecipeType::Solidity,
        "cat2",
        false,
    );
    let content2 = template2.generate();
    assert!(content2.contains("complex-slug"));
    assert!(content2.contains("Complex Title"));
    assert!(content2.contains("A longer description"));
    assert!(content2.contains("type: solidity"));
    assert!(content2.contains("needs_node: false"));
}

#[test]
fn test_justfile_contains_all_recipes() {
    let justfile = JustfileTemplate::new().generate();

    // Should contain basic recipes
    assert!(justfile.contains("default"));
    assert!(justfile.contains("say-hello"));
    assert!(justfile.contains("@just --list"));
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

    let justfile1 = JustfileTemplate::new().generate();
    let justfile2 = JustfileTemplate::new().generate();
    assert_eq!(justfile1, justfile2);
}
