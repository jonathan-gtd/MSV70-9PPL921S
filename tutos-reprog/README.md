# Reprog ECU — MSV70 E85

Conversion éthanol sur **Siemens MSV70** (BMW N52B30, SW 9PPL921S — dump VB67774).

---

## Tableau de référence — Paramètres E85

Toutes les certitudes sont basées sur : description XDF anglaise + catégorie XDF + décodage du nom + cohérence avec les valeurs stock lues dans le bin.

### Obligatoires

**Navigation**

| Statut | Paramètre | Adresse | Guide | Type · Axes · Unité | Conf. |
|---|---|---|---|---|---|
| ✅ | `ip_mff_cor_opm_1_1` | 0x4E3D4 | §04 | Map 12×16 · charge(mg/stk)×RPM · facteur | 100% |
| ✅ | `ip_mff_cor_opm_1_2` | 0x4E554 | §04 | Map 12×16 · charge(mg/stk)×RPM · facteur | 100% |
| ✅ | `ip_mff_cor_opm_2_1` | 0x4E6D4 | §04 | Map 10×12 · charge(mg/stk)×RPM · facteur | 100% |
| ✅ | `ip_mff_cor_opm_2_2` | 0x4E7C4 | §04 | Map 10×12 · charge(mg/stk)×RPM · facteur | 100% |
| ✅ | `ip_mff_cst_opm_1` | 0x437DC | §05 | Map 3×8 · RPM démarreur×TCO(°C) · mg/stk | 100% |
| ✅ | `ip_mff_cst_opm_2` | 0x4380C | §05 | Map 3×8 · RPM démarreur×TCO(°C) · mg/stk | 100% |
| ✅ | `c_tco_n_mff_cst` | 0x44F2F | §05 | Scalaire · °C | 100% |
| ✅ | `ip_ti_cast_opm_1` | 0x43FA4 | §05 | Map 2×10 · TCO admission×TCO liquide(°C) · facteur | 82% |
| ✅ | `ip_ti_cast_opm_2` | 0x43FB8 | §05 | Map 2×10 · TCO admission×TCO liquide(°C) · facteur | 82% |
| ✅ | `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | §06 | Map 8×8 · RPM×TCO(°C) · facteur | 93% |
| ✅ | `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | §06 | Map 8×8 · RPM×TCO(°C) · facteur | 93% |
| ✅ | `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | §06 | Map 8×8 · RPM×TCO(°C) · facteur | 90% |
| ✅ | `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | §06 | Map 8×8 · RPM×TCO(°C) · facteur | 90% |
| ✅ | `ip_iga_bas_max_knk__n__maf` | 0x4323A | §07 | Map 8×8 · RPM×charge(mg/stk) · °CRK | 92% |
| ✅ | `c_t_ti_dly_fl_1` | 0x44EC4 | §07 | Scalaire · s | 100% |
| ✅ | `c_t_ti_dly_fl_2` | 0x44EC6 | §07 | Scalaire · s | 100% |
| ✅ | `c_t_ti_dly_fl_at_1` | 0x44EC8 | §07 | Scalaire · s | 100% |
| ✅ | `c_t_ti_dly_fl_at_2` | 0x44ECA | §07 | Scalaire · s | 100% |
| ✅ | `c_lamb_delta_i_max_lam_adj` | 0x47F5E | §08 | Scalaire · λ | 98% |
| ✅ | `c_lamb_delta_i_min_lam_adj` | 0x47F60 | §08 | Scalaire · λ | 98% |
| ✅ | `ip_fac_lamb_wup` | 0x42764 | §08 | Map 6×6 · RPM×charge(mg/stk) · facteur | 95% |

**Détail**

