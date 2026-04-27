# Reprog ECU — MSV70 E85

Conversion éthanol sur **Siemens MSV70** (BMW N52B30, SW 9PPL921S — dump VB67774).  
État : **FinalV1** — paramètres restants à modifier.

---

## Tableau de référence — Paramètres E85

Toutes les certitudes sont basées sur : description XDF anglaise + catégorie XDF + décodage du nom + cohérence avec les valeurs stock lues dans le bin.

### Obligatoires

| Statut | Paramètre | Adresse | Guide | Type · Unité | Décodage du nom | Description XDF | Ce que ça fait | Conf. |
|---|---|---|---|---|---|---|---|---|
| ✅ | `ip_mff_cor_opm_1_1` | 0x4E3D4 | §04 | Map 12×16 · facteur | **ip**=table · **mff**=mass fuel flow · **cor**=correction · **opm**=operating mode · **1_1**=mode1 banque1 | *Injected fuel mass correction for operating mode 1 (bank 1)* | Multiplicateur masse carburant f(RPM, charge), mode Valvetronic, banque 1. Stock 1.016, E85 : 1.473 | 95% |
| ✅ | `ip_mff_cor_opm_1_2` | 0x4E554 | §04 | Map 12×16 · facteur | **1_2**=mode1 banque2 | *…bank 2* | Même rôle, banque 2 | 95% |
| ✅ | `ip_mff_cor_opm_2_1` | 0x4E6D4 | §04 | Map 10×12 · facteur | **2**=mode papillonné (GD) | *…operating mode 2 (bank 1)* | Même rôle, mode papillonné, banque 1 | 95% |
| ✅ | `ip_mff_cor_opm_2_2` | 0x4E7C4 | §04 | Map 10×12 · facteur | **2_2**=mode papillonné banque2 | *…operating mode 2 (bank 2)* | Même rôle, mode papillonné, banque 2 | 95% |
| ✅ | `ip_mff_cst_opm_1` | 0x437DC | §05 | Map 3×8 · mg/stk | **cst**=cold start · **opm_1**=Valvetronic | *Basic value for cranking injection at operation mode 1* | Masse injectée pendant cranking f(TCO, RPM démarreur), Valvetronic. E85 : ×1.35–2.00 selon TCO | 90% |
| ✅ | `ip_mff_cst_opm_2` | 0x4380C | §05 | Map 3×8 · mg/stk | **opm_2**=papillonné | *…operation mode 2* | Même rôle, mode papillonné | 90% |
| ✅ | `c_tco_n_mff_cst` | 0x44F2F | §05 | Scalaire · °C | **c**=constante · **tco**=temp. liquide · **n**=seuil · **mff_cst**=cold start MFF | *Cool temperature constant* | Seuil TCO d'activation cranking enrichi. Stock 17.25°C, E85 : 25°C | 90% |
| ✅ | `ip_ti_cast_opm_1` | 0x43FA4 | §05 | Map 2×10 · facteur | **ti**=injection time · **cast**=cold/catalyst after start · **opm_1**=Valvetronic | *Initialization value of post start enrichment factor at operation mode 1* | Facteur d'enrichissement post-démarrage f(TCO liquide, TCO admission). E85 : ×1.55–1.65 froid | 80% |
| ✅ | `ip_ti_cast_opm_2` | 0x43FB8 | §05 | Map 2×10 · facteur | **opm_2**=papillonné | *…operation mode 2* | Même rôle, mode papillonné | 80% |
| ✅ | `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | §06 | Map 8×8 · facteur | **ti**=injection time · **tco**=f(TCO) · **pos**=montée charge · **fast**=rapide · **wf**=wall film · **opm_1**=Valvetronic | *fast positive wall film factor* | Film mural positif rapide f(TCO, RPM), Valvetronic. E85 : ×1.25 sous 70°C | 90% |
| ✅ | `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | §06 | Map 8×8 · facteur | **opm_2**=papillonné | *fast positive wall film factor* | Même rôle, mode papillonné | 90% |
| ✅ | `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | §06 | Map 8×8 · facteur | **slow**=composante lente d'accumulation | *total positive wall film factor* | Film mural positif lent f(TCO, RPM), Valvetronic. E85 : ×1.25 sous 70°C | 88% |
| ✅ | `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | §06 | Map 8×8 · facteur | **opm_2**=papillonné | *total positive wall film factor* | Même rôle, mode papillonné | 88% |
| ✅ | `ip_iga_bas_max_knk__n__maf` | 0x4323A | §07 | Map 8×8 · °CRK | **iga**=ignition angle · **bas**=base · **max**=plafond · **knk**=knock · **n**=RPM · **maf**=débit air | *Maximum value for spark retard* | Plafond avance anti-cliquetis f(RPM, MAF). ⚠️ C'est un plafond — lever ne fait rien si le modèle couple demande moins | 90% |
| ✅ | `c_t_ti_dly_fl_1` | 0x44EC4 | §07 | Scalaire · s | **c**=constante · **t**=temps · **ti**=injection time · **dly**=delay · **fl**=full load · **1**=MT copie1 | *Delay time Full load* | Délai activation enrichissement WOT BM. Stock 200 ms, E85 : 0 ms | 90% |
| ✅ | `c_t_ti_dly_fl_2` | 0x44EC6 | §07 | Scalaire · s | **2**=copie 2 | *Delay time Full load* | Copie 2 — modifier identiquement | 90% |
| ✅ | `c_t_ti_dly_fl_at_1` | 0x44EC8 | §07 | Scalaire · s | **at**=automatic transmission | *Delay time Full load AT* | Idem boîte automatique | 90% |
| ✅ | `c_t_ti_dly_fl_at_2` | 0x44ECA | §07 | Scalaire · s | **at_2**=BVA copie 2 | *Delay time Full load AT* | Copie 2 BVA | 90% |
| ✅ | `c_lamb_delta_i_max_lam_adj` | 0x47F5E | §08 | Scalaire · λ | **c**=constante · **lamb**=lambda · **delta**=variation · **i**=intégrateur · **max**=plafond · **lam_adj**=lambda adjustment | *upper limit of trim control I share* | Plafond d'accumulation LTFT (intégrateur). Stock 0.050λ (5%), E85 break-in : 0.25λ | 95% |
| ✅ | `ip_fac_lamb_wup` | 0x42764 | §08 | Map 6×6 · facteur | **fac**=facteur · **lamb**=lambda · **wup**=warm-up | *correction factor for basic lambda warm-up* | Facteur correctif sur consigne lambda pendant warm-up f(MAF, RPM). Stock 1.000, E85 : 1.03–1.08 basses charges | 90% |

