# Reprog ECU — MSV70 E85

Conversion éthanol sur **Siemens MSV70** (BMW N52B30, SW 9PPL921S — dump VB67774).

---

## 🔴 Obligatoire — Carburant de base

Sans ces paramètres, le moteur tourne riche/pauvre de façon incontrôlée et génère des DTCs dès le premier démarrage.

| Paramètre(s) | Stock VB67774 | E85 cible | Notes |
|---|---|---|---|
| `ip_mff_cor_opm_1_1` `_1_2` `_2_1` `_2_2` | 1.016 flat — raw 32768, 192 cellules | 1.473 flat — raw 47516 | Facteur MFF principal ×4 tables. ⚠️ DC injecteur stock ~90% à 6000 rpm WOT → saturation au-delà de ~4900 rpm. Injecteurs ≥380 cc/min requis pour WOT sans limite RPM. |
| `ip_fac_lamb_max_fsd_1` `_2` | 30% à λ=0.85–0.95 — raw 42599 | Break-in 0–500 km : 120–125% (raw ~72082–73722). Stable : 115–120% | Plafond STFT. Lever pour éviter les DTCs pendant la convergence LTFT. Resserrer après stabilisation. |
| `c_lamb_delta_i_max_lam_adj` | 0.050 λ — raw 819 | Break-in 0–500 km : 0.25–0.30 λ. Stable : 0.20 λ | Plafond LTFT. Après tout reflash ou déco batterie : LTFT repart de 0 — 50–100 km de convergence normaux avant de juger la calibration. |

---

## 🔴 Obligatoire — Démarrage froid

| Paramètre(s) | Stock VB67774 | E85 cible | Notes |
|---|---|---|---|
| `ip_mff_cst_opm_1` `_2` | 448mg@−30°C → 50mg@90°C (80rpm) ; 320mg@−30°C → 39mg@90°C (320rpm) | ×2.20@−30°C, ×2.00@−20°C, ×1.80@0°C, ×1.60@10°C, ×1.35@30°C, ×1.00@90°C | Masse cranking f(TCO, RPM) [3×8]. Modifier les 2 modes identiquement. |
| `c_tco_n_mff_cst` | 17.25°C — raw 87 | E80 : 26°C — raw 99. E100 : 32°C — raw 107 | Seuil TCO d'activation du cranking enrichi. Équation 0.75×raw−48. |
| `ip_mff_lgrd_ast` | 18.9mg@−30°C → 4.5mg@85°C | ×1.65@−30°C → ×1.00@85°C | Masse after-start f(TCO) — ~3 s post-allumage, phase 0→800 rpm. |
| `ip_fac_ti_tco_wup_opm_1` `_2` | ~1.000@−30°C, ~0.969@17°C, ~1.000@70°C+ | 1.40@−30°C, 1.35@−10°C, 1.25@0°C, 1.15@20°C, 1.08@40°C, 1.02@60°C, 1.00@80°C+ | Facteur TI warm-up f(TCO). Stock légèrement sous 1.0 entre 10 et 30°C — remplacer la courbe entière. |
| `ip_ti_tco_wup_opm_1` `_2` | Voir bin | ×1.15 à ×1.35 pour TCO < 50°C | Masse warm-up f(TCO, RPM). |
| `ip_ti_wup_opm_1` `_2` | Voir bin | ×1.20 à ×1.35 pour TCO < 50°C | TI warm-up absolu f(TCO). |
| `ip_fac_lamb_wup` | 1.000 flat — raw 128, 36 cellules [6×6 mg/stk × RPM] | 1.08@704rpm/65mg, 1.05@704rpm/100mg, 1.05@1216rpm/65mg, 1.03@1216rpm/100mg, 1.00@>300mg ou >2496rpm | Lambda warm-up f(charge, RPM). Stock entièrement neutre. |
| `ip_fac_lamb_wup_is` | 1.000 flat — raw 128, 12 cellules [3×4] | +5%@704rpm/65mg, +3%@704rpm/100mg, +3%@1216rpm/65mg, +2%@1216rpm/100mg | Lambda warm-up mode IS (bas régimes uniquement). |
| `c_t_ti_dly_fl_1` `_2` | 200ms — raw 20 (les 2 copies) | 0ms — raw 0 | Délai enrichissement pleine charge. Mettre raw=0 sur les 2 copies (boîte MT). |

---

## 🟠 Recommandé — Film mural et transitoires

