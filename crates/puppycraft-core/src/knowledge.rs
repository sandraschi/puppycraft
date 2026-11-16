use crate::models::{
    BreederInfo, DogSize, InsuranceProvider, LegalRule, MedicalProblem, RescueOrg, SeniorSection,
    ToxicSubstance, TrainerInfo, TrainingGuide, TravelDestination, VetClinicInfo,
};
use serde::{Deserialize, Serialize};

pub fn breeders() -> Vec<BreederInfo> {
    vec![
        BreederInfo {
            name: "ÖKV — Österreichischer Kynologenverband".into(),
            region: "Österreich (bundesweit)".into(),
            association: "FCI / ÖKV".into(),
            breeds: "Alle FCI-anerkannten Rassen (Züchtervermittlung pro Rasseclub)".into(),
            website: "https://www.oekv.at".into(),
            health_checks: "Pflicht-Gesundheitsuntersuchungen je Rasse (HD/ED, Augen PRA, Herz, DNA), Wurfabnahme durch Zuchtwart".into(),
            notes: "Seriöse Anlaufstelle in Österreich; immer auch den zuständigen Rassezuchtverein kontaktieren.".into(),
        },
        BreederInfo {
            name: "VDH — Verband für das Deutsche Hundewesen".into(),
            region: "Deutschland (bundesweit)".into(),
            association: "FCI / VDH".into(),
            breeds: "Über 250 Rassen über angeschlossene Mitgliedsvereine".into(),
            website: "https://www.vdh.de".into(),
            health_checks: "Zuchtzulassungsprüfung, HD/ED-Gutachten, Zuchtwart-Kontrolle, Mindestabgabealter 8 Wochen".into(),
            notes: "Goldstandard-Suche über 'VDH Welpenvermittlung'. Vorsicht vor nicht-FCI-'Papieren' ohne echte Kontrolle.".into(),
        },
        BreederInfo {
            name: "SKG — Schweizerische Kynologische Gesellschaft".into(),
            region: "Schweiz".into(),
            association: "FCI / SKG".into(),
            breeds: "Alle FCI-Rassen in der Schweiz".into(),
            website: "https://www.skg.ch".into(),
            health_checks: "Strenge Schweizer Tierschutzverordnung, Zuchtstättenkontrolle, Wesenstest".into(),
            notes: "Gute Züchter geben Welpen nicht vor der 10.-12. Woche bei Kleinhunden ab.".into(),
        },
        BreederInfo {
            name: "ÖRC — Österreichischer Retriever Club".into(),
            region: "Österreich".into(),
            association: "ÖKV / FCI".into(),
            breeds: "Golden Retriever, Labrador, Flat Coated, Toller, Curly, Chesapeake".into(),
            website: "https://www.retrieverclub.at".into(),
            health_checks: "HD/ED-Auswertung, Augenuntersuchung, Wesenstest, EIC/PRA-Gentests".into(),
            notes: "Wartezeiten von 6-12 Monaten sind bei guten Züchtern normal und ein Qualitätsmerkmal.".into(),
        },
        BreederInfo {
            name: "ÖCP — Österreichischer Club der Pudelfreunde".into(),
            region: "Österreich".into(),
            association: "ÖKV / FCI".into(),
            breeds: "Toy-, Zwerg-, Klein- und Großpudel".into(),
            website: "https://www.pudelclub.at".into(),
            health_checks: "PRA-DNA, Patellaluxation, HD (Großpudel), Katarakt".into(),
            notes: "Ideal für Allergiker-Haushalte und Familien.".into(),
        },
    ]
}

pub fn breeder_red_flags() -> Vec<&'static str> {
    vec![
        "Übergabe auf Parkplatz, aus dem Kofferraum oder an der Autobahnraststätte -> SOFORT ABBRECHEN (illegaler Welpenhandel!)",
        "Muttertier wird nicht gezeigt oder wirkt apathisch / verängstigt",
        "Welpe ist jünger als 8 Wochen (in AT/DE gesetzlich verboten!)",
        "Viele verschiedene Moderassen gleichzeitig im Angebot ('Massenzucht')",
        "Kein EU-Heimtierausweis, kein Chip, keine altersgerechte Erstimpfung",
        "Verdächtig niedriger Preis ('Schnäppchen-Welpe' für 400 EUR ohne Papiere)",
        "Züchter stellt dir KEINE Fragen zu Wohnort, Zeit und Erfahrung",
    ]
}

pub fn rescue_orgs() -> Vec<RescueOrg> {
    vec![
        RescueOrg {
            name: "Tierschutz Austria — Tierschutzhaus Vösendorf".into(),
            region: "Wien / Niederösterreich".into(),
            focus: "Größtes Tierheim Österreichs; Hunde aller Größen, Seniorenhunde, Listenhunde-Beratung".into(),
            website: "https://www.tierschutz-austria.at".into(),
            notes: "Mehrere Kennenlern-Spaziergänge vor Adoption, sehr seriöse Einschätzung (Kinder/Katzen).".into(),
        },
        RescueOrg {
            name: "TierQuarTier Wien".into(),
            region: "Wien (22. Bezirk)".into(),
            focus: "Modernes städtisches Tierheim für Fund- und Abgabetiere in Wien".into(),
            website: "https://www.tierquartier.at".into(),
            notes: "Gute Beratung für Stadthunde-Haltung und Wiener Hundeführschein.".into(),
        },
        RescueOrg {
            name: "Arche Noah Graz".into(),
            region: "Steiermark".into(),
            focus: "Hunde, Katzen, Kleintiere; aktive Vermittlung und Trainerbegleitung".into(),
            website: "https://www.archenoah.at".into(),
            notes: "Bietet auch Patenschaften und Probespaziergänge.".into(),
        },
        RescueOrg {
            name: "Tierheim Berlin (Tierschutzverein für Berlin)".into(),
            region: "Berlin / Brandenburg".into(),
            focus: "Eines der größten Tierheime Europas; detaillierte Charakterprofile".into(),
            website: "https://tierschutz-berlin.de".into(),
            notes: "Perfekt für Stadt-Check und ruhige ältere Begleithunde.".into(),
        },
        RescueOrg {
            name: "Rasse-in-Not Vereine (z.B. Retriever in Not, Greyhound/Galgo-Hilfe)".into(),
            region: "DACH-weit (Pflegestellen)".into(),
            focus: "Abgabehunde aus Familien oder Ex-Renn/Jagdhunde auf privaten Pflegestellen".into(),
            website: "https://www.retriever-in-not.de".into(),
            notes: "Riesenvorteil: Pflegestelle kennt Alltag, Stubenreinheit, Alleinbleiben und Katzenverträglichkeit!".into(),
        },
    ]
}

pub fn rescue_foreign_checklist() -> Vec<&'static str> {
    vec![
        "Seriöser Verein mit §11-Erlaubnis (DE) bzw. TRACES-Transportmeldung (EU) — keine Kofferraum-Übergaben",
        "Mittelmeerkrankheiten-Test (ab 6-12 Monate sinnvoll): Leishmaniose, Ehrlichiose, Babesiose, Anaplasmose, Herzwurm (Dirofilaria)",
        "Tollwutimpfung mindestens 21 Tage vor Einreise + gültiger EU-Heimtierausweis + Mikrochip",
        "Direktadoption vs. Pflegestelle: Für Familien mit Kindern oder Katzen IMMER Hund von einer DACH-Pflegestelle wählen!",
        "Sicherheitsgeschirr (Panikgeschirr mit 2. Bauchgurt) + doppelte Leine in den ersten 4-8 Wochen Pflicht",
    ]
}