### Optionnels

| Statut | Paramètre | Adresse | Guide | Type · Unité | Décodage du nom | Description XDF | Ce que ça fait | Conf. |
|---|---|---|---|---|---|---|---|---|
| ⬜ | `ip_fac_lamb_max_fsd_1` | 0x42734 | §08 | Courbe 6 pts · % | **fac**=facteur · **lamb**=lambda · **max**=plafond · **fsd**=fuel system deviation · **1**=mode 1 | *Lambdacontroller maximum threshold for FSD dependent on LAMB_SP* | Plafond STFT autorisé par le contrôleur lambda. Stock ~30%. Uniquement si DTC fuel trim malgré LTFT OK | 85% |
| ⬜ | `ip_fac_lamb_max_fsd_2` | 0x42740 | §08 | Courbe 6 pts · % | **2**=mode 2 | *…mode 2* | Idem mode 2 — modifier simultanément | 85% |
| ⬜ | `ip_fac_lamb_wup_is` | 0x42788 | §08 | Map 3×4 · facteur | **is**=idle speed (ralenti) | *correction factor for basic lambda warm-up during idle* | Facteur warm-up lambda restreint au ralenti. Uniquement si ralenti instable warm-up malgré ip_fac_lamb_wup OK | 90% |
| ⬜ | `ip_lamb_fl__n` | 0x436A2 | §08 | Courbe 12 pts · λ | **lamb**=lambda · **fl**=full load · **n**=f(RPM) | *Lambda full load enrichment* | Consigne lambda WOT f(RPM). Stock VB67774 déjà à 0.920λ — ne modifier que si sonde large bande hors plage | 92% |
| ⬜ | `ip_iga_st_bas_opm_1` | 0x43586 | §07 | Map 6×8 · °CRK | **iga**=ignition angle · **st**=start · **bas**=basic · **opm_1**=Valvetronic | *Basic ignition angle at start at operation mode 1* | Avance allumage de base pendant démarrage f(TCO, RPM). Si démarrage > 5 tours malgré cranking MFF OK | 90% |
| ⬜ | `ip_iga_st_bas_opm_2` | 0x435B6 | §07 | Map 6×8 · °CRK | **opm_2**=papillonné | *…operation mode 2* | Même rôle, mode papillonné | 90% |
| ⬜ | `c_iga_ini` | 0x44B2A | §07 | Scalaire · °CRK | **c**=constante · **iga**=ignition angle · **ini**=initial | *Init value for ignition angle* | Avance initiale premier cycle cranking. Stock 6.0°CRK, E85 : +1° à +2°. Dernier recours | 90% |
| ⬜ | `ip_flow_max_cps` | 0x4FD54 | §10 | Map 12×12 · kg/h | **flow**=débit · **max**=maximum · **cps**=canister purge solenoid | *flow setpoint for MAX_PURGE* | Débit maximal purge canister f(charge, RPM). Réduire −10–15% si STFT oscille lors purges | 90% |
| ⬜ | `ip_flow_cps` | 0x48B90 | §10 | Courbe 16 pts · kg/h | **cps**=canister purge solenoid | *FLOW_CPS for fully opened CPS (CPPWM=100%)* | Débit canister vanne 100% ouverte f(dépression hPa). Réduire −15% si STFT > ±10% lors purges | 90% |

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
