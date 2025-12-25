// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::zeniths::registry::ZenithRegistry;
use colored::*;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;

pub struct EnvironmentChecker;

pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
    pub category: String,
}

pub struct DoctorSummary {
    pub total_tools: usize,
    pub available_tools: usize,
    pub missing_tools: usize,
    pub categories: HashMap<String, CategorySummary>,
}

pub struct CategorySummary {
    pub total: usize,
    pub available: usize,
}

impl EnvironmentChecker {
    pub fn check_tool(tool: &str, category: &str) -> ToolStatus {
        match Command::new(tool).arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .map(|s| s.trim().to_string());
                ToolStatus {
                    name: tool.to_string(),
                    available: true,
                    version,
                    category: category.to_string(),
                }
            }
            _ => ToolStatus {
                name: tool.to_string(),
                available: false,
                version: None,
                category: category.to_string(),
            },
        }
    }

    pub fn check_all(registry: Arc<ZenithRegistry>) -> Vec<ToolStatus> {
        let mut tool_categories: HashMap<String, String> = HashMap::new();
        for zenith in registry.list_all() {
            let category = Self::get_tool_category(zenith.name());
            tool_categories.insert(zenith.name().to_string(), category);
        }

        let mut results = Vec::new();
        for (tool, category) in tool_categories {
            results.push(Self::check_tool(&tool, &category));
        }
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }

    pub fn get_tool_category(tool_name: &str) -> String {
        match tool_name {
            "rust" => "Rust",
            "python" => "Python",
            "prettier" => "JavaScript/TypeScript",
            "clang" => "C/C++",
            "java" => "Java",
            "shell" => "Shell",
            "toml" => "Configuration",
            "ini" => "Configuration",
            _ => "Other",
        }
        .to_string()
    }

    pub fn generate_summary(results: &[ToolStatus]) -> DoctorSummary {
        let mut categories: HashMap<String, CategorySummary> = HashMap::new();
        let mut total_tools = 0;
        let mut available_tools = 0;

        for result in results {
            total_tools += 1;
            if result.available {
                available_tools += 1;
            }

            let category_summary =
                categories
                    .entry(result.category.clone())
                    .or_insert(CategorySummary {
                        total: 0,
                        available: 0,
                    });
            category_summary.total += 1;
            if result.available {
                category_summary.available += 1;
            }
        }

        DoctorSummary {
            total_tools,
            available_tools,
            missing_tools: total_tools - available_tools,
            categories,
        }
    }

    pub fn print_results(results: &[ToolStatus], verbose: bool) -> DoctorSummary {
        let summary = Self::generate_summary(results);

        println!("\n{}", "Tool Environment Check:".bold().underline());
        println!();

        let mut current_category = String::new();
        for res in results {
            if res.category != current_category {
                if !current_category.is_empty() {
                    println!();
                }
                println!("{}", format!("{}:", res.category).cyan().bold());
                current_category = res.category.clone();
            }

            let status = if res.available {
                "✅ Available".green()
            } else {
                "❌ Not Found".red()
            };

            print!("  {:<20} {}", res.name.bold(), status);
            if let Some(v) = &res.version {
                if verbose {
                    print!(" ({})", v.dimmed());
                }
            }
            println!();
        }

        println!();
        println!("{}", "Summary:".bold().underline());
        println!(
            "  Total Tools:    {}",
            summary.total_tools.to_string().white()
        );
        println!(
            "  Available:      {}",
            summary.available_tools.to_string().green()
        );
        println!(
            "  Missing:        {}",
            summary.missing_tools.to_string().red()
        );

        if !summary.categories.is_empty() {
            println!();
            println!("{}", "By Category:".bold());
            for (category, cat_summary) in &summary.categories {
                let status = if cat_summary.available == cat_summary.total {
                    "✓".green()
                } else {
                    "✗".red()
                };
                println!(
                    "  {} {} ({}/{})",
                    status, category, cat_summary.available, cat_summary.total
                );
            }
        }

        summary
    }
}
