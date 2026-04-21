# §1 — Principes : Architecture du warm-up MSV70

## Les 4 phases de démarrage

Le MSV70 distingue quatre phases distinctes entre la mise en route et l'atteinte de la température de fonctionnement (~80°C TCO).

```
Contact + démarreur
        │
        ▼
┌───────────────┐
│   CRANKING    │  démarreur tourne, aucun allumage encore
│               │  paramètres : ip_mff_cst_opm_1/2
│               │  durée : ~0.3 à 1.5 s selon TCO
└───────┬───────┘
        │ premier allumage
        ▼
┌───────────────┐
│  AFTER-START  │  régime monte de 0 → ~800 RPM
│               │  paramètres : ip_mff_lgrd_ast
│               │  durée : ~2 à 5 s
└───────┬───────┘
        │ ralenti stable
        ▼
┌───────────────┐
│   WARM-UP     │  TCO monte de démarrage → ~80°C
│               │  paramètres : ip_fac_ti_tco_wup, ip_fac_lamb_wup,
│               │               ip_n_sp_is, ip_n_max_*__tco
│               │  durée : 3 à 15 min selon TCO initiale
└───────┬───────┘
        │ TCO ≥ ~80°C
        ▼
┌───────────────┐
│     CHAUD     │  boucle fermée lambda stable, corrections annulées
└───────────────┘
```

---

## Ce que le calculateur cherche à faire

**Pendant le cranking** : injecter suffisamment de carburant pour créer un mélange combustible malgré l'éthanol quasi-liquide à froid (point d'ébullition éthanol = 78°C, contre ~35°C pour l'essence).

**Pendant le warm-up** : trois objectifs simultanés et partiellement contradictoires :
1. Chauffer le catalyseur rapidement (émissions — retarder l'allumage dégrade le rendement thermique → chaleur aux gaz → cat chaud en ~25 s)
2. Maintenir un ralenti stable (confort — l'éthanol brûle moins bien froid)
3. Ne pas endommager le moteur (protection — limiter le RPM tant que l'huile n'a pas circulé)

---

## Seuils de température clés

| Seuil | Valeur stock | Signification |
|---|---|---|
| `c_tco_n_mff_cst` | 17.25°C (raw 87) | En dessous → tables cranking enrichies actives |
| TCO ≈ 80°C | fixe (physique) | Boucle fermée lambda devient stable |
| ip_n_max_2__tco à −30°C | 5400 RPM | Limite RPM absolue à très froid |
| ip_n_max_2__tco à 90°C | 7020 RPM | Limite RPM à chaud (= limiteur permanent) |

---

## Ce qui change avec l'E85

Sur essence, le N52B30 stock (bin VB67774) a :
- `ip_fac_lamb_wup` = **1.0001 partout** — aucun enrichissement warm-up configuré
- `ip_fac_ti_tco_wup_opm_1/2` = **~1.000 partout** — pas de correction injection warm-up

BMW n'a pas eu besoin de configurer ces tables pour l'essence car la boucle lambda fermée converge rapidement. Sur E85, l'éthanol ne s'évapore pas à froid : **toutes ces tables doivent être remplies de zéro**.

---

## Ordre de modification recommandé

1. [Cranking](02_CRANKING.md) — `ip_mff_cst_opm_1/2`, `c_tco_n_mff_cst`, `ip_mff_lgrd_ast`
2. [Injection chauffe](03_INJECTION_CHAUFFE.md) — `ip_fac_ti_tco_wup_opm_1/2`
3. [Lambda chauffe](04_LAMBDA_CHAUFFE.md) — `ip_fac_lamb_wup`, `ip_fac_lamb_wup_is`
4. [Ralenti](05_RALENTI.md) — `ip_n_sp_is` (optionnel si les 3 étapes précédentes sont correctes)