| Paramètre | Description XDF (originale) | Décodage du nom | Ce que ça fait |
|---|---|---|---|
| `ip_mff_cor_opm_1_1` | *Injected fuel mass correction for operating mode 1 (bank 1)* | **ip**=table · **mff**=mass fuel flow · **cor**=correction · **opm**=operating mode · **1_1**=mode1 banque1 | Multiplicateur masse carburant f(charge, RPM), mode Valvetronic, banque 1. Stock 1.016, E85 : 1.473 |
| `ip_mff_cor_opm_1_2` | *…bank 2* | **1_2**=mode1 banque2 | Même rôle, banque 2 |
| `ip_mff_cor_opm_2_1` | *…operating mode 2 (bank 1)* | **2**=mode papillonné (GD) | Même rôle, mode papillonné, banque 1 |
| `ip_mff_cor_opm_2_2` | *…operating mode 2 (bank 2)* | **2_2**=mode papillonné banque2 | Même rôle, mode papillonné, banque 2 |
| `ip_mff_cst_opm_1` | *Basic value for cranking injection at operation mode 1* | **cst**=cold start · **opm_1**=Valvetronic | Masse injectée pendant cranking f(RPM démarreur, TCO), Valvetronic. E85 : ×1.35–2.00 selon TCO |
| `ip_mff_cst_opm_2` | *…operation mode 2* | **opm_2**=papillonné | Même rôle, mode papillonné |
| `c_tco_n_mff_cst` | *Cool temperature constant* | **c**=constante · **tco**=temp. liquide · **n**=seuil · **mff_cst**=cold start MFF | Seuil TCO d'activation cranking enrichi. Stock 17.25°C, E85 : 25°C |
| `ip_ti_cast_opm_1` | *Initialization value of post start enrichment factor at operation mode 1* | **ti**=injection time · **cast**=cold/catalyst after start · **opm_1**=Valvetronic | Valeur initiale du facteur d'enrichissement post-démarrage f(TCO liquide, TCO admission). E85 : ×1.55–1.65 froid |
| `ip_ti_cast_opm_2` | *…operation mode 2* | **opm_2**=papillonné | Même rôle, mode papillonné |
| `ip_ti_tco_pos_fast_wf_opm_1` | *fast positive wall film factor* | **ti**=injection time · **tco**=f(TCO) · **pos**=montée charge · **fast**=rapide · **wf**=wall film · **opm_1**=Valvetronic | Film mural positif rapide f(RPM, TCO), Valvetronic. E85 : ×1.25 sous 70°C |
| `ip_ti_tco_pos_fast_wf_opm_2` | *fast positive wall film factor* | **opm_2**=papillonné | Même rôle, mode papillonné |
| `ip_ti_tco_pos_slow_wf_opm_1` | *total positive wall film factor* | **slow**=composante lente · **total**=accumulation totale film | Film mural positif lent f(RPM, TCO), Valvetronic. E85 : ×1.25 sous 70°C |
| `ip_ti_tco_pos_slow_wf_opm_2` | *total positive wall film factor* | **opm_2**=papillonné | Même rôle, mode papillonné |
| `ip_iga_bas_max_knk__n__maf` | *Maximum value for spark retard* | **iga**=ignition angle · **bas**=base · **max**=plafond · **knk**=knock · **n**=RPM · **maf**=débit air | Plafond avance anti-cliquetis f(RPM, MAF). ⚠️ Plafond ≠ avance effective — lever ne fait rien si le modèle couple demande moins |
| `c_t_ti_dly_fl_1` | *Delay time Full load* | **c**=constante · **t**=temps · **ti**=injection time · **dly**=delay · **fl**=full load · **1**=BM copie1 | Délai activation enrichissement WOT boîte manuelle. Stock 200 ms, E85 : 0 ms |
| `c_t_ti_dly_fl_2` | *Delay time Full load* | **2**=copie 2 | Copie 2 — modifier identiquement |
| `c_t_ti_dly_fl_at_1` | *Delay time Full load AT* | **at**=automatic transmission | Idem boîte automatique |
| `c_t_ti_dly_fl_at_2` | *Delay time Full load AT* | **at_2**=BVA copie 2 | Copie 2 BVA |
| `c_lamb_delta_i_max_lam_adj` | *upper limit of trim control I share* | **c**=constante · **lamb**=lambda · **delta**=variation · **i**=intégrateur · **max**=plafond · **lam_adj**=lambda adjustment | Plafond haut LTFT (intégrateur). Stock +0.050λ (+5%), E85 break-in : +0.25λ |
| `c_lamb_delta_i_min_lam_adj` | *lower limit of trim control I share* | **min**=plancher | Plafond bas LTFT (intégrateur). Stock −0.050λ (−5%), E85 break-in : −0.25λ |
| `ip_fac_lamb_wup` | *correction factor for basic lambda warm-up* | **fac**=facteur · **lamb**=lambda · **wup**=warm-up | Facteur correctif sur consigne lambda pendant warm-up f(RPM, MAF). Stock 1.000, E85 : 1.03–1.08 basses charges |

