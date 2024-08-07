use super::{PartialPathMatch, PathSegment, PossibleRouteMatch};
use core::iter;
use std::borrow::Cow;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParamSegment(pub &'static str);

impl PossibleRouteMatch for ParamSegment {
    type ParamsIter = iter::Once<(Cow<'static, str>, String)>;

    fn test<'a>(
        &self,
        path: &'a str,
    ) -> Option<PartialPathMatch<'a, Self::ParamsIter>> {
        let mut matched_len = 0;
        let mut param_offset = 0;
        let mut param_len = 0;
        let mut test = path.chars();

        // match an initial /
        if let Some('/') = test.next() {
            matched_len += 1;
            param_offset = 1;
        }
        for char in test {
            // when we get a closing /, stop matching
            if char == '/' {
                break;
            }
            // otherwise, push into the matched param
            else {
                matched_len += char.len_utf8();
                param_len += char.len_utf8();
            }
        }

        if matched_len == 0 {
            return None;
        }

        let (matched, remaining) = path.split_at(matched_len);
        let param_value = iter::once((
            Cow::Borrowed(self.0),
            path[param_offset..param_len + param_offset].to_string(),
        ));
        Some(PartialPathMatch::new(remaining, param_value, matched))
    }

    fn generate_path(&self, path: &mut Vec<PathSegment>) {
        path.push(PathSegment::Param(self.0.into()));
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WildcardSegment(pub &'static str);

impl PossibleRouteMatch for WildcardSegment {
    type ParamsIter = iter::Once<(Cow<'static, str>, String)>;

    fn test<'a>(
        &self,
        path: &'a str,
    ) -> Option<PartialPathMatch<'a, Self::ParamsIter>> {
        let mut matched_len = 0;
        let mut param_offset = 0;
        let mut param_len = 0;
        let mut test = path.chars();

        // match an initial /
        if let Some('/') = test.next() {
            matched_len += 1;
            param_offset += 1;
        }
        for char in test {
            matched_len += char.len_utf8();
            param_len += char.len_utf8();
        }

        let (matched, remaining) = path.split_at(matched_len);
        let param_value = iter::once((
            Cow::Borrowed(self.0),
            path[param_offset..param_len + param_offset].to_string(),
        ));
        Some(PartialPathMatch::new(remaining, param_value, matched))
    }

    fn generate_path(&self, path: &mut Vec<PathSegment>) {
        path.push(PathSegment::Splat(self.0.into()));
    }
}

#[cfg(test)]
mod tests {
    use super::PossibleRouteMatch;
    use crate::{ParamSegment, StaticSegment, WildcardSegment};

    #[test]
    fn single_param_match() {
        let path = "/foo";
        let def = ParamSegment("a");
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo");
        assert_eq!(matched.remaining(), "");
        let params = matched.params().collect::<Vec<_>>();
        assert_eq!(params[0], ("a".into(), "foo".into()));
    }

    #[test]
    fn single_param_match_with_trailing_slash() {
        let path = "/foo/";
        let def = ParamSegment("a");
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo");
        assert_eq!(matched.remaining(), "/");
        let params = matched.params().collect::<Vec<_>>();
        assert_eq!(params[0], ("a".into(), "foo".into()));
    }

    #[test]
    fn tuple_of_param_matches() {
        let path = "/foo/bar";
        let def = (ParamSegment("a"), ParamSegment("b"));
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo/bar");
        assert_eq!(matched.remaining(), "");
        let params = matched.params().collect::<Vec<_>>();
        assert_eq!(params[0], ("a".into(), "foo".into()));
        assert_eq!(params[1], ("b".into(), "bar".into()));
    }

    #[test]
    fn splat_should_match_all() {
        let path = "/foo/bar/////";
        let def = (
            StaticSegment("foo"),
            StaticSegment("bar"),
            WildcardSegment("rest"),
        );
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo/bar/////");
        assert_eq!(matched.remaining(), "");
        let params = matched.params().collect::<Vec<_>>();
        assert_eq!(params[0], ("rest".into(), "////".into()));
    }
}
