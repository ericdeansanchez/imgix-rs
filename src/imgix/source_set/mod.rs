use crate::constants::{
    SRCSET_DPR_QUALITIES as DPR_QUALITIES, SRCSET_TARGET_DPR_RATIOS as TARGET_RATIOS,
    SRCSET_TARGET_WIDTHS as TARGET_WIDTHS,
};

use crate::url::{Scheme, Url};

/// Primary structure used to represent source sets.
///
/// A [source set] is an ordered set of zero or more image sources
/// and a source size.
///
/// A srcset, or "source set", attribute consists of one or more
/// [image candidate strings], each separated from the next by a
/// U+002C COMMA character (,).
///
/// Each image candidate string consists of zero or more [ASCII
/// whitespace], a [valid non-empty URL], zero or more ASCII
/// whitespace, and zero or one of the following:
///
/// * A width descriptor, _or_
/// * A pixel density descriptor
///
/// For a more detailed explanation see the [Srcset attributes specification].
///
/// [source set]:
/// https://html.spec.whatwg.org/multipage/images.html#source-set
/// [image candidate strings]:
/// https://html.spec.whatwg.org/multipage/images.html#image-candidate-string
/// [ASCII whitespace]:
/// (https://infra.spec.whatwg.org/#ascii-whitespace)
/// [valid non-empty URL]:
/// https://html.spec.whatwg.org/multipage/urls-and-fetching.html#valid-non-empty-url
/// [Srcset attributes specification]:
/// (https://html.spec.whatwg.org/multipage/images.html#srcset-attributes)
#[derive(Debug)]
pub struct SourceSet {
    src: Option<Url>,
    action: Option<Action>,
    srcset: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    media: Option<String>,
    config: Config,
}

impl SourceSet {
    pub fn new() -> Self {
        SourceSet::default()
    }

    pub fn scheme(self, s: Scheme) -> Self {
        SourceSet {
            config: self.config.set_scheme(s),
            ..self
        }
    }

    pub fn domain(self, d: &str) -> Self {
        SourceSet {
            config: self.config.set_domain(d),
            ..self
        }
    }

    pub fn path(self, p: &str) -> Self {
        SourceSet {
            config: self.config.set_path(p),
            ..self
        }
    }

    // TODO: consider `pub struct Params` where `impl From<&[....]> for Params`...
    pub fn params(self, params: &'static [(&'static str, &'static str)]) -> Self {
        SourceSet {
            config: self.config.set_params(params),
            ..self
        }
    }

    pub fn ratios(self, ratios: &'static [u32; 5]) -> Self {
        SourceSet {
            config: self.config.set_ratios(ratios),
            ..self
        }
    }

    pub fn get_ratios(&self) -> &[u32; 5] {
        self.config.get_ratios()
    }

    pub fn targets(self, targets: &'static [u32]) -> Self {
        SourceSet {
            config: self.config.set_targets(targets),
            ..self
        }
    }

    pub fn get_targets(&self) -> &[u32] {
        &self.config.get_targets()
    }

    pub fn variable_quality(self, state: bool) -> Self {
        SourceSet {
            config: self.config.set_use_variable_quality(state),
            ..self
        }
    }

    pub fn uses_variable_quality(&self) -> bool {
        self.config.get_use_variable_quality()
    }

    pub fn qualities(self, qualities: &'static [u32; 5]) -> Self {
        SourceSet {
            config: self.config.set_qualities(qualities),
            ..self
        }
    }
    pub fn get_qualities(&self) -> &[u32] {
        self.config.get_qualities()
    }

    pub fn srcset_attr(&self) -> String {
        self.build_srcset().join(",\n")
    }

    fn build_srcset(&self) -> Vec<String> {
        let url = self.config.to_url();
        let action = Self::infer_action(&url);

        match action {
            Action::PixelDensity => Self::build_pixel_set(&self, &url, &action),
            Action::Viewport => Self::build_viewport_set(&self, &url, &action),
            _ => unimplemented!(),
        }
    }

