//! This internal-module defines an intermediate format used for parsing test
//! options from the [`crate::neotest`] attribute definition.
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Eq};
use syn::Result;

use super::TestInputs;

/// An option argument that can be specified as part of the [`neotest`] attribute.
///
/// All options contain their respective idents.
///
/// [`neotest`]: crate::neotest
pub enum TestOption {
  Fixture(syn::Ident),
  Parameter(syn::Ident),
  TypeParameter(syn::Ident),
  ConstParameter(syn::Ident),
}

impl Parse for TestOption {
  fn parse(input: ParseStream) -> Result<Self> {
    let option: syn::Ident = input.parse()?;
    let option_str = option.to_string();

    match &*option_str {
      "fixture" => Ok(TestOption::Fixture(option)),
      "parameter" => Ok(TestOption::Parameter(option)),
      "type_parameter" => Ok(TestOption::TypeParameter(option)),
      "const_parameter" => Ok(TestOption::ConstParameter(option)),
      _ => Err(syn::Error::new(
        option.span(),
        format!("unknown argument '{option_str}'"),
      )),
    }
  }
}

impl Parse for TestInputs {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut result = TestInputs {
      fixture: None,
      parameters: Vec::default(),
      const_parameters: Vec::default(),
      type_parameters: Vec::default(),
    };

    while !input.is_empty() {
      let option: TestOption = input.parse()?;

      input.parse::<Eq>()?;

      match option {
        TestOption::Fixture(ident) => {
          if result.fixture.is_some() {
            return Err(syn::Error::new(
              ident.span(),
              "fixture argument can only be specified once",
            ));
          }
          result.fixture = Some(input.parse()?);
        }
        TestOption::Parameter(_) => {
          result.parameters.push(input.parse()?);
        }
        TestOption::TypeParameter(_) => {
          result.type_parameters.push(input.parse()?);
        }
        TestOption::ConstParameter(_) => {
          result.const_parameters.push(input.parse()?);
        }
      }
      if !input.is_empty() {
        input.parse::<Comma>()?;
      }
    }

    Ok(result)
  }
}