pub fn trainers() -> Vec<TrainerInfo> {
    vec![
        TrainerInfo {
            name: "ÖGV — Österreichischer Gebrauchshundesport-Verband (Ortsgruppen)".into(),
            city: "Wien, NÖ, OÖ, Steiermark, Tirol (100+ Plätze)".into(),
            association: "ÖKV / ÖGV".into(),
            specialties: "Welpenkurs, Junghunde, Begleithundeprüfung (BH-VT), Sachkundenachweis, Agility".into(),
            methodology: "Moderne Ortsgruppen arbeiten über positive Verstärkung (vorher Probestunde ansehen!).".into(),
            notes: "Günstige Vereinsbeiträge, eingezäunte Abrichteplätze.".into(),
        },
        TrainerInfo {
            name: "Tierschutzqualifizierte Hundetrainer*innen (Messerli-Institut / Vetmeduni Wien)".into(),
            city: "Ganz Österreich (inkl. Wien, Graz, Linz, Salzburg, Alm/Land)".into(),
            association: "Staatlich geprüftes Gütesiegel (BMSGPK / Vetmeduni)".into(),
            specialties: "Alltagstraining, Angst-/Problemhunde, Kind-Hund, Katze-Hund, Einzeltraining".into(),
            methodology: "Streng gewaltfrei, wissenschaftlich fundiert, ohne Einschüchterung.".into(),
            notes: "Offizielle Liste auf tierschutzqualifiziert.at — beste Empfehlung in Österreich.".into(),
        },
        TrainerInfo {
            name: "ÖRV — Österreichischer Rassehundeverein (Abrichteplätze)".into(),
            city: "Wien (z.B. Prater/Floridsdorf), NÖ, Bundesländer".into(),
            association: "ÖKV / ÖRV".into(),
            specialties: "Welpenspielgruppe, Alltagstraining, Breitensport, Rally Obedience".into(),
            methodology: "Vereinssport und Alltagskurse auf eigenen Abrichteplätzen.".into(),
            notes: "Vor Anmeldung immer eine Trainingsstunde ohne Hund zuschauen.".into(),
        },
        TrainerInfo {
            name: "IBH / Trainieren statt Dominieren / CUM CANIS Netzwerk".into(),
            city: "Deutschland, Österreich, Schweiz".into(),
            association: "Gewaltfreie Trainerverbände".into(),
            specialties: "Welpenerziehung, Leinenaggression, Rückruf, Jagdkontrolle, Ruhe-Training".into(),
            methodology: "100% belohnungsbasiert, keine Leinenrucke, keine Schreckreize.".into(),
            notes: "Ideal für sensible Rassen, Tierschutzhunde und Familien mit Kindern.".into(),
        },
    ]
}

pub fn training_guides() -> Vec<TrainingGuide> {
    vec![
        TrainingGuide {
            phase: "Woche 8-12 (Ankommen & Bindung)".into(),
            title: "Stubenreinheit & 18-20h Schlaf".into(),
            summary: "Vertrauen aufbauen, feste Ruhezeiten, alle 2h und nach Schlaf/Spiel/Fressen kurz raus.".into(),
            steps: vec![
                "Nach jedem Aufwachen, Fressen und Spielen sofort ruhig zur selben Pipi-Stelle tragen.".into(),
                "Draußen ruhig loben + Mini-Leckerli; drinnen kommentarlos wegwischen (kein Schimpfen!).".into(),
                "Welpen brauchen 18-20 Stunden Schlaf pro Tag — Überdrehen ('Zoomies') heißt oft: zu wenig Ruhe!".into(),
                "Name positiv verknüpfen: Name sagen -> Blickkontakt -> Keks.".into(),
            ],
            common_mistakes: "Nase ins Malheur drücken (zerstört Vertrauen!), 1-Stunden-Spaziergänge mit einem 10-Wochen-Welpen (Regel: ca. 5 Min pro Lebensmonat am Stück).".into(),
        },
        TrainingGuide {
            phase: "Woche 10-16 (Sozialisationsfenster)".into(),
            title: "Welt entdecken & Beißhemmung".into(),
            summary: "Qualität vor Quantität: Umweltreize positiv kennenlernen, sanftes Maulspiel lernen.".into(),
            steps: vec![
                "Kurze Ausflüge: U-Bahn-Eingang beobachten, Kinderwagen, Regenschirm, verschiedene Untergründe.".into(),
                "Bei Welpenzähnchen: 'Autsch', Spiel 10 Sekunden unterbrechen, weiches Kauspielzeug anbieten.".into(),
                "Ruhe üben: Café-Decke mit Kauartikel — Nicht jeder Passant muss den Welpen streicheln!".into(),
            ],
            common_mistakes: "Welpen auf der Hundewiese von 10 wilden Junghunden überrennen lassen (macht ängstlich oder rüpelig).".into(),
        },
        TrainingGuide {
            phase: "Monat 4-6 (Grundsignale)".into(),
            title: "Alleinbleiben, Rückruf & Leinenführigkeit".into(),
            summary: "Kleinschrittig aufbauen, bevor die Pubertät startet.".into(),
            steps: vec![
                "Alleinbleiben in Sekunden/Minuten aufbauen: Tür kurz schließen, reinkommen bevor der Hund jammert.".into(),
                "Rückruf-Jackpot: Besonderes Superleckerli (Leberwurst/Käse) NUR für den Rückruf.".into(),
                "Lockere Leine: Stehenbleiben wenn Zug drauf ist, jeden Blickkontakt und lockeres Laufen belohnen.".into(),
            ],
            common_mistakes: "Sofort 3 Stunden allein lassen oder Rückruf rufen, wenn man weiß, dass der Hund nicht kommt (verbrennt das Signal).".into(),
        },
        TrainingGuide {
            phase: "Spezial: Familie mit Kindern & Katze".into(),
            title: "Kind-Hund- & Katze-Hund-Management".into(),
            summary: "Räumliche Trennung am Anfang schützt alle Beteiligten.".into(),
            steps: vec![
                "Tabuzone Hundekörbchen: Kinder dürfen den Hund dort NIEMALS stören oder umarmen.".into(),
                "Katzen-Fluchtwege nach oben (Kratzbaum) + Kindergitter als hundefreie Katzenzone.".into(),
                "Geruchstausch in den ersten Tagen; ruhiges Betrachten der Katze beim Hund hochwertig belohnen.".into(),
                "Jagd-/Hüteversuche sofort freundlich umlenken — niemals unkontrolliert hinterherrennen lassen.".into(),
            ],
            common_mistakes: "Glauben 'die machen sich das schon aus' — das endet für Katze oder Kind gefährlich.".into(),
        },
    ]
}

