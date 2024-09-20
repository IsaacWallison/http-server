use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>,
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<Value> {
        self.data.get(key);
        unimplemented!()
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(s: &'buffer str) -> Self {
        let mut data = HashMap::new();

        for substring in s.split('&') {
            let mut key = substring;
            let mut value = "";
            if let Some(i) = substring.find('=') {
                key = &substring[..i];
                value = &substring[i + 1..];
            }

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_val) => {
                    let mut vec = vec![prev_val, value];
                    *existing = Value::Multiple(vec![prev_val, value]);
                }
                Value::Multiple(vec) => vec.push(value)
            })
            .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}