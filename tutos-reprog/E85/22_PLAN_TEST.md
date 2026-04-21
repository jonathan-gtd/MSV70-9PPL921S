# §20 — Plan de Test et Validation Progressif

### Phase 0 : Préparation (Avant toute modification)

```
Checklist matériel :
  ✅ Scanner OBD2 avec log (ISTA, INPA, Torque Pro avec plugin BMW)
  ✅ Sonde lambda large bande recommandée (optionnelle mais idéale)
  ✅ Thermomètre d'ambiance
  ✅ Plein de carburant E85 dans le réservoir

Réinitialisation obligatoire avant le premier démarrage E70 :
  ✅ Via ISTA-D : Service → Fonctions de service → Adaptation du mélange → Réinitialiser
     (ou via NCS Expert : Kraftstoffadaption zurücksetzen)
  Raison : les LTFT calculés sur essence (~0%) vont biaiser la lecture des STFT E70
            dans les premières minutes. Partir à zéro donne une base de lecture fiable
            dès le premier allumage.

Baseline à enregistrer (sur essence, avant le plein E70) :
  - STFT/LTFT à ralenti (doivent être proches de 0%)
  - Consommation sur 20 km de parcours test
  - Comportement démarrage à froid (si possible)
```

### Phase 1 : Application des paramètres injecteurs (Jour 1)

**Modifications :**
- `ip_mff_cor_opm_1_1`, `ip_mff_cor_opm_1_2`, `ip_mff_cor_opm_2_1`, `ip_mff_cor_opm_2_2` → **raw 47 407 (phys 1.473, ×1.45 E85 — open loop safe)**
- c_tco_n_mff_cst → 25°C

**Test immédiat :**
1. Démarrez moteur (si tiède ou chaud)
2. Attendez ralenti stable (1–2 min)
3. Lisez STFT → doivent être entre −10% et +10%
4. Si STFT > +12% : augmentez `ip_mff_cor_opm` (toutes les 4 maps) de +3% supplémentaire
5. Roulez 20 km, vérifiez stabilisation LTFT

**Critère de validation Phase 1 :**
- STFT au ralenti chaud : −5% à +5%
- LTFT après 15 min : −8% à +8%

### Phase 2 : Validation Démarrage Froid (Jour 2)

**Conditions :** Moteur froid (nuit dehors, < 15°C)

1. Démarrage à froid sans pédale :
   - Doit démarrer en ≤ 4 tours
   - Si > 5 tours → augmentez ip_mff_cst_opm_1 de +15%

2. Ralenti post-démarrage (30–60 sec) :
   - Doit être stable à 600–1000 tr/min
   - STFT entre −10% et +15% (normal à froid)
   - Si très instable ou cale → augmentez ip_fac_lamb_wup à froid

3. Transition tiède (40–60°C) :
   - Accélération douce puis franche
   - Aucune hésitation acceptée
   - Si trou → augmentez `ip_ti_tco_pos_fast_wf_opm_1` (et `ip_ti_tco_pos_fast_wf_opm_2`) sur la ligne TCO correspondante de +10%

### Phase 3 : Ajustement Lambda (Jour 3)

1. Moteur chaud (90°C), ralenti 5 min :
   - STFT/LTFT doivent se stabiliser à ±5%
   - Si LTFT dérive → ajustez `ip_mff_cor_opm` (toutes les 4 maps)

2. Roulage 30 km mixte :
   - Log continu STFT par zone de charge
   - Si correction persistante > ±10% : ajuster `ip_mff_cor_opm` (toutes les 4 maps)

3. Pleine charge (accélération 5–6 sec sur route droite sûre) :
   - Si sonde large bande installée : vérifiez lambda 0.90–0.95 à WOT
   - Pas de cliquetis → bon

### Phase 4 : Avance (Jour 4–7)

```
J4 : +2° pleine charge uniquement → roulez 50 km, pas de cliquetis → continuer
J5 : +4° pleine charge → roulez 50 km, vérifiez
J6 : +5° pleine charge → test exigeant (montée, plein régime)
J7 : +6° UNIQUEMENT si aucun cliquetis aux étapes précédentes

À chaque étape :
  - Cliquetis = STOP immédiat, revenez à −1°
  - LTFT qui monte = vérifiez que lambda WOT est correct
```

### Phase 5 : Validation Finale (Jour 10+)

```
100 km variés sur votre parcours test standardisé :
  ✅ STFT/LTFT < ±8% en toutes conditions
  ✅ Démarrage froid < 3 tours
  ✅ Aucun cliquetis
  ✅ Pas de fuite carburant visuelle
  ✅ Consommation stable et prévisible
  ✅ Dernier changement de filtre à essence
```

---

