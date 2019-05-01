// Copyright 2019 Marti Raudsepp <marti@juffo.org>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pg_extern_attr;
extern crate pg_extend;

use pg_extern_attr::pg_extern;
use pg_extend::pg_magic;

/// This tells Postgres this library is a Postgres extension
pg_magic!(version: pg_sys::PG_VERSION_NUM);


/// The pg_extern attribute wraps the function in the proper functions syntax for C extensions
#[pg_extern]
fn rs_nullif(value1: Option<String>, value2: Option<String>) -> Option<String> {
    // The NULLIF function returns a null value if value1 equals value2; otherwise it returns value1
    if value1 == value2 { None } else { value1 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_nullif() {
        assert_eq!(rs_nullif(Some("a".to_string()), Some("-".to_string())), Some("a".to_string()));
        assert_eq!(rs_nullif(Some("a".to_string()), None), Some("a".to_string()));
        assert_eq!(rs_nullif(Some("-".to_string()), Some("-".to_string())), None);
        assert_eq!(rs_nullif(None, Some("-".to_string())), None);
        assert_eq!(rs_nullif(None, None), None);
    }
}
