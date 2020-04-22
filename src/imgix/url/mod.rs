// Rust Standard Library Imports.
use std::fmt::{self, Display};

use super::{constants, validate, Error};

/// Primary structure used to generate imgix URLs.
///
/// An *imgix* URL is comprised of four components:
///
/// * scheme - the scheme being used (https by default).
/// * domain - the domain, i.e. example.domain.net.
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
///
/// This structure is meant to be a crate primitive that
/// crate users _and_ contributors can use to build on. This
/// is part of the reason why many of the building functions
/// can panic. They panic to try to ensure invalid urls are
/// never constructed. This is to provide higher-level structures
/// certain guarantees about the representation of a `Url`.
pub struct Url {
    /// The scheme component of a URL, i.e. https, http, etc.
    scheme: Scheme,
    /// The domain, i.e. example.domain.net
    domain: String,
    /// The library generating the `Url`. If you want to turn on
    /// analytics, call `ix()` and the library version will be set
    /// to the value of `"ixlib=rust-0.1.0"`. It helps us help our
    /// users, but it's _your choice_ to _opt in_. Your code,
    /// Your choice. What you see is what you get (WYSIWYG).
    lib: String,
    /// The path to the image file, e.g. "ixlib=rust-0.1.0"
    path: Option<String>,
    /// The parameters used to construct the query string.
    ///
    /// This structure is a _key-value list_ and been chosen over HashMap,
    /// BTreeMap, and BTreeSet for the following reasons:
    ///
    /// * to give users __flexibility__, by accepting a range of inputs
    /// * to seek __consistency__, by ordering parameters in the order
    ///   they were defined, (WYSIWYG)
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
    /// By default a URL is created with its `scheme` set
    /// to `Scheme::Https` and the `lib` value set to the version
    /// specified in this library's Cargo.toml
    fn default() -> Self {
        Url {
            scheme: Scheme::Https,
            domain: "".to_owned(),
            lib: "".to_owned(),
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
        match validate::domain(&domain) {
            Ok(()) => Url {
                domain: String::from(domain),
                ..Default::default()
            },
            Err(e) => panic!("{}", e),
        }
    }

    /// Set the domain value (i.e. "example.domain.net").
    ///
    /// # Panics
    ///
    /// This method panics if passed an empty string.
    pub fn domain(mut self, d: &str) -> Self {
        match validate::domain(&d) {
            Ok(()) => {
                self.domain = String::from(d);
                self
            }
            Err(e) => panic!("{}", e),
        }
    }

    /// Set the path value to the image file (i.e. 'image/path.png').
    ///
    /// # Panics
    ///
    /// This method panics if passed an empty string.
    pub fn path(mut self, p: &str) -> Self {
        match validate::path(&p) {
            Ok(()) => {
                self.path = Some(String::from(p));
                self
            }
            Err(e) => panic!("{}", e),
        }
    }

    /// Set an arbitrary key-value parameter (i.e. k='w', v='100'
    /// or k='fit', v='crop').
    ///
    /// # Examples
    /// ```
    /// use imgix::Url;
    /// let url = Url::new("example.domain.net").param("w", "320").path("test").lib("");
    /// let right = "https://example.domain.net/test?w=320";
    /// assert_eq!(url.join(), "https://example.domain.net/test?w=320")
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if any key `k` or any value `v` is an empty string,
    /// where `k` and `v` represent string literals.
    pub fn param(mut self, k: &'static str, v: &'static str) -> Self {
        match validate::param_pair(&k, &v) {
            Ok(()) => {
                self.params.push((k, v));
                self
            }
            Err(e) => panic!("{}", e),
        }
    }

    /// Set an arbitrary number of key-value parameters.
    ///
    /// # Examples
    /// ```
    /// use imgix::Url;
    ///
    /// let url = Url::new("example.domain.net")
    ///     .path("test")
    ///     .params(&[("w", "320"), ("h", "640"), ("fit", "crop")]);
    ///
    /// let right = "https://example.domain.net/test?w=320&h=640&fit=crop";
    /// assert_eq!(url.join(), right);
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if any key `k` or any value `v` is an empty string.
    pub fn params(mut self, p: &[(&'static str, &'static str)]) -> Self {
        for (k, v) in p.iter() {
            match validate::param_pair(&k, &v) {
                Ok(()) => self.params.push((k, v)),
                Err(e) => panic!("{}", e),
            }
        }
        self
    }

    /// Set the library version explicitly, see `Url::ix()` for the
    /// implicit default.
    ///
    /// The `Url`'s `lib` value can be set to any `String` by passing
    /// the desired string literal. If the `lib` is a valid ix-lib
    /// parameter if will be considered on the server. However, if
    /// an invalid lib-parameter is passed, e.g. "rust-is-cool", it
    /// will be ignored (appreciated ;) but ignored).
    ///
    /// Examples
    /// ```
    /// use imgix::{lib_version, Scheme, Url};
    /// 
    /// const DOMAIN: &str = "example.domain.net";
    /// const PATH: &str = "image.png";
    ///
    /// let url = Url::new(DOMAIN)
    ///     .lib(&lib_version())
    ///     .path("image.png");
    ///
    /// let right = format!(
    ///     "{scheme}://{domain}/{path}?{lib}",
    ///     scheme = Scheme::Https,
    ///     domain = DOMAIN,
    ///     path = PATH,
    ///     lib = lib_version()
    /// );
    ///
    /// assert_eq!(url.join(), right);
    ///
    /// let url = url.lib("rust-is-cool");
    /// assert_ne!(url.join(), right);
    /// ```
    pub fn lib(mut self, l: &str) -> Self {
        self.lib = String::from(l);
        self
    }

    /// Set the signing token.
    /// TODO: Test token post md5 implementation.
    pub fn token(mut self, t: &str) -> Self {
        self.token = Some(String::from(t));
        self
    }

    // Set the `scheme` value (i.e. `Scheme::Https`).
    pub fn scheme(mut self, s: Scheme) -> Self {
        self.scheme = s;
        self
    }

    // Set the library version to this crate's current `lib_version()`.
    // In the official imgix docs the `ixlib` parameter is used for
    // _diagnostic purposes_. It helps us help our users and customers,
    // but it's _your choice_ to _opt in_. Your code, Your choice.
    // What you see is what you get (WYSIWYG).
    pub fn ix(mut self) -> Self {
        self.lib = constants::lib_version();
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
                // If we make it here then the following is true:
                // * a path has been assigned and is not `None`
                // * a query string was generated successfully and
                //   is either empty or non-empty.
                match (&self.lib.is_empty(), &query.is_empty()) {
                    // All present, no empty fields, construct full url.
                    (false, false) => format!(
                        "{scheme}://{domain}/{path}?{lib}&{query}",
                        scheme = self.scheme,
                        domain = self.domain,
                        path = path,
                        lib = self.lib,
                        query = query,
                    ),
                    // Query string is empty, but lib is non-empty.
                    (false, true) => format!(
                        "{scheme}://{domain}/{path}?{lib}",
                        scheme = self.scheme,
                        domain = self.domain,
                        lib = self.lib,
                        path = path,
                    ),
                    // Lib is empty, but query is non-empty.
                    (true, false) => format!(
                        "{scheme}://{domain}/{path}?{query}",
                        scheme = self.scheme,
                        domain = self.domain,
                        path = path,
                        query = query
                    ),
                    // Both lib and query strings are empty.
                    (true, true) => format!(
                        "{scheme}://{domain}/{path}",
                        scheme = self.scheme,
                        domain = self.domain,
                        path = path,
                    ),
                }
            }
            None => panic!(
                "{}",
                Error::JoinError("cannot `join` when `path` is `None`".to_owned())
            ),
        }
    }

    /// Join a list of key-value parameter pairs.
    ///
    /// This associated function joins a list of key-value pairs. It is
    /// internal to `Url` and doesn't do much validation. This is to avoid
    /// duplicate validations as these parameters are valid at the time of
    /// construction.
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

