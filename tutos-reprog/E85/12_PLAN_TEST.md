# Plan de test, diagnostic et avertissements

---

## Résumé des paramètres impactés

> **Véhicule ciblé :** BMW E9x 330i — N52B30 — Siemens MSV70 (SW 9PPL921S)  
> **Injecteurs stock :** Bosch EV14 13537531634 — pression rail nominale 5000 hPa  
> **Fichier de base :** VB67774_921S_Full.bin  
> **Stratégie :** Facteur injecteur → E85 (×1.473) / Cranking → E70 / Avance → E60 / Film mural → ×1.25

| Statut | Paramètre(s) | Adresse | Fichier | Action |
|---|---|---|---|---|
| **✅ MODIFIER** | `ip_mff_cor_opm_1_1` | 0x4E3D4 | §04 | 1.016 → **1.473** |
| **✅ MODIFIER** | `ip_mff_cor_opm_1_2` | 0x4E554 | §04 | 1.016 → **1.473** |
| **✅ MODIFIER** | `ip_mff_cor_opm_2_1` | 0x4E6D4 | §04 | 1.016 → **1.473** |
| **✅ MODIFIER** | `ip_mff_cor_opm_2_2` | 0x4E7C4 | §04 | 1.016 → **1.473** |
| **✅ MODIFIER** | `c_tco_n_mff_cst` | 0x44F2F | §05 | 17.25 → **25.00 °C** |
| **✅ MODIFIER** | `ip_mff_cst_opm_1` | 0x437DC | §05 | Table 3×8 : ×1.35–2.00 selon TCO |
| **✅ MODIFIER** | `ip_mff_cst_opm_2` | 0x4380C | §05 | Idem — mode papillonné |
| **✅ MODIFIER** | `ip_fac_lamb_wup` | 0x42764 | §08 | 1.000 → **1.03–1.08** basses charges |
| **✅ MODIFIER** | `ip_iga_bas_max_knk__n__maf` | 0x4323A | §07 | +0 à +2.5° haute charge |
| **✅ MODIFIER** | `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | §06 | ×**1.25** |
| **✅ MODIFIER** | `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | §06 | ×**1.25** |
| **✅ MODIFIER** | `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | §06 | ×**1.25** |
| **✅ MODIFIER** | `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | §06 | ×**1.25** |
| **✅ MODIFIER** | `c_lamb_delta_i_max_lam_adj` | (voir XDF) | §08 | 0.050λ → **0.25λ** break-in |
| ⬜ OPTIONNEL | `ip_fac_lamb_max_fsd_1/2` | (voir XDF) | §08 | cellules 3/4 : 30% → 40% — si DTC fuel trim malgré LTFT OK |
| **✅ MODIFIER** | `c_t_ti_dly_fl_1` | 0x44EC4 | §07 | 200 ms → **0 ms** (MT) |
| **✅ MODIFIER** | `c_t_ti_dly_fl_2` | 0x44EC6 | §07 | 200 ms → **0 ms** (MT) |
| ⬜ OPTIONNEL | `ip_lamb_fl__n` | 0x436A2 | §08 | Stock λ 0.920 suffisant. Dé-enrichir à 0.940–0.950 pour +puissance |
| ⬜ OPTIONNEL | `ip_fac_lamb_wup_is` | 0x42788 | §08 | 1.000 → 1.02–1.05 si ralenti instable warm-up |
| ⬜ OPTIONNEL | `ip_iga_st_bas_opm_1/2` | 0x43586/B6 | §07 | +1° colonnes TCO ≤ 0°C si démarrage > 5 tours |
| ⬜ OPTIONNEL | `c_iga_ini` | 0x44B2A | §07 | +1° à +2° si démarrage > 5 tours (dernier recours) |
| ⬜ OPTIONNEL | `KF_FTRANSVL` | 0x5C5EE | §09 | +10% zone mi-charge si trou exclusivement en kickdown brutal |
| ⬜ OPTIONNEL | `ip_flow_max_cps` | (voir XDF) | §10 | −10 à −15% si STFT > ±15% lors purge |
| ⬜ OPTIONNEL | `ip_flow_cps` | (voir XDF) | §10 | −15% si STFT > ±10% lors purge |
| 👁 SURVEILLER | `c_fac_max/min_*_rng_lam_ad` | 0x47F4C–52 | §08 | Limites LTFT −8% / +12% — ne pas modifier |
| ⛔ NE PAS MODIFIER | `c_fac_mff_ti_stnd` (×5 copies) | — | §04 | Overflow XDF — utiliser `ip_mff_cor_opm_*` |
| ⛔ NE PAS MODIFIER | `ip_fac_eff_iga_ch_cold_opm_*` | 0x4A444/A4A8 | — | Retard chauffe catalyseur — E85 produit moins d'EGT |

