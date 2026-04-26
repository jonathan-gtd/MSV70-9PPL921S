# Reprog ECU — MSV70 E85

Conversion éthanol sur **Siemens MSV70** (BMW N52B30, SW 9PPL921S — dump VB67774).

## Fondamentaux

| Document | Contenu |
|---|---|
| [01 — Principes](E85/01_PRINCIPES.md) | Physique de l'éthanol, architecture MFF, pourquoi autant de paramètres |
| [02 — Mythes](E85/02_MYTHES.md) | 8 mythes courants — démontés |
| [03 — Prérequis](E85/03_PREREQUIS.md) | Checklist mécanique et logicielle avant conversion |

---

## Paramètres à modifier

### 🔴 Obligatoire — sans ça le moteur ne tourne pas correctement

| Paramètre(s) | Rôle | Réf. | Notes critiques |
|---|---|---|---|
| `ip_mff_cor_opm_1_1` `_1_2` `_2_1` `_2_2` | Facteur carburant principal | [04](E85/04_INJECTEURS.md) | ×1.473 flat (E83) sur 192/120 cellules. **⚠️ DC injecteur ~90% @ 6000 rpm → saturation WOT >4900 rpm avec injecteurs stock.** Vérifier les temps d'injection en log avant usage WOT. |
| `ip_mff_cst_opm_1/2` | Cranking froid f(TCO, RPM) | [05](E85/05_DEMARRAGE_FROID.md) | Facteurs réels bin : ×2.00@−30°C / ×1.65@−10°C / ×1.35@17°C / ×1.05@90°C |
| `c_tco_n_mff_cst` | Seuil activation cranking enrichi | [05](E85/05_DEMARRAGE_FROID.md) | Stock raw=87 → 17.25°C. E80 → raw=99 → 26°C |
| `ip_mff_lgrd_ast` | After-start (0→800 rpm, ~3s) | [05](E85/05_DEMARRAGE_FROID.md) | ×1.65@−30°C → ×1.00@85°C |
| `ip_fac_ti_tco_wup_opm_1/2` | Facteur warm-up TI f(TCO) | [05](E85/05_DEMARRAGE_FROID.md) | Stock ≈ 1.000. Cible : 1.40@−30°C → 1.25@0°C → 1.00@80°C |
| `ip_ti_tco_wup_opm_1/2` | Masse warm-up f(TCO, RPM) | [05](E85/05_DEMARRAGE_FROID.md) | ×1.15 à ×1.35 pour TCO < 50°C |
| `ip_ti_wup_opm_1/2` | TI warm-up absolu f(TCO) | [05](E85/05_DEMARRAGE_FROID.md) | ×1.20 à ×1.35 pour TCO < 50°C |
| `ip_fac_lamb_wup` | Lambda warm-up f(TCO, RPM) — tous régimes | [10](E85/10_WARMUP_LAMBDA.md) | Stock = 1.000. E85 : +8% à 704rpm/65mg → 0% au-dessus 300mg ou 2496rpm |
| `ip_fac_lamb_wup_is` | Lambda warm-up ralenti uniquement | [10](E85/10_WARMUP_LAMBDA.md) | +5%@704rpm/65mg, +3%@704rpm/100mg, +3%@1216rpm/65mg, +2%@1216rpm/100mg |
| `ip_fac_lamb_max_fsd_1/2` | Limite haute correction STFT | [09](E85/09_LAMBDA.md) | Stock : 35–100%. E85 break-in : 120–125% |
| `c_lamb_delta_i_max_lam_adj` | Limite LTFT | [09](E85/09_LAMBDA.md) | Stock : **0.050 λ (±5%)**. E85 break-in : 0.25–0.30 λ. Réduire à 0.20 après 500 km. |

### 🟠 Recommandé — qualité de roulage

| Paramètre(s) | Rôle | Réf. | Notes |
|---|---|---|---|
| `ip_lamb_bas_4` | Lambda cible WOT f(charge, RPM) | [09](E85/09_LAMBDA.md) | Stock = 0.997 λ partout (stœchio). Réduire à 0.92–0.95 en zone >200 mg/stk et >3000 rpm pour enrichissement protecteur. |
| `ip_ti_tco_pos_fast_wf_opm_1/2` | Film mural — montée de charge rapide | [06](E85/06_FILM_MURAL.md) | ×1.25 première approximation. À affiner >70°C (surplus paroi quasi nul au-delà). |
| `ip_ti_tco_pos_slow_wf_opm_1/2` | Film mural — montée de charge lente | [06](E85/06_FILM_MURAL.md) | Même logique que fast |
| `id_fac_mff_tco_pos_wf` | Film mural dynamique — accélérations | [06](E85/06_FILM_MURAL.md) | +20–25% entre 20°C et 70°C |
| `id_fac_mff_tco_neg_wf` | Film mural dynamique — décélérations | [06](E85/06_FILM_MURAL.md) | +15% sous 60°C |
| `id_mff_inc_wf` / `id_mff_dec_wf` | Incrément/décrément film mural | [06](E85/06_FILM_MURAL.md) | +20% entre 20°C et 60°C |
| `ip_fac_mff_map_wf` | Facteur film mural principal | [06](E85/06_FILM_MURAL.md) | +15% sous 60°C |
| `ip_fac_ti_temp_cor` | Correction TI f(TCO) | [06](E85/06_FILM_MURAL.md) | +20%@−10°C / +18%@0°C / +15%@20°C / +10%@40°C / +5%@60°C / 1.00@80°C |
| `c_t_ti_dly_fl_1/2` | Délai enrichissement WOT | [08](E85/08_DELAI_WOT.md) | Stock 200 ms → E85 : 0 ms |
| `ip_lamb_fl__n` | Cible lambda WOT f(RPM) | [09](E85/09_LAMBDA.md) | Stock VB67774 : déjà 0.920 λ (608–4800 rpm), 0.901 à 5504, 0.871 à 6496. Inchangé sur ce bin. À vérifier à la sonde large bande. |

