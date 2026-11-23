use serde::{Deserialize, Serialize};
use syn::visit::Visit;
use syn::{Expr, ItemFn};

#[derive(Debug, Serialize, Deserialize)]
pub struct AstCheckResult {
    pub passed: bool,
    pub hardcoded_literals_detected: usize,
    pub weak_assertions_detected: usize,
    pub mock_stubbing_detected: bool,
    pub violations: Vec<String>,
}

pub struct AstInspector;

impl AstInspector {
    pub fn inspect_source(code: &str) -> AstCheckResult {
        let mut violations = Vec::new();
        let mut hardcoded = 0;
        let mut weak_asserts = 0;

        if let Ok(file) = syn::parse_file(code) {
            let mut visitor = TestVisitor {
                weak_assertions: 0,
                violations: Vec::new(),
            };
            visitor.visit_file(&file);
            weak_asserts = visitor.weak_assertions;
            violations.extend(visitor.violations);
        } else {
            // Textual heuristics fallback
            if code.contains("assert!(true)") || code.contains("assert_eq!(1, 1)") {
                weak_asserts += 1;
                violations.push("Trivial assertion detected: assert!(true)".to_string());
            }
        }

        if code.contains("return true;") || code.contains("return Ok(())") && code.len() < 40 {
            hardcoded += 1;
            violations.push("Potential stub shortcut detected in core handler".to_string());
        }

        AstCheckResult {
            passed: violations.is_empty(),
            hardcoded_literals_detected: hardcoded,
            weak_assertions_detected: weak_asserts,
            mock_stubbing_detected: false,
            violations,
        }
    }
}

struct TestVisitor {
    weak_assertions: usize,
    violations: Vec<String>,
}

impl<'ast> Visit<'ast> for TestVisitor {
    fn visit_expr(&mut self, node: &'ast Expr) {
        if let Expr::Macro(m) = node {
            let path_str = m
                .mac
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            if path_str == "assert" {
                let tokens = m.mac.tokens.to_string();
                if tokens.trim() == "true" {
                    self.weak_assertions += 1;
                    self.violations.push("Weak assertion: 'assert!(true)' detected".to_string());
                }
            }
        }
        syn::visit::visit_expr(self, node);
    }
}
