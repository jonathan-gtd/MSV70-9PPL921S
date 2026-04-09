# Reprog ECU — MSV70

Documentation et tutoriels pour la reprogrammation du calculateur **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

## Guides de tuning

| Document | Contenu | Statut |
|---|---|---|
| [TUTO E85](TUTO_E85.md) | Conversion éthanol complète — injecteurs, cranking, avance, lambda, film mural | v3.4 — vérifié bin |
| [TUTO RPM Protection](TUTO_RPM_Protection.md) | Modification du coupe-circuit RPM et protection à froid f(TCO) | — |
| [TUTO VMAX](TUTO_VMAX.md) | Suppression ou ajustement du bridage 250 km/h | — |

### Navigation rapide — TUTO E85

| Section | Accès direct |
|---|---|
| Stratégie de calibration | [Pourquoi E85/E70/E60](TUTO_E85.md#tuto-e85--conversion-ethanol-pour-siemens-msv70--bmw-330i-n52b30) |
| Résumé de tous les paramètres impactés | [📌 Tableau synthèse](TUTO_E85.md#-résumé-des-paramètres-impactés-par-la-conversion-e85) |
| Prérequis mécaniques et logiciels | [⚙️ Prérequis](TUTO_E85.md#-prérequis-avant-toute-conversion-e85) |
| Principes E85/E70/E60 — pourquoi l'éthanol change tout | [📋 §0 Principes](TUTO_E85.md#-0-principes-fondamentaux-e85-sur-n52) |
| Pourquoi autant de paramètres pour la même chose ? (4 raisons) | [§0 Les 4 raisons](TUTO_E85.md#pourquoi-autant-de-paramètres-pour-la-même-chose-) |
| Facteur injecteur — les 5 copies | [🔧 §1 Injecteurs](TUTO_E85.md#-1-mise-à-léchelle-des-injecteurs--paramètre-critique) |
| Démarrage froid — cranking, seuil TCO, warm-up | [❄️ §2 Démarrage froid](TUTO_E85.md#-2-démarrage-à-froid-cranking--after-start) |
| Avance à l'allumage — table knock ceiling | [⚡ §3 Avance](TUTO_E85.md#-3-avance-à-lallumage) |
| Consigne lambda — tables WOT et part-load | [🎯 §4 Lambda](TUTO_E85.md#-4-consigne-lambda-richesse-cible) |
| Film mural — slow/fast, pos/neg | [🚿 §5 Film mural](TUTO_E85.md#-5-film-mural-wall-film-correction) |
| Purge canister EVAP | [🔁 §6 EVAP](TUTO_E85.md#-6-purge-canister-evap--impact-e85) |
| Temps mort injecteur, délai WOT, allumage cranking | [🔩 §7 Complémentaires](TUTO_E85.md#-7-paramètres-complémentaires) |
| Surveillance STFT/LTFT, pompe, bougies | [⚠️ §8 Maintenance](TUTO_E85.md#-8-avertissements-et-maintenance) |
| Toutes les valeurs concrètes à entrer dans TunerPro | [📊 §9 Valeurs](TUTO_E85.md#-9-résumé-des-modifications--valeurs-concrètes) |
| Plan de validation progressif jour par jour | [🧪 §10 Plan de test](TUTO_E85.md#-10-plan-de-test-et-validation-progressif) |
| Checklist avant de flasher | [📋 §11 Checklist](TUTO_E85.md#-11-checklist-avant-conversion) |
| Symptôme → cause → solution | [🔍 §12 Diagnostic](TUTO_E85.md#-12-diagnostic-rapide-des-problèmes) |

## Documentation technique

| Document | Contenu |
|---|---|
| [FLASH ECU](FLASH_ECU.md) | Matériel, dump, TunerPro, checksum, RSA, flash, vérification post-flash |
| [Lexique](LEXIQUE.md) | Préfixes Siemens/Bosch, sigles, A-Z, structures de données, adressage |