---

### Optionnels

**Navigation**

| Statut | Paramètre | Adresse | Guide | Type · Axes · Unité | Conf. |
|---|---|---|---|---|---|
| ⬜ | `ip_fac_lamb_max_fsd_1` | 0x42734 | §08 | Courbe 6 pts · f(λ consigne normalisée) · % | 88% |
| ⬜ | `ip_fac_lamb_max_fsd_2` | 0x42740 | §08 | Courbe 6 pts · f(λ consigne normalisée) · % | 88% |
| ⬜ | `ip_fac_lamb_wup_is` | 0x42788 | §08 | Map 3×4 · RPM×charge(mg/stk) · facteur | 95% |
| ⬜ | `ip_lamb_fl__n` | 0x436A2 | §08 | Courbe 12 pts · f(RPM) · λ | 98% |
| ⬜ | `ip_iga_st_bas_opm_1` | 0x43586 | §07 | Map 6×8 · RPM×TCO(°C) · °CRK | 95% |
| ⬜ | `ip_iga_st_bas_opm_2` | 0x435B6 | §07 | Map 6×8 · RPM×TCO(°C) · °CRK | 95% |
| ⬜ | `c_iga_ini` | 0x44B2A | §07 | Scalaire · °CRK | 95% |
| ⬜ | `ip_flow_max_cps` | 0x4FD54 | §10 | Map 12×12 · RPM×charge(mg/stk) · kg/h | 93% |
| ⬜ | `ip_flow_cps` | 0x48B90 | §10 | Courbe 16 pts · f(dépression hPa) · kg/h | 93% |

**Détail**

| Paramètre | Description XDF (originale) | Décodage du nom | Ce que ça fait |
|---|---|---|---|
| `ip_fac_lamb_max_fsd_1` | *Lambdacontroller maximum threshold for FSD dependent on LAMB_SP* | **fac**=facteur · **lamb**=lambda · **max**=plafond · **fsd**=fuel system deviation · **1**=mode 1 | Plafond STFT autorisé par le contrôleur lambda f(consigne λ). Stock ~30%. Uniquement si DTC fuel trim malgré LTFT OK |
| `ip_fac_lamb_max_fsd_2` | *…mode 2* | **2**=mode 2 | Idem mode 2 — modifier simultanément |
| `ip_fac_lamb_wup_is` | *correction factor for basic lambda warm-up during idle* | **is**=idle speed (ralenti) | Facteur warm-up lambda restreint au ralenti. Uniquement si ralenti instable malgré ip_fac_lamb_wup OK |
| `ip_lamb_fl__n` | *Lambda full load enrichment* | **lamb**=lambda · **fl**=full load · **n**=f(RPM) | Consigne lambda WOT f(RPM). Stock VB67774 déjà à 0.920λ — ne modifier que si sonde large bande hors plage |
| `ip_iga_st_bas_opm_1` | *Basic ignition angle at start at operation mode 1* | **iga**=ignition angle · **st**=start · **bas**=basic · **opm_1**=Valvetronic | Avance allumage de base au démarrage f(RPM, TCO). Si démarrage > 5 tours malgré cranking MFF OK |
| `ip_iga_st_bas_opm_2` | *…operation mode 2* | **opm_2**=papillonné | Même rôle, mode papillonné |
| `c_iga_ini` | *Init value for ignition angle* | **c**=constante · **iga**=ignition angle · **ini**=initial | Avance initiale premier cycle cranking. Stock 6.0°CRK, E85 : +1° à +2°. Dernier recours |
| `ip_flow_max_cps` | *flow setpoint for MAX_PURGE* | **flow**=débit · **max**=maximum · **cps**=canister purge solenoid | Débit maximal purge canister f(RPM, charge). Réduire −10–15% si STFT oscille lors purges |
| `ip_flow_cps` | *FLOW_CPS for fully opened CPS (CPPWM=100%)* | **cps**=canister purge solenoid | Débit canister vanne 100% ouverte f(dépression). Réduire −15% si STFT > ±10% lors purges |

