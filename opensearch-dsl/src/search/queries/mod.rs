//! Allows constructing OpenSearch search query.
//!
//! OpenSearch provides a full Query DSL (Domain Specific Language) based on
//! JSON to define queries. Think of the Query DSL as an AST (Abstract Syntax
//! Tree) of queries, consisting of two types of clauses:
//!
//! **Leaf query clauses**
//!
//! Leaf query clauses look for a particular value in a particular field, such
//! as the match, term or range queries. These queries can be used by
//! themselves.
//!
//! **Compound query clauses**
//!
//! Compound query clauses wrap other leaf or compound queries and are used to
//! combine multiple queries in a logical fashion (such as the bool or dis_max
//! query), or to alter their behavior (such as the constant_score query).
//!
//! <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl.html>

// Public modules
pub mod params;

pub mod compound;
pub mod custom;
pub mod full_text;
pub mod geo;
pub mod joining;
pub mod shape;
pub mod span;
pub mod specialized;
pub mod term_level;

pub use self::compound::*;
pub use self::custom::*;
pub use self::full_text::*;
pub use self::geo::*;
pub use self::joining::*;
pub use self::shape::*;
pub use self::span::*;
pub use self::specialized::*;
pub use self::term_level::*;

// Very special queries
mod match_all_query;
mod match_none_query;
mod query_collection;

pub use self::match_all_query::*;
pub use self::match_none_query::*;
pub use self::query_collection::*;

use crate::util::*;

macro_rules! query {
    ($($variant:ident($query:ty)),+ $(,)?) => {
        /// A container enum for supported OpenSearch query types
        #[derive(Clone, PartialEq, Serialize, Deserialize)]
        #[serde(untagged)]
        #[allow(missing_docs)]
        pub enum Query {
            $(
                $variant($query),
            )*
        }

        impl IntoIterator for Query {
            type Item = Self;
            type IntoIter = std::option::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                if self.should_skip() {
                    None.into_iter()
                } else {
                    Some(self).into_iter()
                }
            }
        }

        impl ShouldSkip for Query {
            fn should_skip(&self) -> bool {
                match self {
                    $(
                        Self::$variant(q) => q.should_skip(),
                    )+
                }
            }
        }

        impl std::fmt::Debug for Query {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(q) => q.fmt(f),
                    )+
                }
            }
        }

        $(
            impl From<$query> for Query {
                fn from(q: $query) -> Self {
                    Query::$variant(q)
                }
            }

            impl PartialEq<$query> for Query {
                fn eq(&self, other: &$query) -> bool {
                    match self {
                        Self::$variant(query) => query.eq(other),
                        _ => false,
                    }
                }
            }

            impl PartialEq<Query> for $query {
                fn eq(&self, other: &Query) -> bool {
                    match other {
                        Query::$variant(query) => self.eq(query),
                        _ => false,
                    }
                }
            }

            impl From<$query> for Option<Query> {
                fn from(q: $query) -> Self {
                    if q.should_skip() {
                        None
                    } else {
                        Some(Query::$variant(q))
                    }
                }
            }

            impl IntoIterator for $query {
                type Item = $query;

                type IntoIter = std::option::IntoIter<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    if self.should_skip() {
                        None.into_iter()
                    } else {
                        Some(self).into_iter()
                    }
                }
            }
        )+
    };
}

query!(
    Bool(BoolQuery),
    Prefix(PrefixQuery),
    Regexp(RegexpQuery),
    Wildcard(WildcardQuery),
    TermsSet(TermsSetQuery),
    Term(TermQuery),
    Terms(TermsQuery),
    TermsLookup(TermsLookupQuery),
    Exists(ExistsQuery),
    Range(RangeQuery),
    Ids(IdsQuery),
    ConstantScore(ConstantScoreQuery),
    DistanceFeatureDate(DistanceFeatureQuery<chrono::DateTime<chrono::Utc>>),
    DistanceFeatureGeo(DistanceFeatureQuery<crate::GeoLocation>),
    Match(MatchQuery),
    MatchBoolPrefix(MatchBoolPrefixQuery),
    MatchPhrasePrefix(MatchPhrasePrefixQuery),
    MatchAll(MatchAllQuery),
    MatchNone(MatchNoneQuery),
    MatchPhrase(MatchPhraseQuery),
    MultiMatch(MultiMatchQuery),
    Nested(NestedQuery),
    Boosting(BoostingQuery),
    DisMax(DisMaxQuery),
    Pinned(PinnedQuery),
    Percolate(PercolateQuery),
    PercolateLookup(PercolateLookupQuery),
    FunctionScore(FunctionScoreQuery),
    RankFeature(RankFeatureQuery),
    RankFeatureSaturation(RankFeatureSaturationQuery),
    RankFeatureLogarithm(RankFeatureLogarithmQuery),
    RankFeatureSigmoid(RankFeatureSigmoidQuery),
    RankFeatureLinear(RankFeatureLinearQuery),
    MoreLikeThis(MoreLikeThisQuery),
    Fuzzy(FuzzyQuery),
    GeoDistance(GeoDistanceQuery),
    GeoBoundingBox(GeoBoundingBoxQuery),
    GeoShapeLookup(GeoShapeLookupQuery),
    GeoShape(GeoShapeQuery),
    ShapeLookup(ShapeLookupQuery),
    Shape(ShapeQuery),
    Json(JsonQuery),
    Wrapper(WrapperQuery),
    Script(ScriptQuery),
    ScriptScore(ScriptScoreQuery),
    ParentId(ParentIdQuery),
    HasParent(HasParentQuery),
    HasChild(HasChildQuery),
    SimpleQueryString(SimpleQueryStringQuery),
    QueryString(QueryStringQuery),
    CombinedFields(CombinedFieldsQuery),
    SpanContaining(SpanContainingQuery),
    SpanFieldMasking(SpanFieldMaskingQuery),
    SpanFirst(SpanFirstQuery),
    SpanMulti(SpanMultiQuery),
    SpanNear(SpanNearQuery),
    SpanNot(SpanNotQuery),
    SpanOr(SpanOrQuery),
    SpanTerm(SpanTermQuery),
    SpanWithin(SpanWithinQuery),
    Knn(KnnQuery),
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn partial_eq() {
    assert_eq!(
      Query::term("field", "value"),
      Query::from(Query::term("field", "value"))
    );
    assert_eq!(
      Query::from(Query::term("field", "value")),
      Query::term("field", "value"),
    );
  }
}
