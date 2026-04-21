# §1 — Principes : Phase de chauffe MSV70

## Qu'est-ce que la phase warm-up

Après le premier allumage et la stabilisation du ralenti (~5 s), le MSV70 entre en phase **warm-up** : la TCO monte de sa valeur de départ jusqu'à ~80°C. Pendant cette phase, la boucle lambda fermée est partiellement instable car la sonde amont n'a pas encore atteint sa température de fonctionnement (~300°C).

Le calculateur applique des corrections spécifiques pour compenser :
- l'évaporation difficile de l'éthanol entre 0°C et 70°C (point d'ébullition 78°C)
- l'instabilité de la régulation lambda pendant la montée en température

---

## Paramètres actifs pendant le warm-up

| Groupe | Paramètres | Rôle |
|---|---|---|
| Facteur TI | `ip_fac_ti_tco_wup_opm_1/2` | Multiplicateur sur temps d'injection f(TCO) |
| Masse absolue TI | `ip_ti_tco_wup_opm_1/2` | Masse carburant absolue f(TCO, RPM) |
| Facteur lambda | `ip_fac_lamb_wup` | Multiplicateur sur consigne λ f(TCO, RPM) |
| Facteur lambda IS | `ip_fac_lamb_wup_is` | Même facteur, actif spécifiquement au ralenti |

---

## Pourquoi les tables sont vides sur le N52B30 stock

Sur le bin VB67774 (essence) :
- `ip_fac_ti_tco_wup_opm_1/2` = **~1.000 partout**
- `ip_fac_lamb_wup` = **1.0001 partout** (raw ≈ 47)

BMW n'a pas eu besoin de configurer ces tables pour l'essence : la boucle fermée lambda converge rapidement dès que la sonde est active. Sur E85, l'éthanol ne s'évapore pas à froid — **les tables doivent être construites de zéro**.

---

## Fin de la phase warm-up

La phase warm-up se termine quand TCO atteint ~80°C. À ce point :
- `ip_fac_ti_tco_wup_opm` doit retourner à **1.000**
- `ip_fac_lamb_wup` doit retourner à **1.000**

Si ces valeurs ne retombent pas à 1.000 à 80°C, le moteur fonctionnera riche en permanence une fois chaud — ce qui se manifeste par un STFT fortement négatif (< −10%).