| Paramètre(s) | Stock VB67774 | E85 cible | Notes |
|---|---|---|---|
| `ip_ti_tco_pos_fast_wf_opm_1` `_2` | 22mg@608rpm/−30°C → 4.5mg@608rpm/115°C ; 72.5mg@5600rpm/−30°C → 31mg@5600rpm/115°C | ×1.25 sous 70°C | Film mural montée rapide f(TCO, RPM) [8×8]. Première approximation — affiner si à-coups observés. |
| `ip_ti_tco_pos_slow_wf_opm_1` `_2` | Voir bin | ×1.25 sous 70°C | Film mural montée lente. Même logique que fast. |
| `id_fac_mff_tco_pos_wf` | 0.061@−30°C → 0.029@90°C | ×1.20–1.25 entre 20°C et 70°C | Facteur MFF film dynamique accélérations f(TCO). |
| `id_fac_mff_tco_neg_wf` | Voir bin | ×1.15 sous 60°C | Facteur MFF film dynamique décélérations f(TCO). L'éthanol s'évapore plus lentement — film résiduel plus important. |
| `id_mff_inc_wf` | 0.106mg@704rpm → 0.021mg@3000rpm+ | ×1.20 entre 20°C et 60°C | Incrément MFF reconstitution film en accélération f(RPM). |
| `id_mff_dec_wf` | Voir bin | ×1.20 entre 20°C et 60°C | Décrément MFF récupéré du film en décélération. |
| `ip_fac_mff_map_wf` | 0.121@124hPa, 0.102@324hPa, 0.062@724hPa, 0.035@924hPa | ×1.15 uniforme | Facteur film mural principal f(MAP). |
| `ip_fac_ti_temp_cor` | 1.000@−20°C→50°C ; 1.013@60°C, 1.027@70°C, 1.039@80°C+ | 1.20@−20°C, 1.18@0°C, 1.15@20°C, 1.10@40°C, 1.05@60°C, 1.00@80°C+ | Correction TI f(TCO) [12 pts]. Stock enrichit légèrement à chaud (compensation carburant chaud) — remplacer entièrement la courbe. |
| `ip_lamb_bas_4` | 0.997λ flat — raw 16351, 64 cellules [8×8 mg/stk × RPM] | 0.92–0.95λ en zone >200mg/stk et >3000rpm. Zone basse charge/bas régime : 0.997λ inchangé | Lambda cible WOT. Valider à la sonde large bande. |

---

## ⚪ Optionnel — Allumage

Modifier uniquement après validation complète de l'injection (ip_mff_cor, warm-up, film mural) et confirmation absence de cliquetis sur E85.

| Paramètre(s) | Stock VB67774 | E85 cible | Notes |
|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | −15° à +43.5°CRK — table 8×8 f(charge, RPM) | +2° à +4° | Plafond avance anti-cliquetis. ⚠️ PLAFOND ≠ AVANCE EFFECTIVE : sur MSV70, avance = min(modèle couple, plafond). Lever le plafond n'a d'effet que si le modèle couple était déjà limité par ce plafond. Vérifier en log. |
| `ip_iga_st_bas_opm_1` `_2` | −5.6° à +14.6°CRK — table 6×8 f(TCO, RPM 80–920) | +2° à +3° à TCO < 0°C | Table d'allumage démarrage — axes : TCO × [80–920 rpm]. Ce n'est PAS la table principale en charge. `ip_iga_bas_knk` (avance principale) est absent du XDF 9PPL921S. |
| `c_iga_ini` | 6.0°CRK — raw 111 | 7.0°CRK (raw 114) à 8.0°CRK (raw 116) | Avance cranking premiers cycles. Optionnel — uniquement si démarrage froid difficile malgré ip_mff_cst_opm correctement calibré. |

---

## Si changement d'injecteurs

> Stock injecteurs 13537531634 (~237 cc/min chaud) : `c_fac_mff_ti_stnd_1` raw=56567 → 0.3394 ms/mg.
> ⚠️ Pour E85 WOT sans limitation RPM, il faut au minimum **380 cc/min** (DC stock@6000rpm ≈ 70%, ×1.473 → 103% → saturation).
> Ne modifier les paramètres ci-dessous que si les injecteurs sont remplacés.

| Paramètre(s) | Stock VB67774 | Après remplacement | Notes |
|---|---|---|---|
| `c_fac_mff_ti_stnd_1` `_2` `_mon` | raw 56567 → 0.3394 ms/mg (éq. 0.000006×raw) | Recalculer depuis débit réel injecteur chaud | **5 copies à modifier simultanément.** Oublier `_mon` → DTC cohérence injection immédiat. |
| `c_fac_mff_ti_stnd[0]` `[1]` | raw 28284 → 0.3394 ms/mg (éq. 0.000012×raw — raw différent) | raw = moitié du raw des copies `_1`/`_2` | Copies SOI/EOI — équation différente (×2). Adresses 0x045AAC et 0x045AAE. |
| `ip_ti_min` | 1.300ms flat — raw 325 (valeur simplifiée sur ce bin) | Recalibrer depuis fiche constructeur | Dead time injecteur f(tension batterie). Stock non calibré finement — obligatoire si injecteurs remplacés. |

---

## Surveillance post-flash (observer, ne pas modifier)

> Après tout reflash ou déconnexion batterie : LTFT repart de 0 — le moteur tourne ~35% riche pendant 50–100 km. Ne pas juger la calibration avant convergence complète.

| Paramètre OBD | Seuils normaux E85 | Alarme |
|---|---|---|
| STFT B1/B2 | −10% à +10% | >+15% stable = enrichissement insuffisant |
| LTFT B1/B2 | −5% à +15% (0–200 km), puis −5% à +5% | >+20% ou <−10% stable = problème de base |
| DC injecteur | <80% WOT (<85% limite absolue) | >85% → saturation, lean non détectable |
| Pression rail `0x580A` | 4.8–5.2 bar chaud WOT | <4.5 bar WOT = pompe insuffisante |

---

## Guides

| Document | Contenu |
|---|---|
| [01 — Principes](E85/01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF, pourquoi autant de paramètres |
| [02 — Mythes](E85/02_MYTHES.md) | 8 mythes courants |
| [03 — Prérequis](E85/03_PREREQUIS.md) | Checklist mécanique et logicielle avant conversion |
| [18 — Plan de test](E85/18_PLAN_TEST.md) | Protocole de validation en phases |
| [19 — Diagnostic](E85/19_DIAGNOSTIC.md) | Symptôme → cause → solution |
| [16 — Surveillance](E85/16_SURVEILLANCE.md) | Protocole de logging et validation |
| [20 — Avertissements](E85/20_AVERTISSEMENTS.md) | Pompe, filtre, bougies, hiver |