**Ordre d'application recommandé :**

| Étape | Paramètre(s) clé(s) | Priorité |
|---|---|---|
| **1** | `ip_mff_cor_opm_*` (×4) | **CRITIQUE — base de tout** |
| **2** | `ip_mff_cst_opm_1/2` + `c_tco_n_mff_cst` + `ip_fac_lamb_wup` | **CRITIQUE — démarrage froid** |
| **3** | `ip_fac_lamb_max_fsd_1/2` + `c_lamb_delta_i_max_lam_adj` | **CRITIQUE — anti-DTC break-in** |
| **4** | `ip_ti_tco_pos_slow/fast_wf_opm_1/2` (×4) | Important — transitions lisses |
| **5** | `ip_iga_bas_max_knk__n__maf` + `c_t_ti_dly_fl_1/2` | Important — avance + WOT |

---

## Plan de test progressif

### Phase 0 — Préparation (avant toute modification)

```
Checklist matériel :
  ✅ Scanner OBD2 avec log (ISTA, INPA, ou Torque Pro + plugin BMW)
  ✅ Sonde lambda large bande recommandée (optionnelle mais idéale)
  ✅ Plein de carburant E85 dans le réservoir

Réinitialisation obligatoire avant le premier démarrage E85 :
  Via ISTA-D : Service → Fonctions de service → Adaptation du mélange → Réinitialiser
  Raison : les LTFT calculés sur essence (~0%) vont biaiser les STFT E85
            dans les premières minutes. Partir à zéro = lecture fiable dès le premier allumage.

Baseline à enregistrer (sur essence, avant le plein E85) :
  - STFT/LTFT au ralenti (doivent être proches de 0%)
  - Comportement démarrage à froid (si possible)
```

### Phase 1 — Application des paramètres injecteurs (Jour 1)

**Modifications :** ip_mff_cor_opm_* → raw 47 407 (1.473) + c_tco_n_mff_cst → 25°C + FSD/LTFT élargis

**Test immédiat :**
1. Démarrer moteur (si tiède ou chaud)
2. Attendre ralenti stable (1–2 min)
3. Lire STFT → doivent être entre −10% et +10%
4. Si STFT > +12% : augmenter `ip_mff_cor_opm` (toutes les 4 maps) de +3% supplémentaire
5. Rouler 20 km, vérifier stabilisation LTFT

**Critère de validation Phase 1 :**
- STFT au ralenti chaud : −5% à +5%
- LTFT après 15 min : −8% à +8%

### Phase 2 — Validation démarrage froid (Jour 2)

**Conditions :** Moteur froid (nuit dehors, < 15°C)

1. Démarrage à froid sans pédale :
   - Doit démarrer en ≤ 4 tours
   - Si > 5 tours → augmenter ip_mff_cst_opm_1 de +15%

2. Ralenti post-démarrage (30–60 sec) :
   - Doit être stable à 600–1000 tr/min
   - STFT entre −10% et +15% (normal à froid)
   - Si très instable ou cale → augmenter ip_fac_lamb_wup à froid

3. Transition tiède (40–60°C) :
   - Accélération douce puis franche
   - Aucune hésitation acceptée
   - Si trou → augmenter `ip_ti_tco_pos_fast_wf_opm_1/2` sur la ligne TCO concernée de +10%

### Phase 3 — Ajustement lambda (Jour 3)

1. Moteur chaud (90°C), ralenti 5 min :
   - STFT/LTFT doivent se stabiliser à ±5%
   - Si LTFT dérive → ajuster `ip_mff_cor_opm` (toutes les 4 maps)

2. Roulage 30 km mixte :
   - Log continu STFT par zone de charge
   - Si correction persistante > ±10% : ajuster `ip_mff_cor_opm`

3. Pleine charge (accélération 5–6 sec sur route droite sûre) :
   - Si sonde large bande installée : vérifier lambda 0.90–0.95 à WOT
   - Pas de cliquetis → bon

### Phase 4 — Avance (Jour 4–7)

```
J4 : +0.5° charge moyenne uniquement → roulez 50 km, pas de cliquetis → continuer
J5 : +1.0° pleine charge → roulez 50 km, vérifiez
J6 : +2.0° pleine charge → test exigeant (montée, plein régime)
J7 : +2.5° UNIQUEMENT si aucun cliquetis aux étapes précédentes

À chaque étape :
  - Cliquetis = STOP immédiat, revenez à −1°
  - Maximum recommandé sur N52 atmosphérique : +2.5°
```

### Phase 5 — Validation finale (Jour 10+)

```
100 km variés sur votre parcours test standardisé :
  ✅ STFT/LTFT < ±8% en toutes conditions
  ✅ Démarrage froid < 3 tours
  ✅ Aucun cliquetis
  ✅ Pas de fuite carburant visuelle
  ✅ Consommation stable et prévisible
  ✅ Filtre à essence changé à 200 km (dépôts dissous par l'éthanol)
```

