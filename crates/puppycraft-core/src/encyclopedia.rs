use crate::models::{Breed, DogSize, LivingType, MatchProfile, MatchResult};

#[allow(clippy::too_many_arguments)]
fn b(
    id: &str,
    name_de: &str,
    name_en: &str,
    fci_group: &str,
    origin: &str,
    size: DogSize,
    w_min: f32,
    w_max: f32,
    lifespan: &str,
    energy: u8,
    diff: u8,
    bark: u8,
    shed: u8,
    kid: u8,
    cat: u8,
    apt: u8,
    rural: u8,
    alone: u8,
    exercise_min: u16,
    cost_eur: u16,
    temperament: &str,
    health: &str,
    ideal_for: &str,
    watch_out: &str,
) -> Breed {
    Breed {
        id: id.into(),
        name_de: name_de.into(),
        name_en: name_en.into(),
        fci_group: fci_group.into(),
        origin: origin.into(),
        size,
        weight_kg_min: w_min,
        weight_kg_max: w_max,
        lifespan_years: lifespan.into(),
        energy,
        beginner_difficulty: diff,
        barking: bark,
        shedding: shed,
        kid_friendly: kid,
        cat_friendly: cat,
        apartment_fit: apt,
        rural_fit: rural,
        alone_tolerance: alone,
        exercise_minutes_day: exercise_min,
        monthly_cost_eur: cost_eur,
        temperament: temperament.into(),
        health_notes: health.into(),
        ideal_for: ideal_for.into(),
        watch_out: watch_out.into(),
    }
}

