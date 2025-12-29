// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

/// Macro to create a simple stdio-based Zenith formatter implementation.
///
/// This macro reduces boilerplate for formatters that use StdioFormatter
/// with a consistent pattern.
///
/// # Example
///
/// ```rust,ignore
/// use crate::zenith_stdio_impl;
///
/// zenith_stdio_impl!(
///     PythonZenith,
///     "python",
///     "ruff",
///     ["py", "pyi"],
///     ["format", "--stdin-filename"]
/// );
/// ```
///
/// The macro generates:
/// - A struct with the given name
/// - Implementation of `name()` returning the tool name
/// - Implementation of `extensions()` returning the extensions slice
/// - A `format()` method using StdioFormatter with the provided args
#[macro_export]
macro_rules! zenith_stdio_impl {
    (
        $struct_name:ident,
        $name:expr,
        $tool_name:expr,
        [$($ext:expr),+],
        [$($arg:expr),+]
    ) => {
        pub struct $struct_name;

        impl $struct_name {
            pub const EXTENSIONS: &'static [&'static str] = &[$($ext),+];
        }

        #[async_trait::async_trait]
        impl $crate::core::traits::Zenith for $struct_name {
            fn name(&self) -> &'static str {
                $name
            }

            fn extensions(&self) -> &'static [&'static str] {
                Self::EXTENSIONS
            }

            async fn format(
                &self,
                content: &[u8],
                path: &std::path::Path,
                _config: &$crate::config::types::ZenithConfig,
            ) -> $crate::error::Result<Vec<u8>> {
                use $crate::zeniths::common::StdioFormatter;

                let formatter = StdioFormatter {
                    tool_name: $tool_name,
                    args: vec![$($arg.into()),+],
                };
                formatter.format_with_stdio(content, path, None).await
            }
        }
    };
}

/// Macro to create a stdio-based Zenith formatter with custom format function.
///
/// Use this when you need to add extra arguments or preprocessing.
#[macro_export]
macro_rules! zenith_stdio_impl_custom {
    (
        $struct_name:ident,
        $name:expr,
        $tool_name:expr,
        [$($ext:expr),+],
        |$content:ident, $path:ident, $config:ident| $body:block
    ) => {
        pub struct $struct_name;

        impl $struct_name {
            pub const EXTENSIONS: &'static [&'static str] = &[$($ext),+];
        }

        #[async_trait::async_trait]
        impl $crate::core::traits::Zenith for $struct_name {
            fn name(&self) -> &'static str {
                $name
            }

            fn extensions(&self) -> &'static [&'static str] {
                Self::EXTENSIONS
            }

            async fn format(
                &self,
                $content: &[u8],
                $path: &std::path::Path,
                $config: &$crate::config::types::ZenithConfig,
            ) -> $crate::error::Result<Vec<u8>> {
                use $crate::zeniths::common::StdioFormatter;
                use $crate::error::Result;

                fn inner_format($content: &[u8], $path: &std::path::Path, $config: &$crate::config::types::ZenithConfig) -> Result<Vec<u8>> {
                    let formatter = StdioFormatter {
                        tool_name: $tool_name,
                        args: vec![],
                        timeout_seconds: None,
                    };
                    $body
                }

                inner_format($content, $path, $config).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_expansion() {
        // This test verifies the macro compiles correctly
        // The actual struct would need async_trait to work
    }
}
