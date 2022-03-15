//! Klipper Data-Dictionary stuff

pub struct Dictionary {
    build_version: String,
    version: String,
    commands: Vec<Command>,
    responses: Vec<Response>,
    enums: Vec<Enum>,
}