pub fn all_breeds() -> Vec<Breed> {
    vec![
        b(
            "golden-retriever",
            "Golden Retriever",
            "Golden Retriever",
            "FCI 8 (Apportierhunde)",
            "Schottland / UK",
            DogSize::Large,
            25.0,
            34.0,
            "10-12",
            3,
            1,
            2,
            4,
            5,
            5,
            3,
            5,
            3,
            90,
            160,
            "Freundlich, geduldig, menschenbezogen, leicht erziehbar, liebt Wasser",
            "HD/ED, PRA (Augen), Neigung zu Übergewicht & Tumoren im Alter",
            "Familien mit Kindern & Katze, Ersthundebesitzer, Therapie-/Familienhund",
            "Haart stark im Fellwechsel; braucht täglich geistige und körperliche Auslastung",
        ),
        b(
            "labrador-retriever",
            "Labrador Retriever",
            "Labrador Retriever",
            "FCI 8 (Apportierhunde)",
            "Neufundland / UK",
            DogSize::Large,
            25.0,
            36.0,
            "10-12",
            4,
            1,
            2,
            4,
            5,
            4,
            3,
            5,
            3,
            90,
            160,
            "Fröhlich, verfressen, extrem sozial, arbeitsfreudig",
            "HD/ED, EIC, Adipositas-Neigung (Futter abwiegen!)",
            "Aktive Familien mit Kindern, Anfänger, Apportier-Fans",
            "Sehr stürmisch in den ersten 2 Jahren; frisst auf Spaziergängen alles",
        ),
        b(
            "cavalier-king-charles",
            "Cavalier King Charles Spaniel",
            "Cavalier King Charles Spaniel",
            "FCI 9 (Gesellschaftshunde)",
            "Großbritannien",
            DogSize::Small,
            5.5,
            8.5,
            "10-14",
            2,
            1,
            2,
            3,
            5,
            5,
            5,
            3,
            2,
            45,
            110,
            "Sanft, anhänglich, ruhig, liebt Kuscheln und moderate Spaziergänge",
            "MVD (Herzklappen), Syringomyelie — nur vom geprüften Züchter mit Herzschall!",
            "Einsame Oldies / Senioren, Innenstadt-Wohnung, ruhige Familien",
            "Ungern lange allein; Erbkrankheiten erfordern seriöse Züchterauswahl",
        ),
        b(
            "bichon-frise",
            "Bichon Frisé",
            "Bichon Frise",
            "FCI 9 (Gesellschaftshunde)",
            "Frankreich / Belgien",
            DogSize::Small,
            4.0,
            6.5,
            "12-15",
            2,
            1,
            2,
            1,
            5,
            5,
            5,
            3,
            3,
            45,
            105,
            "Fröhlich, unkompliziert, haart kaum, verspielt aber anpassungsfähig",
            "Patellaluxation, Tränenstein, regelmäßige Fellpflege nötig",
            "Einsame Oldies, Allergiker-freundlich, Innenstadt-Wohnung, Familien",
            "Alle 6-8 Wochen Hundefriseur einplanen",
        ),
        b(
            "malteser",
            "Malteser",
            "Maltese",
            "FCI 9 (Gesellschaftshunde)",
            "Zentraler Mittelmeerraum",
            DogSize::Toy,
            3.0,
            4.0,
            "12-15",
            2,
            1,
            3,
            1,
            3,
            5,
            5,
            2,
            2,
            40,
            95,
            "Anhänglich, klug, verspielt, haart nicht",
            "Zahnstein, Patellaluxation, Tränenkanäle",
            "Senioren / Oldies, kleine Stadtwohnung, ruhige Haushalte",
            "Sehr zart gebaut — nicht ideal für stürmische Kleinkinder ohne Aufsicht",
        ),
        b(
            "havaneser",
            "Havaneser",
            "Havanese",
            "FCI 9 (Gesellschaftshunde)",
            "Kuba / Westlicher Mittelmeerraum",
            DogSize::Small,
            4.5,
            7.0,
            "13-15",
            2,
            1,
            2,
            1,
            5,
            5,
            5,
            3,
            3,
            50,
            105,
            "Clown, extrem freundlich, kaum Jagdtrieb, haart kaum",
            "Katarakt, Patellaluxation; sehr robuste Kleinrasse",
            "Familien mit Kindern & Katze, Senioren, Ersthundebesitzer, Innenstadt",
            "Langes Fell braucht regelmäßiges Bürsten oder praktischen Kurzhaarschnitt",
        ),
        b(
            "zwergpudel",
            "Zwergpudel / Kleinpudel",
            "Miniature Poodle",
            "FCI 9 (Gesellschaftshunde)",
            "Frankreich / Deutschland",
            DogSize::Small,
            5.0,
            12.0,
            "13-16",
            3,
            1,
            2,
            1,
            5,
            5,
            5,
            4,
            3,
            60,
            120,
            "Hochintelligent, gelehrig, sportlich und doch wohnungstauglich, haart nicht",
            "PRA, Patellaluxation; regelmäßiges Scheren nötig",
            "Fast alle: Familien mit Kids & Katze, aktive Senioren, Stadt & Land",
            "Braucht Kopfarbeit, sonst erfindet er eigene Aufgaben",
        ),
        b(
            "grosspudel",
            "Großpudel (Königspudel)",
            "Standard Poodle",
            "FCI 9 (Gesellschaftshunde)",
            "Frankreich / Deutschland",
            DogSize::Large,
            20.0,
            30.0,
            "12-15",
            4,
            2,
            2,
            1,
            5,
            4,
            4,
            5,
            3,
            90,
            150,
            "Eleganter Allrounder, hochintelligent, kaum Haarausfall, sensibel",
            "Magendrehung, HD, Addison-Krankheit",
            "Aktive Familien mit Kindern, Allergiker, Hundesport",
            "Pflegeaufwand beim Fell; braucht klare, sanfte Führung",
        ),
        b(
            "berner-sennenhund",
            "Berner Sennenhund",
            "Bernese Mountain Dog",
            "FCI 2 (Sennenhunde)",
            "Schweiz",
            DogSize::Giant,
            36.0,
            52.0,
            "7-9",
            3,
            2,
            2,
            5,
            5,
            5,
            1,
            5,
            3,
            75,
            210,
            "Gutmütig, ruhig, wachsam ohne Aggression, liebt Kälte und Berge",
            "Leider kürzere Lebenserwartung (Krebs, HD/ED); hitzeempfindlich",
            "Almöhi / Bergbauer, Haus mit großem Garten, Familien auf dem Land",
            "Nicht für Innenstadt-Wohnungen mit Treppen oder heiße Dachgeschosse",
        ),
        b(
            "leonberger",
            "Leonberger",
            "Leonberger",
            "FCI 2 (Molossoide)",
            "Deutschland",
            DogSize::Giant,
            48.0,
            75.0,
            "8-10",
            3,
            2,
            2,
            5,
            5,
            5,
            1,
            5,
            3,
            80,
            240,
            "Sanfter Riese, gelassen, familienlieb, beeindruckende Erscheinung",
            "HD/ED, LPN1/2 (Polyneuropathie), Magendrehung",
            "Almöhi / großes Grundstück, erfahrene Familien mit viel Platz",
            "Hohe Futter- & Tierarztkosten, sabbert & haart stark",
        ),
        b(
            "border-collie",
            "Border Collie",
            "Border Collie",
            "FCI 1 (Hütehunde)",
            "Großbritannien",
            DogSize::Medium,
            14.0,
            20.0,
            "12-15",
            5,
            5,
            3,
            4,
            3,
            2,
            1,
            5,
            2,
            150,
            140,
            "Extrem intelligent, reaktionsschnell, braucht echte Aufgabe & Ruhe-Training",
            "MDR1-Gendefekt, CEA, Epilepsie",
            "Almöhi / Hof mit Schafen, sehr sportliche erfahrene Hundesportler",
            "Hütet sonst Kinder, Autos oder Katzen! Kein Couch- oder Stadtanfängerhund",
        ),
        b(
            "australian-shepherd",
            "Australian Shepherd",
            "Australian Shepherd",
            "FCI 1 (Hütehunde)",
            "USA",
            DogSize::Medium,
            18.0,
            29.0,
            "12-15",
            5,
            4,
            3,
            4,
            3,
            3,
            2,
            5,
            2,
            130,
            150,
            "Arbeitsfreudig, wachsam, eng gebunden, Schutztrieb möglich",
            "MDR1-Defekt, HD/ED, Katarakt",
            "Aktive Outdoor-Menschen, Almöhi, erfahrene Familien",
            "Ohne Konsequenz und Ruhelernen schnell überdreht und territorial",
        ),
        b(
            "deutscher-schaeferhund",
            "Deutscher Schäferhund",
            "German Shepherd",
            "FCI 1 (Schäferhunde)",
            "Deutschland",
            DogSize::Large,
            22.0,
            40.0,
            "10-12",
            4,
            4,
            4,
            5,
            4,
            2,
            2,
            5,
            3,
            120,
            175,
            "Loyal, mutig, arbeitswillig, starker Schutz- und Wachtrieb",
            "HD/ED, Cauda Equina, DM (Degenerative Myelopathie)",
            "Almöhi / Haus mit Grundstück, Hundesportler mit Erfahrung",
            "Braucht konsequente Ausbildung; in manchen Städten/Mietwohnungen Vorbehalte",
        ),
        b(
            "dackel",
            "Dackel (Teckel)",
            "Dachshund",
            "FCI 4 (Dachshunde)",
            "Deutschland",
            DogSize::Small,
            4.5,
            9.5,
            "12-16",
            3,
            3,
            4,
            2,
            3,
            2,
            4,
            4,
            3,
            60,
            100,
            "Charakterstark, witzig, mutig, eigener Kopf, starker Jagdtrieb",
            "Dackellähme (Bandscheibenvorfall) — keine vielen Treppen!",
            "Stadt & Land (Erdgeschoss/Lift), humorvolle Menschen mit Konsequenz",
            "Bellfreudig; Jagdtrieb gegenüber Katzen/Kaninchen; Rücken schonen",
        ),
        b(
            "franzoesische-bulldogge",
            "Französische Bulldogge",
            "French Bulldog",
            "FCI 9 (Gesellschaftshunde)",
            "Frankreich",
            DogSize::Small,
            9.0,
            14.0,
            "9-11",
            2,
            2,
            1,
            2,
            5,
            4,
            5,
            2,
            3,
            45,
            180,
            "Clownig, menschenbezogen, bellt kaum, mag kurze Runden",
            "ACHTUNG Qualzucht-Risiko: Brachycephalie (Atemnot), Wirbelsäule, Allergien, Hitzeintoleranz",
            "Stadtbewohner, wenn aus seriöser Freiatmungs-Zucht (besser: Retromops/Boston/Bichon)",
            "Hohe Tierarztkosten; bei Hitze über 24 Grad extrem gefährdet",
        ),
        b(
            "mops",
            "Mops (bevorzugt Retromops)",
            "Pug",
            "FCI 9 (Gesellschaftshunde)",
            "China / UK",
            DogSize::Small,
            6.5,
            9.0,
            "11-14",
            1,
            1,
            2,
            3,
            5,
            5,
            5,
            2,
            3,
            40,
            165,
            "Gemütlich, charmant, friedlich mit Kindern & Katzen",
            "ACHTUNG: Standard-Mops oft Atemnot (BOAS) & Augenprobleme; nur Retro-Zucht wählen!",
            "Ruhige Haushalte (nur mit nachgewiesener Nasenlänge/Freiatmung)",
            "Schnarcht, haart überraschend viel, Hitzeempfindlichkeit",
        ),
        b(
            "beagle",
            "Beagle",
            "Beagle",
            "FCI 6 (Laufhunde)",
            "Großbritannien",
            DogSize::Medium,
            10.0,
            16.0,
            "12-15",
            4,
            3,
            4,
            3,
            5,
            3,
            3,
            4,
            3,
            90,
            130,
            "Immer fröhlich, robust, extrem sozial mit Kindern und Hunden",
            "Neigung zu Übergewicht, Ohrenentzündungen, Bandscheiben",
            "Aktive Familien mit Kindern",
            "Starker Nasen- & Jagdtrieb (Freilauf schwierig), lautstarkes Heulen",
        ),
        b(
            "jack-russell",
            "Jack Russell Terrier",
            "Jack Russell Terrier",
            "FCI 3 (Terrier)",
            "Großbritannien / Australien",
            DogSize::Small,
            5.0,
            8.0,
            "13-16",
            5,
            4,
            4,
            3,
            3,
            1,
            3,
            5,
            3,
            90,
            105,
            "Energiebündel im Kleinformat, furchtlos, schlau, jagdpassionierter Terrier",
            "Patellaluxation, Linsenluxation",
            "Sportliche Menschen, Reiterhof / Almöhi",
            "Unterschätzter Powerhund; starker Jagdtrieb gegen Katzen/Kleintiere",
        ),
        b(
            "shiba-inu",
            "Shiba Inu",
            "Shiba Inu",
            "FCI 5 (Spitze & Urtyp)",
            "Japan",
            DogSize::Small,
            8.0,
            11.0,
            "12-15",
            3,
            4,
            2,
            4,
            3,
            2,
            4,
            4,
            4,
            75,
            120,
            "Katzenhaft unabhängig, sehr sauber, ruhig im Haus, eigenwillig",
            "Allergien, Patellaluxation, Glaukom",
            "Erfahrene Einzelpersonen/Paare, die keinen 'Will-to-please'-Hund wollen",
            "Schwieriger Rückruf, 'Shiba Scream', mag oft keine fremden Hunde",
        ),
        b(
            "whippet",
            "Whippet",
            "Whippet",
            "FCI 10 (Windhunde)",
            "Großbritannien",
            DogSize::Medium,
            11.0,
            16.0,
            "12-15",
            3,
            2,
            1,
            1,
            5,
            3,
            5,
            4,
            4,
            60,
            125,
            "Drinnen ein ruhiger Couch-Dekorations-Hund, draußen ein 55 km/h Sprinter",
            "Sehr gesund; dünne Haut (Wintermantel nötig!), narkoseempfindlich",
            "Innenstadt-Wohnung, ruhige Familien, Oldies mit Sprintmöglichkeit",
            "Sichtjäger: Katze im Freien wird gejagt, im Haus meist akzeptiert",
        ),
        b(
            "greyhound",
            "Greyhound (auch Ex-Racer Rescue)",
            "Greyhound",
            "FCI 10 (Windhunde)",
            "Großbritannien",
            DogSize::Large,
            27.0,
            38.0,
            "10-13",
            2,
            2,
            1,
            1,
            4,
            2,
            5,
            4,
            4,
            60,
            155,
            "Überraschend fauler '45-mph Couch Potato', sanftmütig, bellt fast nie",
            "Osteosarkom-Vorsorge, dünne Haut, Kälteempfindlichkeit",
            "Innenstadt-Wohnung mit Aufzug, ruhige Menschen, Tierschutz-Adoptanten",
            "Starker Sichtjäger — Freilauf nur in eingezäunten Zonen; oft nicht katzenfest",
        ),
        b(
            "chihuahua",
            "Chihuahua",
            "Chihuahua",
            "FCI 9 (Gesellschaftshunde)",
            "Mexiko",
            DogSize::Toy,
            1.5,
            3.0,
            "14-18",
            2,
            2,
            4,
            2,
            2,
            4,
            5,
            2,
            3,
            35,
            85,
            "Mutig, loyal, extrem handlich (passt in Flugzeugkabine & Tokio-Tasche)",
            "Offene Fontanelle, Patellaluxation, Zahnprobleme, Unterkühlung",
            "Ruhige Einzelpersonen, Reisende, Stadtwohnung ohne kleine Kinder",
            "Wird oft verhätschelt und dann zum Kläffer; sehr zerbrechlich für Kids",
        ),
        b(
            "samojede",
            "Samojede",
            "Samoyed",
            "FCI 5 (Nordische Schlittenhunde)",
            "Sibirien / Russland",
            DogSize::Large,
            18.0,
            28.0,
            "12-14",
            4,
            3,
            4,
            5,
            5,
            3,
            2,
            5,
            2,
            110,
            165,
            "Das berühmte 'Samojeden-Lächeln', freundlich zu allen, gesprächig",
            "HD, Diabetes, Nieren; leidet im Hochsommer",
            "Almöhi / kühle Regionen, sehr aktive Familien",
            "Extremes Haaren und viel Bellen/Singen; nicht gern allein",
        ),
        b(
            "vizsla",
            "Magyar Vizsla",
            "Hungarian Vizsla",
            "FCI 7 (Vorstehhunde)",
            "Ungarn",
            DogSize::Medium,
            20.0,
            29.0,
            "12-14",
            5,
            3,
            2,
            2,
            5,
            3,
            3,
            5,
            2,
            120,
            150,
            "Klettenhund ('Velcro Dog'), sensibel, sportlich, kurzhaarig ohne Unterwolle",
            "HD, Allergien, Kälteempfindlichkeit im Winter",
            "Läufer, Radfahrer, aktive Familien mit viel Zeit",
            "Verkraftet wenig Alleinsein und harte Erziehung überhaupt nicht",
        ),
        b(
            "rhodesian-ridgeback",
            "Rhodesian Ridgeback",
            "Rhodesian Ridgeback",
            "FCI 6 (Schweißhunde)",
            "Südliches Afrika",
            DogSize::Large,
            32.0,
            41.0,
            "10-12",
            4,
            4,
            1,
            2,
            4,
            2,
            3,
            5,
            3,
            110,
            180,
            "Stolz, ruhig im Haus, bellt kaum, spätreif (erst mit 3 Jahren erwachsen)",
            "Dermoid Sinus, HD/ED, Schilddrüse",
            "Erfahrene, ruhige aber konsequente Menschen mit Platz",
            "Große Kraft, Jagdtrieb, braucht echte Souveränität",
        ),
        b(
            "boxer",
            "Deutscher Boxer",
            "Boxer",
            "FCI 2 (Molossoide)",
            "Deutschland",
            DogSize::Large,
            25.0,
            33.0,
            "10-12",
            4,
            3,
            2,
            2,
            5,
            4,
            3,
            4,
            2,
            95,
            170,
            "Verspielter Kindskopf bis ins Alter, extrem kinderlieb, wachsam",
            "Herzerkrankungen (Aortenstenose), Tumore, leichte Kurzköpfigkeit",
            "Sportliche Familien mit älteren Kindern",
            "Stürmisch (wirft Kleinkinder vor Freude um), hitzeempfindlich",
        ),
        b(
            "mittelspitz",
            "Deutscher Mittelspitz",
            "German Spitz (Mittelspitz)",
            "FCI 5 (Spitze)",
            "Deutschland",
            DogSize::Small,
            7.0,
            11.0,
            "13-16",
            3,
            2,
            4,
            3,
            4,
            4,
            4,
            5,
            4,
            60,
            105,
            "Wachsam, treu, kein Jagdtrieb (!), klebt an Hof und Familie",
            "Sehr gesund und langlebig; Patellaluxation prüfen",
            "Almöhi / Hofhund ohne Wilderei, auch aktive Senioren auf dem Land",
            "Meldet zuverlässig jeden Besucher — in hellhörigen Mietshäusern Training nötig",
        ),
        b(
            "sheltie",
            "Shetland Sheepdog (Sheltie)",
            "Shetland Sheepdog",
            "FCI 1 (Hütehunde)",
            "Schottland (Shetlandinseln)",
            DogSize::Small,
            6.0,
            11.0,
            "12-15",
            3,
            2,
            4,
            4,
            5,
            5,
            4,
            5,
            3,
            70,
            115,
            "Sanfter Mini-Collie, sehr gelehrig, kinder- und katzenlieb",
            "MDR1-Defekt, CEA, regelmäßig kämmen",
            "Familien mit Kindern & Katze, aktive Senioren, Agility-Einsteiger",
            "Sensibel und teils bellfreudig bei Aufregung",
        ),
        b(
            "cocker-spaniel",
            "English Cocker Spaniel",
            "English Cocker Spaniel",
            "FCI 8 (Stöberhunde)",
            "Großbritannien",
            DogSize::Medium,
            12.0,
            15.0,
            "12-15",
            3,
            2,
            2,
            3,
            5,
            4,
            4,
            4,
            3,
            80,
            130,
            "Fröhlich, wedelnd, verschmust, handliche Mittelgröße",
            "Ohrenentzündungen (lange Schlappohren!), PRA, Nierenerkrankung",
            "Familien mit Kindern, aktive Oldies, Stadt & Land",
            "Stöbert gern im Gebüsch; Ohren regelmäßig kontrollieren",
        ),
        b(
            "mischling-tierheim",
            "Ruhiger Tierheim-Mischling (Adult)",
            "Calm Adult Shelter Mix",
            "Tierschutz / Mischling",
            "Lokal / Tierheim",
            DogSize::Medium,
            10.0,
            22.0,
            "12-16",
            2,
            1,
            2,
            2,
            4,
            4,
            4,
            4,
            4,
            60,
            115,
            "Charakter bereits fertig entwickelt und vom Tierheim eingeschätzt; oft sehr dankbar",
            "Robust durch genetische Vielfalt; vorherigen Tierarztcheck einsehen",
            "Einsame Oldies (älterer Hund!), Familien mit Katze (katzengetestet!), Ersthundehalter",
            "Vorgeschichte beachten — Kennenlernen & Probespaziergänge im Tierheim nutzen",
        ),
    ]
}