/// Primary value for expressing which scheme a url uses.
///
/// This is an enum to define and enforce the crate semantics of what
/// it _means_ for a url to be valid for our use-case. A url can be
/// in one of two _states_: it either uses https or it uses http. While
/// this can be achieved by toggling https on and off via a boolean value,
/// a boolean value weakens the semantics and constrains the range of possible
/// schemes that can be used in the future.
///
/// Using `Scheme::Https` is also more explicit than saying `url.https(true)`,
/// it also has the added benefit of being _discoverable_. When usage is
/// `url.scheme(Scheme::...)`, the range of possible schemes can be discovered
/// by IDE code completion tools.
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
    const DOMAIN: &str = "test.domain.com";
    const DOMAIN2: &str = "test.domain2.com";
    const PNG_PATH: &str = "images/test-image.png";
    const JPG_PATH: &str = "images/test-image.jpg";
    const BASIC_PARAMS: &[(&str, &str)] = &[("w", "640"), ("h", "720"), ("fit", "crop")];

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
    fn test_default_url() {
        // Test the default representation of a `Url`.
        let default = Url::default();
        assert_eq!(default.scheme, Scheme::Https);
        assert_eq!(default.domain, "".to_owned());
        assert_eq!(default.lib, "".to_owned());
        assert_eq!(default.params, vec![]);
        assert_eq!(default.path, None);
        assert_eq!(default.token, None);
    }

    #[test]
    fn test_url_new() {
        let url = Url::new(DOMAIN);
        assert_eq!(url.domain, DOMAIN);

        let url = Url::new(DOMAIN2);
        assert_ne!(url.domain, "".to_owned());
    }

    #[test]
    fn test_url_domain() {
        let url = Url::new(DOMAIN);
        assert_ne!(url.domain, DOMAIN2);

        let url = url.domain(DOMAIN2);
        assert_eq!(url.domain, DOMAIN2);
    }

    #[test]
    #[should_panic]
    fn test_construct_empty_domain() {
        let _ = Url::new("");
    }

    #[test]
    #[should_panic]
    fn test_assign_empty_domain() {
        let _ = Url::default().domain("");
    }

    #[test]
    fn test_assign_path() {
        let url = Url::default().path(PNG_PATH);
        assert_eq!(url.path, Some(PNG_PATH.to_owned()));
    }

    #[test]
    #[should_panic]
    fn test_assign_empty_path() {
        let _ = Url::default().path("");
    }

    #[test]
    fn test_assign_param() {
        const K: &str = "w";
        const V: &str = "320";
        let url = Url::default().param(K, V);
        for (k, v) in url.params.iter() {
            assert_eq!(*k, K);
            assert_eq!(*v, V);
        }
    }

    #[test]
    #[should_panic]
    fn test_assign_empty_key_param() {
        const V: &str = "320";
        const KE: &str = "";
        let _ = Url::default().param(KE, V);
    }

    #[test]
    #[should_panic]
    fn test_assign_empty_value_param() {
        const VE: &str = "";
        const K: &str = "w";
        let _ = Url::default().param(K, VE);
    }

    #[test]
    fn test_assign_params() {
        let url = Url::default().params(BASIC_PARAMS);
        // Test params assigned correctly.
        for (left, right) in url.params.iter().zip(BASIC_PARAMS.iter()) {
            assert_eq!(left.0, right.0);
            assert_eq!(left.1, right.1);
        }
    }

    #[test]
    #[should_panic]
    fn test_assign_params_mismatch() {
        // This assertion is necessary for this test's validity.
        // If the slices were of length 3 and length 4, and only differed
        // in the 4th position, we would not know it as `zip` only zips
        // pairs––the 4th item is discarded (only 3 pairs would be made).
        assert_eq!(BASIC_PARAMS.len(), HAS_AR.len());

        const HAS_AR: &[(&str, &str)] = &[("w", "640"), ("h", "720"), ("ar", "4:3")];
        let url = Url::default().params(BASIC_PARAMS);
        // Test params assigned correctly.
        for (left, right) in url.params.iter().zip(HAS_AR.iter()) {
            // This test is designed to fail on the third iteration.
            assert_eq!(left.0, right.0);
            assert_eq!(left.1, right.1);
        }
    }

    #[test]
    fn test_url_png_src() {
        // Test a `Url` is constructed correctly.
        let right = format!(
            "{scheme}://{domain}/{path}",
            scheme = HTTPS,
            domain = DOMAIN,
            path = PNG_PATH,
        );

        let url = Url::new(DOMAIN).path(PNG_PATH);

        // Test all fields.
        assert_eq!(url.scheme, Scheme::Https);
        assert_eq!(url.domain, DOMAIN);
        assert_eq!(url.lib, "".to_owned());
        assert_eq!(url.path, Some(String::from(PNG_PATH)));
        assert!(url.params.is_empty());
        assert!(url.token.is_none());
        assert_eq!(url.join(), right);
    }

    #[test]
    fn test_url_jpg_src() {
        // Test a `Url` is constructed correctly.
        let right = format!(
            "{scheme}://{domain}/{path}",
            scheme = HTTP,
            domain = DOMAIN,
            path = JPG_PATH,
        );

        let url = Url::new(DOMAIN).path(JPG_PATH).scheme(Scheme::Http);

        // Test all fields.
        assert_eq!(url.scheme, Scheme::Http);
        assert_eq!(url.domain, DOMAIN);
        assert_eq!(url.lib, "".to_owned());
        assert_eq!(url.path, Some(String::from(JPG_PATH)));
        assert!(url.params.is_empty());
        assert_eq!(url.join(), right);
    }
}
