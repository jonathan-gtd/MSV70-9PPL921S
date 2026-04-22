# §6 — Délai enrichissement WOT

> Sur E85, l'enrichissement pleine charge est critique pour protéger le moteur. Le MSV70 applique un délai de 200 ms entre la détection de l'état WOT et l'injection de ce surplus. Ce délai crée un bref lean en boucle ouverte à chaque accélération franche — à éliminer. Quatre copies existent : deux pour les boîtes manuelles (MT), deux pour les automatiques (AT). Modifier uniquement les copies correspondant à la boîte du véhicule.

---

<a id="p1"></a>
## ① `c_t_ti_dly_fl_1` — Délai WOT boîte manuelle, copie 1

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC4 |
| Structure | Constante scalaire |

**Rôle :** Délai entre la détection du full load flag et l'application de l'enrichissement WOT, pour les boîtes manuelles. Stock 200 ms — conçu sur essence pour éviter des enrichissements intempestifs lors de brèves sollicitations pédale. Sur E85, ce délai crée un lean transitoire non compensé (boucle ouverte WOT) à chaque appui franc sur l'accélérateur.

**Avant / Après :**

| | ◀ Stock | ✏️ E85 |
|---|---|---|
| `c_t_ti_dly_fl_1` | 0.200 s (200 ms) | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération franche WOT | Enrichissement immédiat, aucun délai perceptible | Trou bref → vérifier les 2 copies MT mises à zéro |

---

<a id="p2"></a>
## ② `c_t_ti_dly_fl_2` — Délai WOT boîte manuelle, copie 2

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC6 |
| Structure | Constante scalaire |

**Rôle :** Deuxième copie du délai WOT MT. L'ECU alterne entre ces deux copies selon le contexte d'exécution. Si _1 est mis à zéro mais _2 reste à 200 ms, l'enrichissement peut encore être retardé de façon intermittente selon la situation moteur. Les deux doivent être à zéro.

**Avant / Après :**

| | ◀ Stock | ✏️ E85 |
|---|---|---|
| `c_t_ti_dly_fl_2` | 0.200 s (200 ms) | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| WOT répété plusieurs fois | Richesse WOT immédiate à chaque sollicitation | Délai intermittent → _2 non modifié |

---

<a id="p3"></a>
## ③ `c_t_ti_dly_fl_at_1` — Délai WOT boîte automatique, copie 1

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC8 |
| Structure | Constante scalaire |

**Rôle :** Même logique que la version MT mais pour les boîtes automatiques ZF 6HP. L'ECU sélectionne les copies MT ou AT selon le type de boîte configuré dans les paramètres véhicule. Modifier uniquement si le véhicule est AT — inutile sur MT.

**Avant / Après :**

| | ◀ Stock | ✏️ E85 (si AT) |
|---|---|---|
| `c_t_ti_dly_fl_at_1` | 0.200 s (200 ms) | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Kickdown AT | Enrichissement immédiat lors du passage de rapport | Délai perceptible → _at_2 également à zéro |

---

<a id="p4"></a>
## ④ `c_t_ti_dly_fl_at_2` — Délai WOT boîte automatique, copie 2

| Champ | Valeur |
|---|---|
| Adresse | 0x44ECA |
| Structure | Constante scalaire |

**Rôle :** Deuxième copie du délai WOT AT. Même logique que _at_1. Modifier uniquement sur véhicule AT, en même temps que _at_1.

**Avant / Après :**

| | ◀ Stock | ✏️ E85 (si AT) |
|---|---|---|
| `c_t_ti_dly_fl_at_2` | 0.200 s (200 ms) | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| WOT AT répété | Réponse WOT immédiate et constante | Délai aléatoire → vérifier les 2 copies AT |