    fn infer_action(url: &Url) -> Action {
        let mut has_width = false;
        let mut has_height = false;
        let mut has_aspect_ratio = false;

        for param in url.get_params() {
            if param.0 == "w" {
                has_width = true;
            }

            if param.0 == "h" {
                has_height = true;
            }

            if param.0 == "ar" {
                has_aspect_ratio = true;
            }
        }

        if has_width || (has_aspect_ratio && has_height) {
            return Action::PixelDensity;
        }

        return Action::Viewport;
    }

    fn build_pixel_set(&self, url: &Url, action: &Action) -> Vec<String> {
        if self.uses_variable_quality() {
            create_variable_quality_set(&url, self.get_ratios(), &action, self.get_qualities())
        } else {
            create_srcset(&url, self.get_ratios(), &action)
        }
    }

    fn build_viewport_set(&self, url: &Url, action: &Action) -> Vec<String> {
        create_srcset(&url, self.get_targets(), &action)
    }
}

impl Default for SourceSet {
    fn default() -> Self {
        SourceSet {
            src: None,
            action: None,
            srcset: None,
            sizes: None,
            media: None,
            config: Config::default(),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    ArtDirection,
    PixelDensity,
    Viewport,
}

impl From<Url> for SourceSet {
    fn from(url: Url) -> Self {
        let action = Self::infer_action(&url);

        let srcset = match action {
            Action::Viewport => create_srcset(&url, &TARGET_WIDTHS[..], &action),
            Action::PixelDensity => create_srcset(&url, &TARGET_RATIOS[..], &action),
            Action::ArtDirection => unimplemented!(),
        };

        SourceSet {
            src: Some(url),
            action: Some(action),
            srcset: Some(srcset),
            ..Default::default()
        }
    }
}

fn create_srcset(url: &Url, targets: &[u32], action: &Action) -> Vec<String> {
    let mut srcset = Vec::new();

    for t in targets {
        srcset.push(candidate(&url, &t.to_string(), &action));
    }
    return srcset;
}

fn create_variable_quality_set(
    url: &Url,
    ratios: &[u32],
    action: &Action,
    qualities: &[u32],
) -> Vec<String> {
    let mut srcset = Vec::new();

    for (r, q) in ratios.iter().zip(qualities) {
        let more = format!("&q={quality}", quality = q);
        srcset.push(candidate_and(&url, &r.to_string(), action, &more));
    }
    return srcset;
}

fn candidate(url: &Url, value: &str, action: &Action) -> String {
    let (descriptor, key) = match action {
        Action::Viewport => ("w", "w"),
        Action::PixelDensity => ("x", "dpr"),
        Action::ArtDirection => unimplemented!(),
    };

    let param = if url.has_params() {
        format!("&{key}={value}", key = key, value = value)
    } else {
        format!("?{key}={value}", key = key, value = value)
    };

    format!(
        "{url}{param} {value}{descriptor}",
        url = url.join(),
        param = param,
        value = value,
        descriptor = descriptor
    )
}

fn candidate_and(url: &Url, value: &str, action: &Action, more: &str) -> String {
    let (descriptor, key) = match action {
        Action::Viewport => ("w", "w"),
        Action::PixelDensity => ("x", "dpr"),
        Action::ArtDirection => unimplemented!(),
    };

    let param = if url.has_params() {
        format!("&{key}={value}", key = key, value = value)
    } else {
        format!("?{key}={value}", key = key, value = value)
    };

    format!(
        "{url}{more}{param} {value}{descriptor}",
        url = url.join(),
        param = param,
        more = more,
        value = value,
        descriptor = descriptor
    )
}

#[derive(Debug)]
pub struct Config {
    scheme: Option<Scheme>,
    domain: Option<String>,
    path: Option<String>,
    params: Option<&'static [(&'static str, &'static str)]>,
    lib: Option<String>,
    token: Option<String>,
    targets: Option<&'static [u32]>,
    ratios: Option<&'static [u32; 5]>,
    qualities: Option<&'static [u32]>,
    use_variable_quality: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            scheme: None,
            domain: None,
            path: None,
            params: None,
            lib: None,
            token: None,
            targets: None,
            ratios: None,
            qualities: None,
            use_variable_quality: None,
        }
    }
}

