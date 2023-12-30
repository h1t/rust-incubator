use std::{io::Write, sync::Mutex};

use slog::{o, Drain, Duplicate, FnValue, Level, PushFnValue, Record};
use slog_json::{Json, JsonBuilder};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn drain<E, I>(stderr: E, stdout: I) -> impl Drain<Ok = (), Err = slog::Never>
where
    E: Write,
    I: Write,
{
    fn kv<W: Write>(json: JsonBuilder<W>) -> JsonBuilder<W> {
        json.add_key_value(o!(
            "msg" => PushFnValue(move |record : &Record, ser| {
                ser.emit(record.msg())
            }),
            "time" => FnValue(move |_ : &Record| {
                    OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .ok()
            }),
            "lvl" => FnValue(move |r : &Record| r.level().as_str()),
        ))
    }

    let warn = kv(Json::new(stderr)).build();
    let info = kv(Json::new(stdout)).build();

    Mutex::new(Duplicate(
        warn.filter(|r| r.level() >= Level::Warning),
        info.filter(|r| r.level() < Level::Warning),
    ))
    .fuse()
}