pub fn legal_rules() -> Vec<LegalRule> {
    vec![
        LegalRule {
            region: "Wien (Österreich)".into(),
            topic: "Hundeführschein, Sachkunde & Öffis".into(),
            summary: "Allgemeiner Sachkundenachweis für Neuhundehalter; Pflicht-Hundeführschein für Listenhunde; Maulkorb+Leine in Öffis.".into(),
            details: "1) Seit 2019: 4-stündiger Sachkundenachweis vor Anschaffung für alle, die in den letzten 2 Jahren keinen Hund hatten.\n2) Listenhunde (Bullterrier, Staff, Pitbull, Rottweiler, Dogo Argentino, Tosa Inu, Mastiff etc.): Verpflichtender Hundeführschein mit Praxisprüfung, 0,5 Promille-Grenze, Maulkorb+Leine im öffentlichen Raum.\n3) Wiener Linien (U-Bahn, Bim, Bus): Leine UND Maulkorb Pflicht (ausgenommen geschlossene Transportbox); Hund braucht Halbpreis-Ticket (außer mit Jahreskarte in bestimmten Tarifen prüfen).\n4) Chip + Registrierung in der Heimtierdatenbank binnen 1 Monat Pflicht.\n5) Hundesteuer Wien: ca. 72 EUR/Jahr (1. Hund). Haftpflicht mind. 725.000 EUR Deckung.".into(),
        },
        LegalRule {
            region: "Niederösterreich / Bundesländer (AT)".into(),
            topic: "NÖ Hundehaltegesetz (2023-2025)".into(),
            summary: "Allgemeine Sachkunde für ALLE neuen Hundehalter in NÖ + Haftpflicht + Meldepflicht bei Gemeinde.".into(),
            details: "1) NÖ Sachkundenachweis (Theoriekurs 3h bei Tierarzt + Hundetrainer) innerhalb von 6 Monaten.\n2) Max. 5 Hunde pro Haushalt ohne Sonderbewilligung.\n3) Verpflichtende Haftpflichtversicherung (mind. 725.000 EUR).\n4) In Ortschaften Leinen- ODER Maulkorbpflicht; in Öffis/Veranstaltungen beides.".into(),
        },
        LegalRule {
            region: "Deutschland (Bund & Länder)".into(),
            topic: "Hundesteuer, §11, Sachkunde & Listenhunde".into(),
            summary: "Stark föderal geregelt: Hundesteuer je Kommune, Niedersachsen Hundeführerschein, NRW 20/40-Regel.".into(),
            details: "1) Tierschutz-Hundeverordnung: Mind. 2x täglich Auslauf (insg. 1h), Verbot von Stachelhalsbändern, Erziehungsgeräten mit Strom/Reiz, Ausstellungsverbot für Qualzuchten & kupierte Hunde.\n2) Niedersachsen: Verpflichtender Sachkundenachweis (Theorie vor Anschaffung + Praxis im 1. Jahr) für Ersthundehalter.\n3) NRW: Sachkundenachweis + Versicherung + Chip für Hunde > 20 kg oder > 40 cm Widerrist.\n4) Berlin/Hamburg: Leinenpflicht mit Ausnahmen/Hundeführerschein.".into(),
        },
        LegalRule {
            region: "Schweiz".into(),
            topic: "AMICUS, Haftpflicht & kantonale Kurse".into(),
            summary: "Obligatorische Registrierung in AMICUS; kantonal unterschiedliche Pflichtkurse (z.B. Zürich).".into(),
            details: "1) Mikrochip + Registrierung in AMICUS durch Tierarzt innerhalb 10 Tagen nach Übernahme.\n2) Kantone wie Zürich/Genf/Tessin verlangen praktische Ausbildungskurse bzw. Rassebewilligungen.\n3) Privathaftpflicht mit Hundedeckung dringend empfohlen/oft Pflicht.".into(),
        },
    ]
}

pub fn travel_destinations() -> Vec<TravelDestination> {
    vec![
        TravelDestination {
            id: "wien".into(),
            city_or_country: "Wien (Österreich)".into(),
            dog_friendliness_score: 94,
            rating_label: "Extrem hundefreundlich (Paradies)".into(),
            transit_rules: "Wiener Linien (U-Bahn, Tram, Bus): Hunde erlaubt mit Leine UND Maulkorb (oder geschlossener Tasche). ÖBB-Züge: Maulkorb+Leine, Halbpreisticket.".into(),
            restaurants_shops: "Fast alle Cafés, Beisln, Heurige und Gastgärten heißen Hunde herzlich willkommen (oft gibt es sofort ein Wasserschüsserl). Lebensmittelgeschäfte (Supermärkte) sind aus Hygienegründen tabu; Einkaufszentren oft erlaubt.".into(),
            entry_requirements: "EU-Heimtierausweis, Mikrochip, gültige Tollwutimpfung (mind. 21 Tage alt bei Erstimpfung).".into(),
            local_tips: "Über 200 Hundezonen (z.B. Prater-Hundezone, Donauinsel, Wienerberg). Achtung: Schlosspark Schönbrunn & Tiergarten sind für Hunde gesperrt (Ausnahme Assistenzhunde).".into(),
        },
        TravelDestination {
            id: "tokio".into(),
            city_or_country: "Tokio / Japan".into(),
            dog_friendliness_score: 18,
            rating_label: "Extrem restriktiv & aufwendig".into(),
            transit_rules: "U-Bahn, JR-Züge und Shinkansen: Hunde dürfen NICHT an der Leine mitfahren! Nur komplett unsichtbar in einer geschlossenen Box/Tasche (Gesamtgewicht meist max. 10 kg inkl. Box, Kantenlänge begrenzt) + Gepäckticket.".into(),
            restaurants_shops: "In Tokio kommt der Hund fast NIRGENDS rein: Normale Restaurants, Cafés, Supermärkte, Tempelanlagen und Museen verbieten Hunde strikt. Nur wenige spezielle 'Dog Cafés' oder Outdoor-Terrassen in Vierteln wie Daikanyama/Odaiba.".into(),
            entry_requirements: "EXTREM STRENG (min. 7 Monate Vorlauf!): 1) ISO-Mikrochip, 2) 2x Tollwutimpfung, 3) Blut-Titertest (FAVN) in anerkanntem Labor, 4) danach 180 TAGE Wartezeit vor Einreise! Sonst bis zu 180 Tage Quarantäne am Flughafen Narita/Haneda.".into(),
            local_tips: "Für Urlaub mit Hund ist Japan/Tokio meist ungeeignet — lieber gute Betreuung über den PuppyCraft Verleih-Kalender in der Heimat organisieren!".into(),
        },
        TravelDestination {
            id: "deutschland".into(),
            city_or_country: "Deutschland (z.B. München, Berlin, Nordsee)".into(),
            dog_friendliness_score: 82,
            rating_label: "Sehr hundefreundlich".into(),
            transit_rules: "DB Fernverkehr: Kleine Hunde in Box kostenlos, größere mit Maulkorb+Leine zum halben Fahrpreis. S-/U-Bahn regional teils ohne Maulkorb-Pflicht, Maulkorb aber immer mitführen.".into(),
            restaurants_shops: "In Restaurants/Biergärten meist willkommen; viele Hundestrände an Nord- und Ostsee.".into(),
            entry_requirements: "EU-Heimtierausweis, Chip, gültige Tollwutimpfung. Achtung: Einfuhrverbot für bestimmte Listenhunde (Pitbull, AmStaff, StaffBull, Bullterrier) bei längerem Aufenthalt über 4 Wochen!".into(),
            local_tips: "In Wäldern zur Brut- und Setzzeit (Frühjahr/Sommer) regionale Leinenpflicht beachten.".into(),
        },
        TravelDestination {
            id: "italien".into(),
            city_or_country: "Italien (inkl. Südtirol & Strände)".into(),
            dog_friendliness_score: 78,
            rating_label: "Hundefreundlich (Maulkorb mitführen)".into(),
            transit_rules: "Öffis & Fähren verlangen Leine und mitgeführten (teils angelegten) Maulkorb.".into(),
            restaurants_shops: "Hunde in Trattorien und auf Terrassen meist gern gesehen; viele ausgewiesene 'Bau Beach' Hundestrände.".into(),
            entry_requirements: "EU-Heimtierausweis, Chip, Tollwutimpfung.".into(),
            local_tips: "Im Sommer Sandmücken-Prophylaxe (Leishmaniose-Schutz, z.B. Spot-on/Halsband) mit dem Tierarzt besprechen!".into(),
        },
        TravelDestination {
            id: "kroatien".into(),
            city_or_country: "Kroatien (Adria / Istrien / Dalmatien)".into(),
            dog_friendliness_score: 72,
            rating_label: "Gut (mit Hitze- und Sandmückenschutz)".into(),
            transit_rules: "Fähren (Jadrolinija): Hunde auf Außendecks mit Leine & Maulkorb; Busse teils restriktiv.".into(),
            restaurants_shops: "Auf Restaurant-Terrassen fast überall erlaubt.".into(),
            entry_requirements: "EU-Heimtierausweis, Chip, gültige Tollwutimpfung.".into(),
            local_tips: "Viele schöne Hundestrände in Istrien (Rovinj, Porec, Krk). Sandmücken- & Herzwurm-Prophylaxe wichtig.".into(),
        },
        TravelDestination {
            id: "norwegen-uk".into(),
            city_or_country: "Norwegen / Großbritannien".into(),
            dog_friendliness_score: 58,
            rating_label: "Hundefreundlich vor Ort, aber strenge Einreise".into(),
            transit_rules: "Norwegen Züge/Fähren hundefreundlich (spezielle Tierabteile). UK Öffis in London erlauben Hunde an kurzer Leine (auf Rolltreppen tragen!).".into(),
            restaurants_shops: "UK-Pubs sind legendär hundefreundlich; Norwegen in Restaurants drinnen strenger.".into(),
            entry_requirements: "PFLICHT: Tierärztlich dokumentierte Bandwurm-Behandlung (Echinococcus / Praziquantel) 24-120 Stunden VOR Einreise im EU-Heimtierausweis!".into(),
            local_tips: "In Norwegen gilt vom 1. April bis 20. August landesweite strikte Leinenpflicht zum Schutz der Wildtiere.".into(),
        },
    ]
}

