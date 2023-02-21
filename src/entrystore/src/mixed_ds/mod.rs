// Copyright 2021 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Segment-structured storage which implements efficient proactive eviction.
//! This storage type is suitable for use in simple key-value cache backends.
//! See: [`::seg`] crate for more details behind the underlying storage design.

use crate::EntryStore;

use mixed_ds::Index;

mod resp;

pub struct MixedDs {
    data: ::mixed_ds::Index,
}

impl MixedDs {
    /// Create `Seg` storage based on the config and the `TimeType` which is
    /// used to interpret various expiry time formats.
    pub fn new() -> Result<Self, std::io::Error> {
        let data = Index::new();

        Ok(Self { data })
    }
}

impl EntryStore for MixedDs {
    fn expire(&mut self) {
    }

    fn clear(&mut self) {
    }
}
