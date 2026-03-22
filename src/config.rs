use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct WeddingConfig {
    pub ceremony: CeremonyConfig,
    pub venue: VenueConfig,
    pub couple: CoupleConfig,
}

#[derive(Clone, Deserialize)]
pub struct CeremonyConfig {
    pub date_utc: String,
    pub date_display: String,
    pub day_of_week: String,
    pub day_number: String,
    pub month_year: String,
    pub reception_time: String,
    pub ceremony_time: String,
}

#[derive(Clone, Deserialize)]
pub struct VenueConfig {
    pub name: String,
    pub address: String,
    pub location_line: String,
    pub province: String,
    pub maps_embed_url: String,
    pub maps_directions_url: String,
}

#[derive(Clone, Deserialize)]
pub struct CoupleConfig {
    pub groom_name: String,
    pub bride_name: String,
    pub title: String,
}

fn load_config() -> WeddingConfig {
    let json = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/wedding.json"));
    serde_json::from_str(json).expect("Failed to parse wedding config")
}

static CONFIG: std::sync::OnceLock<WeddingConfig> = std::sync::OnceLock::new();

pub fn wedding_config() -> &'static WeddingConfig {
    CONFIG.get_or_init(load_config)
}