pub fn travel_packing_list() -> Vec<&'static str> {
    vec![
        "EU-Heimtierausweis (inkl. Chipnummer, aktueller Tollwutimpfung & ggf. Titertest)",
        "Kopie der Haftpflichtversicherung (oft in Italien/AT verlangt)",
        "Gut sitzender Maulkorb (Pflicht in ÖBB, Wiener Linien, italienischen Öffis)",
        "Faltbarer Wassernapf + ausreichend Trinkwasser für unterwegs",
        "Gewohntes Futter für alle Reisetage (Futterumstellung stresst den Magen)",
        "Kotbeutel + Reiseapotheke (Zeckenzange, Desinfektion, Kohletabletten, Pfotensalbe)",
        "Geprüfte Autobox / Crash-getestetes Sicherheitsgeschirr für die Fahrt",
        "Hundedecke (vertrauter Geruch gibt Sicherheit im Hotel)",
    ]
}

pub fn toxic_substances() -> Vec<ToxicSubstance> {
    vec![
        ToxicSubstance {
            name: "Schokolade / Kakao (Theobromin)".into(),
            danger_level: "HOCH (lebensbedrohlich bei dunkler Schokolade)".into(),
            toxic_dose: "Ab ca. 20 mg Theobromin/kg Körpergewicht Symptome; ab 100 mg/kg tödlich. Zartbitter/Backkakao ist 10x giftiger als Vollmilch!".into(),
            symptoms: "Erbrechen, Durchfall, Unruhe, Hecheln, Herzrasen, Zittern, Krampfanfälle (oft nach 2-6 Stunden).".into(),
            action: "SOFORT Tierarzt anrufen (Gewicht des Hundes + Schokoladensorte + Gramm bereithalten)!".into(),
        },
        ToxicSubstance {
            name: "Weintrauben & Rosinen".into(),
            danger_level: "SEHR HOCH (akutes Nierenversagen!)".into(),
            toxic_dose: "Bereits kleine Mengen (ca. 10-30 g Trauben/kg oder noch weniger Rosinen) können tödliches Nierenversagen auslösen.".into(),
            symptoms: "Erbrechen, Lethargie, Bauchschmerzen, verminderter/ausbleibender Urinabsatz.".into(),
            action: "SOFORT zum Tierarzt / Notdienst (Erbrechen auslösen innerhalb 2h + Infusion)!".into(),
        },
        ToxicSubstance {
            name: "Xylit / Birkenzucker (E967)".into(),
            danger_level: "EXTREM HOCH (akute Lebensgefahr!)".into(),
            toxic_dose: "Schon 0,1 g/kg verursacht schweren Unterzucker; 0,5 g/kg akutes Leberversagen! Oft in zuckerfreien Kaugummis, Bonbons, Backwaren, Erdnussbutter.".into(),
            symptoms: "Innerhalb 15-60 Min: Schwäche, Taumeln, Zittern, Krämpfe, Koma.".into(),
            action: "NOTFALL! Sofort in die Tierklinik fahren, auf dem Weg anrufen!".into(),
        },
        ToxicSubstance {
            name: "Zwiebeln, Knoblauch, Lauch, Schnittlauch".into(),
            danger_level: "HOCH (Zerstörung roter Blutkörperchen)".into(),
            toxic_dose: "Ab ca. 5 g Zwiebel/kg (roh, gekocht oder als Pulver in Essensresten!).".into(),
            symptoms: "Blasse Schleimhäute, Schwäche, dunkler Urin, beschleunigte Atmung (oft 1-3 Tage verzögert).".into(),
            action: "Tierarzt kontaktieren, Menge und Zeitpunkt angeben.".into(),
        },
        ToxicSubstance {
            name: "Macadamianüsse".into(),
            danger_level: "MITTEL-HOCH".into(),
            toxic_dose: "Ab ca. 2 g/kg Körpergewicht.".into(),
            symptoms: "Schwäche der Hinterbeine, Zittern, Fieber, Erbrechen.".into(),
            action: "Tierarzt kontaktieren.".into(),
        },
        ToxicSubstance {
            name: "Ibuprofen / Paracetamol / Diclofenac (Menschen-Schmerzmittel)".into(),
            danger_level: "EXTREM HOCH".into(),
            toxic_dose: "Schon eine einzige Tablette kann bei kleinen Hunden Magenbluten und Nieren-/Leberversagen auslösen.".into(),
            symptoms: "Blutiges Erbrechen, schwarzer Kot, Bauchschmerz, Apathie.".into(),
            action: "SOFORT Tierklinik + Packung mitnehmen!".into(),
        },
        ToxicSubstance {
            name: "Frostschutzmittel (Ethylenglykol)".into(),
            danger_level: "EXTREM HOCH (schmeckt leider süß!)".into(),
            toxic_dose: "Wenige Milliliter reichen für tödliches Nierenversagen.".into(),
            symptoms: "'Betrunkener' Gang, Erbrechen, starker Durst, dann Nierenversagen.".into(),
            action: "SOFORTIGER NOTFALL — jede Minute zählt für das Gegenmittel!".into(),
        },
        ToxicSubstance {
            name: "Gekochte Geflügel-/Schweineknochen".into(),
            danger_level: "HOCH (Splittern + Aujeszky bei rohem Schwein!)".into(),
            toxic_dose: "Gekochte Knochen splittern messerscharf; rohes Schweinefleisch kann das tödliche Aujeszky-Virus übertragen.".into(),
            symptoms: "Würgen, Bauchschmerz, Verstopfung, innere Verletzungen.".into(),
            action: "Bei Verschlucken von Splittern sofort Tierarzt fragen (Sauerkraut-Trick nur nach Rücksprache).".into(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChocolateCheckResult {
    pub total_theobromine_mg: f32,
    pub mg_per_kg: f32,
    pub severity: String,
    pub advice: String,
}

pub fn calculate_chocolate_toxicity(
    dog_weight_kg: f32,
    chocolate_grams: f32,
    cocoa_type: &str,
) -> ChocolateCheckResult {
    let mg_per_g = match cocoa_type {
        "white" => 0.05,
        "milk" => 2.0,
        "dark_70" => 14.0,
        "baking" => 16.0,
        _ => 5.5, // standard dark ~50%
    };
    let w = dog_weight_kg.max(0.5);
    let total = chocolate_grams.max(0.0) * mg_per_g;
    let per_kg = total / w;

    let (severity, advice) = if per_kg < 15.0 {
        (
            "Gering (beobachten)".into(),
            "Meist nur leichte Magen-Darm-Reizung. Hund gut beobachten; bei Unsicherheit Tierarzt anrufen.".into(),
        )
    } else if per_kg < 40.0 {
        (
            "Mittel — Tierarzt anrufen!".into(),
            "Magen-Darm- & Herz-Kreislauf-Symptome wahrscheinlich. Sofort Tierarzt kontaktieren (ggf. Erbrechen auslösen).".into(),
        )
    } else if per_kg < 60.0 {
        (
            "HOCH — Sofort in die Tierklinik!".into(),
            "Schwere Vergiftung (Herzrhythmusstörungen, Zittern). Unverzüglich Notdienst aufsuchen!".into(),
        )
    } else {
        (
            "AKUTE LEBENSGEFAHR — SOFORT NOTDIENST!".into(),
            "Potenziell tödliche Dosis! Keine Zeit verlieren, sofort Tierklinik anrufen und losfahren!".into(),
        )
    };

    ChocolateCheckResult {
        total_theobromine_mg: total,
        mg_per_kg: per_kg,
        severity,
        advice,
    }
}

pub fn calculate_daily_food(weight_kg: f32, stage: &str, activity: &str) -> (u16, u16, String) {
    let w = weight_kg.max(1.0) as f64;
    let rer = 70.0 * w.powf(0.75);
    let stage_factor = match stage {
        "puppy_young" => 2.5,
        "puppy_older" => 2.0,
        "senior" => 1.3,
        _ => 1.6, // adult
    };
    let act_factor = match activity {
        "low" => 0.85,
        "high" => 1.25,
        _ => 1.0,
    };
    let kcal = (rer * stage_factor * act_factor).round() as u16;
    // Typical dry food ~360 kcal / 100g
    let dry_g = ((f64::from(kcal) / 3.6).round()) as u16;
    let barf_g = match stage {
        "puppy_young" | "puppy_older" => (w * 50.0).round() as u16,
        _ => (w * 25.0).round() as u16,
    };
    let note = format!(
        "Richtwert ca. {kcal} kcal/Tag (~{dry_g} g Trockenfutter ODER ~{barf_g} g Frischfutter/BARF). Leckerlis abziehen!"
    );
    (kcal, dry_g, note)
}

pub fn calculate_tco(size: DogSize, with_insurance: bool) -> (u16, u16, u16) {
    let setup_eur: u16 = match size {
        DogSize::Toy | DogSize::Small => 450,
        DogSize::Medium => 600,
        DogSize::Large | DogSize::Giant => 800,
    };
    let base_monthly: u16 = match size {
        DogSize::Toy | DogSize::Small => 95,
        DogSize::Medium => 130,
        DogSize::Large => 170,
        DogSize::Giant => 220,
    };
    let monthly = base_monthly + if with_insurance { 45 } else { 0 };
    let emergency_buffer = 2500;
    (setup_eur, monthly, emergency_buffer)
}

pub fn starter_kit_items() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Y-Brustgeschirr (gut gepolstert)",
            "Schont Halswirbelsäule & Luftröhre im Vergleich zum Halsband",
        ),
        (
            "2m Führleine + 5-10m Schleppleine",
            "Alltag + sicheres Rückruftraining ohne Flexileinen-Unfälle",
        ),
        (
            "Adressmarke + Chip-Registrierung",
            "TASSO (DE) / Animaldata / Heimtierdatenbank (AT) / AMICUS (CH)",
        ),
        (
            "Ruhiger Schlafplatz / Körbchen",
            "An zugfreiem, ruhigem Ort — Tabuzone für Kinder!",
        ),
        (
            "2x Edelstahlnapf + rutschfeste Matte",
            "Hygienisch und spülmaschinenfest",
        ),
        (
            "Kotbeutel + Tasche",
            "Immer mindestens 3 Stück in jeder Jacke",
        ),
        (
            "Kong / Schleckmatte / Kauwurzel",
            "Stressabbau durch Kauen und Schlecken",
        ),
        (
            "Maulkorb (bequem, hechel-fähig!)",
            "Mit Leberwurst positiv trainieren — Pflicht für Öffis & Notfälle",
        ),
        (
            "Geprüfte Transportbox / Autogurt",
            "Gesetzliche Ladungssicherung im Auto",
        ),
        (
            "Erste-Hilfe-Set + Zeckenzange",
            "Desinfektion, Verbandsmaterial, Kohletabletten (nach Tierarztrücksprache)",
        ),
    ]
}

pub fn body_language_signals() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Züngeln / Lippen lecken (ohne Futter)",
            "Beschwichtigungssignal ('Calming Signal'): 'Ich bin gestresst / bitte Abstand'",
        ),
        (
            "Gähnen (wenn nicht müde)",
            "Stressabbau oder Unwohlsein in angespannter Situation",
        ),
        (
            "Kopf abwenden / Blick abwenden",
            "Höfliche Deeskalation: 'Ich will keinen Konflikt'",
        ),
        (
            "Einfrieren ('Freeze') + starrer Blick",
            "WARNSTUFE! Direkt vor möglichem Abschnappen — Situation sofort entschärfen!",
        ),
        (
            "Walauge (Weißes im Auge sichtbar)",
            "Starke Anspannung / Überforderung (oft bei Kinder-Umarmungen)",
        ),
        (
            "Rute hoch & steif + schnelles kleines Wedeln",
            "Aufregung/Anspannung — NICHT automatisch 'freundlich'!",
        ),
        (
            "Vorderkörpertiefstellung ('Play Bow')",
            "Eindeutige Spielaufforderung: 'Alles was folgt ist Spaß!'",
        ),
        (
            "Sich schütteln nach Begegnung",
            "Stress abschütteln — gutes Zeichen der Entspannung danach",
        ),
    ]
}

