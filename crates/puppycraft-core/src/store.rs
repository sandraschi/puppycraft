use crate::error::PuppyCraftError;
use crate::models::{
    DiaryEntry, DogProfile, ParkSpot, PhotoRecord, SitterEvent, VaccineRecord, VetVisit,
};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

fn db_err<E: std::fmt::Display>(e: E) -> PuppyCraftError {
    PuppyCraftError::Store(e.to_string())
}

pub fn data_dir() -> PathBuf {
    if let Some(v) = std::env::var_os("PUPPYCRAFT_DATA").filter(|v| !v.is_empty()) {
        return PathBuf::from(v);
    }
    if let Some(v) = std::env::var_os("LOCALAPPDATA").filter(|v| !v.is_empty()) {
        return PathBuf::from(v).join("puppycraft");
    }
    if let Some(v) = std::env::var_os("HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(v).join(".puppycraft");
    }
    PathBuf::from("puppycraft-data")
}

pub struct PuppyStore {
    conn: Mutex<Connection>,
}

impl PuppyStore {
    pub fn open_default() -> Result<Self, PuppyCraftError> {
        let dir = data_dir();
        std::fs::create_dir_all(&dir).map_err(db_err)?;
        Self::open_path(&dir.join("puppycraft.db"))
    }

    pub fn open_path(path: &Path) -> Result<Self, PuppyCraftError> {
        let conn = Connection::open(path).map_err(db_err)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=FULL;
             PRAGMA foreign_keys=ON;",
        )
        .map_err(db_err)?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.init_schema()?;
        store.seed_defaults_if_empty()?;
        Ok(store)
    }

    pub fn open_in_memory() -> Result<Self, PuppyCraftError> {
        let conn = Connection::open_in_memory().map_err(db_err)?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.init_schema()?;
        store.seed_defaults_if_empty()?;
        Ok(store)
    }

    fn lock(&self) -> Result<MutexGuard<'_, Connection>, PuppyCraftError> {
        self.conn.lock().map_err(db_err)
    }

    fn init_schema(&self) -> Result<(), PuppyCraftError> {
        let conn = self.lock()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                breed TEXT NOT NULL,
                birth_date TEXT NOT NULL,
                weight_kg REAL NOT NULL,
                chip_id TEXT NOT NULL DEFAULT '',
                insurance TEXT NOT NULL DEFAULT '',
                vet_name TEXT NOT NULL DEFAULT '',
                vet_phone TEXT NOT NULL DEFAULT '',
                notes TEXT NOT NULL DEFAULT ''
            );
            CREATE TABLE IF NOT EXISTS diary_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dog_name TEXT NOT NULL,
                date TEXT NOT NULL,
                title TEXT NOT NULL,
                note TEXT NOT NULL,
                mood TEXT NOT NULL DEFAULT 'Fröhlich',
                tag TEXT NOT NULL DEFAULT 'Alltag',
                weight_kg REAL,
                photo_path TEXT
            );
            CREATE TABLE IF NOT EXISTS sitter_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dog_name TEXT NOT NULL,
                sitter_name TEXT NOT NULL,
                relation TEXT NOT NULL,
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL,
                feeding_notes TEXT NOT NULL DEFAULT '',
                walk_notes TEXT NOT NULL DEFAULT '',
                emergency_contact TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'Geplant'
            );
            CREATE TABLE IF NOT EXISTS vaccinations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dog_name TEXT NOT NULL,
                vaccine_name TEXT NOT NULL,
                date_given TEXT NOT NULL,
                next_due TEXT NOT NULL,
                vet_clinic TEXT NOT NULL DEFAULT '',
                batch_no TEXT NOT NULL DEFAULT '',
                notes TEXT NOT NULL DEFAULT ''
            );
            CREATE TABLE IF NOT EXISTS photos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dog_name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                caption TEXT NOT NULL DEFAULT '',
                date TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS vet_visits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                dog_name TEXT NOT NULL,
                date TEXT NOT NULL,
                clinic TEXT NOT NULL,
                reason TEXT NOT NULL,
                diagnosis_meds TEXT NOT NULL DEFAULT '',
                cost_eur REAL NOT NULL DEFAULT 0.0
            );
            CREATE TABLE IF NOT EXISTS park_spots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                city TEXT NOT NULL,
                fenced INTEGER NOT NULL DEFAULT 1,
                water_nearby INTEGER NOT NULL DEFAULT 0,
                rating INTEGER NOT NULL DEFAULT 5,
                notes TEXT NOT NULL DEFAULT ''
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )
        .map_err(db_err)?;
        Ok(())
    }

    fn seed_defaults_if_empty(&self) -> Result<(), PuppyCraftError> {
        let conn = self.lock()?;
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM dogs", [], |r| r.get(0))
            .map_err(db_err)?;
        if count == 0 {
            conn.execute(
                "INSERT INTO dogs (name, breed, birth_date, weight_kg, chip_id, insurance, vet_name, vet_phone, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    "Beppo",
                    "Zwergpudel / Mischling",
                    "2026-06-15",
                    6.8_f64,
                    "900118000123456",
                    "Hunde-Haftpflicht + OP-Schutz",
                    "Dr. Muster (Wien)",
                    "+43 1 555 0199",
                    "Liebt Käsewürfel, trägt in der U-Bahn brav seinen Maulkorb."
                ],
            )
            .map_err(db_err)?;

            conn.execute(
                "INSERT INTO diary_entries (dog_name, date, title, note, mood, tag, weight_kg)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    "Beppo",
                    "2026-11-10",
                    "Erste U-Bahn-Fahrt in Wien!",
                    "Mit Maulkorb und Leberwurst-Tube super entspannt gemeistert.",
                    "Stolz",
                    "Training",
                    6.7_f64
                ],
            )
            .map_err(db_err)?;

            conn.execute(
                "INSERT INTO sitter_events (dog_name, sitter_name, relation, start_date, end_date, feeding_notes, walk_notes, emergency_contact, status)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    "Beppo",
                    "Oma Helga",
                    "Verwandte",
                    "2026-11-22",
                    "2026-11-24",
                    "90g Trockenfutter morgens & abends; KEINE Weintrauben!",
                    "3x täglich 25 Min, im Park an der Schleppleine lassen.",
                    "Dr. Muster +43 1 555 0199",
                    "Bestätigt"
                ],
            )
            .map_err(db_err)?;

            conn.execute(
                "INSERT INTO vaccinations (dog_name, vaccine_name, date_given, next_due, vet_clinic, batch_no, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    "Beppo",
                    "SHPPiL + Tollwut",
                    "2026-09-15",
                    "2027-09-15",
                    "Tierklinik Wien",
                    "EU-2026-A",
                    "EU-Heimtierausweis eingetragen, 21-Tage-Frist abgelaufen."
                ],
            )
            .map_err(db_err)?;

            conn.execute(
                "INSERT INTO park_spots (name, city, fenced, water_nearby, rating, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    "Prater Hauptallee Hundezone",
                    "Wien (2. Bezirk)",
                    1,
                    1,
                    5,
                    "Riesige eingezäunte Wiese, Schatten, Trinkbrunnen im Sommer."
                ],
            )
            .map_err(db_err)?;

            conn.execute(
                "INSERT INTO park_spots (name, city, fenced, water_nearby, rating, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    "Donauinsel Hundestrand (Nordbrücke)",
                    "Wien (21. Bezirk)",
                    0,
                    1,
                    5,
                    "Perfekt für Wasserhunde im Sommer; nicht eingezäunt."
                ],
            )
            .map_err(db_err)?;
        }
        Ok(())
    }

    pub fn list_dogs(&self) -> Result<Vec<DogProfile>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, name, breed, birth_date, weight_kg, chip_id, insurance, vet_name, vet_phone, notes FROM dogs ORDER BY id ASC")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(DogProfile {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    breed: r.get(2)?,
                    birth_date: r.get(3)?,
                    weight_kg: r.get::<_, f64>(4)? as f32,
                    chip_id: r.get(5)?,
                    insurance: r.get(6)?,
                    vet_name: r.get(7)?,
                    vet_phone: r.get(8)?,
                    notes: r.get(9)?,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    pub fn upsert_dog(&self, d: &DogProfile) -> Result<i64, PuppyCraftError> {
        let conn = self.lock()?;
        if d.id > 0 {
            conn.execute(
                "UPDATE dogs SET name=?1, breed=?2, birth_date=?3, weight_kg=?4, chip_id=?5, insurance=?6, vet_name=?7, vet_phone=?8, notes=?9 WHERE id=?10",
                params![
                    d.name,
                    d.breed,
                    d.birth_date,
                    f64::from(d.weight_kg),
                    d.chip_id,
                    d.insurance,
                    d.vet_name,
                    d.vet_phone,
                    d.notes,
                    d.id
                ],
            )
            .map_err(db_err)?;
            Ok(d.id)
        } else {
            conn.execute(
                "INSERT INTO dogs (name, breed, birth_date, weight_kg, chip_id, insurance, vet_name, vet_phone, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    d.name,
                    d.breed,
                    d.birth_date,
                    f64::from(d.weight_kg),
                    d.chip_id,
                    d.insurance,
                    d.vet_name,
                    d.vet_phone,
                    d.notes
                ],
            )
            .map_err(db_err)?;
            Ok(conn.last_insert_rowid())
        }
    }

    pub fn list_diary(&self, limit: usize) -> Result<Vec<DiaryEntry>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, dog_name, date, title, note, mood, tag, weight_kg, photo_path FROM diary_entries ORDER BY date DESC, id DESC LIMIT ?1")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([limit as i64], |r| {
                Ok(DiaryEntry {
                    id: r.get(0)?,
                    dog_name: r.get(1)?,
                    date: r.get(2)?,
                    title: r.get(3)?,
                    note: r.get(4)?,
                    mood: r.get(5)?,
                    tag: r.get(6)?,
                    weight_kg: r.get::<_, Option<f64>>(7)?.map(|v| v as f32),
                    photo_path: r.get(8)?,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_diary(
        &self,
        dog_name: &str,
        date: &str,
        title: &str,
        note: &str,
        mood: &str,
        tag: &str,
        weight_kg: Option<f32>,
        photo_path: Option<&str>,
    ) -> Result<i64, PuppyCraftError> {
        if title.trim().is_empty() || note.trim().is_empty() {
            return Err(PuppyCraftError::Invalid(
                "title and note must not be empty".into(),
            ));
        }
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO diary_entries (dog_name, date, title, note, mood, tag, weight_kg, photo_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                dog_name,
                date,
                title,
                note,
                mood,
                tag,
                weight_kg.map(f64::from),
                photo_path
            ],
        )
        .map_err(db_err)?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_sitter_events(&self) -> Result<Vec<SitterEvent>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, dog_name, sitter_name, relation, start_date, end_date, feeding_notes, walk_notes, emergency_contact, status FROM sitter_events ORDER BY start_date ASC, id ASC")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(SitterEvent {
                    id: r.get(0)?,
                    dog_name: r.get(1)?,
                    sitter_name: r.get(2)?,
                    relation: r.get(3)?,
                    start_date: r.get(4)?,
                    end_date: r.get(5)?,
                    feeding_notes: r.get(6)?,
                    walk_notes: r.get(7)?,
                    emergency_contact: r.get(8)?,
                    status: r.get(9)?,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_sitter_event(
        &self,
        dog_name: &str,
        sitter_name: &str,
        relation: &str,
        start_date: &str,
        end_date: &str,
        feeding_notes: &str,
        walk_notes: &str,
        emergency_contact: &str,
    ) -> Result<i64, PuppyCraftError> {
        if sitter_name.trim().is_empty() || start_date.trim().is_empty() {
            return Err(PuppyCraftError::Invalid(
                "sitter_name and start_date are required".into(),
            ));
        }
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO sitter_events (dog_name, sitter_name, relation, start_date, end_date, feeding_notes, walk_notes, emergency_contact, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'Geplant')",
            params![
                dog_name,
                sitter_name,
                relation,
                start_date,
                end_date,
                feeding_notes,
                walk_notes,
                emergency_contact
            ],
        )
        .map_err(db_err)?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_vaccinations(&self) -> Result<Vec<VaccineRecord>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, dog_name, vaccine_name, date_given, next_due, vet_clinic, batch_no, notes FROM vaccinations ORDER BY next_due ASC")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(VaccineRecord {
                    id: r.get(0)?,
                    dog_name: r.get(1)?,
                    vaccine_name: r.get(2)?,
                    date_given: r.get(3)?,
                    next_due: r.get(4)?,
                    vet_clinic: r.get(5)?,
                    batch_no: r.get(6)?,
                    notes: r.get(7)?,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    pub fn add_vaccination(
        &self,
        dog_name: &str,
        vaccine_name: &str,
        date_given: &str,
        next_due: &str,
        vet_clinic: &str,
        notes: &str,
    ) -> Result<i64, PuppyCraftError> {
        if vaccine_name.trim().is_empty() || date_given.trim().is_empty() {
            return Err(PuppyCraftError::Invalid(
                "vaccine_name and date_given are required".into(),
            ));
        }
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO vaccinations (dog_name, vaccine_name, date_given, next_due, vet_clinic, batch_no, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, '', ?6)",
            params![dog_name, vaccine_name, date_given, next_due, vet_clinic, notes],
        )
        .map_err(db_err)?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_photos(&self) -> Result<Vec<PhotoRecord>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, dog_name, file_path, caption, date FROM photos ORDER BY id DESC")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(PhotoRecord {
                    id: r.get(0)?,
                    dog_name: r.get(1)?,
                    file_path: r.get(2)?,
                    caption: r.get(3)?,
                    date: r.get(4)?,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    pub fn add_photo(
        &self,
        dog_name: &str,
        file_path: &str,
        caption: &str,
        date: &str,
    ) -> Result<i64, PuppyCraftError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO photos (dog_name, file_path, caption, date) VALUES (?1, ?2, ?3, ?4)",
            params![dog_name, file_path, caption, date],
        )
        .map_err(db_err)?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, PuppyCraftError> {
        let conn = self.lock()?;
        conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
            r.get(0)
        })
        .optional()
        .map_err(db_err)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), PuppyCraftError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )
        .map_err(db_err)?;
        Ok(())
    }

    pub fn update_sitter_status(&self, id: i64, status: &str) -> Result<(), PuppyCraftError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE sitter_events SET status = ?1 WHERE id = ?2",
            params![status, id],
        )
        .map_err(db_err)?;
        Ok(())
    }

    pub fn delete_sitter_event(&self, id: i64) -> Result<(), PuppyCraftError> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM sitter_events WHERE id = ?1", [id])
            .map_err(db_err)?;
        Ok(())
    }

    pub fn list_vet_visits(&self) -> Result<Vec<VetVisit>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, dog_name, date, clinic, reason, diagnosis_meds, cost_eur FROM vet_visits ORDER BY date DESC, id DESC")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(VetVisit {
                    id: r.get(0)?,
                    dog_name: r.get(1)?,
                    date: r.get(2)?,
                    clinic: r.get(3)?,
                    reason: r.get(4)?,
                    diagnosis_meds: r.get(5)?,
                    cost_eur: r.get::<_, f64>(6)? as f32,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    pub fn add_vet_visit(
        &self,
        dog_name: &str,
        date: &str,
        clinic: &str,
        reason: &str,
        diagnosis_meds: &str,
        cost_eur: f32,
    ) -> Result<i64, PuppyCraftError> {
        if reason.trim().is_empty() || date.trim().is_empty() {
            return Err(PuppyCraftError::Invalid(
                "date and reason are required".into(),
            ));
        }
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO vet_visits (dog_name, date, clinic, reason, diagnosis_meds, cost_eur)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                dog_name,
                date,
                clinic,
                reason,
                diagnosis_meds,
                f64::from(cost_eur)
            ],
        )
        .map_err(db_err)?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list_park_spots(&self) -> Result<Vec<ParkSpot>, PuppyCraftError> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, name, city, fenced, water_nearby, rating, notes FROM park_spots ORDER BY rating DESC, id DESC")
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(ParkSpot {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    city: r.get(2)?,
                    fenced: r.get::<_, i64>(3)? != 0,
                    water_nearby: r.get::<_, i64>(4)? != 0,
                    rating: r.get::<_, i64>(5)? as u8,
                    notes: r.get(6)?,
                })
            })
            .map_err(db_err)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
    }

    pub fn add_park_spot(
        &self,
        name: &str,
        city: &str,
        fenced: bool,
        water_nearby: bool,
        rating: u8,
        notes: &str,
    ) -> Result<i64, PuppyCraftError> {
        if name.trim().is_empty() || city.trim().is_empty() {
            return Err(PuppyCraftError::Invalid(
                "name and city are required".into(),
            ));
        }
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO park_spots (name, city, fenced, water_nearby, rating, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                name,
                city,
                i64::from(fenced),
                i64::from(water_nearby),
                i64::from(rating.clamp(1, 5)),
                notes
            ],
        )
        .map_err(db_err)?;
        Ok(conn.last_insert_rowid())
    }
}