---

### Ne pas modifier

| Paramètre | Adresse | Raison |
|---|---|---|
| `c_fac_mff_ti_stnd_*` (×5 copies) | — | Overflow XDF — passer par `ip_mff_cor_opm_*` à la place |
| `ip_fac_eff_iga_ch_cold_opm_*` | 0x4A444/A4A8 | Retard chauffe catalyseur — E85 produit moins d'EGT, ne pas toucher |
| `ip_mff_lgrd_ast` | 0x4387C | Limiteur de gradient TI post-démarrage — pas un enrichissement |
| `KF_FTRANSVL` | 0x5C5EE | Module BLSHUB = gestion couple/Valvetronic, rien à voir avec le carburant |

---

## Surveillance post-flash (observer, ne pas modifier)

> Après tout reflash ou déconnexion batterie : LTFT repart de 0 — 50–100 km de convergence normaux.

| Paramètre OBD | Seuils normaux E85 | Alarme |
|---|---|---|
| STFT B1/B2 | −10% à +10% | >+15% stable = enrichissement insuffisant |
| LTFT B1/B2 | −5% à +15% (0–200 km), puis −5% à +5% | >+20% ou <−10% stable = problème de base |
| DC injecteur | <80% WOT (<85% limite absolue) | >85% → saturation, lean non détectable |
| Pression rail | 4.8–5.2 bar chaud WOT | <4.5 bar WOT = pompe insuffisante |

---

## Guides

| Document | Contenu |
|---|---|
| [00 — Sommaire](E85/00_SOMMAIRE.md) | Index et navigation |
| [01 — Principes](E85/01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF |
| [02 — Mythes](E85/02_MYTHES.md) | Idées reçues et réalités |
| [03 — Prérequis](E85/03_PREREQUIS.md) | Checklist avant conversion |
| [04 — Enrichissement](E85/04_ENRICHISSEMENT.md) | Facteur de masse carburant E85 (ip_mff_cor_opm ×4) |
| [05 — Démarrage froid](E85/05_DEMARRAGE_FROID.md) | Cranking et enrichissement cold start |
| [06 — Film mural](E85/06_FILM_MURAL.md) | Correction transitoires (wall film ×1.25) |
| [07 — Allumage](E85/07_ALLUMAGE.md) | Avance E60-safe + délai WOT + avance cranking |
| [08 — Lambda](E85/08_LAMBDA.md) | Anti-DTC break-in, warm-up lambda, richesse WOT |
| [09 — Transitoire](E85/09_TRANSITOIRE.md) | Transitoire pleine charge (rien à modifier) |
| [10 — EVAP](E85/10_EVAP.md) | Purge canister — oscillations STFT |
| [11 — Surveillance](E85/11_SURVEILLANCE.md) | Indicateurs OBD à lire pendant la mise au point |
| [12 — Plan de test](E85/12_PLAN_TEST.md) | Résumé paramètres, plan 5 phases, diagnostic, avertissements |