pub fn first_aid_guides() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Magendrehung (NOTFALL!)", "Symptome: Aufgeblähter harter Bauch, erfolgloses Würgen, Unruhe, Speicheln. SOFORT Tierklinik anrufen & losfahren — jede Viertelstunde zählt!"),
        ("Hitzschlag (Sommer / Auto)", "Nie im parkenden Auto lassen! Bei Taumeln/dunkelroten Schleimhäuten: Hund in Schatten, mit kühlem (nicht eiskaltem!) Wasser an Pfoten & Bauch kühlen, sofort Tierarzt."),
        ("Zecke entfernen", "Zeckenhaken/Pinzette direkt an der Haut ansetzen, langsam gerade herausziehen (kein Öl/Kleber!). Stelle desinfizieren."),
        ("Pfotenverletzung / Schnitt", "Mit sauberem Wasser spülen, desinfizieren, zwischen den Zehen abpolstern, lockeren Verband + Socke, Tierarzt bei tiefem Schnitt."),
        ("Vergiftung", "Nicht selbst Erbrechen auslösen ohne Tierarzt-Anweisung! Giftrest/Verpackung einpacken und sofort in die Klinik."),
    ]
}

pub fn dog_names() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Beppo",
            "Wienerisch / Charmant",
            "Klassischer Wiener Bub-Name, perfekt für Dackel & Mischlinge",
        ),
        (
            "Sissi",
            "Wienerisch / Imperial",
            "Für elegante Hundedamen mit eigenem Kopf",
        ),
        (
            "Kipferl",
            "Wienerisch / Süß",
            "Für kleine goldene oder lockige Wirbelwinde",
        ),
        (
            "Strudel",
            "Wienerisch / Witzig",
            "Für energiegeladene Welpen",
        ),
        (
            "Milo",
            "Klassisch (2-Silber)",
            "Sehr gut rufbar, freundlich und zeitlos",
        ),
        (
            "Luna",
            "Klassisch (2-Silber)",
            "Beliebt, weicher Klang, perfekt für den Rückruf",
        ),
        (
            "Bruno",
            "Klassisch / Bodenständig",
            "Für gemütliche Rüden oder Sennenhunde",
        ),
        (
            "Freya",
            "Nordisch",
            "Für Samojeden, Shelties oder selbstbewusste Hündinnen",
        ),
        ("Odin", "Nordisch", "Für große Hirten- und Berghunde"),
        (
            "Taro",
            "Japanisch",
            "Passend für Shiba Inu oder ruhige Begleiter",
        ),
        ("Almo", "Alpin / Almöhi", "Für echte Berg- und Hofhunde"),
        (
            "Rosi",
            "Gemütlich",
            "Perfekt für Senioren-Begleiter oder Dackel",
        ),
    ]
}

