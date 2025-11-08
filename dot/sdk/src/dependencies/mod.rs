//! Dependency checking for different recipe pathways
//!
//! This module provides functionality to detect whether required dependencies
//! are installed for each recipe pathway (Polkadot SDK, Solidity, XCM, etc.)

use crate::config::RecipePathway;
use std::process::Command;

/// Represents a single dependency requirement
#[derive(Debug, Clone, PartialEq)]
pub struct Dependency {
    /// Human-readable name of the dependency
    pub name: String,
    /// Command to check for existence (e.g., "rustc", "node")
    pub command: String,
    /// Minimum required version (if any)
    pub min_version: Option<String>,
    /// URL to installation page
    pub install_url: String,
    /// Platform-specific installation instructions
    pub install_instructions: String,
}

/// Result of a dependency check
#[derive(Debug, Clone, PartialEq)]
pub struct DependencyCheckResult {
    /// The dependency that was checked
    pub dependency: Dependency,
    /// Whether the dependency is installed
    pub installed: bool,
    /// Detected version (if installed)
    pub version: Option<String>,
}

/// Check if a command exists in PATH
fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get version of a command
fn get_version(command: &str, version_flag: &str) -> Option<String> {
    Command::new(command)
        .arg(version_flag)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

impl Dependency {
    /// Check if this dependency is installed
    pub fn check(&self) -> DependencyCheckResult {
        let installed = command_exists(&self.command);
        let version = if installed {
            get_version(&self.command, "--version")
        } else {
            None
        };

        DependencyCheckResult {
            dependency: self.clone(),
            installed,
            version,
        }
    }
}

/// Get required dependencies for a pathway
pub fn get_pathway_dependencies(pathway: &RecipePathway) -> Vec<Dependency> {
    match pathway {
        RecipePathway::Runtime => vec![
            Dependency {
                name: "Rust".to_string(),
                command: "rustc".to_string(),
                min_version: Some("1.80.0".to_string()),
                install_url: "https://rustup.rs/".to_string(),
                install_instructions: "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh".to_string(),
            },
            Dependency {
                name: "Cargo".to_string(),
                command: "cargo".to_string(),
                min_version: None,
                install_url: "https://rustup.rs/".to_string(),
                install_instructions: "Cargo is installed with Rust via rustup".to_string(),
            },
        ],
        RecipePathway::Contracts => vec![
            Dependency {
                name: "Node.js".to_string(),
                command: "node".to_string(),
                min_version: Some("20.0.0".to_string()),
                install_url: "https://nodejs.org/".to_string(),
                install_instructions: "Download from https://nodejs.org/ or use a version manager like nvm:\n  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash\n  nvm install 20".to_string(),
            },
            Dependency {
                name: "npm".to_string(),
                command: "npm".to_string(),
                min_version: Some("10.0.0".to_string()),
                install_url: "https://nodejs.org/".to_string(),
                install_instructions: "npm is installed with Node.js".to_string(),
            },
            Dependency {
                name: "npx".to_string(),
                command: "npx".to_string(),
                min_version: None,
                install_url: "https://nodejs.org/".to_string(),
                install_instructions: "npx is installed with npm".to_string(),
            },
            Dependency {
                name: "Hardhat (optional)".to_string(),
                command: "hardhat".to_string(),
                min_version: None,
                install_url: "https://hardhat.org/".to_string(),
                install_instructions: "npm install -g hardhat (optional - can be installed per-project)".to_string(),
            },
        ],
        RecipePathway::Xcm | RecipePathway::BasicInteraction | RecipePathway::Testing => vec![
            Dependency {
                name: "Node.js".to_string(),
                command: "node".to_string(),
                min_version: Some("20.0.0".to_string()),
                install_url: "https://nodejs.org/".to_string(),
                install_instructions: "Download from https://nodejs.org/ or use a version manager like nvm:\n  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash\n  nvm install 20".to_string(),
            },
            Dependency {
                name: "npm".to_string(),
                command: "npm".to_string(),
                min_version: Some("10.0.0".to_string()),
                install_url: "https://nodejs.org/".to_string(),
                install_instructions: "npm is installed with Node.js".to_string(),
            },
            Dependency {
                name: "npx".to_string(),
                command: "npx".to_string(),
                min_version: None,
                install_url: "https://nodejs.org/".to_string(),
                install_instructions: "npx is installed with npm".to_string(),
            },
        ],
        RecipePathway::RequestNew => vec![], // No dependencies for request-new
    }
}

/// Check all dependencies for a pathway
pub fn check_pathway_dependencies(pathway: &RecipePathway) -> Vec<DependencyCheckResult> {
    get_pathway_dependencies(pathway)
        .iter()
        .map(|dep| dep.check())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_dependencies() {
        let deps = get_pathway_dependencies(&RecipePathway::Runtime);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].name, "Rust");
        assert_eq!(deps[1].name, "Cargo");
    }

    #[test]
    fn test_contracts_dependencies() {
        let deps = get_pathway_dependencies(&RecipePathway::Contracts);
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].name, "Node.js");
        assert_eq!(deps[1].name, "npm");
        assert_eq!(deps[2].name, "npx");
        assert_eq!(deps[3].name, "Hardhat (optional)");
    }

    #[test]
    fn test_xcm_dependencies() {
        let deps = get_pathway_dependencies(&RecipePathway::Xcm);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].name, "Node.js");
    }

    #[test]
    fn test_request_new_no_dependencies() {
        let deps = get_pathway_dependencies(&RecipePathway::RequestNew);
        assert_eq!(deps.len(), 0);
    }

    #[test]
    fn test_dependency_check() {
        let dep = Dependency {
            name: "Rust".to_string(),
            command: "rustc".to_string(),
            min_version: None,
            install_url: "https://rustup.rs/".to_string(),
            install_instructions: "curl https://sh.rustup.rs | sh".to_string(),
        };

        let result = dep.check();
        // Should be installed since we're running tests
        assert_eq!(result.dependency.name, "Rust");
        // We don't assert installed=true because CI might not have it
    }
}
