# §23 — Principes Fondamentaux E85 sur N52

### Quelle teneur en éthanol est réellement dans votre réservoir ?

Le "E85" vendu en pompe France ne contient **pas forcément 85% d'éthanol**. La norme NF EN 15376 et l'arrêté du 30 septembre 2011 autorisent une plage de **60 à 85% d'éthanol** selon la saison :

| Saison | Teneur éthanol typique (France) | Raison |
|---|---|---|
| Été (juin–sept) | 75–85% | Pression de vapeur favorable |
| Mi-saison (oct, mai) | 65–75% | Transition |
| Hiver (nov–avril) | 60–72% | Ajout essence pour faciliter démarrage froid |

> **Pour votre cas (65–70% d'éthanol) : les calculs de cette section précisent les facteurs pour E65 et E70 — pas uniquement E85 pur.** Le facteur d'enrichissement requis est environ 8–12% inférieur à ce que nécessiterait un E85 à 85%.

### Pourquoi l'éthanol change tout ?

| Propriété | Essence 95 | E85 (85%) | E70 (70%) | E65 (65%) |
|---|---|---|---|---|
| Pouvoir calorifique inférieur (PCI) | 44 MJ/kg | ~26 MJ/kg | ~29 MJ/kg | ~30 MJ/kg |
| AFR stœchiométrique | 14.7:1 | 9.55:1 | 10.18:1 | 10.41:1 |
| Indice d'octane (RON) | 95 | ~108 | ~104 | ~103 |
| Température d'ébullition | ~35–200°C | ~78°C | ~75°C | ~73°C |
| Chaleur de vaporisation | ~305 kJ/kg | ~840 kJ/kg | ~700 kJ/kg | ~670 kJ/kg |
| Densité | ~0.745 kg/L | ~0.789 kg/L | ~0.775 kg/L | ~0.770 kg/L |

**Conséquence directe sur la calibration :**
- L'AFR stœchiométrique est inférieur → le calculateur doit injecter plus de masse de carburant pour le même débit d'air
- L'évaporation à froid est difficile → enrichissement cranking/warm-up indispensable
- L'octane élevé → avance à l'allumage augmentable → gain de puissance

### Architecture du MSV70 : Ce qui interagit avec le carburant

Le MSV70 sur N52 utilise les mécanismes suivants pour contrôler l'injection :
```
MFF (mg/stk)  →  [c_fac_mff_ti_stnd]  ×  [ip_mff_cor_opm]  →  TI (ms)  →  injecteur

Lambda setpoint  →  [consigne interne]  →  boucle fermée O2  →  correction STFT/LTFT

Démarrage  →  [ip_mff_cst_opm]  →  dose cranking ouverte

Warm-up  →  [ip_fac_lamb_wup]  →  facteur d'enrichissement post-démarrage
```

### opm_1 vs opm_2 — Deux modes de fonctionnement N52

Tout au long de ce tuto, les paramètres apparaissent en deux variantes : `_opm_1` et `_opm_2`. Comprendre la différence est essentiel pour savoir laquelle modifier.

| Mode | Suffixe | Condition d'activation | Fréquence sur N52 |
|---|---|---|---|
| **Valvetronic actif** (mode normal) | `_opm_1` | Fonctionnement normal — levée variable Valvetronic contrôle le débit d'air, le papillon reste ouvert à fond | **~95% du temps** |
| **Mode papillonné** (throttled mode) | `_opm_2` | Valvetronic en défaut ou désactivé — le papillon reprend le rôle de doseur d'air | Failsafe uniquement |

Le N52B30 est un moteur **Valvetronic** : en conditions normales, c'est le système de levée variable des soupapes d'admission (pas le papillon) qui contrôle la quantité d'air admis. C'est le mode opm_1. Le mode opm_2 n'est actif qu'en cas de panne Valvetronic ou de défaut électrique — le moteur passe alors en fonctionnement « papillon conventionnel » pour rentrer au garage.

**Conséquence pratique :** Pour une conversion E85 sur un N52B30 fonctionnel, les tables `_opm_1` sont celles qui comptent. Les tables `_opm_2` doivent également être modifiées (même facteur), car si le Valvetronic tombe en panne après votre conversion E85, vous voulez que le mode dégradé soit aussi calibré correctement.

### Pourquoi autant de paramètres pour la même chose ?

En parcourant ce tuto, vous allez rencontrer plusieurs paramètres qui semblent faire la même chose — 4 maps de correction injection, 4 tables lambda, 4 tables de film mural, etc. Il y a en réalité **4 raisons distinctes**, qui ne se traitent pas de la même façon.

---

#### Raison 1 — Modules firmware indépendants (les 5 copies de `c_fac_mff_ti_stnd` — et pourquoi on n'y touche pas)

Le MSV70 a **5 copies** du facteur injecteur réparties dans trois modules logiciels indépendants. L'approche naïve serait de les passer toutes à ×1.45 pour E85. Mais il y a un problème XDF :

| Copie | Équation | Max encodable | Cible E85 | Possible ? |
|---|---|---|---|---|
| `c_fac_mff_ti_stnd[0]` / `[1]` | 0.000012 × X | 0.786 | 0.491 | ✅ |
| `c_fac_mff_ti_stnd_1` / `_2` / `_mon` | **0.000006 × X** | **0.393** | 0.491 | **❌ overflow** |

Les trois copies `_1`, `_2` et `_mon` utilisent un coefficient deux fois plus petit : avec uint16 max=65535, le maximum physique encodable est `65535 × 0.000006 = 0.393 ms/mg` — inférieur à la cible E85 de 0.491. Il est impossible d'y écrire la valeur correcte.

**Solution retenue : `ip_mff_cor_opm`**. Ces maps sont des **multiplicateurs d'injection** appliqués après `c_fac_mff_ti_stnd`, avec l'équation `0.000031 × X` (max encodable = 2.031). On y applique le facteur ×1.45, `c_fac` reste au stock sur toutes ses copies, et le monitoring ne voit aucune divergence.

**Conséquence pour la calibration :** modifier les 4 maps `ip_mff_cor_opm_*`, pas `c_fac_mff_ti_stnd`.

---

#### Raison 2 — Deux bancs moteur

Le N52B30 a deux groupes de cylindres (1–3 et 4–6) avec une sonde lambda par banc. L'ECU peut corriger chaque banc indépendamment. Si les LTFT divergent entre banc 1 et banc 2 après la conversion, c'est le signe d'un problème mécanique (injecteur, sonde) — pas une erreur de calibration.

---

#### Raison 3 — Modes de fonctionnement moteur (`_opm_1` / `_opm_2`)

Voir section ci-dessus. Deux modes de pilotage de l'air → deux jeux de tables pour la même grandeur physique.

**Conséquence pour la calibration :** toujours modifier les deux. En pratique `_opm_1` est actif 95% du temps, mais `_opm_2` doit être cohérent pour le mode dégradé.

---

#### Raison 4 — Sous-phénomènes physiques distincts (film `pos`/`neg`, `slow`/`fast`)

Ici ce n'est **pas** la même grandeur dupliquée — c'est un phénomène complexe découpé en sous-zones parce que le comportement physique est réellement différent selon le contexte :

> La table pleine charge WOT est **`ip_lamb_fl__n`** (courbe 1×12 f(RPM), λ 0.920 stock). Ne pas la confondre avec les consignes de charge partielle — erreur fréquente dans les guides E85.

**Film mural `slow` / `fast` :**
Le film de carburant sur les parois du collecteur a deux constantes de temps très différentes :
- **Slow** : film résiduel qui s'accumule et s'évapore entre injections — dynamique de plusieurs secondes
- **Fast** : compensation instantanée lors d'un tip-in brutal — dynamique de quelques millisecondes

Ce sont deux phénomènes superposés, pas deux copies du même calcul.

**Film mural `pos` / `neg` :**
Deux directions opposées du même phénomène :
- **Pos** : à l'accélération, le film absorbe du carburant → l'ECU injecte plus pour compenser
- **Neg** : à la décélération, le film restitue du carburant → l'ECU injecte moins pour ne pas sur-enrichir

**Conséquence pour la calibration :** comprendre quelle table est active avant de toucher. Modifier les tables `_neg_*` à la place des `_pos_*` peut dégrader le comportement sans aucun effet sur la zone que vous vouliez corriger.

---

**Résumé — comment identifier le pattern**

| Pattern rencontré | Raison | Action |
|---|---|---|
| 5 copies `c_fac_mff_ti_stnd` | Modules firmware indépendants | **Ne pas modifier** — 3 copies ne peuvent pas encoder 0.491 (overflow XDF). Utiliser `ip_mff_cor_opm_*` à la place |
| `_1` / `_2` sur lambda | Deux bancs moteur | Modifier les deux — divergence = problème mécanique |
| `_opm_1` / `_opm_2` | Deux modes Valvetronic/papillon | Modifier les deux — opm_2 = fallback panne |
| `_bas_1/2/3/4`, `slow`/`fast`, `pos`/`neg` | Sous-phénomènes physiques distincts | Identifier quelle zone est active avant de modifier |

---