pub fn quiz_questions() -> Vec<(&'static str, [&'static str; 3], usize, &'static str)> {
    vec![
        (
            "Wie viele Stunden Schlaf braucht ein junger Welpe pro Tag?",
            ["8-10 Stunden", "12-14 Stunden", "18-20 Stunden"],
            2,
            "Welpen wachsen und verarbeiten Reize im Schlaf: 18-20 Stunden sind normal. Zu wenig Schlaf macht überdrehte 'Schnapp-Krokodile'.",
        ),
        (
            "Was ist bei der Einreise mit Hund in die Wiener Linien (U-Bahn/Tram) Pflicht?",
            ["Nur Leine", "Leine UND Maulkorb (oder geschlossene Box)", "Gar nichts bei kleinen Hunden"],
            1,
            "In Wien gilt in allen öffentlichen Verkehrsmitteln ausnahmslos Leinen- UND Maulkorbpflicht (oder geschlossene Transportbox).",
        ),
        (
            "Welche Zutat in zuckerfreien Kaugummis ist für Hunde lebensgefährlich?",
            ["Xylit (Birkenzucker)", "Zitronensäure", "Minzöl in Spuren"],
            0,
            "Xylit (E967) verursacht bereits in kleinsten Mengen einen massiven Insulin-Ausstoß (Unterzucker) und akutes Leberversagen.",
        ),
        (
            "Woran erkennst du einen illegalen Welpenhändler sofort?",
            ["Er fragt nach deiner Wohnsituation", "Übergabe auf Parkplatz / aus Kofferraum ohne Muttertier", "Welpe ist gechipt mit EU-Pass"],
            1,
            "Seriöse Züchter zeigen immer Muttertier und Aufzuchtort und fragen kritisch nach deinem Alltag.",
        ),
        (
            "Warum ist Tokio/Japan als Reiseziel mit eigenem Hund so schwierig?",
            ["Hunde dürfen dort nicht bellen", "180-Tage-Titer-Vorlauf + Hunde fast überall in Öffis/Restaurants verboten", "Es gibt dort kein Hundefutter"],
            1,
            "Japan verlangt 180 Tage Wartezeit nach Blut-Titertest und schließt Hunde aus fast allen Restaurants, Geschäften und normalen Zügen aus.",
        ),
    ]
}

pub fn vaccine_schedule_template() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "SHPPiL (8 Wochen)",
            "Grundimmunisierung 1: Staupe, Hepatitis, Parvovirose, Parainfluenza, Leptospirose",
            "Pflicht-Core",
        ),
        (
            "SHPPiL + Tollwut (12 Wochen)",
            "Grundimmunisierung 2 + Tollwut (ab 12 Wochen, für Reisen 21 Tage Wartezeit!)",
            "Pflicht-Core",
        ),
        (
            "SHPPiL (16 Wochen)",
            "Abschluss Grundimmunisierung gegen maternale Antikörper-Lücke",
            "Pflicht-Core",
        ),
        (
            "SHPPiL + Tollwut (15 Monate)",
            "1. Jahres-Booster schließt Grundimmunisierung ab",
            "Pflicht-Core",
        ),
        (
            "Leptospirose (jährlich)",
            "Schutz vor Wasserpfützen-/Nager-Bakterien (nur 12 Monate wirksam!)",
            "Empfohlen jährlich",
        ),
        (
            "SHP / Tollwut (alle 3 Jahre)",
            "Langzeit-Auffrischung laut Herstellerbeipackzettel",
            "Alle 3 Jahre",
        ),
        (
            "Zwingerhusten (Bordetella)",
            "Für Hundeschule, Hundepension oder Ausstellungen",
            "Optional / Pension",
        ),
        (
            "Entwurmung / Kotprobe",
            "Alle 3 Monate (oder gezielte Kotuntersuchung) je nach Jagd-/Fressverhalten",
            "Quartalsweise",
        ),
    ]
}

pub fn separation_training_plan() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Stufe 1: Jacke & Schlüssel (0 Min)",
            "Jacke anziehen, Schlüssel nehmen, sofort wieder hinsetzen — Reiz entkoppeln!",
        ),
        (
            "Stufe 2: Zimmer-Tür (10-60 Sek)",
            "Kurz ins Nebenzimmer, Tür zu, ruhig wieder rein BEVOR der Hund fiept.",
        ),
        (
            "Stufe 3: Haustür (2-5 Min)",
            "Wohnungstür schließen, kurz im Stiegenhaus warten, entspannt zurück.",
        ),
        (
            "Stufe 4: Kurzer Einkauf (15-30 Min)",
            "Nach guter Auslastung + Schleckmatte; mit Kamera prüfen ob der Hund schläft.",
        ),
        (
            "Stufe 5: Alltag (1-4 Std)",
            "Steigern nur wenn Stufe 4 völlig entspannt war. Welpen unter 6 Monaten max. 2-3 Std!",
        ),
    ]
}

pub fn daily_routine(stage: &str) -> Vec<(&'static str, &'static str)> {
    match stage {
        "puppy" => vec![
            ("07:00", "Sofort kurz Pipi-Wiese -> Frühstück -> 10 Min ruhiges Spiel -> Schlafen"),
            ("10:00", "Pipi-Wiese -> 10 Min Umwelttraining / Sozialisierung -> Schlafen"),
            ("13:00", "Pipi-Wiese -> Mittagessen -> Kauartikel -> 2-3 Std Mittagsruhe"),
            ("16:30", "Pipi-Wiese -> kurzes Rückrufspiel -> Abendessen -> Kuscheln"),
            ("22:30", "Letzte ruhige Pipi-Runde vor dem Schlafen (Welpen brauchen 18-20h Schlaf!)"),
        ],
        "senior" => vec![
            ("08:00", "Gemütliche Schnüffelrunde (20 Min, weicher Untergrund) -> Frühstück + Gelenkzusatz"),
            ("13:00", "Kurze Löserunde -> viel ungestörte Schlafenszeit"),
            ("17:30", "Entspannte Abendrunde (kein Ballhetzen!) -> leicht verdauliches Abendessen"),
            ("21:30", "Kurze Nachtrunde, Rampe/Teppiche gegen Rutschen bereithalten"),
        ],
        _ => vec![
            ("07:30", "30 Min Morgenrunde + Frühstück"),
            ("13:00", "20 Min Mittagspause / Schnüffelrunde + Kopfarbeit"),
            ("17:30", "45-60 Min Hauptspaziergang (Freilauf/Schleppleine/Training) + Abendessen"),
            ("22:00", "10 Min letzte Löserunde"),
        ],
    }
}

pub fn insurance_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Hundehalter-Haftpflicht (PFLICHT!)", "Mind. 3-5 Mio. EUR Deckung; wichtig: Mietsachschäden & Fremdbetreuung (Partner/Oma!) einschließen."),
        ("OP-Kosten-Versicherung", "Übernimmt teure Operationen (Kreuzbandriss, Magendrehung, Fremdkörper: oft 2.000-6.000 EUR)."),
        ("Voll-Krankenversicherung", "Deckt auch Diagnostik/Medikamente — auf Wartezeiten (1-3 Monate), Rasse-Ausschlüsse und Kündigungsrecht im Schadensfall achten!"),
    ]
}

pub fn senior_care_tips() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Gelenke & Rutschfestigkeit", "Läufer/Teppiche auf Glattböden auslegen; Autorampe statt Sprung aus dem Kofferraum."),
        ("Kognitive Dysfunktion (Hunde-Demenz)", "Feste Routinen beibehalten, nachts kleines Nachtlicht, sanfte Nasenarbeit hält das Gehirn fit."),
        ("Halbjährlicher Senior-Check", "Ab 7-8 Jahren (Riesenrassen ab 6): Blutbild, Herz abhören, Zähne und Nierenwerte prüfen lassen."),
    ]
}

