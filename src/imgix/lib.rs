// Rust Standard Library Imports.
use std::fmt::{self, Display};

// Module declarations.
pub mod constants;
pub mod util;

/// Re-exports.
pub use util::command_prelude;
pub use util::errors::{Error, Result};

/// Primary structure used to generate imgix URLs.
///
/// An imgix URL is comprised of four components:
///
/// * scheme - the scheme being used (https by default).
/// * domain - the imgix domain, i.e. example.imgix.net.
/// * path - the path to the image file.
/// * query - the query string constructed from `params`.
///
/// ```text
///             domain
///         ┌──────┴──────┐
/// https://www.example.com/image/path.png/?w=320&h=640
/// └─┬─┘                  └──────┬───────┘ └────┬────┘
/// scheme                      path           params
/// ```
pub struct Url {
    /// The scheme component of a URL, i.e. https, http, etc.
    scheme: Scheme,
    /// The imgix domain, i.e. example.imgix.net
    domain: String,
    /// The imgix library generating the `Url`.
    lib: String,
    /// The path to the image file.
    path: Option<String>,
    /// The parameters used to construct the query string.
    ///
    /// This structure is a _key-value list_ and been chosen over HashMap,
    /// BTreeMap, and BTreeSet for the following reasons:
    ///
    /// * to give users __flexibility__, by accepting a range of inputs
    /// * to seek __consistency__, by parameters are ordered
    ///   as they are defined, WYSIWYG
    /// * to give users control
    ///
    /// The query-string is built up during a single iterative pass over this
    /// key-value list, visiting each key-value pair in the order the user
    /// has specified. Therefore, the order in which parameters are listed
    /// is the same order they will appear in the generated `Url`'s query
    /// string.
    params: Vec<(&'static str, &'static str)>,
    /// Optional signing token used to sign URLs.
    token: Option<String>,
}

impl Default for Url {
    /// By default a imgix URL is created with its `scheme` set
    /// to `Scheme::Https` and the `lib` value set to the version
    /// specified in this library's Cargo.toml
    fn default() -> Self {
        Url {
            scheme: Scheme::Https,
            domain: String::new(),
            lib: constants::lib_version(),
            params: vec![],
            path: None,
            token: None,
        }
    }
}

impl Url {
    /// Construct a new `Url` given a domain.
    ///
    /// # Panics
    ///
    /// This constructor will fail if the `domain` is an empty string.
    pub fn new(domain: &'static str) -> Self {
        assert!(!domain.is_empty());
        Url {
            domain: String::from(domain),
            ..Default::default()
        }
    }

    /// Set the domain value (i.e. "example.domain.com").
    ///
    /// # Panics
    ///
    /// This method panics if the domain `h` is an empty string.
    pub fn domain(mut self, h: &str) -> Self {
        assert!(!h.is_empty());
        self.domain = String::from(h);
        self
    }

    /// Set the path value to the image file (i.e. 'image/path.png').
    ///
    /// # Panics
    ///
    /// This method panics if the path string `r` is empty (i.e. "").
    pub fn path(mut self, r: &str) -> Self {
        assert!(!r.is_empty());
        self.path = Some(String::from(r));
        self
    }

    /// Set an arbitrary key-value parameter (i.e. k='w', v='100'
    /// or k='fit', v='crop').
    ///
    /// # Panics
    ///
    /// This method panics if the key `k` or the value `v` is an empty string.
    pub fn param(mut self, k: &'static str, v: &'static str) -> Self {
        assert!(!k.is_empty());
        assert!(!v.is_empty());
        self.params.push((k, v));
        self
    }

    /// Set an arbitrary number of key-value parameters.
    ///
    /// # Panics
    ///
    /// This method panics if any key `k` or any value `v` is an empty string.
    pub fn params(mut self, p: &[(&'static str, &'static str)]) -> Self {
        for (k, v) in p.iter() {
            assert!(!k.is_empty());
            assert!(!v.is_empty());
            self.params.push((k, v));
        }
        self
    }

    /// Set the `lib` or library.
    pub fn lib(mut self, l: &str) -> Self {
        self.lib = String::from(l);
        self
    }