impl Config {
    fn set_scheme(self, s: Scheme) -> Self {
        Config {
            scheme: Some(s),
            ..self
        }
    }

    fn set_domain(self, d: &str) -> Self {
        Config {
            domain: Some(String::from(d)),
            ..self
        }
    }

    fn set_path(self, p: &str) -> Self {
        Config {
            path: Some(String::from(p)),
            ..self
        }
    }

    fn set_params(self, params: &'static [(&'static str, &'static str)]) -> Self {
        Config {
            params: Some(params),
            ..self
        }
    }

    fn set_ratios(self, ratios: &'static [u32; 5]) -> Self {
        Config {
            ratios: Some(ratios),
            ..self
        }
    }

    fn get_ratios(&self) -> &[u32; 5] {
        &self.ratios.unwrap_or(&TARGET_RATIOS)
    }

    fn set_targets(self, targets: &'static [u32]) -> Self {
        Config {
            targets: Some(targets),
            ..self
        }
    }

    fn get_targets(&self) -> &[u32] {
        &self.targets.unwrap_or(&TARGET_WIDTHS[..])
    }

    fn set_qualities(self, qualities: &'static [u32; 5]) -> Self {
        Config {
            qualities: Some(qualities),
            ..self
        }
    }

    fn get_qualities(&self) -> &[u32] {
        &self.qualities.unwrap_or(&DPR_QUALITIES[..])
    }

    fn set_use_variable_quality(self, state: bool) -> Self {
        Config {
            use_variable_quality: Some(state),
            ..self
        }
    }

    fn get_use_variable_quality(&self) -> bool {
        self.use_variable_quality.unwrap_or(true)
    }

