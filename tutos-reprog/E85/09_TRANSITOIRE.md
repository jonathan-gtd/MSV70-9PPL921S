# Enrichissement transitoire pleine charge

Paramètres pilotant l'enrichissement lors d'un appel de couple brutal (kickdown). **Ne pas modifier en première intention.** Le film mural (§06) couvre la majorité des transitoires — diagnostiquer §06 en premier. Intervenir ici uniquement si des trous d'accélération persistent **exclusivement lors de kickdown brutal**, après validation §06.

---

## OPTIONNEL

<a id="p1"></a>
## ① `KF_FTRANSVL` — Facteur de transition pleine charge f(charge)

| Champ | Valeur |
|---|---|
| Adresse | 0x5C5EE |
| Type | Courbe 1×8 (uniforme sur tous les RPM) |
| Unité | facteur (sans dimension) |
| Axes | X = charge normalisée (0.0–0.983) |

**Rôle :** Multiplicateur sur la masse carburant calculée lors d'une transition vers la pleine charge. À charge normalisée ~0.5, le facteur stock est ~0.39 — l'enrichissement transitoire n'est appliqué qu'à ~40% en zone intermédiaire. Sur E85, si le facteur est insuffisant, le mélange est lean de façon transitoire lors des kickdowns.

**Avant / Après :**

| Charge normalisée | ◀ Stock VB67774 | ✅ E85 (si trous kickdown) |
|---|---|---|
| 0.000 | 0.000 | 0.000 |
| 0.098 | 0.049 | 0.049 |
| 0.197 | 0.098 | 0.098 |
| 0.295 | 0.147 | 0.147 |
| 0.393 | 0.197 | **0.217** |
| 0.492 | 0.393 | **0.432** |
| 0.786 | 0.688 | **0.757** |
| 0.983 | 0.983 | 0.983 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération progressive | Lisse, aucun trou | → problème film mural §06, pas transitoire |
| Kickdown brutal uniquement | Aucun trou bref | Trou < 0.5s lors de kickdown → +10% zone 0.393–0.786 |

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
