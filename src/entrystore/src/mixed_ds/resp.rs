// Copyright 2023 Pelikan Foundation LLC.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! This module defines how `Seg` storage will be used to execute `Redis`
//! storage commands.

use super::*;
use protocol_common::*;

use protocol_resp::*;

use mixed_ds::StrValue;
use std::time::Duration;

impl Execute<Request, Response> for MixedDs {
    fn execute(&mut self, request: &Request) -> Response {
        match request {
            Request::Get(get) => self.get(get),
            Request::Set(set) => self.set(set),
            _ => Response::error("not supported"),
        }
    }
}

fn stringify_key(buffer: &[u8]) -> Option<&str> {
    std::str::from_utf8(buffer).ok()
}
impl Storage for MixedDs {
    fn get(&mut self, get: &Get) -> Response {
        let maybe_key = stringify_key(get.key());
        if let Some(key) = maybe_key {
            if let Some(item) = self.data.get(key) {
                match item {
                    StrValue::Numeric(num) => Response::bulk_string(format!("{num}").as_bytes()),
                    StrValue::StringValue(string) => Response::bulk_string(string.as_bytes()),
                }
            } else {
                Response::null()
            }
        } else {
            Response::error("Only UTF-8 keys are supported")
        }
    }

    fn set(&mut self, set: &Set) -> Response {
        let ttl = match set.expire_time().unwrap_or(ExpireTime::default()) {
            ExpireTime::Seconds(n) => n,
            _ => 0,
        };
        let maybe_key = stringify_key(set.key());
        if let Some(key) = maybe_key {
            let maybe_value = stringify_key(set.value());

            if let Some(value) = maybe_value {
                let str_value = StrValue::StringValue(value.to_string());
                self.data.set(key.to_string(), str_value);
                Response::simple_string("OK")
            } else {
                Response::error("Only UTF-8 values are supported")
            }
        } else {
            Response::error("Only UTF-8 keys are supported")
        }
    }
}
