use itertools::izip;
use rustpython_ast::Constant;
use rustpython_parser::ast::{Cmpop, Expr, ExprKind};

use crate::ast::types::Range;
use crate::registry::Diagnostic;
use crate::violations;

/// E721
pub fn type_comparison(ops: &[Cmpop], comparators: &[Expr], location: Range) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = vec![];

    for (op, right) in izip!(ops, comparators) {
        if !matches!(op, Cmpop::Is | Cmpop::IsNot | Cmpop::Eq | Cmpop::NotEq) {
            continue;
        }
        match &right.node {
            ExprKind::Call { func, args, .. } => {
                if let ExprKind::Name { id, .. } = &func.node {
                    // Ex) type(False)
                    if id == "type" {
                        if let Some(arg) = args.first() {
                            // Allow comparison for types which are not obvious.
                            if !matches!(
                                arg.node,
                                ExprKind::Name { .. }
                                    | ExprKind::Constant {
                                        value: Constant::None,
                                        kind: None
                                    }
                            ) {
                                diagnostics
                                    .push(Diagnostic::new(violations::TypeComparison, location));
                            }
                        }
                    }
                }
            }
            ExprKind::Attribute { value, .. } => {
                if let ExprKind::Name { id, .. } = &value.node {
                    // Ex) types.IntType
                    if id == "types" {
                        diagnostics.push(Diagnostic::new(violations::TypeComparison, location));
                    }
                }
            }
            _ => {}
        }
    }

    diagnostics
}
