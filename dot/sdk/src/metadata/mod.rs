/// Metadata extraction and project detection
/// Project type auto-detection from file system
pub mod detection;
/// YAML frontmatter parsing for README files
pub mod frontmatter;

pub use detection::{detect_project_type, ProjectDetectionError};
pub use frontmatter::{
    parse_frontmatter, parse_frontmatter_from_file, FrontmatterData, FrontmatterError,
};
