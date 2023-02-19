use syn::punctuated::Punctuated;
use syn::Token;

/// A struct containing parameter inputs that can be specified for a test.
///
/// Parameter inputs contain the identifier of the parameter being substituted,
/// along with an array of each possibly valid input.
///
/// This input is formed from the the `parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///     /* ... */
///     parameter = a as [0xdead, 0xbeef, 0xc0ffee],
///     parameter = b as ["hello", "world"],
///     /* ... */
/// )]
/// fn test_value(a: u32, b: &str) { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[derive(Clone)]
pub(crate) struct ParameterInput {
  pub ident: syn::Ident,
  pub inputs: syn::ExprArray,
}

impl syn::parse::Parse for ParameterInput {
  /// Parses the input from the parse stream
  ///
  /// Expected input is in the form `<ident> as [<expr0>, <expr1>, ...]`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// v as [1, 2, 3]
  /// ```
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident: syn::Ident = input.parse()?;
    input.parse::<Token![as]>()?;
    let inputs: syn::ExprArray = input.parse()?;

    Ok(ParameterInput { ident, inputs })
  }
}

/// A sequence of types, each with potential attribute specifications.
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct TypeSequence {
  pub bracket_token: syn::token::Bracket,
  pub elems: syn::punctuated::Punctuated<syn::Type, Token![,]>,
}

impl syn::parse::Parse for TypeSequence {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let content;
    let bracket_token = syn::bracketed!(content in input);
    let mut elems = Punctuated::new();

    while !content.is_empty() {
      let first: syn::Type = content.parse()?;
      elems.push_value(first);
      if content.is_empty() {
        break;
      }
      let punct = content.parse()?;
      elems.push_punct(punct);
    }

    Ok(TypeSequence {
      bracket_token,
      elems,
    })
  }
}

/// A struct containing generic Type parameter inputs that can be specified for
/// a test.
///
/// Type parameter inputs contain the identifier of the type-parameter being
/// substituted, along with a sequence of types to substituted -- modeled in
/// an array.
///
/// This input is formed from the the `type_parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///   /* ... */
///   type_parameter = T as [u32, u64, usize],
///   type_parameter = U as [std::string::String, f32]
///   /* ... */
/// )]
/// fn test_value<T,U>() { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct TypeParameterInput {
  pub ident: syn::Ident,
  pub inputs: TypeSequence,
}

impl syn::parse::Parse for TypeParameterInput {
  /// Parses the input from the parse stream
  ///
  /// Expected input is in the form `<ident> as [<type>, <type>, ...]`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// T as [u8, String, ::std::vec::Vec]
  /// ```
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident: syn::Ident = input.parse()?;
    input.parse::<Token![as]>()?;
    let inputs: TypeSequence = input.parse()?;

    Ok(TypeParameterInput { ident, inputs })
  }
}

/// A struct containing const parameter inputs that can be specified for a test.
///
/// `ConstParameterInput` inputs contain the identifier of the const-generic
/// parameter being substituted, along with an array of each possibly valid input.
///
/// This input is formed from the the `const_parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///   /* ... */
///   const_parameter = VALUE as [1, 2, 3],
///   /* ... */
/// )]
/// fn test_value<const VALUE: usize>() { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct ConstParameterInput {
  pub ident: syn::Ident,
  pub inputs: syn::ExprArray,
}

impl ConstParameterInput {}

impl syn::parse::Parse for ConstParameterInput {
  /// Parses the input from the parse stream
  ///
  /// Expected input is in the form `<ident> as [<expr>, <expr>, ...]`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// V as [0xdeadbeef, 0xbadf00d, 0xc0ffee]
  /// ```
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let result: ParameterInput = input.parse()?;

    Ok(ConstParameterInput {
      ident: result.ident,
      inputs: result.inputs,
    })
  }
}

/// A struct containing fixture input for a test.
///
/// `FixtureInput` names just the type name of a fixture test.
///
/// This input is formed from the the `const_parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///   /* ... */
///   fixture = Fixture,
///   /* ... */
/// )]
/// fn test_value(f: &Fixture) { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[derive(Clone)]
pub(crate) struct FixtureInput {
  pub ident: syn::Ident,
}

impl syn::parse::Parse for FixtureInput {
  /// Parses the fixture name from the parse stream
  ///
  /// Expected input is in the form of just `<ident>`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// TestFixture
  /// ```
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident: syn::Ident = input.parse()?;
    Ok(FixtureInput { ident })
  }
}

#[derive(Clone)]
pub(crate) struct TestInputs {
  pub fixture: Option<FixtureInput>,
  pub parameters: Vec<ParameterInput>,
  pub const_parameters: Vec<ConstParameterInput>,
  pub type_parameters: Vec<TypeParameterInput>,
}

/// An option argument that can be specified as part of the [`neotest`] attribute.
///
/// All options contain their respective idents.
///
/// [`neotest`]: crate::neotest
pub(crate) enum TestOption {
  Fixture(syn::Ident),
  Parameter(syn::Ident),
  TypeParameter(syn::Ident),
  ConstParameter(syn::Ident),
}

impl syn::parse::Parse for TestOption {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

impl syn::parse::Parse for TestInputs {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut result = TestInputs {
      fixture: None,
      parameters: Vec::default(),
      const_parameters: Vec::default(),
      type_parameters: Vec::default(),
    };

    while !input.is_empty() {
      let option: TestOption = input.parse()?;

      input.parse::<syn::Token![=]>()?;

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
        input.parse::<syn::Token![,]>()?;
      }
    }

    return Ok(result);
  }
}