    /// Set the signing `token`.
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(String::from(token));
        self
    }

    // Set the `scheme` value (i.e. Scheme::Https).
    pub fn scheme(mut self, s: Scheme) -> Self {
        self.scheme = s;
        self
    }

    /// Join the components of a `Url` (i.e. `scheme` + `domain` + `path` +
    /// `params`) where the resulting string has the following form:
    ///
    /// {scheme}://{domain}/{path}?{lib}{query}
    ///
    /// This function will only `join` the components of a `Url` if a `path`
    /// has been specified.
    ///
    /// # Panics
    ///
    /// This function will panic if the image `path` has not been specified.
    /// (i.e. if the `path` is `None`). This is to ensure that a `Url` is
    /// joined if it is in a _valid_ state.
    pub fn join(&self) -> String {
        // Join this url, only-if a `path` has been specified.
        match self.path {
            Some(ref path) => {
                let query = Self::join_params(&self.params);

                format!(
                    "{scheme}://{domain}/{path}?{lib}{query}",
                    scheme = self.scheme,
                    domain = self.domain,
                    path = path,
                    lib = self.lib,
                    query = query,
                )
            }
            None => {
                panic!("failed: cannot `Url::join` when `path` is `None`.");
            }
        }
    }

    /// Join a list of key-value parameter pairs.
    ///
    /// # Examples
    ///
    /// # Panics
    ///
    /// This function panics if any key `k` or any value `v` is an empty string.
    pub fn join_params(p: &[(&'static str, &'static str)]) -> String {
        let mut result = String::new();

        // I the parameter list is empty, do no work.
        if p.is_empty() {
            return result;
        }

        // Otherwise, construct the result by appending parameters one after another
        // (i.e. {key}={value}{"&" | ""}).
        // The result has the form: k0=v0&k1=v1&k2=v2
        let mut it = 1usize;
        let end = p.len();
        for (k, v) in p.iter() {
            assert!(!k.is_empty());
            assert!(!v.is_empty());
            result.push_str(k);
            result.push('=');
            result.push_str(v);

            // Avoid pushing a trailing '&' if there are no more parameter pairs.
            if it < end {
                result.push('&');
            }
            it += 1;
        }
        return result;
    }
}

#[derive(Debug, PartialEq)]
pub enum Scheme {
    Https,
    Http,
}

impl Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scheme::Https => write!(f, "{}", "https"),
            Scheme::Http => write!(f, "{}", "http"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const HTTPS: &str = "https";
    const HTTP: &str = "http";
    const HOST: &str = "test.domain.com";
    const PNG_PATH: &str = "test-image.png";

    #[test]
    fn test_join_params() {
        let left = Url::join_params(&[("w", "300")]);
        assert_eq!(left, String::from("w=300"));

        let left = Url::join_params(&[("w", "300"), ("h", "600")]);
        assert_eq!(left, String::from("w=300&h=600"));

        let left = Url::join_params(&[("w", "300"), ("h", "600"), ("fit", "crop")]);
        assert_eq!(left, String::from("w=300&h=600&fit=crop"));
    }

    #[test]
    fn test_basic_imgix_url() {
        let right = format!(
            "{scheme}://{domain}/{path}?{ixlib}",
            scheme = HTTPS,
            domain = HOST,
            path = PNG_PATH,
            ixlib = constants::lib_version(),
        );
        let url = Url::new(HOST).path(PNG_PATH);

        // Test all fields.
        assert_eq!(url.scheme, Scheme::Https);
        assert_eq!(url.domain, HOST);
        assert_eq!(url.lib, constants::lib_version());
        assert_eq!(url.path, Some(String::from(PNG_PATH)));
        assert!(url.params.is_empty());
        assert!(url.token.is_none());

        // Test the joined url.
        assert_eq!(url.join(), right);
    }

    #[test]
    fn test_basic_imgix_url_scheme() {
        let right = format!(
            "{scheme}://{domain}/{path}?{ixlib}",
            scheme = HTTP,
            domain = HOST,
            path = PNG_PATH,
            ixlib = constants::lib_version()
        );

        // Construct a url with http scheme.
        // Note: https is the default scheme.
        let url = Url::new(HOST).path(PNG_PATH).scheme(Scheme::Http);

        assert_eq!(url.scheme, Scheme::Http);
        assert_eq!(url.domain, HOST);
        assert_eq!(url.lib, constants::lib_version());
        assert_eq!(url.path, Some(String::from(PNG_PATH)));
        assert!(url.params.is_empty());
        assert_eq!(url.join(), right);

        // Now switch back to https.
        let url = url.scheme(Scheme::Https);
        assert_eq!(url.scheme, Scheme::Https);
    }
}
