use super::{PartialPathMatch, PathSegment};
use std::borrow::Cow;
mod param_segments;
mod static_segment;
mod tuples;
pub use param_segments::*;
pub use static_segment::*;

/// Defines a route which may or may not be matched by any given URL,
/// or URL segment.
///
/// This is a "horizontal" matching: i.e., it treats a tuple of route segments
/// as subsequent segments of the URL and tries to match them all. For a "vertical"
/// matching that sees a tuple as alternatives to one another, see [`RouteChild`](super::RouteChild).
pub trait PossibleRouteMatch {
    type ParamsIter: IntoIterator<Item = (Cow<'static, str>, String)>;

    fn test<'a>(
        &self,
        path: &'a str,
    ) -> Option<PartialPathMatch<'a, Self::ParamsIter>>;

    fn generate_path(&self, path: &mut Vec<PathSegment>);
}
