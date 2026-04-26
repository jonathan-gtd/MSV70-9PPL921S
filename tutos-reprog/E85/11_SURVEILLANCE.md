# Surveillance — Indicateurs OBD pendant la mise au point

Ces valeurs ne se programment pas dans le bin — elles se **lisent** via OBD (ISTA, INPA, scanner générique). Les surveiller à chaque session pour détecter un problème avant qu'il devienne critique.

**Scanner recommandé pour N52 :** ISTA/D via interface ENET ou K+DCAN (accès à tous les paramètres BMW).

---

## ① Duty cycle injecteur — saturation haut régime

| Champ | Valeur |
|---|---|
| Source | Calculé (non encodé dans le bin) |
| Formule | `DC (%) = TI_ms / T_cycle_ms × 100` |
| Unité | % |
| Limite safe | **85%** |

**Rôle :** Le duty cycle est le ratio temps d'ouverture / temps disponible par cycle moteur. Au-delà de 85%, l'injecteur entre en saturation : le débit plafonne, le mélange devient pauvre de façon non compensable. Sur E85, le TI est ×1.45 plus long qu'à l'essence → le risque apparaît plus tôt à haut régime. La saturation est silencieuse (boucle ouverte WOT) et directement destructrice.

**Temps disponible par régime :**

| RPM | T_cycle | TI max safe (85%) |
|---|---|---|
| 4000 RPM | 30.0 ms | 25.5 ms |
| 5500 RPM | 21.8 ms | 18.5 ms |
| 6500 RPM | 18.5 ms | **15.7 ms** |

**Duty cycle estimé @ 6500 RPM WOT :**

| Condition | Essence stock | E85, injecteurs stock |
|---|---|---|
| TI estimé | ~8–10 ms | ~12–14 ms |
| Duty cycle | ~43–54% | ~65–76% |
| Statut | Confortable | Dans les limites, marge faible |

**Vérification :**

| PID OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| Injection Time (Ti) | < 15.7 ms @ 6500 RPM | > 15 ms → ne pas pousser la puissance sans nouveaux injecteurs |
| Duty cycle calculé | < 85% à tous régimes | Proche 85% → marges insuffisantes, pas de gain puissance |

---

## ② STFT / LTFT — convergence de l'adaptation

| PID OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | Hors plage → facteur ip_mff_cor_opm_* à ajuster |
| LTFT après 200–500 km | ±5% | > ±10% → calibration injecteurs à affiner |
| STFT pendant warm-up (TCO 30–70°C) | −10% à +10% | > +15% → ip_fac_lamb_wup trop faible |
| STFT WOT | Non significatif (boucle ouverte) | Surveiller Ti et DC à la place |

**Procédure de test sécurisée :**

```
1. Démarrer à chaud (TCO > 80°C, STFT actif)
2. Laisser tourner 30 secondes → lire STFT moyen
3. STFT dans [−10%, +15%] → rouler, laisser LTFT converger
4. STFT > +15% → couper, augmenter le facteur de 2–3%, reflasher, recommencer
5. Ne jamais dépasser 3 min avec STFT > +20%
```

| Condition STFT | Durée max | Risque | Action |
|---|---|---|---|
| STFT +5% à +12% | Plusieurs minutes | Dans les limites — ECU compense | Observer, ajuster |
| STFT > +12% (plafond LTFT) | 2–3 min max | Légère surchauffe soupapes échappement | Couper, ajuster facteur |
| STFT > +12% ET LTFT bloqué à +12% | **< 30 secondes** | ECU ne compense plus → mélange pauvre permanent | **Couper immédiatement** |

---

## ③ Lambda WOT — richesse pleine charge

| Signal | Unité | ✅ Cible | ⚠️ Action |
|---|---|---|---|
| Lambda WOT (sonde large bande) | λ | 0.90–0.95 | Hors plage → ajuster `ip_lamb_fl__n` (§08) |
| Lambda au cranking froid | λ | Non mesurable (sonde froide) | Surveiller durée de démarrage à la place |

---

## ④ Retrait d'avance knock control

| Signal | ✅ Cible | ⚠️ Action |
|---|---|---|
| Knock retard (ISTA PID) | 0° en croisière | Retrait répété → réduire l'avance de −1° (§07) |
| LTFT avance stable | Aucun retrait fréquent | Retrait fréquent → trop d'avance dans `ip_iga_bas_max_knk` |
| Son de cliquetis | Absent | Métal bref → −1° immédiat, vérifier avant de rouler |
