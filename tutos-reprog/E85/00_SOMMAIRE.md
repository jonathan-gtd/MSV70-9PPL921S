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

- **[04 — Injecteurs](04_INJECTEURS.md)** — Facteur de masse carburant — le plus critique
  - [① `ip_mff_cor_opm_1_1` — Multiplicateur Valvetronic, mode 1](04_INJECTEURS.md#p1)
  - [② `ip_mff_cor_opm_1_2` — Multiplicateur Valvetronic, mode 2](04_INJECTEURS.md#p2)
  - [③ `ip_mff_cor_opm_2_1` — Multiplicateur papillonné (GD), mode 1](04_INJECTEURS.md#p3)
  - [④ `ip_mff_cor_opm_2_2` — Multiplicateur papillonné (GD), mode 2](04_INJECTEURS.md#p4)
  - [⑤ `c_t_ti_dly_fl_1` — Délai WOT MT, copie 1](04_INJECTEURS.md#p5)
  - [⑥ `c_t_ti_dly_fl_2` — Délai WOT MT, copie 2](04_INJECTEURS.md#p6)
  - [⑦ `c_tco_n_mff_cst` — Seuil TCO cranking enrichi](04_INJECTEURS.md#p7)
  - [⑧ Duty cycle — saturation haut régime (surveillance)](04_INJECTEURS.md#p8)
  - [⑨ `ip_ti_min` — Dead time f(tension batterie)](04_INJECTEURS.md#p9)

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

- **[07 — Avance allumage](07_AVANCE.md)** — Plafond knock — exploite l'octane de l'E85
  - [① `ip_iga_bas_max_knk__n__maf` — Plafond anti-cliquetis f(MAF × RPM)](07_AVANCE.md#p1)
  - [② `ip_iga_st_bas_opm_1` — Avance cranking Valvetronic](07_AVANCE.md#p2)
  - [③ `ip_iga_st_bas_opm_2` — Avance cranking papillonné (GD)](07_AVANCE.md#p3)

---

## Modifications optionnelles

Améliorent le comportement mais non bloquantes pour un premier essai.

- **[08 — Délai WOT](08_DELAI_WOT.md)** — Délai avant enrichissement pleine charge
  - [① `c_t_ti_dly_fl_1` — MT, copie 1](08_DELAI_WOT.md#p1)
  - [② `c_t_ti_dly_fl_2` — MT, copie 2](08_DELAI_WOT.md#p2)
  - [③ `c_t_ti_dly_fl_at_1` — AT, copie 1](08_DELAI_WOT.md#p3)
  - [④ `c_t_ti_dly_fl_at_2` — AT, copie 2](08_DELAI_WOT.md#p4)

- **[09 — Lambda WOT](09_LAMBDA.md)** — Prévention DTC et richesse pleine charge
  - [① `ip_fac_lamb_max_fsd_1` — Plafond WRAF, mode 1 (anti-DTC)](09_LAMBDA.md#p1)
  - [② `ip_fac_lamb_max_fsd_2` — Plafond WRAF, mode 2 (anti-DTC)](09_LAMBDA.md#p2)
  - [③ `c_lamb_delta_i_max_lam_adj` — Plafond LTFT intégral (anti-DTC)](09_LAMBDA.md#p3)
  - [④ `ip_lamb_fl__n` — Lambda cible WOT f(RPM) — OPTIONNEL](09_LAMBDA.md#p4)

---

## Paramètres secondaires

À consulter selon le comportement observé. Ne pas modifier en aveugle.

- **[10 — Warm-up lambda](10_WARMUP_LAMBDA.md)** — Enrichissement warm-up tous régimes et ralenti
  - [① `ip_fac_lamb_wup` — Facteur warm-up lambda, tous régimes](10_WARMUP_LAMBDA.md#p1)
  - [② `ip_fac_lamb_wup_is` — Facteur warm-up lambda, ralenti](10_WARMUP_LAMBDA.md#p2)

- **[11 — Avance cranking](11_AVANCE_CRANKING.md)** — Démarrage difficile malgré §05 correct
  - [① `c_iga_ini` — Avance initiale cranking](11_AVANCE_CRANKING.md#p1)

- **[12 — Transitoire](12_TRANSITOIRE.md)** — À-coups à l'accélération
  - [① `KF_FTRANSVL` — Facteur transition pleine charge](12_TRANSITOIRE.md#p1)
  - [② `KL_STEND_TRANS` — Facteur démarrage transition](12_TRANSITOIRE.md#p2)
  - [③ `KL_FUPSRF_TRANS` — Correction surface pression transitoire](12_TRANSITOIRE.md#p3)
  - [④ `KL_PIRG_TRANS` — Pression résiduelle gaz brûlés](12_TRANSITOIRE.md#p4)

- **[13 — LTFT](13_LTFT.md)** — STFT/LTFT saturés en adaptation
  - [① `c_fac_max_h_rng_lam_ad` — Limite haute LTFT, haute charge](13_LTFT.md#p1)
  - [② `c_fac_max_l_rng_lam_ad` — Limite haute LTFT, basse charge](13_LTFT.md#p2)
  - [③ `c_fac_min_h_rng_lam_ad` — Limite basse LTFT, haute charge](13_LTFT.md#p3)
  - [④ `c_fac_min_l_rng_lam_ad` — Limite basse LTFT, basse charge](13_LTFT.md#p4)
  - [⑤ `c_lam_mv_dyw_dly` — Fenêtre dynamique STFT](13_LTFT.md#p5)

- **[14 — EVAP](14_EVAP.md)** — Oscillations STFT > ±15% à chaud
  - [① `ip_flow_max_cps` — Débit maximal purge canister](14_EVAP.md#p1)
  - [② `ip_flow_cps` — Débit nominal purge canister](14_EVAP.md#p2)

- **[15 — Chauffe catalyseur](15_CHAUFFE_CAT.md)** — Sans catalyseur uniquement
  - [① `ip_fac_eff_iga_ch_cold_opm_1` — Retard allumage chauffe cat, Valvetronic](15_CHAUFFE_CAT.md#p1)
  - [② `ip_fac_eff_iga_ch_cold_opm_2` — Retard allumage chauffe cat, papillonné (GD)](15_CHAUFFE_CAT.md#p2)

---

## Validation

À suivre après chaque session de modifications.

| # | Fichier | Contenu |
|---|---|---|
| 16 | [Résumé](16_RESUME.md) | Tableau complet : modifier / surveiller / ne pas toucher |
| 17 | [Plan de test](17_PLAN_TEST.md) | Protocole de validation en 5 phases |
| 18 | [Diagnostic](18_DIAGNOSTIC.md) | Symptôme → cause → solution |
| 19 | [Avertissements](19_AVERTISSEMENTS.md) | Surveillance STFT/LTFT, pompe, filtre, bougies, hiver |
| 20 | [Vérification](20_VERIFICATION.md) | Niveaux de confiance par section |
| 21 | [Conclusion](21_CONCLUSION.md) | Avertissement légal et responsabilité |
