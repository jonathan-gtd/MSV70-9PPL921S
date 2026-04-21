# §3 — Injection pendant la chauffe

## Contexte

Une fois le ralenti stabilisé (~5 s après démarrage), le calculateur entre en phase "warm-up". La boucle lambda fermée est partiellement active mais instable car la sonde amont n'a pas encore atteint sa température de fonctionnement (~300°C). Le MSV70 applique des facteurs correcteurs sur le temps d'injection (TI) pour maintenir un mélange combustible pendant toute la montée en température.

Sur essence stock (bin VB67774), ces tables sont toutes à **~1.000** — BMW n'a pas eu besoin de les configurer. **Sur E85, elles sont indispensables.**

---

## ip_fac_ti_tco_wup_opm_1 / opm_2

**Type :** COURBE 2D  
**Axe X :** TCO (°C)  
**Unité :** sans unité (facteur multiplicateur sur TI)  
**Stock N52B30 :** ~1.000 partout

Facteur multiplicateur appliqué directement sur le temps d'injection pendant la phase de chauffe, indépendamment des corrections lambda.

**Profil E85 recommandé :**

| TCO | Facteur |
|-----|---------|
| −30°C | 1.40 |
| −10°C | 1.35 |
| 0°C | 1.25 |
| 20°C | 1.15 |
| 40°C | 1.08 |
| 60°C | 1.02 |
| ≥ 80°C | 1.00 |

> `opm_1` et `opm_2` doivent être identiques. La divergence entre les deux provoque un comportement différent selon le mode sélectionné par l'index interne.

---

## ip_ti_tco_wup_opm_1 / opm_2

**Type :** TABLE 3D  
**Axes :** X = TCO (°C), Y = RPM  
**Unité :** mg/stk (masse absolue)

Complément de `ip_fac_ti_tco_wup_opm_*` — masse absolue injectée pendant le warm-up. Agit en addition du facteur multiplicateur.

**Modification E85 :** ×1.15 à ×1.35 pour TCO < 50°C. Mettre `opm_1` et `opm_2` identiques.

---

## ip_ti_wup_opm_1 / opm_2

**Type :** COURBE 2D  
**Axe X :** TCO (°C)  
**Unité :** ms (durée d'injection)

Valeur absolue de durée d'injection pendant le warm-up. Complète les tables 3D ci-dessus pour les conditions à bas régime.

**Modification E85 :** ×1.20 à ×1.35 pour TCO < 50°C, cohérent avec `ip_fac_ti_tco_wup_opm_1`.

---

## Interaction entre les trois tables

```
TI_final = TI_base × ip_fac_ti_tco_wup_opm + ip_ti_tco_wup_opm + ip_ti_wup_opm
                      (facteur multiplicateur)   (masse absolue)   (durée absolue)
```

Les trois tables s'additionnent. Si `ip_fac_ti_tco_wup_opm` est correctement calibré, `ip_ti_tco_wup_opm` et `ip_ti_wup_opm` n'ont besoin que d'ajustements fins.

**En première approche :** calibrer `ip_fac_ti_tco_wup_opm_1/2` d'abord. Les deux autres ne sont à toucher que si le comportement warm-up reste insuffisant.

---

## Symptômes d'une calibration insuffisante

| Symptôme | Cause probable |
|---|---|
| Calage 10–30 s après démarrage | Facteur trop faible à la TCO de démarrage |
| À-coups pendant la montée en température | Profil de facteur trop abrupt entre deux points |
| STFT fortement négatif (−15 à −25%) chaud | Facteur encore actif au-delà de 80°C — vérifier le point à 80°C |
| Fumées noires pendant la chauffe | Facteur trop élevé — réduire de 10% |
