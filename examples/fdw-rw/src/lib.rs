// Copyright 2018 Liz Frost <web@stillinbeta.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pg_extend;
extern crate pg_extern_attr;

use pg_extend::pg_fdw::{ForeignRow, ForeignData, OptionMap};
use pg_extend::{pg_datum, pg_magic, pg_type};
use pg_extern_attr::pg_foreignwrapper;

use std::collections::HashMap;
use std::sync::RwLock;

// Needs feature(staticmutex)
// use std::sync::{StaticMutex, MUTEX_INIT};
// static LOCK: StaticMutex = MUTEX_INIT;
static mut _CACHE: Option<RwLock<HashMap<String,String>>> = None;

fn get_cache() -> &'static RwLock<HashMap<String,String>> {
    // let _g = LOCK.lock().unwrap();
    unsafe {
        if let None = _CACHE {
            let rw = RwLock::new(HashMap::new());
            _CACHE = Some(rw)
        }
        &_CACHE.as_ref().unwrap()
    }
}

/// This tells Postges this library is a Postgres extension
pg_magic!(version: pg_sys::PG_VERSION_NUM);

#[pg_foreignwrapper]
struct CacheFDW{
    inner: Vec<(String,String)>,
}

struct MyRow {
    key: String,
    value: String,
}

impl ForeignRow for MyRow {
    fn get_field(
        &self,
        name: &str,
        _typ: pg_type::PgType,
        _opts: OptionMap,
    ) -> Result<Option<pg_datum::PgDatum>, &str> {
        match name {
            "key" => Ok(Some(self.key.clone().into())),
            "value" => Ok(Some(self.value.clone().into())),
            _ => Err("unknown field"),
        }
    }
}

impl Iterator for CacheFDW {
    type Item = Box<ForeignRow>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.pop() {
            None => None,
            Some((k, v)) => Some(Box::new(MyRow{key: k.to_string(), value: v.to_string()})),
        }
    }
}

impl ForeignData for CacheFDW {
    fn begin(_sopts: OptionMap, _topts: OptionMap) -> Self {
        let c = get_cache().read().unwrap();
        let vecs = c.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        CacheFDW { inner: vecs }
    }
}