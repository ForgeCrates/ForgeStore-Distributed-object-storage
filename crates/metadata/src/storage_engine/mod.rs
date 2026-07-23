use std::sync::Arc;
use rocksdb::{DB, Options};

#[derive(Clone)]
pub struct MetadataDB {
    db: Arc<DB>,
}

impl MetadataDB {
    pub fn open(path: &str) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path)?;

        Ok(Self { db: Arc::new(db) })
    }

    pub fn inner(&self) -> &DB {
        &self.db
    }
}