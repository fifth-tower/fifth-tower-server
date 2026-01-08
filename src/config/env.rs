use std::collections::HashMap;

use tower::{common::hashmap, server_lib::EnvKey};

pub(crate) fn get_env(dev_mode: bool) -> HashMap<EnvKey, String> {
    if dev_mode {
        get_dev_env()
    } else {
        get_pro_env()
    }
}

#[cfg(not(feature = "db_pro"))]
fn get_dev_env() -> HashMap<EnvKey, String> {
    hashmap! {
        EnvKey::DatabaseUrl=>"postgres://postgres:12345678@127.0.0.1/fifth-tower?currentSchema=public".into(),
    }
}

#[cfg(feature = "db_pro")]
fn get_dev_env() -> HashMap<EnvKey, String> {
    hashmap! {
         EnvKey::DatabaseUrl=>"postgres://postgres:12345678@152.32.239.4/fifth-tower?currentSchema=public".into(),
    }
}

fn get_pro_env() -> HashMap<EnvKey, String> {
    hashmap! {
        EnvKey::DatabaseUrl=>"postgres://postgres:12345678@127.0.0.1/fifth-tower?currentSchema=public".into(),
    }
}
