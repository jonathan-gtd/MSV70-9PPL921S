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
  - [① `ip_mff_cor_opm_1_1` — Multiplicateur Valvetronic, mode 1](04_INJECTEURS.md#①-ip_mff_cor_opm_1_1--multiplicateur-injection-valvetronic-mode-1)
  - [② `ip_mff_cor_opm_1_2` — Multiplicateur Valvetronic, mode 2](04_INJECTEURS.md#②-ip_mff_cor_opm_1_2--multiplicateur-injection-valvetronic-mode-2)
  - [③ `ip_mff_cor_opm_2_1` — Multiplicateur papillonné (GD), mode 1](04_INJECTEURS.md#③-ip_mff_cor_opm_2_1--multiplicateur-injection-papillonné-gd-mode-1)
  - [④ `ip_mff_cor_opm_2_2` — Multiplicateur papillonné (GD), mode 2](04_INJECTEURS.md#④-ip_mff_cor_opm_2_2--multiplicateur-injection-papillonné-gd-mode-2)
  - [⑤ `ip_ti_min` — Dead time f(tension batterie)](04_INJECTEURS.md#⑤-ip_ti_min--dead-time-injecteur-ftension-batterie)
  - [⑥ Duty cycle — saturation haut régime](04_INJECTEURS.md#⑥-duty-cycle-injecteur--saturation-haut-régime-surveillance)
  - [⑦ `c_tco_n_mff_cst` — Seuil TCO cranking enrichi](04_INJECTEURS.md#⑦-c_tco_n_mff_cst--seuil-tco-activation-cranking-enrichi)
  - [⑧ `c_t_ti_dly_fl_1` — Délai WOT MT, copie 1](04_INJECTEURS.md#⑧-c_t_ti_dly_fl_1--délai-enrichissement-wot-boîte-manuelle-copie-1)
  - [⑨ `c_t_ti_dly_fl_2` — Délai WOT MT, copie 2](04_INJECTEURS.md#⑨-c_t_ti_dly_fl_2--délai-enrichissement-wot-boîte-manuelle-copie-2)

- **[05 — Démarrage froid](05_DEMARRAGE_FROID.md)** — Cranking et after-start sur éthanol
  - [① `ip_mff_cst_opm_1` — Cranking Valvetronic](05_DEMARRAGE_FROID.md#①-ip_mff_cst_opm_1--masse-carburant-cranking-mode-valvetronic)
  - [② `ip_mff_cst_opm_2` — Cranking papillonné (GD)](05_DEMARRAGE_FROID.md#②-ip_mff_cst_opm_2--masse-carburant-cranking-mode-papillonné-gd)
  - [③ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi](05_DEMARRAGE_FROID.md#③-c_tco_n_mff_cst--seuil-tco-activation-cranking-enrichi)
  - [④ `ip_mff_lgrd_ast` — Enrichissement after-start](05_DEMARRAGE_FROID.md#④-ip_mff_lgrd_ast--enrichissement-after-start-phase-post-démarrage)

- **[06 — Film mural](06_FILM_MURAL.md)** — Compensation paroi collecteur (×1.25)
  - [① `ip_ti_tco_pos_slow_wf_opm_1` — Film lent positif, Valvetronic](06_FILM_MURAL.md#①-ip_ti_tco_pos_slow_wf_opm_1--film-lent-positif-valvetronic)
  - [② `ip_ti_tco_pos_slow_wf_opm_2` — Film lent positif, papillonné (GD)](06_FILM_MURAL.md#②-ip_ti_tco_pos_slow_wf_opm_2--film-lent-positif-papillonné-gd)
  - [③ `ip_ti_tco_pos_fast_wf_opm_1` — Film rapide positif, Valvetronic](06_FILM_MURAL.md#③-ip_ti_tco_pos_fast_wf_opm_1--film-rapide-positif-valvetronic)
  - [④ `ip_ti_tco_pos_fast_wf_opm_2` — Film rapide positif, papillonné (GD)](06_FILM_MURAL.md#④-ip_ti_tco_pos_fast_wf_opm_2--film-rapide-positif-papillonné-gd)

- **[07 — Avance allumage](07_AVANCE.md)** — Plafond knock — exploite l'octane de l'E85
  - [① `ip_iga_bas_max_knk__n__maf` — Plafond anti-cliquetis f(MAF × RPM)](07_AVANCE.md#①-ip_iga_bas_max_knk__n__maf--plafond-avance-anti-cliquetis-fmaf--rpm)
  - [② `ip_iga_st_bas_opm_1` — Avance cranking Valvetronic](07_AVANCE.md#②-ip_iga_st_bas_opm_1--avance-cranking-mode-valvetronic)
  - [③ `ip_iga_st_bas_opm_2` — Avance cranking papillonné (GD)](07_AVANCE.md#③-ip_iga_st_bas_opm_2--avance-cranking-mode-papillonné-gd)

---

## Modifications optionnelles

Améliorent le comportement mais non bloquantes pour un premier essai.

- **[08 — Délai WOT](08_DELAI_WOT.md)** — Délai avant enrichissement pleine charge
  - [① `c_t_ti_dly_fl_1` — MT, copie 1](08_DELAI_WOT.md#①-c_t_ti_dly_fl_1--délai-wot-boîte-manuelle-copie-1)
  - [② `c_t_ti_dly_fl_2` — MT, copie 2](08_DELAI_WOT.md#②-c_t_ti_dly_fl_2--délai-wot-boîte-manuelle-copie-2)
  - [③ `c_t_ti_dly_fl_at_1` — AT, copie 1](08_DELAI_WOT.md#③-c_t_ti_dly_fl_at_1--délai-wot-boîte-automatique-copie-1)
  - [④ `c_t_ti_dly_fl_at_2` — AT, copie 2](08_DELAI_WOT.md#④-c_t_ti_dly_fl_at_2--délai-wot-boîte-automatique-copie-2)

- **[09 — Lambda WOT](09_LAMBDA.md)** — Richesse pleine charge et prévention DTC
  - [① `ip_lamb_fl__n` — Lambda cible WOT f(RPM)](09_LAMBDA.md#①-ip_lamb_fl__n--lambda-cible-pleine-charge-frpm--optionnel)
  - [② `ip_fac_lamb_max_fsd_1` — Plafond WRAF, mode 1 (anti-DTC)](09_LAMBDA.md#②-ip_fac_lamb_max_fsd_1--plafond-correction-wraf-instantanée-mode-1-anti-dtc)
  - [③ `ip_fac_lamb_max_fsd_2` — Plafond WRAF, mode 2 (anti-DTC)](09_LAMBDA.md#③-ip_fac_lamb_max_fsd_2--plafond-correction-wraf-instantanée-mode-2-anti-dtc)
  - [④ `c_lamb_delta_i_max_lam_adj` — Plafond LTFT intégral (anti-DTC)](09_LAMBDA.md#④-c_lamb_delta_i_max_lam_adj--plafond-ltft-intégral-anti-dtc)

---

## Paramètres secondaires

À consulter selon le comportement observé. Ne pas modifier en aveugle.

- **[10 — Warm-up lambda](10_WARMUP_LAMBDA.md)** — Ralenti instable chaud
  - [① `ip_fac_lamb_wup_is` — Facteur warm-up lambda](10_WARMUP_LAMBDA.md#①-ip_fac_lamb_wup_is--facteur-warm-up-lambda-ralenti-uniquement)

- **[11 — Avance cranking](11_AVANCE_CRANKING.md)** — Démarrage difficile malgré §05 correct
  - [① `c_iga_ini` — Avance initiale cranking](11_AVANCE_CRANKING.md#①-c_iga_ini--avance-initiale-cranking--optionnel)

- **[12 — Transitoire](12_TRANSITOIRE.md)** — À-coups à l'accélération
  - [① `KF_FTRANSVL` — Facteur transition pleine charge](12_TRANSITOIRE.md#①-kf_ftransvl--facteur-de-transition-pleine-charge-fcharge--rpm)
  - [② `KL_STEND_TRANS` — Facteur démarrage transition](12_TRANSITOIRE.md#②-kl_stend_trans--facteur-de-démarrage-de-transition)
  - [③ `KL_FUPSRF_TRANS` — Correction surface pression transitoire](12_TRANSITOIRE.md#③-kl_fupsrf_trans--correction-surface-pression-carburant-transitoire)
  - [④ `KL_PIRG_TRANS` — Pression résiduelle gaz brûlés](12_TRANSITOIRE.md#④-kl_pirg_trans--pression-résiduelle-gaz-brûlés-en-transitoire)

- **[13 — LTFT](13_LTFT.md)** — STFT/LTFT saturés en adaptation
  - [① `c_fac_max_h_rng_lam_ad` — Limite haute LTFT, haute charge](13_LTFT.md#①-c_fac_max_h_rng_lam_ad--limite-haute-ltft-haute-charge)
  - [② `c_fac_max_l_rng_lam_ad` — Limite haute LTFT, basse charge](13_LTFT.md#②-c_fac_max_l_rng_lam_ad--limite-haute-ltft-basse-charge)
  - [③ `c_fac_min_h_rng_lam_ad` — Limite basse LTFT, haute charge](13_LTFT.md#③-c_fac_min_h_rng_lam_ad--limite-basse-ltft-haute-charge)
  - [④ `c_fac_min_l_rng_lam_ad` — Limite basse LTFT, basse charge](13_LTFT.md#④-c_fac_min_l_rng_lam_ad--limite-basse-ltft-basse-charge)
  - [⑤ `c_lam_mv_dyw_dly` — Fenêtre dynamique STFT](13_LTFT.md#⑤-c_lam_mv_dyw_dly--fenêtre-dynamique-stft)

- **[14 — EVAP](14_EVAP.md)** — Oscillations STFT > ±15% à chaud
  - [① `ip_flow_max_cps` — Débit maximal purge canister](14_EVAP.md#①-ip_flow_max_cps--débit-maximal-de-purge-canister)
  - [② `ip_flow_cps` — Débit nominal purge canister](14_EVAP.md#②-ip_flow_cps--débit-nominal-de-purge-canister)

- **[15 — Chauffe catalyseur](15_CHAUFFE_CAT.md)** — Sans catalyseur uniquement
  - [① `ip_fac_eff_iga_ch_cold_opm_1` — Retard allumage chauffe cat, Valvetronic](15_CHAUFFE_CAT.md#①-ip_fac_eff_iga_ch_cold_opm_1--retard-allumage-chauffe-catalyseur-valvetronic)
  - [② `ip_fac_eff_iga_ch_cold_opm_2` — Retard allumage chauffe cat, papillonné (GD)](15_CHAUFFE_CAT.md#②-ip_fac_eff_iga_ch_cold_opm_2--retard-allumage-chauffe-catalyseur-papillonné-gd)

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
