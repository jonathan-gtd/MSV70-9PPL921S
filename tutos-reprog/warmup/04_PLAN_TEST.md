# §6 — Plan de test warm-up

## Protocole de validation

### Phase 1 — Premier démarrage à froid

**Conditions :** moteur froid (TCO < 20°C), préférablement < 10°C pour valider les corrections basses températures.

**À observer :**

| Contrôle | Valeur attendue | Problème si hors plage |
|---|---|---|
| Démarrage en < 3 cranking | Oui | Cranking insuffisant — augmenter `ip_mff_cst_opm` |
| Calage dans les 5 s post-démarrage | Non | After-start insuffisant — augmenter `ip_mff_lgrd_ast` |
| Ralenti stable dans les 10 s | Oui | Injection ou lambda chauffe insuffisant |
| STFT immédiat après démarrage | Entre −20% et +20% | Plage normale pendant convergence sonde |

---

### Phase 2 — Montée en température (TCO 20°C → 80°C)

**Outil requis :** OBD live data (ISTA, INPA, ou tout scanner capable de lire STFT/LTFT et TCO en temps réel).

**À enregistrer toutes les 10°C de TCO :**

| Paramètre | Valeur attendue |
|---|---|
| STFT | Entre −10% et +10% |
| Régime ralenti | Conforme à `ip_n_sp_is[TCO]` ± 50 RPM |
| Absence de calage | Oui |
| Absence de fumées noires | Oui |

Un STFT fortement négatif (< −10%) pendant la chauffe indique un facteur d'injection ou lambda warm-up trop élevé pour cette plage de TCO. Réduire le facteur correspondant de 5–10%.

Un STFT fortement positif (> +10%) persistant indique un facteur insuffisant.

---

### Phase 3 — Moteur chaud (TCO ≥ 80°C)

**Critique :** vérifier que les facteurs warm-up sont bien annulés à 80°C.

| Contrôle | Valeur attendue |
|---|---|
| STFT chaud (régime stabilisé) | Entre −5% et +5% |
| LTFT après 50+ km | Entre −8% et +8% |
| Pas de richesse résiduelle | `ip_fac_ti_tco_wup_opm` = 1.00 à 80°C |
| Pas d'enrichissement lambda résiduel | `ip_fac_lamb_wup` = 1.00 à 80°C |

Si STFT chaud est fortement négatif après la phase warm-up, vérifier que les profils retombent bien à 1.00 à partir de 80°C.

---

### Phase 4 — Démarrage hivernal (si possible)

**Conditions :** TCO < 0°C (véhicule laissé une nuit dehors en hiver).

| Contrôle | Valeur attendue |
|---|---|
| Démarrage | < 5 cranking |
| Pas de calage | Pendant 30 min de conduite |
| STFT pendant warm-up | Entre −20% et +20% (boucle encore en convergence) |

---

## Tableau de décision post-test

| Symptôme observé | Paramètre à corriger | Direction |
|---|---|---|
| Pas de démarrage à froid | `ip_mff_cst_opm_1/2` | ×1.10 sur les points TCO concernés |
| Calage 2–5 s post-démarrage | `ip_mff_lgrd_ast` | ×1.10 |
| Ralenti instable 0→30°C | `ip_fac_ti_tco_wup_opm` ou `ip_fac_lamb_wup` | +5–10% sur les points TCO concernés |
| STFT −15% à chaud | `ip_fac_ti_tco_wup_opm` ou `ip_fac_lamb_wup` | Vérifier retour à 1.00 à 80°C |
| STFT +15% chaud persistant (>200 km) | `ip_lamb_bas_1` | −0.01 λ |