pub fn medical_problems() -> Vec<MedicalProblem> {
    vec![
        MedicalProblem {
            name: "HD & ED (Hüft- und Ellbogendysplasie)".into(),
            affected_breeds: "Deutscher Schäferhund, Golden/Labrador Retriever, Berner Sennenhund, Leonberger, Boxer".into(),
            urgency: "Mittel (chronisch, früh abklären)".into(),
            symptoms: "Steifes Aufstehen nach Ruhe, 'Hasenhoppeln' beim Laufen, Unlust beim Treppensteigen oder Springen.".into(),
            prevention_and_action: "Nur Welpen von HD/ED-geröntgten Eltern (ÖKV/VDH) kaufen! Welpen im 1. Jahr keine Treppen/hohen Sprünge zumuten, schlank halten, Physiotherapie/Hydrotherapie nutzen.".into(),
        },
        MedicalProblem {
            name: "Schliafhansel (Mäusegerste / Grannen)".into(),
            affected_breeds: "Alle Rassen (besonders Schlappohren & langhaarige Pfoten im Sommer in Wien/NÖ)".into(),
            urgency: "HOCH — wandert täglich tiefer ins Gewebe!".into(),
            symptoms: "Plötzliches heftiges Kopfschütteln nach Wiesenlauf, Niesen mit Nasenbluten, geschwollene/beleckte Stelle zwischen den Zehen, zugekniffenes Auge.".into(),
            prevention_and_action: "Nach jedem Sommerspaziergang Pfotenzwischenräume & Ohren kontrollieren, Pfotenfell kurz halten. Bei Verdacht SOFORT zum Tierarzt (nicht mit Pinzette abreißen — Rest wandert weiter!).".into(),
        },
        MedicalProblem {
            name: "Allergien (Futterallergie & Umwelt-Atopie)".into(),
            affected_breeds: "Französische Bulldogge, Westie, Golden/Labrador, Boxer, Schäferhund, Shar Pei".into(),
            urgency: "Mittel (quälender Juckreiz)".into(),
            symptoms: "Ständiges Pfotenlecken (braun verfärbtes Fell), wiederkehrende Ohrenentzündungen, Kratzen an Bauch/Achseln, weicher Kot.".into(),
            prevention_and_action: "8-10 Wochen strikte Ausschlussdiät (z.B. Pferd/Insekt/Känguru, keine fremden Leckerlis!) oder Dermatologie-Abklärung (Cytopoint/Apoquel/Desensibilisierung).".into(),
        },
        MedicalProblem {
            name: "Ohrenentzündung (Otitis externa)".into(),
            affected_breeds: "Cocker Spaniel, Golden Retriever, Pudel, Beagle, Basset".into(),
            urgency: "Mittel-Hoch (sehr schmerzhaft)".into(),
            symptoms: "Kopfschiefhalten, Kratzen am Ohr, braun-schwarzer Belag, süßlicher/übler Geruch.".into(),
            prevention_and_action: "Ohren nach dem Schwimmen sanft trocknen. Keine Wattestäbchen verwenden! Tierarzt prüft Trommelfell vor Ohrentropfen.".into(),
        },
        MedicalProblem {
            name: "Magendrehung (Torsio ventriculi)".into(),
            affected_breeds: "Großpudel, Deutscher Schäferhund, Leonberger, Berner, Dogge, Boxer".into(),
            urgency: "AKUTER NOTFALL — Lebensgefahr binnen Stunden!".into(),
            symptoms: "Aufgeblähter, trommelharter Bauch, erfolgloses Würgen (nur weißer Schaum), starke Unruhe, Hecheln, Kreislaufkollaps.".into(),
            prevention_and_action: "Nach dem Fressen mind. 60-90 Min Ruhe! 2-3 kleinere Mahlzeiten statt 1 Riesenportion. Bei Symptomen SOFORT ins nächste 24/7-Tierspital fahren!".into(),
        },
        MedicalProblem {
            name: "Bandscheibenvorfall / Dackellähme (IVDD)".into(),
            affected_breeds: "Dackel, Französische Bulldogge, Beagle, Corgi".into(),
            urgency: "HOCH (bei Ausfällen innerhalb 24h zum Neuro-Spezialisten!)".into(),
            symptoms: "Aufjaulen beim Hochheben, gekrümmter Rücken, Überköten der Hinterpfoten, unsicherer Gang bis Lähmung.".into(),
            prevention_and_action: "Keine Treppen, keine Sprünge von Sofa/Auto (Rampen nutzen!), schlank halten. Bei Lähmungserscheinungen sofort in Überweisungspraxis/Tierspital (CT/MRT).".into(),
        },
        MedicalProblem {
            name: "Brachycephales Syndrom (BOAS / Atemnot)".into(),
            affected_breeds: "Mops, Französische Bulldogge, Englische Bulldogge".into(),
            urgency: "Mittel bis Hoch (Hitzschlag-Gefahr!)".into(),
            symptoms: "Lautes Schnorcheln schon in Ruhe, Würgen, blaue Zunge bei Hitze oder Aufregung.".into(),
            prevention_and_action: "Im Sommer nur früh morgens/spät abends raus, Brustgeschirr statt Halsband. Ggf. spezialisierte Gaumensegel-/Nasenloch-OP.".into(),
        },
        MedicalProblem {
            name: "MDR1-Gendefekt (Arzneimittel-Überempfindlichkeit)".into(),
            affected_breeds: "Border Collie, Australian Shepherd, Sheltie, Collie, Bobtail".into(),
            urgency: "Vorsorge-Pflicht vor Narkose & Wurmkur!".into(),
            symptoms: "Schwere neurologische Vergiftung durch bestimmte Antiparasitika (z.B. Ivermectin, Doramectin) oder Narkosemittel.".into(),
            prevention_and_action: "Einmaligen MDR1-Gentest machen und Ergebnis im EU-Heimtierausweis + Sitter-Steckbrief vermerken!".into(),
        },
    ]
}

pub fn insurance_providers() -> Vec<InsuranceProvider> {
    vec![
        InsuranceProvider {
            name: "Hundehalter-Haftpflicht (z.B. Allianz, Wiener Städtische, UNIQA, Generali, VHV)".into(),
            category: "Haftpflicht (PFLICHT in Wien/NÖ/vielen DE-Ländern)".into(),
            monthly_eur: "5 - 12 EUR/Monat (oder in Privathaftpflicht je nach Tarif)".into(),
            coverage: "Personen-, Sach- und Vermögensschäden (mind. 3-10 Mio. EUR wählen!), inkl. Mietsachschäden & Fremdbetreuung.".into(),
            waiting_period: "Keine (sofort ab Vertragsbeginn)".into(),
            notes: "Unbedingt prüfen, ob 'Fremdhüten' (Partner, Oma, Freunde beim Hundsi-Verleih) und unangeleintes Laufen in Hundezonen gedeckt sind!".into(),
        },
        InsuranceProvider {
            name: "Agria Tierversicherung (AT / DE)".into(),
            category: "Kranken- & OP-Versicherung (Spezialist seit 1890)".into(),
            monthly_eur: "25 - 65 EUR/Monat (je nach Rasse, Alter & Selbstbehalt)".into(),
            coverage: "Flexible Bausteine (OP, Heilbehandlung, Medikamente, Physio), lebenslange Deckung ohne Kündigung im Alter.".into(),
            waiting_period: "20 Tage (Unfälle sofort)".into(),
            notes: "Großer Vorteil: Agria verzichtet meist auf das einseitige Kündigungsrecht nach einem Schadensfall.".into(),
        },
        InsuranceProvider {
            name: "Lassie Tierversicherung (AT / DE)".into(),
            category: "App-basierte OP- & Vollkrankenversicherung".into(),
            monthly_eur: "20 - 58 EUR/Monat".into(),
            coverage: "Erstattung bis 4-facher GOT-Satz, Vorsorgebudget (Impfung/Zahnstein) im Top-Tarif, Rabatt für abgeschlossene App-Kurse.".into(),
            waiting_period: "30 Tage (Unfall sofort, spezielle Gelenk-OPs teils länger)".into(),
            notes: "Sehr transparente App-Einreichung per Foto der Tierarztrechnung.".into(),
        },
        InsuranceProvider {
            name: "Helvetia PetCare / GRAWE (Österreich-Fokus)".into(),
            category: "Kranken- & OP-Versicherung".into(),
            monthly_eur: "22 - 55 EUR/Monat".into(),
            coverage: "Stationäre & ambulante Heilbehandlung, OPs, teils inkl. Haftpflicht-Kombination.".into(),
            waiting_period: "3 Monate (Unfälle meist 24h)".into(),
            notes: "Gute lokale Ansprechpartner in Österreich; auf jährliche Höchstgrenze achten.".into(),
        },
        InsuranceProvider {
            name: "Barmenia / HanseMerkur / Uelzener (DE & teils AT)".into(),
            category: "OP-Schutz & Vollversicherung".into(),
            monthly_eur: "18 - 70 EUR/Monat".into(),
            coverage: "Barmenia/HanseMerkur bieten starke OP- und Volltarife bis 100% Erstattung; Uelzener ist Klassiker für Mehrhundehalter.".into(),
            waiting_period: "1 - 3 Monate".into(),
            notes: "Früh abschließen (am besten als Welpe), denn Vorerkrankungen werden bei jedem Anbieter ausgeschlossen!".into(),
        },
    ]
}

