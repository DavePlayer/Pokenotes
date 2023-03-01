use uuid::Uuid;

pub fn remove_surrealdb_id_prefix<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    let s = s.replace("⟨", "").replace("⟩", "");
    let uuid_string = s.split(":").last().unwrap();
    Uuid::parse_str(uuid_string).map_err(serde::de::Error::custom)
}
