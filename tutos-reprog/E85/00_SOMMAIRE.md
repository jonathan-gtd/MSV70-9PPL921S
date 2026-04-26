# Conversion E85 — Sommaire

Guide complet de conversion éthanol pour le **Siemens MSV70** (BMW N52B30, SW 9PPL921S).

---

## Avant de commencer

| # | Fichier | Contenu |
|---|---|---|
| 01 | [Principes](01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF, pourquoi autant de paramètres |
| 02 | [Mythes](02_MYTHES.md) | 8 idées reçues sur l'E85 — démontées avec sources |
| 03 | [Prérequis](03_PREREQUIS.md) | Checklist mécanique et logicielle avant toute conversion |

---

## Modifications obligatoires

À faire dans cet ordre. Ne pas passer à l'étape suivante sans avoir validé la précédente.

- **[04 — Enrichissement](04_ENRICHISSEMENT.md)** — Facteur de masse carburant — le plus critique
  - [① `ip_mff_cor_opm_1_1` — Multiplicateur Valvetronic, mode 1](04_ENRICHISSEMENT.md#p1)
  - [② `ip_mff_cor_opm_1_2` — Multiplicateur Valvetronic, mode 2](04_ENRICHISSEMENT.md#p2)
  - [③ `ip_mff_cor_opm_2_1` — Multiplicateur papillonné (GD), mode 1](04_ENRICHISSEMENT.md#p3)
  - [④ `ip_mff_cor_opm_2_2` — Multiplicateur papillonné (GD), mode 2](04_ENRICHISSEMENT.md#p4)

- **[05 — Démarrage froid](05_DEMARRAGE_FROID.md)** — Cranking et after-start sur éthanol
  - [① `ip_mff_cst_opm_1` — Cranking Valvetronic](05_DEMARRAGE_FROID.md#p1)
  - [② `ip_mff_cst_opm_2` — Cranking papillonné (GD)](05_DEMARRAGE_FROID.md#p2)
  - [③ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi](05_DEMARRAGE_FROID.md#p3)
  - [④ `ip_mff_lgrd_ast` — Enrichissement after-start](05_DEMARRAGE_FROID.md#p4)

- **[06 — Film mural](06_FILM_MURAL.md)** — Compensation paroi collecteur (×1.25)
  - [① `ip_ti_tco_pos_slow_wf_opm_1` — Film lent positif, Valvetronic](06_FILM_MURAL.md#p1)
  - [② `ip_ti_tco_pos_slow_wf_opm_2` — Film lent positif, papillonné (GD)](06_FILM_MURAL.md#p2)
  - [③ `ip_ti_tco_pos_fast_wf_opm_1` — Film rapide positif, Valvetronic](06_FILM_MURAL.md#p3)
  - [④ `ip_ti_tco_pos_fast_wf_opm_2` — Film rapide positif, papillonné (GD)](06_FILM_MURAL.md#p4)

- **[07 — Allumage](07_ALLUMAGE.md)** — Avance E60-safe + délai WOT
  - [① `ip_iga_bas_max_knk__n__maf` — Plafond anti-cliquetis f(MAF × RPM)](07_ALLUMAGE.md#p1)
  - [② `c_t_ti_dly_fl_1` — Délai WOT boîte manuelle, copie 1](07_ALLUMAGE.md#p2)
  - [③ `c_t_ti_dly_fl_2` — Délai WOT boîte manuelle, copie 2](07_ALLUMAGE.md#p3)
  - [④ `c_t_ti_dly_fl_at_1/2` — Délai WOT boîte automatique](07_ALLUMAGE.md#p4)

- **[08 — Lambda](08_LAMBDA.md)** — Anti-DTC break-in, warm-up lambda, richesse WOT
  - [① `ip_fac_lamb_max_fsd_1` — Plafond WRAF, mode 1](08_LAMBDA.md#p1)
  - [② `ip_fac_lamb_max_fsd_2` — Plafond WRAF, mode 2](08_LAMBDA.md#p2)
  - [③ `c_lamb_delta_i_max_lam_adj` — Plafond LTFT intégral](08_LAMBDA.md#p3)
  - [④ `ip_fac_lamb_wup` — Facteur warm-up lambda, tous régimes](08_LAMBDA.md#p4)

---

## Modifications optionnelles

Améliorent le comportement mais non bloquantes pour un premier essai.

- **[08 — Lambda (suite)](08_LAMBDA.md)** — Paramètres optionnels
  - [⑤ `ip_fac_lamb_wup_is` — Warm-up lambda ralenti (si instable)](08_LAMBDA.md#p5)
  - [⑥ `ip_lamb_fl__n` — Lambda cible WOT f(RPM)](08_LAMBDA.md#p6)

- **[07 — Allumage (suite)](07_ALLUMAGE.md)** — Avance cranking
  - [⑤ `ip_iga_st_bas_opm_1` — Avance cranking Valvetronic (si démarrage > 5 tours)](07_ALLUMAGE.md#p5)
  - [⑥ `ip_iga_st_bas_opm_2` — Avance cranking papillonné GD](07_ALLUMAGE.md#p6)
  - [⑦ `c_iga_ini` — Avance initiale cranking (dernier recours)](07_ALLUMAGE.md#p7)

- **[09 — Transitoire](09_TRANSITOIRE.md)** — À-coups à l'accélération (kickdown uniquement)
  - [① `KF_FTRANSVL` — Facteur transition pleine charge](09_TRANSITOIRE.md#p1)

- **[10 — EVAP](10_EVAP.md)** — Oscillations STFT lors de la purge canister
  - [① `ip_flow_max_cps` — Débit maximal purge canister](10_EVAP.md#p1)
  - [② `ip_flow_cps` — Débit nominal purge canister](10_EVAP.md#p2)

---

## Validation et suivi

| # | Fichier | Contenu |
|---|---|---|
| 11 | [Surveillance](11_SURVEILLANCE.md) | Indicateurs OBD à lire pendant la mise au point |
| 12 | [Plan de test](12_PLAN_TEST.md) | Résumé paramètres, plan 5 phases, diagnostic, avertissements |
