[![Build][build-badge]][build-link]
[![Docs][docs-badge]][docs-link]
[![Apache 2.0 Licensed][license-apache-badge]][license-apache-link]
[![MIT Licensed][license-mit-badge]][license-mit-link]

# Rust xUnit Testing Framework

Neotest is a powerful and dynamic [xUnit][xunit-link] testing framework for Rust.

**🚧 Note:** This project is currently a work-in-progress.

## Features

* [x] **Test Tixtures** with custom setup and teardown to simplify test boilerplate
* [x] **Parameterized Testing** that generates all parameter input combinations (#1)
* [ ] **Generic-Parameterized Testing** which substitutes different types or
      `const` values for tests (#2)
* [ ] **Sub-tests** for more granular reporting of test failures (#3)

[xunit-link]: https://en.wikipedia.org/wiki/XUnit
[docs-badge]: https://github.com/bitwizeshift/neotest/actions/workflows/deploy-gh-pages.yaml/badge.svg
[docs-link]: https://bitwizeshift.github.io/neotest/neotest
[build-badge]: https://github.com/bitwizeshift/neotest/actions/workflows/build.yaml/badge.svg
[build-link]: https://github.com/bitwizeshift/neotest/actions/workflows/build.yaml
[license-apache-badge]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-apache-link]: http://www.apache.org/licenses/LICENSE-2.0
[license-mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: http://opensource.org/licenses/MIT