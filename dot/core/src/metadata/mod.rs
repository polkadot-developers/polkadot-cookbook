/// Metadata extraction and recipe detection
pub mod detection;
pub mod frontmatter;

pub use detection::{detect_recipe_type, RecipeDetectionError};
pub use frontmatter::{
    parse_frontmatter, parse_frontmatter_from_file, FrontmatterData, FrontmatterError,
};
