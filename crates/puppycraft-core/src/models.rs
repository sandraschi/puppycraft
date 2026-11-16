use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DogSize {
    Toy,
    Small,
    Medium,
    Large,
    Giant,
}

impl DogSize {
    pub fn label_de(self) -> &'static str {
        match self {
            Self::Toy => "Sehr klein (bis 4 kg)",
            Self::Small => "Klein (4-10 kg)",
            Self::Medium => "Mittel (10-25 kg)",
            Self::Large => "Groß (25-45 kg)",
            Self::Giant => "Sehr groß (45+ kg)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LivingType {
    CityApartment,
    SuburbanHouse,
    RuralFarmAlmoehi,
}

impl LivingType {
    pub fn label_de(self) -> &'static str {
        match self {
            Self::CityApartment => "Innenstadt-Wohnung",
            Self::SuburbanHouse => "Haus mit Garten / Stadtrand",
            Self::RuralFarmAlmoehi => "Almöhi / Bergbauer / Alleinlage",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub name_de: String,
    pub name_en: String,
    pub fci_group: String,
    pub origin: String,
    pub size: DogSize,
    pub weight_kg_min: f32,
    pub weight_kg_max: f32,
    pub lifespan_years: String,
    /// 1 (couch potato) .. 5 (working/athlete)
    pub energy: u8,
    /// 1 (very easy) .. 5 (expert only)
    pub beginner_difficulty: u8,
    /// 1 (rarely) .. 5 (very vocal)
    pub barking: u8,
    /// 1 (hardly sheds) .. 5 (heavy shedding)
    pub shedding: u8,
    /// 1 .. 5
    pub kid_friendly: u8,
    /// 1 .. 5 (5 = very cat-safe, 1 = high prey drive)
    pub cat_friendly: u8,
    /// 1 .. 5 (5 = great for city apartment)
    pub apartment_fit: u8,
    /// 1 .. 5 (5 = thrives on farm/mountains)
    pub rural_fit: u8,
    /// 1 .. 5 (5 = handles moderate alone time well)
    pub alone_tolerance: u8,
    pub exercise_minutes_day: u16,
    pub monthly_cost_eur: u16,
    pub temperament: String,
    pub health_notes: String,
    pub ideal_for: String,
    pub watch_out: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchProfile {
    pub preset_name: String,
    pub living: LivingType,
    pub has_elevator_or_ground: bool,
    pub has_kids: bool,
    pub kids_count: u8,
    pub has_cats: bool,
    pub is_senior_owner: bool,
    pub first_time_owner: bool,
    pub daily_exercise_min: u16,
    pub alone_hours_day: f32,
    pub low_shedding_preferred: bool,
    pub quiet_neighbor_needed: bool,
    pub max_size: DogSize,
}

impl Default for MatchProfile {
    fn default() -> Self {
        Self::preset("city_single")
    }
}

impl MatchProfile {
    pub fn preset(key: &str) -> Self {
        match key {
            "oldie" => Self {
                preset_name: "Einsamer Oldie (Senior allein)".into(),
                living: LivingType::CityApartment,
                has_elevator_or_ground: true,
                has_kids: false,
                kids_count: 0,
                has_cats: false,
                is_senior_owner: true,
                first_time_owner: true,
                daily_exercise_min: 45,
                alone_hours_day: 2.0,
                low_shedding_preferred: false,
                quiet_neighbor_needed: true,
                max_size: DogSize::Medium,
            },
            "family_kids_cat" => Self {
                preset_name: "Familie mit 3 Kids & Katze".into(),
                living: LivingType::SuburbanHouse,
                has_elevator_or_ground: true,
                has_kids: true,
                kids_count: 3,
                has_cats: true,
                is_senior_owner: false,
                first_time_owner: true,
                daily_exercise_min: 90,
                alone_hours_day: 3.5,
                low_shedding_preferred: false,
                quiet_neighbor_needed: false,
                max_size: DogSize::Large,
            },
            "almoehi" => Self {
                preset_name: "Almöhi / Bergbauer / Alleinlage".into(),
                living: LivingType::RuralFarmAlmoehi,
                has_elevator_or_ground: true,
                has_kids: false,
                kids_count: 0,
                has_cats: false,
                is_senior_owner: false,
                first_time_owner: false,
                daily_exercise_min: 150,
                alone_hours_day: 2.0,
                low_shedding_preferred: false,
                quiet_neighbor_needed: false,
                max_size: DogSize::Giant,
            },
            "sporty" => Self {
                preset_name: "Sportler / Outdoor-Junkie".into(),
                living: LivingType::SuburbanHouse,
                has_elevator_or_ground: true,
                has_kids: false,
                kids_count: 0,
                has_cats: false,
                is_senior_owner: false,
                first_time_owner: false,
                daily_exercise_min: 150,
                alone_hours_day: 4.0,
                low_shedding_preferred: false,
                quiet_neighbor_needed: false,
                max_size: DogSize::Large,
            },
            _ => Self {
                preset_name: "Innenstadt-Single (Ersthund)".into(),
                living: LivingType::CityApartment,
                has_elevator_or_ground: true,
                has_kids: false,
                kids_count: 0,
                has_cats: false,
                is_senior_owner: false,
                first_time_owner: true,
                daily_exercise_min: 60,
                alone_hours_day: 4.0,
                low_shedding_preferred: true,
                quiet_neighbor_needed: true,
                max_size: DogSize::Medium,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub breed: Breed,
    pub score: u8,
    pub pros: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogProfile {
    pub id: i64,
    pub name: String,
    pub breed: String,
    pub birth_date: String,
    pub weight_kg: f32,
    pub chip_id: String,
    pub insurance: String,
    pub vet_name: String,
    pub vet_phone: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiaryEntry {
    pub id: i64,
    pub dog_name: String,
    pub date: String,
    pub title: String,
    pub note: String,
    pub mood: String,
    pub tag: String,
    pub weight_kg: Option<f32>,
    pub photo_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitterEvent {
    pub id: i64,
    pub dog_name: String,
    pub sitter_name: String,
    pub relation: String,
    pub start_date: String,
    pub end_date: String,
    pub feeding_notes: String,
    pub walk_notes: String,
    pub emergency_contact: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaccineRecord {
    pub id: i64,
    pub dog_name: String,
    pub vaccine_name: String,
    pub date_given: String,
    pub next_due: String,
    pub vet_clinic: String,
    pub batch_no: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoRecord {
    pub id: i64,
    pub dog_name: String,
    pub file_path: String,
    pub caption: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreederInfo {
    pub name: String,
    pub region: String,
    pub association: String,
    pub breeds: String,
    pub website: String,
    pub health_checks: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RescueOrg {
    pub name: String,
    pub region: String,
    pub focus: String,
    pub website: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerInfo {
    pub name: String,
    pub city: String,
    pub association: String,
    pub specialties: String,
    pub methodology: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingGuide {
    pub phase: String,
    pub title: String,
    pub summary: String,
    pub steps: Vec<String>,
    pub common_mistakes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalRule {
    pub region: String,
    pub topic: String,
    pub summary: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelDestination {
    pub id: String,
    pub city_or_country: String,
    pub dog_friendliness_score: u8,
    pub rating_label: String,
    pub transit_rules: String,
    pub restaurants_shops: String,
    pub entry_requirements: String,
    pub local_tips: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicSubstance {
    pub name: String,
    pub danger_level: String,
    pub toxic_dose: String,
    pub symptoms: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VetVisit {
    pub id: i64,
    pub dog_name: String,
    pub date: String,
    pub clinic: String,
    pub reason: String,
    pub diagnosis_meds: String,
    pub cost_eur: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkSpot {
    pub id: i64,
    pub name: String,
    pub city: String,
    pub fenced: bool,
    pub water_nearby: bool,
    pub rating: u8,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalProblem {
    pub name: String,
    pub affected_breeds: String,
    pub urgency: String,
    pub symptoms: String,
    pub prevention_and_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceProvider {
    pub name: String,
    pub category: String,
    pub monthly_eur: String,
    pub coverage: String,
    pub waiting_period: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VetClinicInfo {
    pub name: String,
    pub tier_type: String,
    pub region: String,
    pub specialties: String,
    pub emergency_24h: bool,
    pub when_to_go: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeniorSection {
    pub title: String,
    pub summary: String,
    pub tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSrc {
    pub name: &'static str,
    pub description: &'static str,
    pub skill_md: &'static str,
}
