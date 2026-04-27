# Enrichissement transitoire pleine charge

Le film mural (§06) couvre la majorité des transitoires sur E85. Aucun paramètre spécifique transitoire n'est à modifier pour une conversion E85. Si des trous persistent **exclusivement lors de kickdown brutal** après validation §06 complète, diagnostiquer la pression rail (DTC P0087, mesure WOT) avant toute autre intervention.

> `KF_FTRANSVL` (0x5C5EE, module BLSHUB) est un paramètre de gestion charge/Valvetronic — pas de la carburant. Ne pas modifier.

---

## NE PAS MODIFIER

<a id="p2"></a>
## ② `KL_STEND_TRANS` — Facteur de démarrage de transition

| Champ | Valeur |
|---|---|
| Adresse | 0x53CA0 |
| Type | Courbe 1×4 |
| Unité | facteur (sans dimension) |

**Rôle :** Facteur d'initialisation de la transition WOT — valeur de départ du transitoire. Stock : 0.983 constant. Définit l'amplitude initiale de l'enrichissement dès l'entrée en transitoire, avant que `KF_FTRANSVL` prenne le relais. Sur E85, cette valeur est correcte — ne pas modifier.

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `KL_STEND_TRANS` (4 pts) | **0.983 (constant)** | **Inchangé** |

---

<a id="p3"></a>
## ③ `KL_FUPSRF_TRANS` — Correction surface pression carburant transitoire

| Champ | Valeur |
|---|---|
| Adresse | 0x5BE78 |
| Type | Courbe 1×8 |
| Unité | %/hPa |

**Rôle :** Correction de la masse carburant transitoire en fonction de la pression de carburant. Uniforme à 0.1092 %/hPa. Pas de modification nécessaire si la pression rail reste nominale (5 bar).

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `KL_FUPSRF_TRANS` (8 pts) | **0.1092 %/hPa (constant)** | **Inchangé** |

---

<a id="p4"></a>
## ④ `KL_PIRG_TRANS` — Pression résiduelle gaz brûlés en transitoire

| Champ | Valeur |
|---|---|
| Adresse | 0x5BE56 |
| Type | Courbe 1×8 |
| Unité | hPa |

**Rôle :** Modèle de pression résiduelle des gaz brûlés dans la chambre lors d'une transition de charge. Stock : 100 hPa constant. Sur E85, la combustion est légèrement différente mais cette valeur reste dans les limites du modèle — pas de modification nécessaire.

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `KL_PIRG_TRANS` (8 pts) | **100.0 hPa (constant)** | **Inchangé** |

> Si des trous persistent après validation §06 et KF_FTRANSVL : diagnostiquer la pression rail (DTC P0087, mesure pression en WOT).