pub fn vet_clinics() -> Vec<VetClinicInfo> {
    vec![
        VetClinicInfo {
            name: "Lokale Haustierarzt-Praxis (Allgemeinmedizin)".into(),
            tier_type: "1. Ebene: Kleine Veterinärpraxis".into(),
            region: "In deinem Bezirk / Ort".into(),
            specialties: "Impfungen, Chip, Schliafhansel-Erstcheck, Durchfall, kleine Wunden, Blutbild, Senior-Vorsorge".into(),
            emergency_24h: false,
            when_to_go: "Für 90% aller Routine- und Alltagsprobleme. Persönliche Beziehung zum Haustierarzt spart Stress und Geld!".into(),
        },
        VetClinicInfo {
            name: "Überweisungspraxen / Facharztzentren (z.B. Orthopädie, Kardiologie, Dermatologie, Augen)".into(),
            tier_type: "2. Ebene: Überweisungspraxis".into(),
            region: "Ballungsräume (Wien, Graz, München, Berlin, Zürich)".into(),
            specialties: "CT/MRT, Kreuzband-OP (TPLO), Herzultraschall (MVD/Cavalier), Bandscheiben-OP, Allergie-Dermatologie, ECVO-Augen".into(),
            emergency_24h: false,
            when_to_go: "Wenn dein Haustierarzt gezielt an einen Spezialisten überweist oder komplexe Diagnostik (MRT, Spezial-OP) nötig ist.".into(),
        },
        VetClinicInfo {
            name: "Vetmeduni Wien — Tierspital (Universitätsklinik für Kleintiere)".into(),
            tier_type: "3. Ebene: Tierspital (24/7 Notdienst)".into(),
            region: "Wien (21. Bezirk, Veterinärplatz 1)".into(),
            specialties: "24/7 Notfallambulanz, Intensivstation, Chirurgie, Innere Medizin, Neurologie, Onkologie, Blutbank".into(),
            emergency_24h: true,
            when_to_go: "Bei akuten Lebensgefahren (Magendrehung, Vergiftung, Atemnot, schwerer Unfall) oder komplexen Fällen nachts/am Wochenende.".into(),
        },
        VetClinicInfo {
            name: "Tierklinik Korneuburg / Tierklinik Strebersdorf / Tierklinik Aspern".into(),
            tier_type: "3. Ebene: Regionale Tierklinik".into(),
            region: "Wien & Niederösterreich".into(),
            specialties: "Notdienst, stationäre Aufnahme, Chirurgie, Bildgebung (Röntgen/Ultraschall/CT)".into(),
            emergency_24h: true,
            when_to_go: "Schnelle Klinik-Versorgung in Wien/NÖ; vor der Fahrt IMMER kurz telefonisch ankündigen!".into(),
        },
        VetClinicInfo {
            name: "LMU Tierklinik München / FU Berlin Kleintierklinik / Tierspital Zürich (Vetsuisse)".into(),
            tier_type: "3. Ebene: Uni-Tierspital (DE / CH)".into(),
            region: "Deutschland & Schweiz".into(),
            specialties: "Hochspezialisierte Universitätsmedizin, 24/7 Intensivstation, MRT/CT, Kardiologie, Neurochirurgie".into(),
            emergency_24h: true,
            when_to_go: "Notfälle & Überweisungen auf Maximalversorger-Niveau in DE/CH.".into(),
        },
    ]
}

pub fn senior_dog_sections() -> Vec<SeniorSection> {
    vec![
        SeniorSection {
            title: "Wann ist ein Hund 'alt'? (Größenabhängig!)".into(),
            summary: "Kleine Hunde werden oft 14-17 Jahre alt (Senior ab ~9-10 J.), Riesenrassen wie Berner/Leonberger schon ab 6-7 Jahren.".into(),
            tips: vec![
                "Kleine Rassen (<10 kg): Senior ab ca. 9-10 Jahren.".into(),
                "Mittlere Rassen (10-25 kg): Senior ab ca. 8 Jahren.".into(),
                "Große & Riesenrassen (>25 kg): Senior ab ca. 6-7 Jahren.".into(),
            ],
        },
        SeniorSection {
            title: "Gelenke, Arthrose & Mobilität".into(),
            summary: "Viele alte Hunde zeigen Schmerzen nicht durch Jaulen, sondern durch Rückzug oder langsames Aufstehen.".into(),
            tips: vec![
                "Rutschfeste Teppichläufer auf Parkett/Fliesen auslegen.".into(),
                "Hunderampe fürs Auto oder Tragegeschirr (z.B. Help-Em-Up) bei Treppen nutzen.".into(),
                "Tierärztliche Schmerztherapie (z.B. Librela-Antikörper, NSAIDs, Physio) schenkt enorme Lebensqualität!".into(),
            ],
        },
        SeniorSection {
            title: "Hunde-Demenz (Kognitive Dysfunktion / CCD)".into(),
            summary: "Nächtliche Unruhe, Starren gegen Wände oder Vergessen der Stubenreinheit können Anzeichen von CCD sein.".into(),
            tips: vec![
                "Feste Tagesabläufe und Möbel nicht ständig umstellen.".into(),
                "Kleines Nachtlicht einschalten, wenn Sehkraft nachlässt.".into(),
                "Sanfte Schnüffelspiele halten das Gehirn aktiv ohne die Gelenke zu überlasten.".into(),
            ],
        },
        SeniorSection {
            title: "Herz, Nieren, Zähne & Vorsorge-Check".into(),
            summary: "Ab dem Seniorenalter empfiehlt sich alle 6 Monate ein geriatrischer Check beim Haustierarzt.".into(),
            tips: vec![
                "Großes Blutbild + Nieren-/Leberwerte + Schilddrüse.".into(),
                "Herz abhören (besonders bei Cavalier, Dackel, Pudel) & Blutdruck messen.".into(),
                "Zahnstein/Schmerzen prüfen — entzündete Zähne belasten Herz und Nieren!".into(),
            ],
        },
        SeniorSection {
            title: "Würdevoller Abschied & Lebensqualität".into(),
            summary: "Die HHHHHMM-Skala hilft objektiv zu bewerten: Hurt (Schmerz), Hunger, Hydration, Hygiene, Happiness, Mobility, More good days than bad.".into(),
            tips: vec![
                "Mobile Tierärzte ermöglichen einen ruhigen Abschied zu Hause in vertrauter Umgebung.".into(),
                "Rechtzeitig mit dem Haustierarzt über Palliativplan und Notfall-Telefonnummer sprechen.".into(),
            ],
        },
    ]
}
