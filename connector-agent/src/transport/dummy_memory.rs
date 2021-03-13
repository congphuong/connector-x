use crate::destinations::memory::MemoryDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::dummy::DummySource;
use crate::typesystem::TypeConversion;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::str::from_utf8;

impl TypeConversion<Bytes, String> for DummyMemoryTransport {
    fn convert(val: Bytes) -> String {
        from_utf8(&val[..]).unwrap().to_string()
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for DummyMemoryTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for DummyMemoryTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

pub struct DummyMemoryTransport;

impl_transport!(
    [],
    DummyMemoryTransport,
    DummyTypeSystem => DummyTypeSystem,
    DummySource => MemoryDestination,
    ([DummyTypeSystem::F64], [DummyTypeSystem::F64]) => (f64, f64) conversion all,
    ([DummyTypeSystem::I64], [DummyTypeSystem::I64]) => (i64, i64) conversion all,
    ([DummyTypeSystem::Bool], [DummyTypeSystem::Bool]) => (bool, bool) conversion all,
    ([DummyTypeSystem::String], [DummyTypeSystem::String]) => (String, String) conversion all,
    ([DummyTypeSystem::DateTime], [DummyTypeSystem::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
);
