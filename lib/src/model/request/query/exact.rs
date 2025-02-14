use itertools::Itertools;
use wiremock::matchers::{query_param, QueryParamExactMatcher};

use super::{HttpQueryParamsStub, super::matcher::RequestMatcherStub};

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamExactMatcher> {
    type Error = anyhow::Error;

    fn try_from(queries: &HttpQueryParamsStub) -> anyhow::Result<Self> {
        queries.get_queries()
            .ok_or_else(|| anyhow::Error::msg(""))
            .map(|iter| {
                iter.filter(|q| q.is_exact_match())
                    .filter_map(|it| QueryParamExactMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryParamExactMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherStub) -> anyhow::Result<Self> {
        query.equal_to_as_str()
            .map(|equal| query_param(query.key.as_str(), equal.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact query matcher found"))
    }
}