fn size_rank(s: DogSize) -> u8 {
    match s {
        DogSize::Toy => 1,
        DogSize::Small => 2,
        DogSize::Medium => 3,
        DogSize::Large => 4,
        DogSize::Giant => 5,
    }
}

pub fn find_breed(query: &str) -> Option<Breed> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    all_breeds().into_iter().find(|br| {
        br.id.to_lowercase() == q
            || br.name_de.to_lowercase().contains(&q)
            || br.name_en.to_lowercase().contains(&q)
    })
}

pub fn match_breeds(profile: &MatchProfile) -> Vec<MatchResult> {
    let mut results: Vec<MatchResult> = all_breeds()
        .into_iter()
        .map(|br| score_breed(&br, profile))
        .collect();
    results.sort_by_key(|a| std::cmp::Reverse(a.score));
    results
}

fn score_breed(br: &Breed, p: &MatchProfile) -> MatchResult {
    let mut score: i32 = 70;
    let mut pros = Vec::new();
    let mut warnings = Vec::new();

    // Size check
    if size_rank(br.size) > size_rank(p.max_size) {
        score -= 22;
        warnings.push(format!("Größer als gewünscht ({})", br.size.label_de()));
    } else {
        score += 4;
    }

    // Living type
    match p.living {
        LivingType::CityApartment => {
            score += (i32::from(br.apartment_fit) - 3) * 8;
            if br.apartment_fit >= 4 {
                pros.push("Sehr gut für Innenstadt-Wohnung geeignet".into());
            } else if br.apartment_fit <= 2 {
                warnings.push("Wenig geeignet für reine Stadtwohnung".into());
            }
            if !p.has_elevator_or_ground && matches!(br.size, DogSize::Large | DogSize::Giant) {
                score -= 15;
                warnings.push("Schwer zu tragen bei Treppen ohne Lift (Gelenke/Alter)".into());
            }
            if !p.has_elevator_or_ground && br.id == "dackel" {
                score -= 12;
                warnings.push("Dackel-Rücken verträgt tägliches Treppensteigen schlecht".into());
            }
        }
        LivingType::SuburbanHouse => {
            score += 4;
        }
        LivingType::RuralFarmAlmoehi => {
            score += (i32::from(br.rural_fit) - 3) * 8;
            if br.rural_fit >= 4 {
                pros.push("Blüht auf Hof / Alm / im ländlichen Raum richtig auf".into());
            } else if br.rural_fit <= 2 {
                warnings.push("Sehr zart/kälteempfindlich für raues Landleben".into());
            }
        }
    }

    // Kids
    if p.has_kids {
        score += (i32::from(br.kid_friendly) - 3) * 8;
        if br.kid_friendly >= 4 {
            pros.push("Sehr kinderfreundlich und geduldig".into());
        } else if br.kid_friendly <= 2 {
            warnings.push("Nicht ideal für trubelige Haushalte mit Kindern".into());
        }
        if p.kids_count >= 3 && br.size == DogSize::Toy {
            score -= 18;
            warnings.push("Sehr zerbrechlich bei 3 Kindern".into());
        }
    }

    // Cats
    if p.has_cats {
        score += (i32::from(br.cat_friendly) - 3) * 9;
        if br.cat_friendly >= 4 {
            pros.push("Gute Verträglichkeit mit Katzen".into());
        } else if br.cat_friendly <= 2 {
            warnings.push("Starker Jagd-/Hütetrieb gegenüber Katzen!".into());
        }
    }

    // Senior owner ("Einsamer Oldie")
    if p.is_senior_owner {
        if br.energy <= 2 && matches!(br.size, DogSize::Toy | DogSize::Small | DogSize::Medium) {
            score += 12;
            pros.push("Handliche Größe & entspanntes Tempo für Senioren".into());
        }
        if br.energy >= 4 || matches!(br.size, DogSize::Giant) {
            score -= 20;
            warnings.push("Körperlich zu fordernd / schwer zu halten im Alter".into());
        }
    }

    // Experience
    if p.first_time_owner {
        if br.beginner_difficulty <= 2 {
            score += 8;
            pros.push("Sehr anfängerfreundlich".into());
        } else if br.beginner_difficulty >= 4 {
            score -= 22;
            warnings.push("Anspruchsvolle Rasse — eher für Hundeprofis".into());
        }
    }

    // Exercise match
    let diff_min = i32::from(p.daily_exercise_min) - i32::from(br.exercise_minutes_day);
    if diff_min < -30 {
        score -= 20;
        warnings.push(format!(
            "Braucht ca. {} Min/Tag Auslauf (mehr als angegeben)",
            br.exercise_minutes_day
        ));
    } else if diff_min >= 0 {
        score += 5;
    }

    // Alone time
    if p.alone_hours_day >= 5.0 && br.alone_tolerance <= 2 {
        score -= 16;
        warnings.push("Verkraftet längeres Alleinsein schlecht".into());
    }

    // Quiet neighbors
    if p.quiet_neighbor_needed {
        if br.barking <= 2 {
            score += 6;
            pros.push("Ruhige, bellarme Rasse".into());
        } else if br.barking >= 4 {
            score -= 12;
            warnings.push("Bellfreudig — Ärger mit Nachbarn möglich".into());
        }
    }

    // Low shedding
    if p.low_shedding_preferred {
        if br.shedding <= 1 {
            score += 10;
            pros.push("Haart kaum / allergikerfreundlicher".into());
        } else if br.shedding >= 4 {
            score -= 10;
            warnings.push("Starkes Haaren".into());
        }
    }

    let clamped = score.clamp(5, 99) as u8;
    MatchResult {
        breed: br.clone(),
        score: clamped,
        pros,
        warnings,
    }
}
