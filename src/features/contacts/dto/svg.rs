use axum::body::Body;

pub(crate) struct Svg(String);

impl From<String> for Svg {
    fn from(svg_string: String) -> Self {
        Self(svg_string)
    }
}

impl From<Svg> for Body {
    fn from(value: Svg) -> Self {
        Body::from(value.0)
    }
}
