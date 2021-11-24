use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, QueryStringValue<'a>>,
}

#[derive(Debug)]
pub enum QueryStringValue<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut QueryStringValue| match existing {
                    QueryStringValue::Single(prev_val) => {
                        *existing = QueryStringValue::Multiple(vec![prev_val, val]);
                    }
                    QueryStringValue::Multiple(vec) => vec.push(val),
                })
                .or_insert(QueryStringValue::Single(val));
        }

        QueryString { data }
    }
}
