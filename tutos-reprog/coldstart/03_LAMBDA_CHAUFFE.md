# §4 — Lambda pendant la chauffe

## Contexte

En parallèle des corrections sur TI, le MSV70 dispose d'un facteur multiplicateur appliqué directement sur la **consigne lambda** pendant la chauffe. C'est un deuxième levier, indépendant des tables d'injection.

Sur essence stock N52B30 : `ip_fac_lamb_wup` = **1.0001 partout** (raw ≈ 47). BMW n'a configuré aucun enrichissement warm-up lambda sur essence — la boucle fermée suffit. **Sur E85, cette table est à construire de zéro.**

---

## ip_fac_lamb_wup

**Adresse :** 0x42764  
**Type :** CARTOGRAPHIE 6×6  
**Axes :** X = MAF (mg/stk), Y = RPM  
**Unité :** facteur multiplicateur sur consigne lambda  
**Stock :** 1.000 partout

Actif uniquement pendant la phase warm-up. Enrichit les basses charges (faible MAF) où l'éthanol s'évapore mal, sans toucher la pleine charge.

```
STOCK :
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1216 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1760 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2016 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2496 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
3008 rpm :    1.000  1.000  1.000  1.000  1.000  1.000

OBJECTIF E85 :
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.08   1.08   1.05   1.03   1.00   1.00
1216 rpm :    1.08   1.07   1.05   1.03   1.00   1.00
1760 rpm :    1.07   1.06   1.04   1.03   1.00   1.00
2016 rpm :    1.06   1.05   1.04   1.03   1.00   1.00
2496 rpm :    1.05   1.05   1.03   1.03   1.00   1.00
3008 rpm :    1.03   1.03   1.03   1.00   1.00   1.00
```

Vérification 30 s après démarrage : STFT entre −10% et +15% (normal, sonde en montée). Après 2 min warm-up : STFT −5% à +5%.

---

## ip_fac_lamb_wup_is

**Type :** CARTOGRAPHIE 3D  
**Axes :** X = MAF (mg/stk), Y = RPM  
**Unité :** sans unité  
**Stock :** 1.0001 partout

Complément de `ip_fac_lamb_wup` pour les régimes bas (mode IS = idle/start). Actif au ralenti pendant le warm-up.

**Modification :** appliquer le même profil que `ip_fac_lamb_wup`. Les deux tables doivent être cohérentes — une divergence se manifeste par un ralenti instable pendant la chauffe.

---

## Interaction avec ip_fac_ti_tco_wup

Les deux mécanismes agissent en même temps mais sur des grandeurs différentes :

| Paramètre | Agit sur | Indépendant de la boucle lambda ? |
|---|---|---|
| `ip_fac_ti_tco_wup_opm` | TI direct | Oui — appliqué avant la boucle |
| `ip_fac_lamb_wup` | Consigne λ | Non — la boucle corrige autour de la consigne |

En pratique : `ip_fac_ti_tco_wup_opm` est l'enrichissement "en avance" (open-loop), `ip_fac_lamb_wup` est l'enrichissement "de consigne" (qui oriente la boucle fermée).

**Ne pas doubler les deux** : si `ip_fac_ti_tco_wup_opm` est déjà à 1.35, ne pas mettre `ip_fac_lamb_wup` à 1.35 simultanément — le résultat serait un enrichissement ×1.35² = ×1.82.

**Approche recommandée :**
- `ip_fac_ti_tco_wup_opm` : profil progressif principal (80% de la correction)
- `ip_fac_lamb_wup` : profil complémentaire plus modéré (20% de la correction)

---

## ip_lamb_bas_* — consignes lambda de base

Les quatre tables `ip_lamb_bas_1/2/3/4` définissent la consigne lambda par zone de fonctionnement :

| Table | Zone | Stock | Modifier pour E85 ? |
|---|---|---|---|
| `ip_lamb_bas_1` | Charge partielle, mode 1 | 0.997 λ | Non en première intention |
| `ip_lamb_bas_2` | Charge partielle, mode 2 | 0.997 λ | Non |
| `ip_lamb_bas_3` | Charge partielle, mode 3 | 0.997 λ | Non |
| `ip_lamb_bas_4` | WOT / haute charge | 0.997 λ | Optionnel — réduire à **0.92–0.95 λ** en zone > 200 mg/stk et > 3000 RPM |

`ip_lamb_bas_1/2/3` : ne pas modifier. La boucle fermée lambda converge automatiquement si `c_fac_mff_ti_stnd` est correct. Toucher uniquement si LTFT > +10% après 200+ km de convergence.

`ip_lamb_bas_4` : la réduction à 0.92–0.95 en WOT est un enrichissement protecteur basse température cylindre, possible sur E85 grâce à son indice d'octane élevé. Ne pas descendre sous 0.90.

---

## Points de vigilance

| Risque | Précaution |
|---|---|
| `ip_fac_lamb_wup` non remis à 1.00 à 80°C | Le moteur tourne riche chaud — STFT fortement négatif |
| `ip_fac_lamb_wup` ≠ `ip_fac_lamb_wup_is` | Instabilité ralenti pendant chauffe |
| Enrichissement TI + lambda simultané trop fort | Moteur noie ses bougies — vérifier STFT chaud : doit être entre −5% et +5% |
