# §10 — Enrichissement transitoire pleine charge

> Paramètres pilotant l'enrichissement lors d'un appel de couple brutal (kickdown). **Ne pas modifier en première intention.** Le film mural (§4) couvre la majorité des transitoires — diagnostiquer §4 en premier. Intervenir ici uniquement si des trous d'accélération persistent exclusivement lors de kickdown brutal, après validation §4.

---

<a id="p1"></a>
## ① `KF_FTRANSVL` — Facteur de transition pleine charge f(charge × RPM)

| Champ | Valeur |
|---|---|
| Adresse | 0x5C5EE |
| Structure | Map 8×8, uint16 |
| Équation | `0.000015 × raw` |
| Axes | X = charge normalisée (0.0–0.983), Y = RPM (0–6500) |

**Rôle :** Multiplicateur sur la masse carburant calculée lors d'une transition vers la pleine charge. À charge normalisée ~0.5, le facteur stock est ~0.39 — l'enrichissement transitoire n'est appliqué qu'à ~40% en zone intermédiaire. Sur E85, si le facteur est insuffisant, le mélange est lean de façon transitoire lors des kickdowns. La table est uniforme sur tous les régimes (toutes les lignes RPM ont les mêmes valeurs).

**Avant / Après :**

```
STOCK :
        charge →  0.000  0.098  0.197  0.295  0.393  0.492  0.786  0.983
tous RPM :        0.000  0.049  0.098  0.147  0.197  0.393  0.688  0.983

OBJECTIF E85 (si trous kickdown persistent après §4 validé) :
Augmenter les cellules zone 0.393–0.786 de +10 à +20% :
        charge →  0.000  0.098  0.197  0.295  0.393  0.492  0.786  0.983
tous RPM :        0.000  0.049  0.098  0.147  0.217  0.432  0.757  0.983
```

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération progressive | Lisse, aucun trou | → problème film mural §4, pas transitoire |
| Kickdown brutal uniquement | Aucun trou bref | Trou < 0.5s lors de kickdown → `KF_FTRANSVL` +10% zone mi-charge |

---

<a id="p2"></a>
## ② `KL_STEND_TRANS` — Facteur de démarrage de transition

| Champ | Valeur |
|---|---|
| Adresse | 0x53CA0 |
| Structure | Courbe 1×4, uint16 |
| Équation | `0.000015 × raw` |

**Rôle :** Facteur d'initialisation de la transition WOT — valeur de départ du transitoire. Stock : ~0.983 constant sur tous les points. Ce facteur définit l'amplitude initiale de l'enrichissement dès l'entrée en transitoire, avant que `KF_FTRANSVL` prenne le relais.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `KL_STEND_TRANS` (4 pts) | **0.983 (constant)** | **Ne pas modifier** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Entrée WOT | Pas de coupure au tout début de l'accélération | Coupure < 0.1s → KL_STEND_TRANS insuffisant |

---

<a id="p3"></a>
## ③ `KL_FUPSRF_TRANS` — Correction surface pression carburant transitoire

| Champ | Valeur |
|---|---|
| Adresse | 0x5BE78 |
| Structure | Courbe 1×8, uint16 |
| Équation | `0.000005 × raw` (%/hPa) |

**Rôle :** Correction de la masse carburant transitoire en fonction de la pression de carburant. Sur E85, la pression rail est identique à l'essence (5 bar) mais la densité du carburant est différente. Cette table est uniforme (0.1092 %/hPa sur tous les points stock) — elle n'a pas besoin d'être modifiée si la pression rail reste nominale.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `KL_FUPSRF_TRANS` (8 pts) | **0.1092 %/hPa (constant)** | **Ne pas modifier** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Pression rail en WOT | 5 bar stable | Chute pression → problème pompe, pas de cette table |

---

<a id="p4"></a>
## ④ `KL_PIRG_TRANS` — Pression résiduelle gaz brûlés en transitoire

| Champ | Valeur |
|---|---|
| Adresse | 0x5BE56 |
| Structure | Courbe 1×8, uint16 |
| Équation | `0.039063 × raw` (hPa) |

**Rôle :** Modèle de pression résiduelle des gaz brûlés dans la chambre lors d'une transition de charge. Utilisé pour corriger la masse d'air estimée en transitoire. Stock : 100 hPa constant. Sur E85, la combustion est légèrement différente (vitesse de flamme, EGT) mais cette valeur de 100 hPa reste dans les limites du modèle — pas de modification nécessaire.

**Avant / Après :**

| | ◀ Valeur stock | ▶ E85 |
|---|---|---|
| `KL_PIRG_TRANS` (8 pts) | **100.0 hPa (constant)** | **Ne pas modifier** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Comportement général transitoire | Aucun trou ni heurt | Problème → diagnostiquer §4 film mural en premier |
