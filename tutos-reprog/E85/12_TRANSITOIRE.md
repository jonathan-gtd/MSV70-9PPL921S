# §10 — Enrichissement transitoire pleine charge

**Paramètres identifiés dans le bin :**

| Paramètre | Adresse | Structure | Description XDF |
|---|---|---|---|
| `KF_FTRANSVL` | 0x5C5EE | 8×8, uint16, ×0.000015 | Facteur de transition Volllast (pleine charge) |
| `KL_FUPSRF_TRANS` | 0x5BE78 | 1×8, uint16, ×0.000005 %/hPa | Fupsrf — correction surface pression carburant transitoire |
| `KL_STEND_TRANS` | 0x53CA0 | 1×4, uint16, ×0.000015 | Start factor — facteur de démarrage de transition |
| `KL_PIRG_TRANS` | 0x5BE56 | 1×8, uint16, ×0.039063 hPa | Pression résiduelle gaz brûlés en transitoire |

**Valeurs stock extraites du bin :**

`KF_FTRANSVL` — axe X (facteur charge 0.0–0.983), axe Y (RPM 0–6500) :
```
        charge →  0.000  0.098  0.197  0.295  0.393  0.492  0.786  0.983
tous RPM :        0.000  0.049  0.098  0.147  0.197  0.393  0.688  0.983
```
> Toutes les lignes RPM sont identiques — le facteur de transition WOT dépend uniquement de la charge normalisée, pas du régime. À charge maximale (0.983), le facteur atteint ~1.0 (enrichissement plein).

`KL_STEND_TRANS` (4 pts) : `[0.9829, 0.9829, 0.9829, 0.9829]` — facteur constant ~0.98, pas de variation en fonction du paramètre d'entrée.

`KL_FUPSRF_TRANS` (8 pts, tous identiques) : `0.1092 %/hPa` — correction de surface de pression de carburant uniforme sur toute la plage.

`KL_PIRG_TRANS` (8 pts, tous identiques) : `100.0 hPa` — pression résiduelle gaz brûlés constante.

**Impact E85 :**

Ces tables pilotent l'enrichissement temporaire lors d'un appel de couple brutal (kickdown). La `KF_FTRANSVL` est un multiplicateur sur la masse carburant calculée : à charge normalisée ~0.5, le facteur est ~0.39 stock — ce qui signifie que le transitoire n'applique que ~40% de l'enrichissement maximum en zone intermédiaire.

Sur E85, si vous constatez des **trous d'accélération brefs uniquement lors de kickdown** (et non lors d'une accélération progressive), c'est l'indice que ces tables manquent d'enrichissement transitoire. Augmenter les cellules de mi-charge de `KF_FTRANSVL` de +10 à +20% dans la zone 0.393–0.786 peut corriger ce symptôme.

**Verdict : ne pas modifier en première intention.** Le film mural §5 couvre la majorité des transitoires — diagnostiquer §5 d'abord. Intervenir sur `KF_FTRANSVL` uniquement si les trous persistent après validation §5 et exclusivement sur kickdown brutal.

---

