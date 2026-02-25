use std::collections::HashMap;

use moonmath_types::{ContentPage, Taxonomy};

/// Build taxonomy indices from a list of content pages.
pub fn build_taxonomy(pages: &[ContentPage]) -> Taxonomy {
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();

    for page in pages {
        for tag in &page.frontmatter.tags {
            tags.entry(tag.clone())
                .or_default()
                .push(page.slug.clone());
        }
        if let Some(ref category) = page.frontmatter.category {
            categories
                .entry(category.clone())
                .or_default()
                .push(page.slug.clone());
        }
    }

    Taxonomy { tags, categories }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moonmath_types::Frontmatter;

    #[test]
    fn test_build_taxonomy() {
        let pages = vec![
            ContentPage {
                slug: "formulas/calculus/chain-rule".to_string(),
                frontmatter: Frontmatter {
                    title: "Chain Rule".to_string(),
                    tags: vec!["calculus".to_string(), "derivatives".to_string()],
                    category: Some("calculus".to_string()),
                    ..default_frontmatter()
                },
                html: String::new(),
                source: String::new(),
            },
            ContentPage {
                slug: "formulas/algebra/quadratic".to_string(),
                frontmatter: Frontmatter {
                    title: "Quadratic Formula".to_string(),
                    tags: vec!["algebra".to_string()],
                    category: Some("algebra".to_string()),
                    ..default_frontmatter()
                },
                html: String::new(),
                source: String::new(),
            },
        ];

        let taxonomy = build_taxonomy(&pages);
        assert_eq!(taxonomy.tags["calculus"].len(), 1);
        assert_eq!(taxonomy.tags["derivatives"].len(), 1);
        assert_eq!(taxonomy.tags["algebra"].len(), 1);
        assert_eq!(taxonomy.categories["calculus"].len(), 1);
    }

    fn default_frontmatter() -> Frontmatter {
        Frontmatter {
            title: String::new(),
            date: None,
            tags: Vec::new(),
            category: None,
            difficulty: None,
            interactive: None,
            latex: None,
            draft: false,
            weight: None,
            description: None,
            prerequisites: Vec::new(),
            lean4_status: None,
        }
    }
}