    fn to_url(&self) -> Url {
        let msg = "neither `domain` nor `path` can be `None`";
        match (&self.domain, &self.path) {
            (None, None) | (None, _) | (_, None) => panic!(msg),
            (Some(domain), Some(path)) => Url::new(&domain)
                .path(&path)
                .params(self.params.unwrap_or_default()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_url_dpr_w() {
        let url = Url::new("test.imgix.net")
            .path("image.png")
            .params(&[("w", "320")]);

        let left = Some(vec![
            "https://test.imgix.net/image.png?w=320&dpr=1 1x".to_owned(),
            "https://test.imgix.net/image.png?w=320&dpr=2 2x".to_owned(),
            "https://test.imgix.net/image.png?w=320&dpr=3 3x".to_owned(),
            "https://test.imgix.net/image.png?w=320&dpr=4 4x".to_owned(),
            "https://test.imgix.net/image.png?w=320&dpr=5 5x".to_owned(),
        ]);

        let s = SourceSet::from(url);
        assert_eq!(left, s.srcset);
    }

    #[test]
    fn test_from_url_dpr_ar_h() {
        // TODO: encode!!!
        let url = Url::new("test.imgix.net")
            .path("image.png")
            .params(&[("ar", "4%3A3"), ("h", "320")]);

        let left = Some(vec![
            "https://test.imgix.net/image.png?ar=4%3A3&h=320&dpr=1 1x".to_owned(),
            "https://test.imgix.net/image.png?ar=4%3A3&h=320&dpr=2 2x".to_owned(),
            "https://test.imgix.net/image.png?ar=4%3A3&h=320&dpr=3 3x".to_owned(),
            "https://test.imgix.net/image.png?ar=4%3A3&h=320&dpr=4 4x".to_owned(),
            "https://test.imgix.net/image.png?ar=4%3A3&h=320&dpr=5 5x".to_owned(),
        ]);

        let s = SourceSet::from(url);
        assert_eq!(left, s.srcset);
    }

    #[test]
    fn test_srcset_variable_quality() {
        let s = SourceSet::new()
            .scheme(Scheme::Https)
            .domain("test.imgix.net")
            .path("image.png")
            .params(&[("w", "640")]);

        let left = "https://test.imgix.net/image.png?w=640&q=75&dpr=1 1x,
https://test.imgix.net/image.png?w=640&q=50&dpr=2 2x,
https://test.imgix.net/image.png?w=640&q=35&dpr=3 3x,
https://test.imgix.net/image.png?w=640&q=23&dpr=4 4x,
https://test.imgix.net/image.png?w=640&q=20&dpr=5 5x";

        assert_eq!(left, s.srcset_attr());
    }

    #[test]
    fn test_srcset_variable_custom_quality() {
        let s = SourceSet::new()
            .scheme(Scheme::Https)
            .domain("test.imgix.net")
            .path("image.png")
            .params(&[("w", "640")])
            .qualities(&[100, 90, 80, 70, 60]);

        let left = "https://test.imgix.net/image.png?w=640&q=100&dpr=1 1x,
https://test.imgix.net/image.png?w=640&q=90&dpr=2 2x,
https://test.imgix.net/image.png?w=640&q=80&dpr=3 3x,
https://test.imgix.net/image.png?w=640&q=70&dpr=4 4x,
https://test.imgix.net/image.png?w=640&q=60&dpr=5 5x";

        assert_eq!(left, s.srcset_attr());
    }

    #[test]
    fn test_srcset_custom_viewport() {
        let s = SourceSet::new()
            .scheme(Scheme::Https)
            .domain("test.imgix.net")
            .path("image.png")
            .targets(&[1024, 512, 256, 128, 64]);

        let left = "https://test.imgix.net/image.png?w=1024 1024w,
https://test.imgix.net/image.png?w=512 512w,
https://test.imgix.net/image.png?w=256 256w,
https://test.imgix.net/image.png?w=128 128w,
https://test.imgix.net/image.png?w=64 64w";

        assert_eq!(left, s.srcset_attr());
    }

    #[test]
    fn test_source_from_url_viewport() {
        let url = Url::new("test.imgix.net").path("image.png");

        let left = Some(vec![
            "https://test.imgix.net/image.png?w=100 100w".to_owned(),
            "https://test.imgix.net/image.png?w=116 116w".to_owned(),
            "https://test.imgix.net/image.png?w=135 135w".to_owned(),
            "https://test.imgix.net/image.png?w=156 156w".to_owned(),
            "https://test.imgix.net/image.png?w=181 181w".to_owned(),
            "https://test.imgix.net/image.png?w=210 210w".to_owned(),
            "https://test.imgix.net/image.png?w=244 244w".to_owned(),
            "https://test.imgix.net/image.png?w=283 283w".to_owned(),
            "https://test.imgix.net/image.png?w=328 328w".to_owned(),
            "https://test.imgix.net/image.png?w=380 380w".to_owned(),
            "https://test.imgix.net/image.png?w=441 441w".to_owned(),
            "https://test.imgix.net/image.png?w=512 512w".to_owned(),
            "https://test.imgix.net/image.png?w=594 594w".to_owned(),
            "https://test.imgix.net/image.png?w=689 689w".to_owned(),
            "https://test.imgix.net/image.png?w=799 799w".to_owned(),
            "https://test.imgix.net/image.png?w=927 927w".to_owned(),
            "https://test.imgix.net/image.png?w=1075 1075w".to_owned(),
            "https://test.imgix.net/image.png?w=1247 1247w".to_owned(),
            "https://test.imgix.net/image.png?w=1446 1446w".to_owned(),
            "https://test.imgix.net/image.png?w=1678 1678w".to_owned(),
            "https://test.imgix.net/image.png?w=1946 1946w".to_owned(),
            "https://test.imgix.net/image.png?w=2257 2257w".to_owned(),
            "https://test.imgix.net/image.png?w=2619 2619w".to_owned(),
            "https://test.imgix.net/image.png?w=3038 3038w".to_owned(),
            "https://test.imgix.net/image.png?w=3524 3524w".to_owned(),
            "https://test.imgix.net/image.png?w=4087 4087w".to_owned(),
            "https://test.imgix.net/image.png?w=4741 4741w".to_owned(),
            "https://test.imgix.net/image.png?w=5500 5500w".to_owned(),
            "https://test.imgix.net/image.png?w=6380 6380w".to_owned(),
            "https://test.imgix.net/image.png?w=7401 7401w".to_owned(),
            "https://test.imgix.net/image.png?w=8192 8192w".to_owned(),
        ]);

        let s = SourceSet::from(url);
        assert_eq!(left, s.srcset);
    }
}
