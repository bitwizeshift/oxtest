use syn::{spanned::Spanned, Block, Expr, Item, Macro, Stmt};

use crate::input::SubtestInput;
use crate::syn_utils::TryIdent;

/// A node in the graph of sections.
///
/// This represents discovery of a `subtest!` macro invocation, which stores the
/// contents of its name, which discovery it is, and any subsections defined
/// within that.
pub struct Section {
  pub index: usize,
  pub name: syn::Ident,
  subsections: Vec<Section>,
}

/// The root node for a graph of sections.
///
/// This type only contains a list of subsection nodes.
#[derive(Default)]
pub struct SectionGraph {
  subsections: Vec<Section>,
}

impl Section {
  /// Returns a slice of sub-[`Section`] objects.
  pub fn subsections(&self) -> &[Section] {
    &self.subsections
  }
}

impl SectionGraph {
  /// Creates a [`SectionGraph`] by examining the [`Block`] for the presence of
  /// the [`subtest`] macro
  ///
  /// [`subtest`]: crate::subtest
  pub fn discover_subtests(block: &Block) -> syn::Result<Self> {
    let mut result = Self::default();

    examine_block(&mut result.subsections, block, false)?;

    Ok(result)
  }

  /// Returns a slice of sub-[`Section`] objects.
  pub fn subsections(&self) -> &[Section] {
    &self.subsections
  }
}

//-----------------------------------------------------------------------------
// Parsing Login
//-----------------------------------------------------------------------------

fn define_subsection<F>(sections: &mut Vec<Section>, name: syn::Ident, f: F) -> syn::Result<()>
where
  F: FnOnce(&mut Vec<Section>) -> syn::Result<()>,
{
  let mut section = Section {
    index: sections.len(),
    name,
    subsections: Default::default(),
  };
  f(&mut section.subsections)?;
  sections.push(section);
  Ok(())
}

fn examine_block(
  sections: &mut Vec<Section>,
  block: &Block,
  fail_on_macro: bool,
) -> syn::Result<()> {
  examine_stmts(sections, &block.stmts, fail_on_macro)?;
  Ok(())
}

fn examine_stmts(
  sections: &mut Vec<Section>,
  stmts: &[Stmt],
  fail_on_macro: bool,
) -> syn::Result<()> {
  for stmt in stmts {
    match stmt {
      Stmt::Semi(expr, _) => examine_expr(sections, expr, fail_on_macro)?,
      Stmt::Expr(expr) => examine_expr(sections, expr, fail_on_macro)?,
      Stmt::Item(Item::Macro(item_macro)) => {
        examine_macro(sections, &item_macro.mac, fail_on_macro)?
      }
      _ => { /* Ignore any non expressions */ }
    }
  }
  Ok(())
}

fn examine_expr(sections: &mut Vec<Section>, expr: &Expr, fail_on_macro: bool) -> syn::Result<()> {
  match expr {
    Expr::Macro(expr_macro) => examine_macro(sections, &expr_macro.mac, fail_on_macro)?,
    Expr::Loop(expr_loop) => examine_block(sections, &expr_loop.body, true)?,
    Expr::While(expr_while) => examine_block(sections, &expr_while.body, true)?,
    Expr::ForLoop(expr_while) => examine_block(sections, &expr_while.body, true)?,
    Expr::Match(expr_match) => {
      for arm in &expr_match.arms {
        examine_expr(sections, &arm.body, true)?;
      }
    }
    Expr::If(expr_if) => {
      examine_block(sections, &expr_if.then_branch, true)?;
      if let Some((_, expr)) = expr_if.else_branch.as_ref() {
        examine_expr(sections, expr, true)?;
      }
    }
    _ => {}
  }
  Ok(())
}

fn examine_macro(sections: &mut Vec<Section>, mac: &Macro, fail_on_macro: bool) -> syn::Result<()> {
  if mac.path.try_ident().map(|v| v.to_string()) == Some("subtest".to_string()) {
    if fail_on_macro {
      return Err(syn::Error::new(
        mac.span(),
        "Subtests cannot be defined in control-flow blocks like loop, while, for, or if. They must not be conditional."
      ));
    }
    let tokens: proc_macro::TokenStream = mac.tokens.clone().into();

    let input: SubtestInput = syn::parse(tokens)?;
    define_subsection(sections, input.ident, |sections| -> syn::Result<()> {
      examine_stmts(sections, &input.block.stmts, fail_on_macro)
    })?;
  }
  Ok(())
}
