// Query DSL

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};

pub enum Query {
    Match(MatchQuery),
    MatchAll
}

use self::Query::{Match,
                  MatchAll};

impl ToJson for Query {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String, Json>::new();
        match self {
            &MatchAll     => {
                d.insert("match_all".to_string(), Json::Object(BTreeMap::new()));
            },
            &Match(ref q) => { d.insert("match".to_string(), q.to_json()); },
        }
        Json::Object(d)
    }
}

impl Query {
    pub fn build_match(field: String, query: Json) -> MatchQuery {
        MatchQuery {
            field:            field,
            query:            query,
            operator:         None,
            zero_terms_query: None,
            cutoff_frequency: None,
            lenient:          None
        }
    }
}

macro_rules! with {
    ($funcn:ident, $sn:ident, $t:ident) => {
        pub fn $funcn<'a>(&'a mut self, value: $t) -> &'a mut Self {
            self.$sn = Some(value);
            self
        }
    }
}

macro_rules! optional_add {
    ($map:ident, $sn:expr, $field:expr) => {
        match $sn {
            Some(ref value) => { $map.insert($field.to_string(), value.to_json()); }
            _               => ()
        }
    }
}

pub enum MatchType {
    Phrase,
    PhrasePrefix
}

impl ToJson for MatchType {
    fn to_json(&self) -> Json {
        match self {
            &Phrase =>       "phrase".to_json(),
            &PhrasePrefix => "phrase_prefix".to_json()
        }
    }
}

pub struct MatchQuery {
    field:            String,
    query:            Json,
    match_type:       Option<MatchType>,
    operator:         Option<String>,
    zero_terms_query: Option<String>,
    cutoff_frequency: Option<f64>,
    lenient:          Option<bool>,
    analyzer:         Option<String>
}

impl MatchQuery {
    with!(with_type, match_type, MatchType);
    with!(with_operator, operator, String);
    with!(with_zero_terms_query, zero_terms_query, String);
    with!(with_cutoff_frequency, cutoff_frequency, f64);
    with!(with_lenient, lenient, bool);
    with!(with_analyzer, analyzer, String);

    pub fn build(self) -> Query {
        Match(self)
    }
}

impl ToJson for MatchQuery {
    fn to_json(&self) -> Json {
        let mut inner = BTreeMap::new();
        inner.insert("query".to_string(), self.query.to_json());
        optional_add!(inner, self.match_type, "type");
        optional_add!(inner, self.operator, "operator");
        optional_add!(inner, self.zero_terms_query, "zero_terms_query");
        optional_add!(inner, self.cutoff_frequency, "cutoff_frequency");
        optional_add!(inner, self.lenient, "lenient");
        optional_add!(inner, self.analyzer, "analyzer");

        let mut d = BTreeMap::new();
        d.insert(self.field.clone(), Json::Object(inner));

        Json::Object(d)
    }
}