### Resserrement post-break-in (après 500 km)

```
1. Lire LTFT avec ISTA → cible ±5%
2. Resserrer ip_fac_lamb_max_fsd_1/2 : 40% → 35%
3. Resserrer c_lamb_delta_i_max_lam_adj : 0.25λ → 0.15λ
4. Reflasher
```

---

## Diagnostic rapide

| Symptôme | Cause probable | Solution |
|---|---|---|
| Pas de démarrage à froid | ip_mff_cst_opm trop pauvre | +20% cranking colonnes froides |
| Démarrage laborieux (10+ tours) | ip_mff_cst_opm insuffisant | +15% cranking + vérifier batterie |
| Cale après démarrage froid | ip_fac_lamb_wup insuffisant | +0.05 à 0.08 aux cellules 704 rpm |
| Trou/hésitation à 40–60°C | Film mural insuffisant | +10% sur `ip_ti_tco_pos_fast_wf_opm_1/2` ligne TCO concernée |
| STFT > +15% en permanence | ip_mff_cor_opm trop faible | +3–5% sur les 4 maps |
| STFT < −15% en permanence | ip_mff_cor_opm trop élevé | −3–5% sur les 4 maps |
| Ralenti instable moteur chaud | EVAP ou STFT oscillant | Vérifier purge canister §10, STFT en temps réel |
| Cliquetis pleine charge | Avance trop haute | −1° à −2° immédiatement, puis diagnostiquer |
| Perte puissance progressive | Filtre bouché ou pompe fatiguée | Changer filtre / tester pompe (≥ 2 L/30 s) |
| Fumée noire échappement | ip_fac_lamb_wup trop riche | −0.05 à 0.08 sur cellules concernées |
| LTFT monte sur autoroute | ip_mff_cor_opm trop faible | +3–5% sur les 4 maps |
| Odeur forte éthanol à l'arrêt | Fuite carburant (joints) | **Inspection immédiate obligatoire** |

---

## Avertissements et maintenance

### Surveillance STFT/LTFT

| Indicateur | Normal | Acceptable | PROBLÈME |
|---|---|---|---|
| STFT | ±5% | ±10% | > ±15% |
| LTFT | ±5% | ±10% | > ±15% |

```
STFT > +15% = Le calculateur rajoute du carburant en permanence
  → ip_mff_cor_opm trop petit

STFT < −15% = Le calculateur enlève du carburant
  → ip_mff_cor_opm trop grand

LTFT élevé en permanence = Dérive systématique de la calibration
  → Ajustez ip_mff_cor_opm (positif si LTFT positif, négatif si négatif)
```

### Pompe à essence

Le N52 consomme ~60 L/h en fonctionnement normal. Avec E85 (+45% de masse carburant), il faut ~87 L/h effectivement consommés.

**Test pompe :**
```
Déconnecter le retour carburant, laisser s'accumuler 30 sec :
  Minimum acceptable : 2.0 L/30 sec (= 240 L/h) → marge suffisante
  Si < 1.5 L/30 sec → Pompe fatiguée, à remplacer avant conversion
```

### Filtre à essence

L'E85 est un excellent solvant : il dissout tous les dépôts accumulés dans le réservoir.

1. Changer le filtre AVANT la conversion
2. Changer à nouveau après 200 km d'E85 (dépôts dissous)
3. Contrôler à 500 km
4. Retour au rythme normal (10 000 km) ensuite

### Bougies d'allumage

```
Référence stock : NGK ILZKBR7A8DG ou DENSO FK20HR11 (iridium)
Gap stock : 0.75–0.80 mm
Gap E85 recommandé : 0.65–0.70 mm (mélange plus dense, allumage à froid amélioré)
Intervalle de remplacement : 20 000 km (vs 30 000 km essence)
```

### Démarrage hivernal

```
À −5°C : Enrichissements cranking à ×1.80 minimum sur cette zone TCO
À −10°C : Très difficile — ajouter 10–15% d'essence 95 dans le réservoir
À −15°C et moins : E85 pur pratiquement impossible
                   → Repasser à l'essence ou utiliser un mélange E50
```

Astuce hiver : gardez toujours un bidon de 5L d'essence 95 si votre configuration E85 est définitive.

### Compatibilité matériaux système carburant N52

| Composant | Compatibilité E85 | Action |
|---|---|---|
| Joints Viton/FKM | Excellente | Aucune |
| Joints NBR (nitrile standard) | Mauvaise | Remplacement nécessaire |
| Tuyaux caoutchouc E85-compatible | Bonne | Vérifier l'état |
| Pompe à essence N52 | Bonne (prévu alcool) | Aucune si état correct |
| Rail d'injection acier/alu | Excellente | Aucune |