### ⚪ Optionnel — gain de performance (après validation injection)

| Paramètre(s) | Rôle | Réf. | Notes |
|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | Plafond anti-cliquetis f(charge, RPM) | [07](E85/07_AVANCE.md) | **⚠️ Plafond ≠ avance effective.** Sur MSV70, avance = min(modèle couple, plafond). Si le modèle couple demande déjà moins, lever le plafond n'a aucun effet. `ip_iga_bas_knk` (table de base avance) absent du XDF actuel. |
| `ip_iga_st_bas_opm_1/2` | Avance de base f(charge, RPM) | [07](E85/07_AVANCE.md) | +2° à +6° en zone haute charge / haut régime. Uniquement après validation complète de l'injection. |
| `c_iga_ini` | Avance cranking | [11](E85/11_AVANCE_CRANKING.md) | Optionnel — +1° (raw 114) à +2° (raw 116) si démarrage difficile malgré calibration injection correcte. |

---

### Si changement d'injecteurs

> Injecteurs stock : Bosch EV14 BMW 13537531634, ~290 cc/min froid / **~237 cc/min chaud** (d'après `c_fac_mff_ti_stnd` = 0.3394 ms/mg).
> **Ne modifier les paramètres ci-dessous que si les injecteurs sont remplacés.**
> Pour E85 WOT sans limitation RPM, il faut au moins **380 cc/min** (DC stock@6000rpm ≈ 70%, ×1.473 = 103% → saturation).

| Paramètre(s) | Rôle | Notes |
|---|---|---|
| `c_fac_mff_ti_stnd_1` / `_2` / `[0]` / `[1]` / `_mon` | Facteur MFF→TI (5 copies) | Stock = 0.3394 ms/mg. Nouvelles valeurs : calculer depuis débit réel injecteur. **Toutes les 5 copies en même temps.** Oublier `_mon` → DTC cohérence injection. |
| `ip_ti_min` | Dead time f(tension batterie) | Recalibrer depuis fiche constructeur du nouvel injecteur. |

---

## Surveillance post-flash (ne pas modifier — observer)

> ⚠️ **Reset LTFT** : après tout reflash ou déconnexion batterie, les LTFT repartent de 0. Le moteur tourne riche (~35% de surplus via `ip_mff_cor` sans compensation) pendant 50–100 km — normal. Ne pas juger la calibration avant convergence complète.
> Noter les LTFT stock **avant** conversion : s'ils sont déjà à +5–8% sur essence, un problème préexistant (injecteur, MAF, fuite) sera masqué par la reprog.

| Paramètre OBD | Seuils normaux E85 | Alarme |
|---|---|---|
| STFT B1/B2 | −10% à +10% | >+15% = enrichissement insuffisant |
| LTFT multiplicatif B1/B2 | −5% à +15% (break-in 0–200 km) puis −5% à +5% | >+20% ou <−10% stable = problème base |
| Temps injection cyl 1–6 (`0x5A42`–`0x5A47`) | Cohérents entre cylindres (<5% d'écart) | Écart >15% = injecteur défaillant |
| DC injecteur calculé | <80% en WOT (<85% limite absolue) | >85% → saturation, lean non détectable |
| Pression rail (`0x580A`) | 4.8–5.2 bar à chaud WOT | <4.5 bar WOT = pompe insuffisante |

---

## Guides complémentaires

| Document | Contenu |
|---|---|
| [12 — Transitoire](E85/12_TRANSITOIRE.md) | Enrichissements WOT transitoires (KF_FTRANSVL) |
| [13 — LTFT](E85/13_LTFT.md) | Surveillance et convergence LTFT — procédure |
| [14 — EVAP](E85/14_EVAP.md) | Canister purge — ne pas modifier avant 500 km |
| [15 — Chauffe cat](E85/15_CHAUFFE_CAT.md) | Stratégie warm-up catalyseur — pas de modification |
| [16 — Surveillance](E85/16_SURVEILLANCE.md) | Protocole de logging et validation |
| [17 — Résumé](E85/17_RESUME.md) | Tableau récapitulatif complet |
| [18 — Plan de test](E85/18_PLAN_TEST.md) | Protocole de validation en phases |
| [19 — Diagnostic](E85/19_DIAGNOSTIC.md) | Symptôme → cause → solution |
| [20 — Avertissements](E85/20_AVERTISSEMENTS.md) | Pompe, filtre, bougies, hiver |
| [21 — Vérification](E85/21_VERIFICATION.md) | Niveaux de confiance par section |
| [22 — Conclusion](E85/22_CONCLUSION.md) | Responsabilité et contexte légal |
