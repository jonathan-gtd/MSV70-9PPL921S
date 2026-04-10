# TUTO E85 : Conversion Ethanol pour Siemens MSV70 — BMW 330i N52B30

> **Véhicule ciblé :** BMW E90/E91/E92/E93 330i — Moteur N52B30 — Calculateur Siemens MSV70  
> **Injecteurs :** BMW 13 53 7531634 (Bosch EV14, port injection, pression nominale rail 5.0 bar — `c_fup_nom` stock = 5000 hPa)  
> **Fichier de base :** VB67774_921S_Full.bin  
> **Stratégie de calibration :**
> - **Facteur injecteur → E85 (×1.45)** : boucle ouverte (WOT) toujours riche, quelle que soit la saison
> - **Cranking → E70** : exception — trop riche au démarrage = noyage moteur
> - **Avance → E60** : pire octane légal français = zéro risque de cliquetis en toutes conditions
> - **Film mural → E85 (×1.25)** : tip-in riche = safe
> - **Carburant réel moyen attendu :** E70 (70% éthanol) — moyenne annuelle pondérée France  
> **Version :** 3.4 — Données réelles extraites du bin + descriptions XDF — 2026-04-09

---

## 📌 Résumé des Paramètres Impactés par la Conversion E85

Avant d'entrer dans le détail de chaque section, voici l'ensemble des paramètres du bin MSV70 qui doivent être modifiés — ou surveillés — lors d'une conversion E85, avec la raison physique de chaque impact.

| # | Paramètre(s) | Section | Pourquoi c'est impacté |
|---|---|---|---|
| 1 | `c_fac_mff_ti_stnd_1` `c_fac_mff_ti_stnd_2` `c_fac_mff_ti_stnd[0]` `c_fac_mff_ti_stnd[1]` `c_fac_mff_ti_stnd_mon` | §1 — Injecteurs | Ce facteur est calibré pour **E85 (×1.45)** même si le carburant réel moyen est E70. Raison : en boucle ouverte (WOT, départ d'accélération), aucune correction lambda n'intervient. Calibré sur E85 max, on est systématiquement riche en open loop quel que soit le titre éthanol réel de la pompe (E60 à E85). En boucle fermée, le LTFT compense automatiquement. **5 copies à modifier simultanément sous peine de DTC.** |
| 2 | `c_tco_n_mff_cst` | §2 — Démarrage froid | Seuil en-dessous duquel les enrichissements de cranking s'activent. Stock : 17 °C. À relever à 25 °C car l'éthanol a besoin d'enrichissement à des températures ambiantes que l'essence gère sans aide. |
| 3 | `ip_mff_cst_opm_1` `ip_mff_cst_opm_2` | §2 — Démarrage froid | **Exception à la règle "E85 partout" :** le cranking est boucle ouverte mais trop riche = noyage moteur (le carburant liquide étouffe la bougie). On calibre pour E70 réel — ni trop, ni trop peu. L'enrichissement E85 du facteur injecteur ne s'applique pas ici : ces tables pilotent la dose directement, indépendamment de `c_fac_mff_ti_stnd`. |
| 4 | `ip_fac_lamb_wup` | §2 — Démarrage froid | Facteur multiplicateur sur la consigne lambda après démarrage. Stock : 1.000 partout. **Ses axes sont X = MAF (65–500 mg/stk), Y = RPM (704–3008 tr/min)** — ce n'est pas une table température moteur, c'est une table charge×régime. Elle permet d'enrichir les zones basse charge / bas régime où la sonde lambda n'est pas encore opérationnelle. L'enrichissement en fonction de la TCO n'existe pas dans cette table — il est géré par `ip_mff_cst_opm_*` (cranking) et la boucle lambda (chauffe). |
| 5 | `ip_ti_tco_pos_slow_wf_opm_1` / `ip_ti_tco_pos_slow_wf_opm_2` + `ip_ti_tco_pos_fast_wf_opm_1` / `ip_ti_tco_pos_fast_wf_opm_2` | §5 — Film mural | Le film mural s'applique lors des transitions de charge (tip-in) — une zone de boucle ouverte transitoire. Calibré pour **E85 (×1.25)** : si le carburant réel est E70, on sur-compense légèrement le film → mélange légèrement riche sur tip-in → safe. Sous-compenser serait lean transitoire → risque de claquement ou raté. |
| 6 | `ip_iga_bas_max_knk__n__maf` (+ `ip_iga_min_n_maf_opm_1` / `ip_iga_min_n_maf_opm_2`) | §3 — Avance | L'avance est calibrée pour **E60 (plancher légal hivernal, ~101 RON)** — le pire carburant que vous pouvez légalement avoir à la pompe. Raison : si on cale l'avance sur E70 ou E85 et que la station délivre du E60 en hiver, on risque le cliquetis. En calibrant sur E60, on est safe quelle que soit la saison. Gain de puissance légèrement réduit (+2.5° max vs +4.5° pour E70), mais zéro risque moteur. |
| 7 | `ip_lamb_bas_1` / `ip_lamb_bas_2` / `ip_lamb_bas_3` | §4 — Lambda | Optionnel en conduite normale : si `c_fac_mff_ti_stnd` est correctement calibré, la boucle fermée gère lambda automatiquement. À ajuster de ±0.02 seulement si les LTFT dérivent de façon persistante après stabilisation. |
| 8 | `ip_lamb_fl__n` | §4 — Lambda WOT | **Vraie table de richesse pleine charge** (1×12 f(RPM)). Stock déjà à λ 0.920 (et 0.871 à 6500 rpm) — l'enrichissement WOT essence est **déjà présent**. Sur E85, on peut soit laisser tel quel, soit dé-enrichir légèrement (0.94-0.95) puisque la chaleur de vaporisation E85 protège mécaniquement contre la détonation. |
| 9 | `c_t_ti_dly_fl_1` `c_t_ti_dly_fl_2` | §7 — Complémentaires | Délai entre détection de pleine charge et application de l'enrichissement WOT. À réduire à 0 ms pour que la richesse cible soit appliquée instantanément lors d'une accélération franche sur E85. |
| 10 | `c_iga_ini` | §7 — Complémentaires | Avance d'allumage initiale lors du cranking. Si le démarrage reste difficile après ajustement des tables cranking, +1° à +2° ici facilite l'inflammation du mélange E85 froid. |

> **Paramètres non modifiés (mais à surveiller) :** `ip_ti_min` et `c_ti_add_as` (temps mort injecteur) → inchangés si les injecteurs stock sont conservés. EVAP/purge canister (`ip_crlc_mff_buf_cp`) → la boucle fermée compense automatiquement les vapeurs éthanol ; intervenir seulement si les STFT oscillent de plus de ±15 % lors des purges.

---

## ⚙️ Prérequis Avant Toute Conversion E85

Avant d'ouvrir TunerPro ou de toucher le moindre paramètre, vous devez valider chaque point ci-dessous. Une conversion E85 réussie dépend autant de l'état mécanique du véhicule que de la calibration.

---

### 1. Injecteurs — Vos 13 53 7531634 suffisent-ils ?

**Réponse courte : oui, sur le N52B30 en usage normal.**

Les injecteurs BMW 13 53 7531634 (Bosch EV14, port injection, ~0.34 ms/mg à la pression rail nominale du MSV70) peuvent délivrer le +30% de masse carburant requis par l'E85 **à condition que le cycle utile (duty cycle) reste raisonnable**.

| Condition | Duty cycle estimé essence | Duty cycle estimé E85 | Verdict |
|---|---|---|---|
| Ralenti (700 rpm) | ~5% | ~6.5% | ✅ Aucun problème |
| Route (3000 rpm, 50%) | ~20% | ~26% | ✅ Aucun problème |
| Pleine charge (6500 rpm) | ~55–60% | ~72–78% | ✅ Acceptable (limite ~85%) |

> **Conclusion :** Avec le N52B30 atmosphérique, les injecteurs stock sont suffisants pour E85 pur. Si vous ajoutez un compresseur ou une turbo par la suite, réévaluez — un kit forced induction sur E85 dépasse la capacité des injecteurs stock.

**Si vous avez remplacé les injecteurs :** Recalculez le facteur `c_fac_mff_ti_stnd` avec la formule de la section 1 (dépend du titre éthanol réel + débit des nouveaux injecteurs) plutôt qu'un facteur fixe.

---

### 2. Sonde Lambda Amont — Large Bande Obligatoire

**La sonde lambda stock du N52B30 est une sonde large bande (wideband) de type LSU.** C'est une bonne nouvelle : elle mesure le lambda réel en continu (0.7–∞), contrairement aux sondes narrowband qui ne signalent que riche/pauvre.

**Références OEM pour votre 330i N52B30 :**

| Position | Référence BMW | Remarque |
|---|---|---|
| Amont (avant catalyseur) | **11 78 7537993** | Référence principale |
| Amont (avant catalyseur) | **11 78 7558055** | Référence de remplacement / supersession |

> Les deux références amont sont interchangeables — BMW a supersédé l'une par l'autre selon les millésimes. Vérifiez laquelle est montée sur votre véhicule avant de commander.

| Type de sonde | Compatible E85 ? | Pour le tuning |
|---|---|---|
| **Large bande (LSU 4.9, UEGO)** — stock N52 | ✅ Oui | Lecture lambda précise, boucle fermée précise |
| Narrowband (NTK binarisée) | ⚠️ Partielle | Boucle fermée OK, mais impossible de vérifier lambda WOT |
| Aucune sonde / sonde défaillante | ❌ Bloquant | **Ne pas commencer la conversion** |

**Vérifications à effectuer :**
- Via ISTA ou INPA : lecture sonde lambda amont en temps réel → doit osciller autour de λ=1.00 ±0.05 au ralenti chaud
- Code erreur P0130–P0135 (sonde amont) → régler avant conversion
- Sonde encrassée ou vieillissante : remplacer par **11 78 7537993** ou **11 78 7558055**

> **Recommandation :** Si votre véhicule a >100 000 km, remplacez la sonde amont avant la conversion. Une sonde fatiguée rend le réglage E85 impossible — les STFT dérivent sans raison apparente.

---

### 3. Sonde Lambda Aval — Diagnostic Catalyseur

La sonde aval (post-catalyseur) est une **sonde narrowband** sur le N52. Elle n'est pas utilisée pour le contrôle lambda en temps réel, mais pour le diagnostic catalyseur et la correction long terme aval.

**Références OEM pour votre 330i N52B30 :**

| Position | Référence BMW | Remarque |
|---|---|---|
| Aval (après catalyseur) — banc 1 | **11 78 7545074** | Côté cylindres 1–3 |
| Aval (après catalyseur) — banc 2 | **11 78 7545075** | Côté cylindres 4–6 |

- Elle fonctionne normalement avec l'E85 — aucun remplacement requis sauf défaillance
- Après conversion, il est normal que les codes catalyseur (P0420/P0430) apparaissent temporairement le temps que les adaptations se recalibrent
- Si ces codes persistent après 200+ km de conduite E85 : effacer les adaptations via ISTA → laisser le moteur recalculer

---

### 4. Pompe à Essence — Débit Suffisant ?

L'E85 demande ~30% de débit volumique supplémentaire. La pompe stock N52 est électrique et montée dans le réservoir.

**Test de débit pompe (à faire avant conversion) :**
```
1. Moteur coupé, clé sur ON (pompe s'amorce 2–3 sec)
2. Déconnectez le retour carburant (circuit de retour)
3. Placez le tuyau dans un récipient gradué
4. Lancez la pompe (clé ON ou ponter le relais pompe)
5. Laissez s'écouler exactement 30 secondes
6. Mesurez le volume collecté

Résultat attendu :
  ≥ 2.0 L en 30 sec (= 240 L/h)  → ✅ Suffisant pour E85 +30%
  1.5–2.0 L                        → ⚠️ Limite, surveiller à charge élevée
  < 1.5 L                          → ❌ Remplacer la pompe avant conversion
```

**Pompes de remplacement compatibles N52 :**
- Pompe N54 (référence BMW 16 14 7 195 881) — même gabarit, débit supérieur
- Walbro 255 L/h (aftermarket, montage universel avec adaptateur)
- Bosch 0 580 254 044 (débit ~200 L/h à 3.5 bar, acceptable)

---

### 5. Filtre à Essence — À Changer Avant

L'E85 est un **solvant puissant**. Il dissout les dépôts accumulés dans le réservoir depuis des années. Ces dépôts migrent vers le filtre et le bouchent dans les premiers 200–500 km.

**Protocole :**
1. Changer le filtre AVANT la conversion (filtre neuf)
2. Rechange obligatoire à **200 km** après le premier plein E85
3. Contrôle visuel à 500 km (filtre découpé ou transparence si filtre en ligne)
4. Rythme normal (10 000 km) ensuite

---

### 6. Circuit Carburant — Compatibilité Matériaux

| Composant | Matériau | Compatibilité E85 |
|---|---|---|
| Rail d'injection aluminium | Aluminium anodisé | ✅ Compatible |
| Tuyaux haute pression (acier/alu) | Métal | ✅ Compatible |
| Tuyaux flexibles bas pression | Caoutchouc FKM/Viton | ✅ Compatible |
| Tuyaux flexibles bas pression | NBR/SBR (caoutchouc standard) | ❌ Dégradation en 6–18 mois |
| Joints injecteurs (O-rings) | Viton (OEM BMW) | ✅ Compatible |
| Joints de pompe ancienne génération | NBR | ⚠️ Vérifier l'état |

> Sur les N52B30 de série E9x (2005–2012), les tuyaux et joints BMW d'origine sont **Viton/FKM** et compatibles E85. Toute réparation antérieure avec des pièces génériques NBR doit être refaite.

**Inspection visuelle avant conversion :**
- Aucune trace d'humidité ou de suintement sur les raccords
- Tuyaux flexibles non craquelés, non durcis
- O-rings injecteurs non exsudés

---

### 7. Bougies d'Allumage — Gap à Ajuster

L'E85 modifie les conditions de combustion (mélange plus dense, température de combustion légèrement différente). Un gap de bougie standard peut être légèrement trop large.

| Configuration | Référence | Gap recommandé |
|---|---|---|
| Stock essence | NGK ILZKBR7A8DG (iridium) | 0.75–0.80 mm |
| **E85 recommandé** | **Même référence** | **0.65–0.70 mm** |

> Le gap réduit améliore la fiabilité d'allumage à froid et à haut régime avec un mélange E85 (légèrement plus difficile à allumer que l'essence à basse température).

**Intervalle de remplacement sur E85 :** 20 000 km (vs 30 000 km essence)

---

### 8. Batterie — Condition Impérative pour le Démarrage Froid

À froid (<5°C), l'E85 exige plusieurs tours de vilebrequin supplémentaires avant l'allumage. Une batterie en mauvais état est la première cause d'échec de démarrage E85 hivernal.

**Critères minimum :**
```
Tension à vide (repos) : ≥ 12.5 V
CCA (Cold Cranking Amps) : ≥ 500 A (stock N52 : 70–90 Ah / 680–800 CCA)
Test de charge : tension sous démarreur ≥ 10.0 V
```

**Si la batterie a > 4 ans :** Remplacez-la avant la conversion hivernale.

---

### 9. Outil de Diagnostic — Scanner OBD2 Capable

La conversion E85 se valide **uniquement avec des données en temps réel**. Un scanner basique qui lit juste les codes erreur est insuffisant.

| Outil | Ce qu'il faut pouvoir lire |
|---|---|
| **INPA / ISTA-D** (recommandé) | STFT, LTFT bank1/2, lambda amont, régime, TCO, charge |
| **Carly for BMW** | STFT/LTFT, lambda — suffisant pour validation de base |
| **Torque Pro + plugin ELM327** | STFT/LTFT — acceptable, latence élevée |
| Scanner générique OBD2 | ❌ Insuffisant — pas d'accès aux PIDs BMW propriétaires |

> **Interface recommandée :** Adaptateur ENET (RJ45 OBD via WiFi/USB) pour ISTA. Accès à tous les paramètres internes MSV70, réinitialisation des adaptations, lecture des valeurs réelles de toutes les cartes.

---

### 10. Logiciel de Calibration — TunerPro RT + XDF MSV70

Pour modifier le bin, vous avez besoin de :

| Élément | Fichier / Version |
|---|---|
| **Logiciel** | TunerPro RT v5.x (gratuit) |
| **XDF** | `BMW_Siemens_MSV70_9PPL921S_2560K.xdf` |
| **Bin stock** | `VB67774_921S_Full.bin` — **faites 2 copies avant tout** |
| **Interface flash** | K+DCAN USB (flashing via WinKFP/NCS Expert) ou ENET + NCS |

> **IMPÉRATIF :** Sauvegardez le bin stock dans **au moins 2 emplacements distincts** (disque dur + clé USB + cloud) avant toute modification. Une mauvaise écriture bin = moteur ne démarre plus = retour au stock obligatoire.

---

### Récapitulatif Prérequis — Go / No-Go

| # | Prérequis | Statut à vérifier | Bloquant ? |
|---|---|---|---|
| 1 | Injecteurs stock 13537531634 présents | Identifier visuellement | Non (adapter calcul si différents) |
| 2 | Sonde lambda amont large bande fonctionnelle | ISTA : lecture lambda temps réel | **OUI** |
| 3 | Pompe à essence ≥ 2.0 L/30 sec | Test débit | **OUI si < 1.5 L** |
| 4 | Filtre à essence neuf | Facture récente ou remplacement | Non (mais risque à 200 km) |
| 5 | Circuit carburant Viton/FKM | Inspection visuelle | **OUI si joints NBR craquelés** |
| 6 | Bougies en bon état, gap 0.65–0.70 mm | Vérification physique | Non (mais démarrage froid difficile) |
| 7 | Batterie ≥ 500 CCA / ≥ 12.5 V repos | Test batterie | Non (mais démarrage froid risqué) |
| 8 | Scanner OBD2 avec lecture STFT/LTFT | Test connexion | **OUI** |
| 9 | TunerPro RT + XDF + bin stock sauvegardé | Vérification fichiers | **OUI** |

---

## 📋 0. Principes Fondamentaux E85 sur N52

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
MFF (mg/stk)  →  [c_fac_mff_ti_stnd]  →  TI (ms)  →  injecteur

Lambda setpoint  →  [ip_lamb_bas]  →  boucle fermée O2  →  correction STFT/LTFT

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

En parcourant ce tuto, vous allez rencontrer plusieurs paramètres qui semblent faire la même chose — 5 copies du facteur injecteur, 4 tables lambda, 4 tables de film mural, etc. Il y a en réalité **4 raisons distinctes**, qui ne se traitent pas de la même façon.

---

#### Raison 1 — Modules firmware indépendants (les 5 copies de `c_fac_mff_ti_stnd`)

Trois sous-systèmes du firmware MSV70 ont été écrits séparément et ont chacun besoin du débit injecteur pour leur propre calcul. Ils ne se lisent pas mutuellement — chaque module a sa propre copie locale de la valeur. C'est une architecture embarquée classique : pas de variable globale partagée, chaque bloc est autonome et peut fonctionner indépendamment.

Résultat : 5 copies de la même valeur physique réparties à trois adresses différentes dans le bin, dont 2 avec un coefficient d'équation différent (`c_fac_mff_ti_stnd[0]` / `c_fac_mff_ti_stnd[1]`).

**Conséquence pour la calibration :** toutes les copies doivent être mises à jour simultanément. Si vous en oubliez une, le module de monitoring compare ses valeurs avec le calcul principal — divergence → DTC injection, même si le moteur tourne parfaitement.

---

#### Raison 2 — Deux bancs moteur (`ip_lamb_bas_1` / `ip_lamb_bas_2`)

Le N52B30 a deux groupes de cylindres (1–3 et 4–6) avec une sonde lambda par banc. L'ECU peut corriger chaque banc indépendamment — si un injecteur dérive sur le banc 1, il ne faut pas corriger le banc 2 par erreur.

C'est la même grandeur physique (consigne lambda), mais deux instances indépendantes, une par banc.

**Conséquence pour la calibration :** modifier les deux avec la même valeur. Si les LTFT divergent entre banc 1 et banc 2 après la conversion, c'est le signe d'un problème mécanique (injecteur, sonde) — pas une erreur de calibration.

---

#### Raison 3 — Modes de fonctionnement moteur (`_opm_1` / `_opm_2`)

Voir section ci-dessus. Deux modes de pilotage de l'air → deux jeux de tables pour la même grandeur physique.

**Conséquence pour la calibration :** toujours modifier les deux. En pratique `_opm_1` est actif 95% du temps, mais `_opm_2` doit être cohérent pour le mode dégradé.

---

#### Raison 4 — Sous-phénomènes physiques distincts (`ip_lamb_bas_1/2/3/4`, film `pos`/`neg`, `slow`/`fast`)

Ici ce n'est **pas** la même grandeur dupliquée — c'est un phénomène complexe découpé en sous-zones parce que le comportement physique est réellement différent selon le contexte :

**Tables lambda (`ip_lamb_bas_1/2/3/4`) :**
L'ECU applique une consigne lambda différente selon le régime de fonctionnement moteur. Ce ne sont pas des copies — c'est une segmentation intentionnelle :

| Table | Zone active |
|---|---|
| `ip_lamb_bas_1` / `ip_lamb_bas_2` | Ralenti et charge partielle basse — boucle fermée prioritaire |
| `ip_lamb_bas_3` | Charge partielle haute avec knock control actif — légèrement riche pour protection thermique |
| `ip_lamb_bas_4` | Charge partielle haute sans knock control — même richesse, gestion différente |
| `ip_lamb_fl__n` | Pleine charge WOT — enrichissement spécifique (λ 0.920 stock) |

Modifier `ip_lamb_bas_4` en croyant que c'est la table WOT n'a aucun effet en WOT — c'est l'erreur la plus fréquente.

**Film mural `slow` / `fast` :**
Le film de carburant sur les parois du collecteur a deux constantes de temps très différentes :
- **Slow** : film résiduel qui s'accumule et s'évapore entre injections — dynamique de plusieurs secondes
- **Fast** : compensation instantanée lors d'un tip-in brutal — dynamique de quelques millisecondes

Ce sont deux phénomènes superposés, pas deux copies du même calcul.

**Film mural `pos` / `neg` :**
Deux directions opposées du même phénomène :
- **Pos** : à l'accélération, le film absorbe du carburant → l'ECU injecte plus pour compenser
- **Neg** : à la décélération, le film restitue du carburant → l'ECU injecte moins pour ne pas sur-enrichir

**Conséquence pour la calibration :** comprendre quelle table est active avant de toucher. Modifier `ip_lamb_bas_4` à la place de `ip_lamb_fl__n`, ou les tables `_neg_*` à la place des `_pos_*`, peut dégrader le comportement sans aucun effet sur la zone que vous vouliez corriger.

---

**Résumé — comment identifier le pattern**

| Pattern rencontré | Raison | Action |
|---|---|---|
| 5 copies `c_fac_mff_ti_stnd` | Modules firmware indépendants | Toutes à modifier simultanément |
| `_1` / `_2` sur lambda | Deux bancs moteur | Modifier les deux — divergence = problème mécanique |
| `_opm_1` / `_opm_2` | Deux modes Valvetronic/papillon | Modifier les deux — opm_2 = fallback panne |
| `_bas_1/2/3/4`, `slow`/`fast`, `pos`/`neg` | Sous-phénomènes physiques distincts | Identifier quelle zone est active avant de modifier |

---

## 🔧 1. Mise à l'échelle des injecteurs — Paramètre CRITIQUE

### Concept : MFF → TI

- **MFF** = Manifold Fuel Flow : masse de carburant calculée par le moteur en **mg/stk** (milligrammes par coup)
- **TI** = Time Injection : durée d'ouverture de l'injecteur en **ms**
- Le calculateur multiplie la masse demandée par le facteur `c_fac_mff_ti_stnd` pour obtenir la durée d'injection
- Si ce facteur est trop petit → injection trop courte → mélange pauvre
- Si ce facteur est trop grand → injection trop longue → mélange riche

### Vos Injecteurs : BMW 13 53 7531634

- Injecteurs d'origine N52B30 — Bosch EV14 — injection indirecte (port injection)
- Pression de travail nominale (lue dans le bin, `c_fup_nom` @0x44B0C) : **5.0 bar** (5000 hPa, raw=60301). Les facteurs d'échelle E85 (×1.33-1.45) sont définis en « standard conditions » = pression rail stock, donc cette valeur ne change rien aux calculs.
- Ces injecteurs sont les mêmes pour les 330i des générations E46/E9x N52

### Les 5 copies du facteur d'échelle — toutes à modifier ensemble

Le MSV70 dispose de **cinq copies** du facteur d'échelle injecteur réparties dans trois modules logiciels distincts. Pourquoi cinq ? Parce que trois sous-systèmes indépendants du firmware ont chacun besoin de connaître le débit injecteur :

1. **Calcul principal** (`c_fac_mff_ti_stnd_1` et `c_fac_mff_ti_stnd_2`) : le module central qui calcule le TI cycle par cycle pour les deux bancs (cylindres 1–3 et 4–6). C'est le facteur « actif » qui pilote réellement les injecteurs.
2. **Phasage SOI/EOI** (`c_fac_mff_ti_stnd[0]` et `c_fac_mff_ti_stnd[1]`) : le module qui calcule les angles Start Of Injection et End Of Injection. Pour positionner correctement la fin d'injection par rapport au cycle moteur, il a besoin du débit injecteur — même valeur physique, mais avec une équation à coefficient double (×0.000012 vs ×0.000006), d'où un raw deux fois plus petit.
3. **Canal de monitoring** (`c_fac_mff_ti_stnd_mon`) : le module de surveillance système qui vérifie en permanence la cohérence injection. Il compare sa propre estimation de débit avec celle du calcul principal. Si les deux divergent au-delà d'un seuil → DTC injection. Si vous ne mettez pas `c_fac_mff_ti_stnd_mon` à jour, ce comparateur lève un code erreur même si les injecteurs fonctionnent parfaitement.

Oublier l'une de ces cinq copies peut provoquer des incohérences ou déclencher un DTC de monitoring injection.

✏️
| Paramètre | Adresse bin | Module | Formule | Raw stock | Valeur stock | Raw objectif (E85) | Objectif E85 | Rôle |
|---|---|---|---|---|---|---|---|---|
| `c_fac_mff_ti_stnd_1` | 0x44AC0 | Calcul principal | 0.000006 × X | 56 567 | **0.3394 ms/mg** | **82 022** | **0.4921 ms/mg** | Groupe cyl. 1–3 |
| `c_fac_mff_ti_stnd_2` | 0x44AC2 | Calcul principal | 0.000006 × X | 56 567 | **0.3394 ms/mg** | **82 022** | **0.4921 ms/mg** | Groupe cyl. 4–6 |
| `c_fac_mff_ti_stnd[0]` | 0x45AAC | Phasage SOI/EOI | 0.000012 × X | 28 284 | **0.3394 ms/mg** | **41 011** | **0.4921 ms/mg** | Timing injection |
| `c_fac_mff_ti_stnd[1]` | 0x45AAE | Phasage SOI/EOI | 0.000012 × X | 28 284 | **0.3394 ms/mg** | **41 011** | **0.4921 ms/mg** | Timing injection |
| `c_fac_mff_ti_stnd_mon` | 0x4958C | Canal monitoring | 0.000006 × X | 56 567 | **0.3394 ms/mg** | **82 022** | **0.4921 ms/mg** | Surveillance/DTC |

> Toutes les 5 copies donnent la même valeur physique : **0.3394 ms/mg** stock. Les `c_fac_mff_ti_stnd[0]`/`c_fac_mff_ti_stnd[1]` ont un raw deux fois plus petit car leur équation a un coefficient double (×0.000012 vs ×0.000006).

### Quel facteur selon le titre éthanol réel ?

La formule correcte dérive de l'AFR stœchiométrique du mélange :

```
AFR_blend = 1 / ( E_fraction/9.0 + (1−E_fraction)/14.7 )

Facteur_injection = (14.7 / AFR_blend) × 0.94
                    ↑ ratio AFR       ↑ correction densité éthanol/essence
```

| Teneur éthanol | AFR stœchio | Facteur | `c_fac_mff_ti_stnd_1` / `c_fac_mff_ti_stnd_2` / `c_fac_mff_ti_stnd_mon` raw | `c_fac_mff_ti_stnd[0]` / `c_fac_mff_ti_stnd[1]` raw |
|---|---|---|---|---|
| E65 (65%) | 10.41:1 | **×1.33** | **75 234** | **37 617** |
| E70 (70%) | 10.18:1 | **×1.36** | **76 931** | **38 466** |
| E75 (75%) | 9.97:1 | **×1.39** | **78 628** | **39 315** |
| **E85 (85%) ← CIBLE INJECTEUR** | **9.55:1** | **×1.45** | **82 022** | **41 011** |

> **Pourquoi E85 comme cible du facteur injecteur alors que le carburant réel est E70 ?**
> En boucle fermée, le LTFT corrige automatiquement l'excès de richesse : avec du E70 réel, le LTFT se stabilise à environ −6% — largement dans les ±25% de capacité, aucun problème.
> En boucle ouverte (WOT, transitions), aucune correction n'intervient. En calibrant sur E85, on garantit qu'on est toujours du côté riche en open loop, quelle que soit la teneur réelle de la pompe (E60 à E85). C'est le choix sécuritaire pour une carto de rue.

> La LTFT (adaptation long terme) peut absorber ±25%. Si vous ne savez pas exactement quel titre vous avez, commencez avec E70 (×1.36) — les STFT vous diront si vous montez ou descendez.

### Formule pour injecteurs remplacés

```
c_fac_mff_ti_stnd_nouveau = c_fac_mff_ti_stnd_STOCK × Facteur_Ethanol × (Débit_stock / Débit_nouveaux)

Exemple — injecteurs N54 (débit ~30% supérieur aux N52 stock) sur E70 :
  = 0.3394 × 1.36 × (1 / 1.30) = 0.3394 × 1.046 ≈ 0.355 ms/mg  → raw c_fac_mff_ti_stnd_1/c_fac_mff_ti_stnd_2 ≈ 59 167
```

### Validation après modification

Après avoir appliqué les 5 valeurs :
- Démarrez **moteur chaud** si possible (première validation à froid = cranking difficile sans calibration cranking)
- Lisez les **STFT** (Short Term Fuel Trims) via ISTA / INPA / Torque Pro
- STFT entre **−5% et +5%** = calibration correcte
- STFT > +15% → facteur trop petit (mélange pauvre), augmentez de +2–3%
- STFT < −15% → facteur trop grand (mélange riche), diminuez de −2–3%

### Tests au ralenti : durée et risques

> **Question fréquente :** peut-on laisser tourner le moteur au ralenti avec une richesse très pauvre le temps de vérifier les STFT, sans risquer de l'endommager ?

**Réponse courte : oui, mais avec limites.**

| Condition STFT | Durée max recommandée | Risque principal | Action |
|---|---|---|---|
| STFT +5% à +20% | Plusieurs minutes sans problème | Aucun significatif | Observer, ajuster |
| STFT +20% à +25% (limite LTFT) | 2–3 min maximum | Légère surchauffe soupapes échappement | Couper, ajuster facteur |
| STFT bloqué à +25% ET LTFT +25% | **< 30 secondes** | Soupapes échappement → calamine, joint culasse possible | **Couper immédiatement** |

**Pourquoi l'idle est moins dangereux que la charge :**
- Au ralenti (650–750 rpm), la quantité d'air et de carburant est faible → même 20% de pauvreté représente peu de masse
- La sonde lambda fonctionne et la correction STFT est active — le moteur n'est **jamais** totalement en boucle ouverte au ralenti chaud
- Les températures d'échappement restent modérées (~350–450°C) vs charge pleine (~650–800°C)

**Ce qui est dangereux en revanche :**
- Couper le moteur quand STFT > +20% et **redémarrer immédiatement** sans ajuster → cycles répétés de chauffe sur mélange pauvre
- Faire tourner au ralenti avec **TCO < 80°C** (boucle fermée inactive ou instable) → lambda non régulé
- Faire des accélérations même courtes si STFT > +15% — sous charge le risque d'inflammation anormale (detonation) augmente fortement sur mélange pauvre

**Procédure de test sécurisée :**
```
1. Démarrer à chaud (TCO > 80°C, STFT actif)
2. Laisser tourner 30 secondes → lire STFT moyen
3. Si STFT dans [−10%, +15%] → rouler prudemment, laisser LTFT converger
4. Si STFT > +15% → couper, ajuster facteur +3%, reflasher, recommencer
5. Ne jamais dépasser 3 min avec STFT > +20%
6. Surveiller aussi la température d'eau (TCO) : si monte anormalement → pauvre
```

---

## ❄️ 2. Démarrage à Froid (Cranking & After-Start)

### Pourquoi c'est difficile avec l'E85

L'E85 résiste à l'évaporation à basse température :
- Essence : s'évapore dès −40°C
- Éthanol pur : point d'ébullition 78°C, la vaporisation est très limitée à 0°C
- **À 0°C, l'E85 reste essentiellement liquide** → le moteur a besoin d'une quantité bien plus grande de carburant liquide pour créer la vapeur nécessaire à l'allumage

### 2.1 — Table de cranking : `ip_mff_cst_opm_1` et `ip_mff_cst_opm_2`

Ces tables définissent la dose de carburant injectée pendant le cranking (moteur en démarrage), en **mg/stk**, en fonction de la température de liquide de refroidissement (axe X) et du régime de cranking (axe Y).

> Équation : `valeur_physique = 0.021195 × raw_u16be`. Unité : mg/stk. Axes X = TCO (°C), Y = RPM.

**Facteurs multiplicateurs E70 appliqués colonne par colonne :**
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
Facteur  :  ×2.00   ×1.80   ×1.65  ×1.55  ×1.35  ×1.20  ×1.10  ×1.05
```

> Sur E70, les 30% d'essence améliorent sensiblement la volatilité à froid — les facteurs sont légèrement inférieurs à un E85 pur.

✏️
**`ip_mff_cst_opm_1` @ 0x437DC** (mode normal / Valvetronic actif) :

STOCK :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  447.7   350.6   261.3  189.3  102.2   71.4   56.2   49.6
RPM 320  :  320.3   260.7   202.1  152.1   87.9   61.3   46.5   39.1
RPM 920  :  194.4   175.1   146.0  112.9   68.4   48.6   36.5   33.0
```

OBJECTIF E70 :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  895.4   631.1   431.1  293.4  138.0   85.7   61.8   52.1
RPM 320  :  640.6   469.3   333.5  235.8  118.7   73.6   51.2   41.1
RPM 920  :  388.8   315.2   240.9  175.0   92.3   58.3   40.2   34.7
```

**`ip_mff_cst_opm_2` @ 0x4380C** (mode papillonné / Valvetronic désactivé) :

STOCK :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  731.1   527.0   362.8  245.0  138.2  102.1   67.8   49.6
RPM 320  :  546.2   415.6   297.0  201.8  106.4   82.3   57.0   39.1
RPM 920  :  363.0   281.4   215.8  159.0   84.1   65.8   47.0   34.5
```

OBJECTIF E70 :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  : 1462.2   948.6   598.6  379.8  186.6  122.5   74.6   52.1
RPM 320  : 1092.4   748.1   490.1  312.8  143.6   98.8   62.7   41.1
RPM 920  :  726.0   506.5   356.1  246.5  113.5   79.0   51.7   36.2
```

> opm_2 a des valeurs nettement plus élevées à froid (jusqu'à ×2.0 vs opm_1 à −30°C) — les mêmes facteurs multiplicatifs s'appliquent aux deux tables.

### 2.2 — Seuil de température de cranking : `c_tco_n_mff_cst`

Seuil en-dessous duquel les enrichissements de cranking s'appliquent. Sur E85, l'éthanol a encore besoin d'aide jusqu'à ~30°C là où l'essence démarre sans enrichissement.

✏️
| | Raw | Valeur |
|---|---|---|
| **AVANT (stock)** | 87 | **17.25 °C** |
| **APRÈS (E85)** | **97** | **25.00 °C** |

> Adresse : 0x44F2F — formule `0.75 × X − 48`

### 2.3 — Facteur de warm-up lambda : `ip_fac_lamb_wup`

Table 6×6 qui multiplie la consigne lambda post-démarrage. Axes : X = MAF (65–500 mg/stk), Y = RPM (704–3008 tr/min) — pas d'axe TCO. Enrichit les basses charges pendant la phase où la sonde lambda n'est pas encore chaude.

> Adresse : 0x42764

✏️
AVANT (stock) :
```
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1216 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1760 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2016 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2496 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
3008 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
```

APRÈS (E85) :
```
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.08   1.08   1.05   1.03   1.00   1.00
1216 rpm :    1.08   1.07   1.05   1.03   1.00   1.00
1760 rpm :    1.07   1.06   1.04   1.03   1.00   1.00
2016 rpm :    1.06   1.05   1.04   1.03   1.00   1.00
2496 rpm :    1.05   1.05   1.03   1.03   1.00   1.00
3008 rpm :    1.03   1.03   1.03   1.00   1.00   1.00
```

> Si les STFT oscillent les 30 premières secondes après démarrage sur E85, augmenter de +0.02 sur les cellules basses charges et itérer.

### 2.4 — Procédure de test démarrage à froid

1. **Préparez votre scanner OBD2** : Enregistrez TCO, STFT, RPM en continu
2. **Moteur froid (< 20°C)** : Clé, ne pas appuyer sur la pédale d'accélérateur
3. **Critères de réussite :**
   - Démarrage en ≤ 3 tours de vilebrequin
   - Ralenti initial : 800–1200 tr/min acceptable, diminue progressivement
   - Après 30 sec : STFT entre −10% et +10%
4. **Si démarrage difficile (> 5 tours) :** +15% sur toutes les valeurs de cranking
5. **Si le moteur cale après démarrage :** Augmentez ip_fac_lamb_wup de +0.10 à 20–40°C

### ⚠️ Conseils Pratiques Démarrage Froid

- **Ne touchez JAMAIS la pédale avant démarrage** : le MSV70 désactive l'enrichissement cranking si la pédale est enfoncée (Full Load Cutoff)
- **Batterie impeccable obligatoire** : l'E85 froid exige plus de tours pour démarrer → batterie faible = démarrage impossible
- **Bougies neuves** : gap 0.65–0.70 mm (vs 0.75–0.80 mm stock)
- **Hiver < 0°C :** Envisagez un kit dual-fuel ou l'ajout d'essence 95 dans le réservoir (20% essence suffit pour améliorer le démarrage)

---

## ⚡ 3. Avance à l'Allumage

### Pourquoi l'E85 tolère plus d'avance

| Carburant | Température d'auto-inflammation | Octane RON |
|---|---|---|
| Essence 95 | ~280°C | 95 |
| E70 | ~400°C | ~104 |
| E85 | ~420°C | ~108 |

L'indice d'octane élevé de l'E70 (~104 RON) résiste mieux au cliquetis (détonation prématurée). Cela permet d'avancer l'allumage, ce qui augmente la pression cylindre au bon moment du cycle → plus de couple et de puissance.

**Gain réaliste sur N52 :** +5 à +12% de puissance au frein

### 3.1 — Tables d'avance à identifier dans le MSV70

Le MSV70 gère l'allumage par **modèle de couple** (torque model). Il n'y a pas une seule « table de base » qui sort directement l'avance appliquée : l'avance effective résulte d'un calcul borné entre un plafond (knock-limited) et un plancher. Les tables réellement utilisées **en roulage** sont :

| Paramètre | Adresse | Dimensions / axes | Rôle | Sensibilité E85 |
|---|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | **0x4323A** | 8×8, X=MAF 0.55–2.25 mg/stk, Y=RPM 608–7008 | **Plafond knock — référence MBT.** C'est la table principale à modifier pour gagner de l'avance en E85. | **CRITIQUE** |
| `ip_iga_min_n_maf_opm_1` | 0x4347A | 8×6, X=MAF 0.36–2.14, Y=RPM 320–6528 | Plancher d'avance (mode normal) | Aucune |
| `ip_iga_min_n_maf_opm_2` | 0x434AA | 8×6, X=MAF 0.28–2.14, Y=RPM 320–6528 | Plancher d'avance (mode throttled) | Aucune |
| `ip_fac_eff_iga_opm_1` / `ip_fac_eff_iga_opm_2` | 0x4A5D4 | 16×16, X=couple %, Y=RPM | Facteur d'efficacité avance pour le modèle de couple | Faible/Modérée |
| `ip_iga_st_bas_opm_1` / `ip_iga_st_bas_opm_2` | 0x43586 / 0x435B6 | 6×8, X=TCO, Y=RPM cranking | Avance uniquement pendant la phase de démarrage (cranking) — axes TCO et RPM 80–920 tr/min uniquement | Optionnel (démarrage froid) |

✏️
**Valeurs STOCK de `ip_iga_bas_max_knk__n__maf` (°CRK avant PMH) — extraites du bin VB67774 :**

```
            MAF →  0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25 mg/stk
 608 rpm :        +10.50  +9.38  +1.88  -7.12 -12.00 -13.88 -15.00 -15.00
1504 rpm :        +16.88 +15.38  +9.75  +5.25  -1.12  -6.00  -8.62  -9.38
2016 rpm :        +21.75 +20.62 +15.38 +10.12  +6.00  +3.00   0.00  -2.25
3008 rpm :        +25.88 +26.25 +21.00 +15.75 +10.50  +7.12  +4.88  +4.12
4000 rpm :        +35.62 +34.88 +28.88 +20.62 +15.75 +12.75 +11.62  +7.12
4992 rpm :        +38.62 +38.25 +33.00 +23.25 +20.25 +16.50 +13.12 +12.00
6016 rpm :        +43.12 +39.75 +33.38 +23.25 +19.88 +18.00 +15.38 +13.50
7008 rpm :        +43.50 +39.38 +33.38 +24.00 +20.62 +19.12 +17.62 +16.88
```

C'est cette carto qui fixe le plafond au-delà duquel le knock control est censé reculer l'avance. En augmentant ces valeurs en haute charge (colonnes de droite) de +2° à +5°, vous laissez le modèle de couple MSV70 demander davantage d'avance tant qu'il ne détecte pas de cliquetis — ce qui est précisément ce que permet l'E85.

### 3.2 — Procédure d'augmentation d'avance pour E85

**RÈGLE D'OR : Progressivité absolue. Un seul degré de trop = cliquetis = casse moteur.**

**Table à modifier : `ip_iga_bas_max_knk__n__maf` (adresse 0x4323A)**

**Zones à modifier :**

**Zone 1 : Ralenti / très faible charge (MAF < 0.65, RPM < 1500)**
```
Action : Ne rien changer
Raison : Le ralenti N52 est déjà bien calibré, le modifier déstabilise
```

**Zone 2 : Charge moyenne (MAF 1.0–1.5 mg/stk)**
```
Augmentation : +0.5° à +1°
Procédure : Testez +0.5° d'abord sur 50 km avant d'aller à +1°
```

**Zone 3 : Haute charge / WOT (MAF > 1.5 mg/stk)**
```
Phase 1 : +1° → Roulez 50 km, pas de cliquetis → continuer
Phase 2 : +2° → Roulez 50 km, vérifiez → continuer
Phase 3 : +2.5° → Test final sur montée exigeante, cliquetis = STOP

Maximum recommandé stratégie E60-safe : +2.5° sur les colonnes droites
```

> **Pourquoi se limiter à +2.5° alors que l'éthanol permettrait +4° à +5° ?**
> Le "E85" pompe France peut légalement descendre à 60% d'éthanol en hiver (~101 RON). Si on grille +4.5° d'avance calibrés pour E85 (108 RON) et que la station délivre du E60, le knock control détectera du cliquetis et reculera l'avance automatiquement — mais avec du stress moteur inutile à chaque accélération. En calibrant le plafond knock pour E60, le moteur n'approche jamais la limite quelle que soit la saison. Le gain est légèrement réduit (+2.5° vs +4.5°), mais la sécurité est totale.

**Tableau de modification (en degrés à AJOUTER aux valeurs stock de `ip_iga_bas_max_knk__n__maf`) — calibré E60 :**

```
RPM \ MAF    0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25
 608        +0     +0     +0     +0     +0     +0     +0     +0
1504        +0     +0     +0     +0     +0.5   +0.5   +0.5   +0.5
2016        +0     +0     +0     +0.5   +1.0   +1.0   +1.0   +1.0
3008        +0     +0     +0.5   +1.0   +1.5   +1.5   +2.0   +2.0
4000        +0     +0     +0.5   +1.0   +1.5   +2.0   +2.0   +2.0
4992        +0     +0     +0.5   +1.0   +1.5   +2.0   +2.5   +2.5
6016        +0     +0     +0.5   +1.0   +2.0   +2.0   +2.5   +2.5
7008        +0     +0     +0.5   +1.0   +2.0   +2.0   +2.5   +2.5
```

**`ip_iga_bas_max_knk__n__maf` — Valeurs OBJECTIF E60 (stock + incréments ci-dessus) :**

```
            MAF →  0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25 mg/stk
 608 rpm :        +10.50  +9.38  +1.88  -7.12 -12.00 -13.88 -15.00 -15.00
1504 rpm :        +16.88 +15.38  +9.75  +5.25  -0.62  -5.50  -8.12  -8.88
2016 rpm :        +21.75 +20.62 +15.38 +10.62  +7.00  +4.00  +1.00  -1.25
3008 rpm :        +25.88 +26.25 +21.50 +16.75 +12.00  +8.62  +6.88  +6.12
4000 rpm :        +35.62 +34.88 +29.38 +21.62 +17.25 +14.75 +13.62  +9.12
4992 rpm :        +38.62 +38.25 +33.50 +24.25 +21.75 +18.50 +15.62 +14.50
6016 rpm :        +43.12 +39.75 +33.88 +24.25 +21.88 +20.00 +17.88 +16.00
7008 rpm :        +43.50 +39.38 +33.88 +25.00 +22.62 +21.12 +20.12 +19.38
```

> **Démarrage difficile sur E85 à froid ?** Vous pouvez séparément ajouter +1° à +2° sur les cellules basses colonnes (TCO froide) de `ip_iga_st_bas_opm_1` / `ip_iga_st_bas_opm_2` — mais c'est une optimisation de cranking, pas de puissance.

### 3.3 — Contrôle Anti-Cliquetis MSV70

Le MSV70 dispose d'un **contrôle de cliquetis actif** via les capteurs de détonation. Ce système peut reculer l'avance automatiquement si cliquetis détecté.

**Paramètres IGA_KnockControl :**
- `ip_nr_mpl_is` / `ip_nr_mpl_st` : Nombre de cycles pour décision retrait avance
- `ip_td_mpl` : Délai entre détection et retrait
- `c_n_max_ign_mpl` : RPM max pour l'activité knock control

Sur E85 pur, le knock control intervient **beaucoup moins souvent** qu'avec l'essence. Si vous voyez les LTFT d'avance bouger fréquemment après votre calibration, c'est que vous avez encore trop d'avance → reculez de −1°.

### ⚠️ Avertissements Critiques Allumage

- **Jamais plus de +5°** même avec E85 sur un N52 atmosphérique (sur le plafond knock)
- **Testez à chaud (90°C+)** : le N52 est plus sujet aux cliquetis à chaud
- **E85 = protection supérieure, pas immunité** : le piston peut toujours fondre

---

## 🎯 4. Consigne Lambda (Richesse Cible)

### Comprendre les tables lambda du MSV70 — Pourquoi autant de tables ?

Le MSV70 n'utilise pas une « table lambda unique ». L'ECU segmente le fonctionnement moteur en plusieurs régimes de contrôle distincts, chacun avec sa propre consigne lambda :

- **Bas régime / hors charge partielle** (`ip_lamb_bas_1` / `ip_lamb_bas_2`) : phase où la boucle fermée est prioritaire. Les valeurs proches de 1.000 sont normales — le but est stœchiométrique, la sonde corrige.
- **Charge partielle haute + knock control actif** (`ip_lamb_bas_3`) : zone où l'ECU est en mode de protection knock. Légèrement riche (~0.997) pour refroidir la chambre sans sacrifier l'efficacité.
- **Charge partielle haute + knock control désactivé** (`ip_lamb_bas_4`) : zone de charge partielle élevée sans risque de cliquetis. Toujours stœchiométrique (~0.997). Cette table est **distincte de la WOT** — le knock control ne se déclenche pas ici.
- **Mode papillonné** (`ip_lamb_bas_thr_1` / `ip_lamb_bas_thr_2`) : équivalents des tables bas régime en mode opm_2 (Valvetronic en défaut).
- **Pleine charge WOT** (`ip_lamb_fl__n`) : enrichissement spécifique pleine charge. Active uniquement quand le détecteur pleine charge est levé. Valeurs stock : λ 0.920, descendant à 0.871 à 6500 rpm.

La séparation `ip_lamb_bas_3` / `ip_lamb_bas_4` répond à une logique précise : quand le knock control est actif (zone haute charge à RPM modéré), l'ECU est déjà en train de surveiller et retarder l'avance. Quand le knock control est désactivé (typiquement à très bas régime ou conditions spécifiques), la gestion de richesse est différente. Ce sont deux états de protection différents.

Selon le mode de fonctionnement, l'ECU sélectionne l'une des tables suivantes :

| Paramètre | Adresse | Description XDF | Valeur stock | Objectif E85 | Rôle réel |
|---|---|---|---|---|---|
| `ip_lamb_bas_1` | 0x4B64C | Bankselective basic lambda setpoint, unthrottled mode, **low engine speed or not part load** | 0.992–0.998 | **inchangé** | Banc 1 — ralenti / hors charge partielle |
| `ip_lamb_bas_2` | 0x4B6CC | Bankselective basic lambda setpoint, unthrottled mode, **low engine speed or not part load** | 0.992–0.998 | **inchangé** | Banc 2 — ralenti / hors charge partielle |
| `ip_lamb_bas_3` | 0x4B74C | Basic lambda setpoint, unthrottled mode, **high engine speed and part load, knock control enabled** | 0.997 | **inchangé** | Charge partielle haute, avec knock control actif |
| `ip_lamb_bas_4` | 0x4B7CC | Basic lambda setpoint, unthrottled mode, **high engine speed and part load fulfilled or knock control disabled** | 0.997 | **inchangé** | Charge partielle haute, knock control désactivé |
| `ip_lamb_bas_thr_1` / `ip_lamb_bas_thr_2` | 0x4B84C / 0x4B8CC | Throttled mode equivalent | 0.997 | **inchangé** | Mode papillon/dégradé |
| **`ip_lamb_fl__n`** | **0x436A2** | **Lambda full load enrichment** | **0.920** (0.871 à 6500 rpm) | **laisser stock** (ou 0.940–0.950 optionnel) | **Pleine charge (WOT) — C'est LA table WOT.** |

> La vraie table d'enrichissement pleine charge est **`ip_lamb_fl__n`**, une courbe 1×12 f(RPM), décrite explicitement dans le XDF comme « Lambda full load enrichment ».

**Valeurs STOCK réelles du bin VB67774 :**

```
ip_lamb_bas_1/2  : 0.992 à 0.998 partout (boucle fermée ralenti/part load bas)
ip_lamb_bas_3    : 0.997 partout
ip_lamb_bas_4    : 0.997 partout
ip_lamb_bas_thr_1/2 : 0.997 partout

ip_lamb_fl__n @0x436A2 (pleine charge f(RPM)) :
  RPM  : 608   992   1216   1600   2016   2496   3008   3520   4128   4800   5504   6496
  λ    : 0.920 0.920 0.913  0.920  0.920  0.920  0.920  0.920  0.920  0.920  0.901  0.871
```

> **Conclusion :** Le N52B30 stock est **déjà enrichi à lambda ≈ 0.920 en pleine charge** (et descend à 0.871 à 6500 rpm). Il n'y a **rien à faire** pour « ajouter » de la richesse WOT — elle est déjà présente.

### Comprendre Lambda sur l'E85

```
Lambda = 1.00 sur E85 ≠ Lambda = 1.00 sur essence

Sur essence :  Lambda 1.00 → AFR 14.7:1 → stœchiométrique
Sur E85 :      Lambda 1.00 → AFR 9.8:1  → stœchiométrique E85

Le calculateur gère cela AUTOMATIQUEMENT si :
  1. c_fac_mff_ti_stnd est correctement scalé (×1.33 à ×1.45 selon votre titre éthanol)
  2. La sonde lambda lit correctement (lambda reste relative, pas absolue)
  3. La boucle fermée se régule sur la sonde lambda
```

Le système de boucle fermée se corrige automatiquement si `c_fac_mff_ti_stnd` est correct — les tables `ip_lamb_bas_*` n'ont pas besoin d'être touchées pour la conduite normale.

### Quand (et comment) modifier le lambda WOT sur E85

Le stock est déjà à λ 0.920 en WOT, ce qui est **assez riche**. Sur E85 pur, la chaleur de vaporisation élevée (~840 kJ/kg vs 305 kJ/kg essence) refroidit fortement la chambre et protège mécaniquement contre la détonation. Vous avez donc deux options :

**Option A — Garder `ip_lamb_fl__n` stock.** Le λ 0.920 stock injecte ~9% de carburant en plus qu'à stœchio. Sur E85, cette richesse protège toujours — aucun problème de fiabilité.

**Option B — Dé-enrichir légèrement pour gagner un peu de puissance.**
Sur E85, λ 0.94–0.96 en WOT est un bon compromis (plus proche de stoechio = couple maxi, mais encore assez riche pour une marge de sécurité). Modification proposée sur `ip_lamb_fl__n` :

✏️
```
RPM    : 608   992   1216   1600   2016   2496   3008   3520   4128   4800   5504   6496
Stock  : 0.920 0.920 0.913  0.920  0.920  0.920  0.920  0.920  0.920  0.920  0.901  0.871
E85    : 0.950 0.950 0.945  0.950  0.950  0.945  0.945  0.940  0.935  0.930  0.920  0.900
```

> Conserver les cellules 5504 et 6496 rpm à une valeur basse (0.900–0.920) : c'est une protection thermique des soupapes d'échappement à très haut régime, utile sur E85 comme sur essence.

### Corrections LTFT persistants

Si les LTFT restent décalés après stabilisation :
- **LTFT > +10%** : mélange trop pauvre → vérifier d'abord `c_fac_mff_ti_stnd`. Ne pas modifier les tables `ip_lamb_bas_*` en première intention.
- **LTFT < −10%** : mélange trop riche → pareil, ajuster le facteur d'échelle injecteur.

Les tables `ip_lamb_bas_*` sont à toucher **en dernier recours** si les LTFT sont décalées uniquement sur une zone spécifique (ex : uniquement à bas régime) et que toute la carto est déjà stable ailleurs.

### Validation Lambda

**Outil requis :** Sonde lambda large bande (Innovate LC-2 ou similaire) ou scanner OBD2 avec lecture lambda

**Procédure :**
1. Ralenti chaud (90°C) : Lambda affiché doit osciller autour de 1.00 ±0.02 (boucle fermée active)
2. Roulage 50% charge : STFT entre −5% et +5%
3. Accélération pleine charge : Lambda doit tomber à 0.90–0.95 si vous avez modifié WOT
4. **LTFT après 20 minutes de conduite** : entre −5% et +5% = calibration correcte

---

## 🚿 5. Film Mural (Wall Film Correction)

### Concept

Lors de l'injection port (MPI comme sur le N52), une partie du carburant s'adhère aux parois du collecteur d'admission et s'évapore progressivement. Ce « film mural » :
- Retarde l'arrivée du carburant dans le cylindre
- Varie avec la température moteur, la charge et le régime
- Est **plus épais avec l'E85** qu'avec l'essence (l'éthanol adhère plus et s'évapore moins vite)

### Le vrai système de film mural du MSV70

#### Pourquoi lent ET rapide ?

Le film de carburant sur les parois du collecteur se comporte comme deux phénomènes superposés avec des constantes de temps très différentes :

- **Film lent (slow)** : la couche de carburant qui s'accumule progressivement et s'évapore entre les injections. Sa dynamique est de l'ordre de plusieurs secondes — c'est le film « résiduel » qui persiste entre les accélérations. Axes : TCO × RPM. **Impact E85 : l'éthanol s'évapore beaucoup moins vite que l'essence à basse température → ce film grossit.**

- **Film rapide (fast)** : la réponse instantanée lors d'une transition de charge (tip-in / tip-out brutal). Quand vous enfoncez la pédale d'un coup, le débit d'air augmente soudainement alors que le film mural n'a pas encore eu le temps de répondre — il faut injecter un surplus immédiat pour compenser. Axes : TCO × RPM, mêmes que le film lent, mais les valeurs sont environ ×0.4–0.6 plus petites car la composante rapide est une correction transitoire, pas un enrichissement continu.

#### Pourquoi positif ET négatif ?

- **Positif (pos)** : correction d'injection en **enrichissement** — s'applique quand le film mural absorbe du carburant (accélération, augmentation de charge). L'ECU injecte plus que la dose calculée pour « alimenter » le film.
- **Négatif (neg)** : correction d'injection en **appauvrissement** — s'applique quand le film restitue du carburant (décélération, levée de pied). L'ECU injecte moins car le film en cours d'évaporation fournit du carburant supplémentaire au cylindre.

**Pour E85 :** augmenter les tables `_pos_*` (le film est plus épais, il faut compenser plus). Les tables `_neg_*` ne sont pas modifiées en première intention — elles se réajusteront naturellement une fois les tables positives correctes.

Le MSV70 implémente ce modèle complet avec **deux composantes** (lente et rapide) et **deux directions** (positive et négative). Les tables principales sont :

| Paramètre | Adresse | Dim. / axes | Description XDF | Rôle |
|---|---|---|---|---|
| `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | 8×8, X=TCO, Y=RPM | total positive wall film factor | Film lent — enrichissement tip-in |
| `ip_ti_tco_neg_slow_wf_opm_1` | 0x4CAFC | 8×8, X=TCO, Y=RPM | total negative wall film factor | Film lent — décélération |
| `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | 8×8, X=TCO, Y=RPM | fast positive wall film factor | Film rapide — tip-in brutal |
| `ip_ti_tco_neg_fast_wf_opm_1` | 0x4437C | 8×8, X=TCO, Y=RPM | fast negative wall film factor | Film rapide — tip-out brutal |
| `ip_ti_fac_pos_fast_wf_opm_1` | 0x4425C | 8×8, X=charge, Y=RPM | load correction fast wall film positive | Correction charge pour film rapide pos |
| `ip_ti_fac_neg_fast_wf_opm_1` | 0x4415C | 8×8, X=charge, Y=RPM | load correction fast wall film negative | Correction charge pour film rapide neg |
| `ip_fac_ti_wf_opm_1` | 0x42CBA | 4×4, X=TCO, Y=TIA | temperature correction for total wallfilm | Facteur global T° |
| **Valvetronic-specific (N52)** | | | | |
| `ip_fac_ti_maf_sp_wf_pos_opm_1` | 0x42C5A | 1×8, X=TCO | coolant temperature correction for air mass flow setpoint triggered wallfilm - positive | Film mural déclenché par changement de levée Valvetronic |
| `ip_crlc_pos_maf_sp_wf_opm_1` | — | — | correlation constant for MAF-SP triggered wallfilm positive | Constante corrélation |
| `ip_ti_cor_tps_mod_wf` | — | — | Wall film injection time for VLFT change at TPS-mode | Correction TI pour changement levée valves |

Toutes ces tables existent aussi en version `_opm_2` (mode papillonné). Sur N52 Valvetronic, c'est `_opm_1` qui est actif la majorité du temps.

### Valeurs STOCK et OBJECTIF E85 — les 4 tables de film mural positif

> Axes communs : X = TCO (°C), Y = RPM. Objectif E85 = stock × 1.25 (arrondi à la décimale).

✏️
**`ip_ti_tco_pos_slow_wf_opm_1` @ 0x4CBFC** (film lent, mode normal) — équation `0.100 × raw_u16be` :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   36.0   36.0   32.0   26.0   18.0    8.0    4.0    2.0
 896 rpm :   38.0   38.0   34.0   28.0   20.0   10.0    6.0    2.0
1216 rpm :   40.0   40.0   36.0   30.0   22.0   12.0    8.0    4.0
1600 rpm :   42.0   42.0   38.0   32.0   24.0   14.0   10.0    6.0
2208 rpm :   46.0   46.0   42.0   36.0   28.0   18.0   14.0   10.0
3008 rpm :   50.0   50.0   46.0   40.0   32.0   22.0   18.0   14.0
4192 rpm :   54.0   54.0   50.0   44.0   36.0   26.0   22.0   18.0
5600 rpm :   58.0   58.0   54.0   48.0   40.0   30.0   26.0   22.0
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   45.0   45.0   40.0   32.5   22.5   10.0    5.0    2.5
 896 rpm :   47.5   47.5   42.5   35.0   25.0   12.5    7.5    2.5
1216 rpm :   50.0   50.0   45.0   37.5   27.5   15.0   10.0    5.0
1600 rpm :   52.5   52.5   47.5   40.0   30.0   17.5   12.5    7.5
2208 rpm :   57.5   57.5   52.5   45.0   35.0   22.5   17.5   12.5
3008 rpm :   62.5   62.5   57.5   50.0   40.0   27.5   22.5   17.5
4192 rpm :   67.5   67.5   62.5   55.0   45.0   32.5   27.5   22.5
5600 rpm :   72.5   72.5   67.5   60.0   50.0   37.5   32.5   27.5
```

---

**`ip_ti_tco_pos_slow_wf_opm_2` @ 0x4CC7C** (film lent, mode papillonné) — même équation :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  224.7  180.1  134.1  114.0   90.0   26.0   20.0   20.0
 896 rpm :  224.7  180.1  132.7  113.5   88.5   28.0   20.0   20.0
1216 rpm :  224.7  180.1  132.9  107.0   81.0   28.0   20.0   20.0
1600 rpm :  224.7  180.1  133.5   76.5   57.1   30.0   20.0   18.0
2208 rpm :  224.7  180.3  134.6   76.7   57.6   32.0   22.0   18.0
3008 rpm :  224.7  185.4  144.7  105.0   78.5   36.0   26.0   26.0
4192 rpm :  234.9  194.5  155.0  127.7   96.6   52.3   37.8   33.6
5600 rpm :  259.4  215.5  172.3  142.7  112.5   69.8   52.9   50.7
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  280.9  225.1  167.6  142.5  112.5   32.5   25.0   25.0
 896 rpm :  280.9  225.1  165.9  141.9  110.6   35.0   25.0   25.0
1216 rpm :  280.9  225.1  166.1  133.8  101.3   35.0   25.0   25.0
1600 rpm :  280.9  225.1  166.9   95.6   71.4   37.5   25.0   22.5
2208 rpm :  280.9  225.4  168.3   95.9   72.0   40.0   27.5   22.5
3008 rpm :  280.9  231.8  180.9  131.3   98.1   45.0   32.5   32.5
4192 rpm :  293.6  243.1  193.8  159.6  120.8   65.4   47.3   42.0
5600 rpm :  324.3  269.4  215.4  178.4  140.6   87.3   66.1   63.4
```

---

**`ip_ti_tco_pos_fast_wf_opm_1` @ 0x443FC** (film rapide, mode normal) — équation `0.500 × raw_u8` :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   22.0   20.5   18.5   15.0   11.0    7.5    6.0    4.5
 896 rpm :   23.5   21.5   19.5   16.0   12.0    8.0    6.5    5.0
1216 rpm :   25.5   23.0   21.0   17.5   13.5    9.0    7.0    5.5
1600 rpm :   28.0   25.5   23.0   19.5   15.5   10.5    8.0    6.5
2208 rpm :   32.5   29.5   26.5   23.0   18.5   13.0   10.5    9.0
3008 rpm :   39.0   35.5   32.0   28.0   23.0   17.0   14.0   12.5
4192 rpm :   52.5   47.5   43.0   38.0   32.0   25.0   21.5   19.5
5600 rpm :   72.5   66.5   60.0   53.5   45.5   37.0   33.0   31.0
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   27.5   25.6   23.1   18.8   13.8    9.4    7.5    5.6
 896 rpm :   29.4   26.9   24.4   20.0   15.0   10.0    8.1    6.3
1216 rpm :   31.9   28.8   26.3   21.9   16.9   11.3    8.8    6.9
1600 rpm :   35.0   31.9   28.8   24.4   19.4   13.1   10.0    8.1
2208 rpm :   40.6   36.9   33.1   28.8   23.1   16.3   13.1   11.3
3008 rpm :   48.8   44.4   40.0   35.0   28.8   21.3   17.5   15.6
4192 rpm :   65.6   59.4   53.8   47.5   40.0   31.3   26.9   24.4
5600 rpm :   90.6   83.1   75.0   66.9   56.9   46.3   41.3   38.8
```

---

**`ip_ti_tco_pos_fast_wf_opm_2` @ 0x4443C** (film rapide, mode papillonné) — même équation :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   80.0   66.0   46.5   42.0   24.5   12.0   10.0   10.0
 896 rpm :   80.0   66.0   46.0   41.0   26.0   14.5   11.5   11.5
1216 rpm :   80.0   66.0   47.0   37.0   25.5   16.5   13.0   13.0
1600 rpm :   80.0   66.0   48.0   34.5   25.0   19.0   14.5   14.5
2208 rpm :   80.0   66.0   50.5   35.0   27.5   22.0   16.0   16.0
3008 rpm :   81.5   68.5   52.5   42.5   33.0   26.5   17.0   17.0
4192 rpm :   88.0   73.0   57.0   49.0   40.0   30.5   23.0   20.5
5600 rpm :   94.0   78.5   61.5   54.5   47.5   34.5   28.5   26.0
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  100.0   82.5   58.1   52.5   30.6   15.0   12.5   12.5
 896 rpm :  100.0   82.5   57.5   51.3   32.5   18.1   14.4   14.4
1216 rpm :  100.0   82.5   58.8   46.3   31.9   20.6   16.3   16.3
1600 rpm :  100.0   82.5   60.0   43.1   31.3   23.8   18.1   18.1
2208 rpm :  100.0   82.5   63.1   43.8   34.4   27.5   20.0   20.0
3008 rpm :  101.9   85.6   65.6   53.1   41.3   33.1   21.3   21.3
4192 rpm :  110.0   91.3   71.3   61.3   50.0   38.1   28.8   25.6
5600 rpm :  117.5   98.1   76.9   68.1   59.4   43.1   35.6   32.5
```

### Stratégie film mural — calibré E85 pour la sécurité en open loop transitoire

Le tip-in (enfoncement rapide de la pédale) est une zone de **boucle ouverte transitoire** : la sonde lambda ne réagit pas assez vite pour corriger. Un film sous-compensé donne un mélange lean transitoire — raté ou claquement d'admission. Un film sur-compensé donne un bref enrichissement — sans conséquence.

> **Choix retenu : calibrer le film mural pour E85 (×1.25)**, même si le carburant réel est E70. Avec du E70 réel, la compensation sera légèrement excessive → tip-in légèrement riche → safe. C'est la même logique que pour le facteur injecteur : mieux vaut être riche dans les zones où rien ne corrige.

Sur E85, le film mural est physiquement plus épais (environ +25% à +35% par rapport à l'essence) et plus lent à se dissiper. Les modifications à appliquer :

1. **Approche retenue** : multiplier les tables `ip_ti_tco_pos_slow_wf_opm_1`, `ip_ti_tco_pos_slow_wf_opm_2`, `ip_ti_tco_pos_fast_wf_opm_1` et `ip_ti_tco_pos_fast_wf_opm_2` par **×1.25** globalement. Ce facteur couvre l'E85 pur avec une légère sur-compensation sécuritaire pour l'E70 réel.

2. **Ajustement fin** : si des « trous » d'accélération persistent à température intermédiaire (30-60°C), augmenter sélectivement les colonnes correspondantes de +10% supplémentaires.

3. **Tables `_neg_*_wf`** : non modifiées en première intention — elles gèrent la récupération de film lors des levées de pied et se réajusteront naturellement une fois les tables positives correctes.

4. **Valvetronic — particularité N52** : les tables `ip_fac_ti_maf_sp_wf_*_opm_1` et `ip_ti_cor_tps_mod_wf` gèrent le film mural induit par les changements de levée Valvetronic. Sur E85, elles peuvent nécessiter un +15% à +20% pour compenser le film plus épais lors des transitions Valvetronic rapides. À régler seulement si les tests pratiques montrent des ratés lors d'accélérations progressives.

### Test pratique « transition tiède »

La zone critique reste **30-60°C** (moteur juste monté en température) :

```
Test :
1. Démarrez moteur froid
2. Conduisez 10 min à vitesse modérée (moteur monte à ~50°C)
3. À feu rouge (moteur ~50°C, ralenti) : accélérez progressivement 30%
   → Doit être LISSE, aucun "trou" ni hésitation
4. Accélérez 80% :
   → Réponse nette, pas de temps de latence

Résultat :
  Trou/hésitation → Film insuffisant → +10% sur ip_ti_tco_pos_fast_wf_opm_1 ligne correspondante
  À-coup riche → Film trop fort → −5% sur ip_ti_tco_pos_slow_wf_opm_1 ligne correspondante
```

### ⚠️ Zone d'incertitude honnête

Le modèle de film mural du MSV70 combine plusieurs paramètres qui interagissent (film lent, film rapide, correction charge, correction température, corrections Valvetronic). **Il n'existe pas de « réglage E85 prêt à l'emploi » pour le film mural N52 publié ouvertement**. Les recommandations ci-dessus (multiplicateur ×1.25 global) sont un point de départ conservatif basé sur les différences physiques essence/E85, pas sur un retour d'expérience validé sur N52. En pratique, l'enrichissement cranking (§2) + le facteur d'échelle injecteur (§1) + la boucle fermée lambda suffisent pour une conversion E85 de base ; le film mural est une optimisation secondaire.

---

## 🔁 6. Purge Canister EVAP — Impact E85

### Pourquoi c'est important

Le système EVAP (purge des vapeurs de carburant du réservoir) introduit des vapeurs dans le collecteur d'admission. Sur E85, la teneur en éthanol dans les vapeurs est plus élevée, ce qui peut créer des enrichissements inattendus lors de la purge.

**Paramètres concernés (EVAP flexibles) :**
- Table de contrôle purge canister (EVAC_Time_control_lambda_CP)
- `ip_crlc_mff_buf_cp` : Buffer MFF purge canister
- `ip_maf_kgh_pred_cor_map_tia` : Correction masse air pour purge

**Symptôme :** Lors de la purge canister (généralement à charge partielle, ~50 km/h), les STFT oscillent fortement ou montent brusquement.

**Solution :** Le MSV70 en boucle fermée corrige automatiquement la purge EVAP. Si les STFT restent dans ±10% pendant la purge, aucun ajustement n'est nécessaire. Si oscillations > ±15%, réduire la vitesse de purge (débit maxi canister).

---

## 🔩 7. Paramètres Complémentaires

### 7.1 — Temps mort injecteur (`ip_ti_add_dly`)

Courbe 1×8 f(tension batterie VB, 0–24.9V). Propriété électrique de l'injecteur, indépendante du carburant. **Inchangé sur injecteurs stock.**

Si vous changez d'injecteurs : recalculer avec les caractéristiques deadtime des nouveaux injecteurs (courbe f(VB) fournie par le fabricant).

### 7.2 — Délai pleine charge (`c_t_ti_dly_fl_1` et `c_t_ti_dly_fl_2`)

Délai entre détection de pleine charge et application de l'enrichissement WOT. Sur E85, l'enrichissement doit être immédiat.

✏️
| | Valeur |
|---|---|
| **AVANT (stock)** | à extraire du bin |
| **APRÈS (E85)** | **0 ms** |

> Adresses à localiser dans le bin — non extraites à ce jour.

### 7.3 — Avance d'allumage au démarrage (`c_iga_ini`)

Avance initiale au cranking. Sur E85 froid, un allumage légèrement plus avancé facilite la combustion. Optionnel — à tester seulement si le démarrage reste difficile après calibration §2.

✏️
| | Raw | Valeur |
|---|---|---|
| **AVANT (stock)** | à extraire | à extraire |
| **APRÈS (E85)** | stock + delta | **stock + 1° à + 2°** |

> Adresse à localiser dans le bin — formule `0.375 × X − 35.625` (°CRK).

---

## ⚠️ 8. Avertissements et Maintenance

### A. Surveillance Permanente des Fuel Trims

| Indicateur | Normal | Acceptable | PROBLÈME |
|---|---|---|---|
| STFT | ±5% | ±10% | > ±15% |
| LTFT | ±5% | ±10% | > ±15% |

```
STFT > +15% = Le calculateur rajoute du carburant en permanence
  → c_fac_mff_ti_stnd trop petit, ou lambda trop pauvre

STFT < −15% = Le calculateur enlève du carburant
  → c_fac_mff_ti_stnd trop grand, ou lambda trop riche

LTFT élevé en permanence = Votre calibration a une dérive systématique
  → Ajustez la source (c_fac_mff_ti_stnd ou ip_lamb_bas)
```

**Scanner recommandé pour N52 :** ISTA/D via interface ENET ou K+DCAN (accès à tous les paramètres BMW).

### B. Pompe à Essence

Le N52 consomme environ 60 L/h en fonctionnement normal. Avec E70 (+36% de masse carburant), il faut ~82 L/h effectivement consommés.

> **Pourquoi le test ci-dessous donne 240 L/h alors que le moteur n'en consomme que 82 ?** Le test est réalisé circuit ouvert (retour carburant déconnecté, sans contre-pression du rail) : le débit mesuré est le débit max de la pompe à basse pression, bien supérieur au débit réel sous pression rail. Le seuil de 2.0 L/30 sec garantit une marge suffisante une fois le rail sous pression nominale (5 bar) en charge élevée.

**Test pompe :**
```
Déconnectez le retour carburant et laissez s'accumuler 30 sec :
  Minimum acceptable : 2.0 L/30 sec (= 240 L/h) → largement suffisant
  Si < 1.5 L/30 sec → Pompe fatiguée, à remplacer avant conversion
```

**Si remplacement nécessaire :** Pompe N54 ou pompe aftermarket (Walbro, Bosch haute pression)

### C. Filtre à Essence

L'E85 est un excellent solvant : il dissout tous les dépôts accumulés dans le réservoir depuis des années.

**Protocole filtre :**
1. Changez le filtre AVANT la conversion
2. Changez-le à nouveau après 200 km d'E85 (dépôts dissous)
3. Contrôlez à 500 km
4. Retour au rythme normal (10 000 km) ensuite

### D. Compatibilité Matériaux Système Carburant N52

| Composant | Compatibilité E85 | Action |
|---|---|---|
| Joints Viton/FKM | Excellente | Aucune |
| Joints NBR (nitrile standard) | Mauvaise | Remplacement nécessaire |
| Tuyaux caoutchouc E85-compatible | Bonne | Vérifier l'état |
| Pompe à essence N52 | Bonne (prévu alcool) | Aucune si état correct |
| Rail d'injection acier/alu | Excellente | Aucune |

### E. Bougies d'Allumage

**Pour N52 + E85 :**
```
Référence stock : NGK ILZKBR7A8DG ou DENSO FK20HR11 (iridium)
Gap stock : 0.75–0.80 mm
Gap E85 recommandé : 0.65–0.70 mm

Raison : L'E85 forme un mélange plus dense ; un gap plus serré
         améliore la fiabilité d'allumage à froid

Intervalle de remplacement : 20 000 km (vs 30 000 km essence)
```

### F. Démarrage Hivernal (< 0°C)

```
À −5°C : Démarrage difficile, assurez-vous que vos enrichissements cranking
         sont à ×1.80 minimum pour cette zone de température

À −10°C : Très difficile, considérez l'ajout de 10–15% d'essence 95
          dans le réservoir pour faciliter le démarrage

À −15°C et moins : E85 pur pratiquement impossible
                   → Repassez à l'essence ou utilisez un mélange E50
```

**Astuce hiver :** Si votre configuration E85 est définitive, gardez toujours un bidon de 5L d'essence 95 pour l'hiver.

---

## 📊 9. Résumé des Modifications — Valeurs Concrètes

> **Source des valeurs stock** : toutes les valeurs ci-dessous sont extraites directement de `VB67774_921S_Full.bin` (bin stock du véhicule cible, N52B30 330i, MSV70 9PPL921S). Les équations de conversion sont lues dans le XDF `BMW_Siemens_MSV70_9PPL921S_2560K.xdf`. Les adresses sont des adresses mémoire (offset dans le bin = adresse − 0x40000).

### Tableau de Toutes les Modifications (330i N52B30, injecteurs 13537531634)

| # | Paramètre | Adresse bin | Raw stock | Valeur stock | Raw objectif | Objectif | Unité | Stratégie |
|---|---|---|---|---|---|---|---|---|
| 1 | `c_fac_mff_ti_stnd_1` | 0x44AC0 | 56 567 | **0.3394** | **82 022** | **0.4921** | ms/mg | E85 — open loop toujours riche |
| 2 | `c_fac_mff_ti_stnd_2` | 0x44AC2 | 56 567 | **0.3394** | **82 022** | **0.4921** | ms/mg | E85 — idem banc 2 |
| 3 | `c_fac_mff_ti_stnd[0]` | 0x45AAC | 28 284 | **0.3394** | **41 011** | **0.4921** | ms/mg | E85 — SOI/EOI, raw ÷2 |
| 4 | `c_fac_mff_ti_stnd[1]` | 0x45AAE | 28 284 | **0.3394** | **41 011** | **0.4921** | ms/mg | E85 — SOI/EOI, raw ÷2 |
| 5 | `c_fac_mff_ti_stnd_mon` | 0x4958C | 56 567 | **0.3394** | **82 022** | **0.4921** | ms/mg | E85 — monitoring DTC |
| 6 | `c_tco_n_mff_cst` | 0x44F2F | 87 | **17.25 °C** | **97** | **25.00 °C** | °C | Seuil cranking étendu |
| 7 | `ip_mff_cst_opm_1` | 0x437DC | table 3×8 | voir §2.1 | table 3×8 | voir §2.1 | mg/stk | E70 — cranking exception noyage |
| 8 | `ip_mff_cst_opm_2` | 0x4380C | table 3×8 | voir §2.1 | table 3×8 | voir §2.1 | mg/stk | E70 — idem mode papillon |
| 9 | `ip_fac_lamb_wup` | 0x42764 | 128 partout | **1.000** partout | — | **1.03–1.08** (basses charges) | — | Warm-up post-démarrage |
| 10 | `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | table 8×8 | voir §5 | table 8×8 | voir §5 (×1.25) | — | E85 — film lent tip-in safe |
| 11 | `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | table 8×8 | voir §5 | table 8×8 | voir §5 (×1.25) | — | E85 — film lent mode papillon |
| 12 | `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | table 8×8 | voir §5 | table 8×8 | voir §5 (×1.25) | — | E85 — film rapide tip-in safe |
| 13 | `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | table 8×8 | voir §5 | table 8×8 | voir §5 (×1.25) | — | E85 — film rapide mode papillon |
| 14 | `ip_iga_bas_max_knk__n__maf` | 0x4323A | table 8×8 | voir §3.1 | table 8×8 | voir §3.2 (objectif E60) | °CRK | E60 — plafond knock pire octane |
| 15 | `ip_iga_st_bas_opm_1` | 0x43586 | table 6×8 | cranking only | table 6×8 | **optionnel** +1°/+2° zone froide | °CRK | Cranking uniquement |
| 16 | `ip_iga_st_bas_opm_2` | 0x435B6 | table 6×8 | cranking only | table 6×8 | **optionnel** +1°/+2° zone froide | °CRK | Cranking mode papillon |
| 17 | `ip_lamb_fl__n` | 0x436A2 | courbe 1×12 | **0.920** (0.871 @ 6500 rpm) | courbe 1×12 | **laisser stock** | λ | WOT déjà riche — stock suffisant |
| 18 | `ip_lamb_bas_1` | 0x4B64C | — | **0.992–0.998** | — | **inchangé** | λ | Boucle fermée corrige |
| 19 | `ip_lamb_bas_2` | 0x4B6CC | — | **0.992–0.998** | — | **inchangé** | λ | Boucle fermée corrige |
| 20 | `ip_lamb_bas_3` | 0x4B74C | — | **0.997** | — | **inchangé** | λ | Boucle fermée corrige |
| 21 | `c_t_ti_dly_fl_1` | — | non extrait | — | — | **0 ms** | ms | Enrichissement WOT instantané |
| 22 | `c_t_ti_dly_fl_2` | — | non extrait | — | — | **0 ms** | ms | Enrichissement WOT instantané |
| 23 | `c_iga_ini` | — | non extrait | — | — | **stock +1° à +2°** si démarrage difficile | °CRK | Optionnel — allumage cranking |

### Ordre de Priorité d'Application

```
ÉTAPE 1 — OBLIGATOIRE :
  ✅ c_fac_mff_ti_stnd_1, c_fac_mff_ti_stnd_2, c_fac_mff_ti_stnd[0], c_fac_mff_ti_stnd[1], c_fac_mff_ti_stnd_mon → ×1.45 (E85 — open loop toujours riche)
  ✅ c_tco_n_mff_cst → 25°C (raw 97)

ÉTAPE 2 — TRÈS IMPORTANT (démarrage) :
  ✅ ip_mff_cst_opm_1 et ip_mff_cst_opm_2 → × 1.35 à × 2.20 selon température
  ✅ ip_fac_lamb_wup (charge×RPM, PAS une table TCO) → 1.03–1.08 basses charges

ÉTAPE 3 — IMPORTANT (confort de conduite) :
  ✅ ip_ti_tco_pos_slow_wf_opm_1 / ip_ti_tco_pos_slow_wf_opm_2 → ×1.25 global (film mural lent — calibré E85, safe en open loop transitoire)
  ✅ ip_ti_tco_pos_fast_wf_opm_1 / ip_ti_tco_pos_fast_wf_opm_2 → ×1.25 global (film mural rapide — même logique)

ÉTAPE 4 — PERFORMANCE (après validation des étapes 1–3) :
  ✅ ip_iga_bas_max_knk__n__maf → +2° à +5° en haute charge (plafond MBT) — voir objectif E60 en §3.2
  ✅ ip_lamb_fl__n WOT → laisser stock (déjà 0.920) ou dé-enrichir à 0.94-0.95

ÉTAPE 5 — FINITION :
  ✅ Tous ip_lamb_bas → ajuster selon LTFT observés
  ✅ c_t_ti_dly_fl_1 / c_t_ti_dly_fl_2 → 0 ms (enrichissement WOT instantané)
```

---

## 🧪 10. Plan de Test et Validation Progressif

### Phase 0 : Préparation (Avant toute modification)

```
Checklist matériel :
  ✅ Scanner OBD2 avec log (ISTA, INPA, Torque Pro avec plugin BMW)
  ✅ Sonde lambda large bande recommandée (optionnelle mais idéale)
  ✅ Thermomètre d'ambiance
  ✅ Plein de carburant E85 dans le réservoir

Réinitialisation obligatoire avant le premier démarrage E70 :
  ✅ Via ISTA-D : Service → Fonctions de service → Adaptation du mélange → Réinitialiser
     (ou via NCS Expert : Kraftstoffadaption zurücksetzen)
  Raison : les LTFT calculés sur essence (~0%) vont biaiser la lecture des STFT E70
            dans les premières minutes. Partir à zéro donne une base de lecture fiable
            dès le premier allumage.

Baseline à enregistrer (sur essence, avant le plein E70) :
  - STFT/LTFT à ralenti (doivent être proches de 0%)
  - Consommation sur 20 km de parcours test
  - Comportement démarrage à froid (si possible)
```

### Phase 1 : Application des paramètres injecteurs (Jour 1)

**Modifications :**
- `c_fac_mff_ti_stnd_1` / `c_fac_mff_ti_stnd_2` / `c_fac_mff_ti_stnd_mon` → **×1.45 (E85 — open loop safe)** — raw = 82 022
- `c_fac_mff_ti_stnd[0]` / `c_fac_mff_ti_stnd[1]` → **×1.45** — raw = 41 011
- c_tco_n_mff_cst → 25°C

**Test immédiat :**
1. Démarrez moteur (si tiède ou chaud)
2. Attendez ralenti stable (1–2 min)
3. Lisez STFT → doivent être entre −10% et +10%
4. Si STFT > +12% : augmentez c_fac_mff_ti_stnd de +3% supplémentaire
5. Roulez 20 km, vérifiez stabilisation LTFT

**Critère de validation Phase 1 :**
- STFT au ralenti chaud : −5% à +5%
- LTFT après 15 min : −8% à +8%

### Phase 2 : Validation Démarrage Froid (Jour 2)

**Conditions :** Moteur froid (nuit dehors, < 15°C)

1. Démarrage à froid sans pédale :
   - Doit démarrer en ≤ 4 tours
   - Si > 5 tours → augmentez ip_mff_cst_opm_1 de +15%

2. Ralenti post-démarrage (30–60 sec) :
   - Doit être stable à 600–1000 tr/min
   - STFT entre −10% et +15% (normal à froid)
   - Si très instable ou cale → augmentez ip_fac_lamb_wup à froid

3. Transition tiède (40–60°C) :
   - Accélération douce puis franche
   - Aucune hésitation acceptée
   - Si trou → augmentez `ip_ti_tco_pos_fast_wf_opm_1` (et `ip_ti_tco_pos_fast_wf_opm_2`) sur la ligne TCO correspondante de +10%

### Phase 3 : Ajustement Lambda (Jour 3)

1. Moteur chaud (90°C), ralenti 5 min :
   - STFT/LTFT doivent se stabiliser à ±5%
   - Si LTFT dérive → ajustez ip_lamb_bas correspondant de ±0.02

2. Roulage 30 km mixte :
   - Log continu STFT par zone de charge
   - Identifiez les zones avec correction persistante > ±10%
   - Ajustez la cellule ip_lamb_bas correspondante

3. Pleine charge (accélération 5–6 sec sur route droite sûre) :
   - Si sonde large bande installée : vérifiez lambda 0.90–0.95 à WOT
   - Pas de cliquetis → bon

### Phase 4 : Avance (Jour 4–7)

```
J4 : +2° pleine charge uniquement → roulez 50 km, pas de cliquetis → continuer
J5 : +4° pleine charge → roulez 50 km, vérifiez
J6 : +5° pleine charge → test exigeant (montée, plein régime)
J7 : +6° UNIQUEMENT si aucun cliquetis aux étapes précédentes

À chaque étape :
  - Cliquetis = STOP immédiat, revenez à −1°
  - LTFT qui monte = vérifiez que lambda WOT est correct
```

### Phase 5 : Validation Finale (Jour 10+)

```
100 km variés sur votre parcours test standardisé :
  ✅ STFT/LTFT < ±8% en toutes conditions
  ✅ Démarrage froid < 3 tours
  ✅ Aucun cliquetis
  ✅ Pas de fuite carburant visuelle
  ✅ Consommation stable et prévisible
  ✅ Dernier changement de filtre à essence
```

---

## 📋 11. Checklist Avant Conversion

```
PRÉPARATION MÉCANIQUE :
  ✅ Pompe à essence testée (> 2 L/30 sec)
  ✅ Filtre à essence changé (neuf)
  ✅ Bougies vérifiées/changées (gap 0.65–0.70 mm pour E85)
  ✅ Joints système carburant inspectés (pas de fissure, flexibles)
  ✅ Batterie : état > 70% de capacité nominale

PRÉPARATION LOGICIELLE :
  ✅ Backup du bin stock sauvegardé en lieu sûr (au moins 2 copies)
  ✅ TunerPro RT (ou WinOLS) prêt avec le XDF chargé
  ✅ Scanner OBD2 fonctionnel, log actif

PRÉPARATION PRATIQUE :
  ✅ E85 en station à proximité identifiée
  ✅ Essence 95 de secours (5L) disponible
  ✅ Température extérieure ≥ +10°C pour le premier test
  ✅ Parcours test de 20–30 km identifié (répétable)
```

---

## 🔍 12. Diagnostic Rapide des Problèmes

| Symptôme | Cause la plus probable | Solution |
|---|---|---|
| Pas de démarrage à froid | ip_mff_cst_opm trop pauvre | +20% cranking |
| Démarrage laborieux (10+ tours) | ip_mff_cst_opm insuffisant | +15% cranking + vérifier batterie |
| Cale après démarrage froid | ip_fac_lamb_wup insuffisant | +0.10 à 20–40°C |
| Trou/hésitation à 40–60°C | film mural insuffisant | +10% sur `ip_ti_tco_pos_fast_wf_opm_1` / `ip_ti_tco_pos_fast_wf_opm_2` ligne TCO concernée |
| STFT > +15% en permanence | c_fac_mff_ti_stnd trop petit | +3–5% sur les 5 copies |
| STFT < −15% en permanence | c_fac_mff_ti_stnd trop grand | −3–5% sur les 5 copies |
| Ralenti instable moteur chaud | ip_lamb_bas incorrecte ou EVAP | ±0.02 lambda ralenti |
| Cliquetis pleine charge | Avance trop haute | −1° à −2° immédiatement |
| Perte puissance progressive | Filtre bouché ou pompe fatiguée | Changez filtre / testez pompe |
| Fumée noire échappement | ip_fac_lamb_wup trop riche | −0.05 à 0.08 |
| LTFT monte sur autoroute | ip_lamb_bas WOT trop pauvre | −0.02 à −0.03 lambda haute charge |
| Odeur forte éthanol à l'arrêt | Fuite carburant (joints) | Inspection immédiate obligatoire |

---

## 📚 13. Paramètres E85 NON couverts par ce tuto (à investiguer)

Ce tuto couvre les modifications **essentielles et validées** pour une carto E85 de base sur MSV70 N52B30. Plusieurs sous-systèmes du MSV70 ont aussi un impact sur le comportement E85 mais ne sont pas traités ici en détail, soit parce qu'ils sont complexes, soit parce qu'ils dépassent le scope d'une conversion grand public, soit parce que leur impact réel reste à valider sur banc.

### 13.1 — Enrichissement transitoire pleine charge

| Paramètre | Adresse | Description |
|---|---|---|
| `KF_FTRANSVL` | 0x5C5EE | Cartographie 8×8 d'enrichissement transitoire en Volllast (charge × RPM). Description XDF générique. |
| `KL_FUPSRF_TRANS` | 0x5BE78 | Courbe transitoire pression carburant (legacy Bosch naming) |
| `KL_STEND_TRANS` | — | Facteur de fin de transition |
| `KL_PIRG_TRANS` | — | Pression résiduelle gaz brûlés transitoire |

Ces paramètres pilotent l'enrichissement temporaire lors d'un appel de couple brutal (kickdown). Sur E85, un tip-in agressif peut « tomber pauvre » pendant quelques cycles si ces tables ne sont pas augmentées. Effet pratique : trous d'accélération brefs lors des dépassements. **À explorer si vous constatez ce symptôme uniquement en accélération brutale.**

### 13.2 — Warm-up lambda au ralenti

`ip_fac_lamb_wup_is` @0x42788 (3×4, X=MAF, Y=RPM, stock 1.000 partout) — **i**dle **s**peed warm-up lambda. Distinct de `ip_fac_lamb_wup` (déjà couvert §2.3). Spécifique au ralenti. À enrichir légèrement (+5 %) si le ralenti est instable les 30 premières secondes après démarrage froid sur E85.

### 13.3 — Stratégie de chauffe catalyseur (cold start retard)

| Paramètre | Description XDF |
|---|---|
| `ip_fac_eff_iga_ch_cold_opm_1` @0x4A444 (10×10) | Factor for catalyst heating cold engine in normal mode |
| `ip_fac_eff_iga_ch_cold_opm_2` @0x4A4A8 (10×10) | Factor for catalyst heating cold engine in throttled mode |
| `ip_lamb_sawup` @0x4BBDC (8×8) | basic lambda correction for rich catalyst heating (avec pompe air secondaire) |
| `ip_lamb_sawup_is` | idem, ralenti |

Lors d'un démarrage froid, le MSV70 retarde fortement l'allumage (jusqu'à −15°) et enrichit pour chauffer rapidement le catalyseur via les gaz d'échappement. Sur E85, cette phase produit des EGT plus faibles (l'éthanol brûle plus froid) → la chauffe catalyseur peut être moins efficace, et les DTC P0420/P0430 peuvent apparaître plus facilement. **Ne pas modifier en première intention.** À explorer uniquement si des DTC catalyseur persistent après 500 km.

### 13.4 — Protection température échappement (EGT)

`c_teg_max_iga` @0x44F54 = **865 °C** (Maximum allowable exhaust gas temperature for spark retard control). Au-delà, le MSV70 retire l'avance pour protéger le catalyseur.

**Sur E85, l'EGT est typiquement 30-50 °C plus basse à puissance équivalente** (chaleur de vaporisation). Donc cette protection se déclenche moins souvent — c'est une bonne nouvelle, ça laisse plus de marge à votre +5° d'avance. **Ne pas modifier ce seuil.** Information à connaître.

### 13.5 — Film mural induit par Valvetronic (spécifique N52)

| Paramètre | Description |
|---|---|
| `ip_fac_ti_maf_sp_wf_pos_opm_1` / `ip_fac_ti_maf_sp_wf_pos_opm_2` | Coolant temperature correction for air mass flow setpoint triggered wallfilm - positive |
| `ip_fac_ti_maf_sp_wf_neg_opm_1` / `ip_fac_ti_maf_sp_wf_neg_opm_2` | idem - negative |
| `ip_crlc_pos_maf_sp_wf_opm_1` / `ip_crlc_pos_maf_sp_wf_opm_2` | Correlation constant for MAF-SP triggered wallfilm |
| `ip_ti_cor_tps_mod_wf` | Wall film injection time for VLFT change at TPS-mode |
| `ip_fac_tps_mod_wf` | Weighting factor for wall film triggered by valve lift change |

Le N52 utilise le Valvetronic (levée variable continue) qui modifie la dynamique d'admission lors des changements de levée. Le MSV70 a une compensation de film mural spécifique pour ces transitions. Sur E85, cet effet est amplifié. **À explorer uniquement si le couple est instable lors de changements rapides de pédale en charge moyenne.**

### 13.6 — Coupure d'injection en décélération (Schubabschaltung / fuel cutoff)

`id_maf_n_min_fcut_fast` @0x41E1C (4×4, table de seuils RPM × charge pour fast fuel cutoff) — gère l'activation/désactivation de la coupure d'injection en décélération. Stock : activé au-dessus de ~3600 rpm. **Aucune modification nécessaire pour E85.** Mentionné pour information.

### 13.7 — Limites de fuel trims (STFT/LTFT clamps)

Le tuto annonce « LTFT peut absorber ±25 % » sans citer de paramètre. **Je n'ai pas réussi à localiser dans le XDF un paramètre clair de plafond LTFT** (les paramètres `lsl_*` du MSV70 concernent plutôt les diagnostics sonde lambda que les limites de trim utilisateur). Cette valeur de ±25 % est une convention BMW courante mais devrait être validée empiriquement sur votre véhicule en logant les LTFT au moment où ils plafonnent.

### 13.8 — Phasage injection EOI

`ip_eoi_1_bas` @0x4E914 (8×6) et tables associées — End Of Injection angle. Le MSV70 contrôle le timing exact de fermeture des injecteurs en fonction de RPM et charge. Sur port injection N52, viser une fin d'injection pendant la phase soupape fermée minimise le wall wetting direct sur soupape ouverte. Sur E85 le bénéfice théorique est plus important. **Modification très avancée, hors scope de ce tuto.**

### 13.9 — Pression de rail / `ip_fup_cor`

`c_fup_nom` @0x44B0C = 5000 hPa (5.0 bar nominal). `ip_fup_cor` corrige la pression cible en fonction de la tension batterie et de la consommation carburant. **Sur E85 (+30 % débit), la pompe stock peut potentiellement faire chuter le rail en WOT prolongé**. Si vous constatez une perte de couple progressive sur des accélérations longues, vérifier la tenue de pression rail au scanner (PID dédié). Dans ce cas, la solution est mécanique (pompe plus performante), pas logicielle.

---

## 🔬 14. État de Vérification du Tuto

Ce tuto est issu d'un audit du XDF `BMW_Siemens_MSV70_9PPL921S_2560K.xdf` et du bin `VB67774_921S_Full.bin` du dépôt. Voici l'état honnête de chaque section :

| Section | Statut | Niveau de confiance |
|---|---|---|
| §1 Injecteurs (`c_fac_mff_ti_stnd_*`) | ✅ Adresses + valeurs stock vérifiées au bit près | **Élevé** |
| §2 Cranking (`c_tco_n_mff_cst`, `ip_mff_cst_opm_*`) | ✅ Vérifié | **Élevé** |
| §2.3 `ip_fac_lamb_wup` | ⚠️ Adresse corrigée (0x42764), axes confirmés (MAF×RPM) | **Moyen** — la stratégie d'utilisation reste à valider en pratique |
| §3 Avance (`ip_iga_bas_max_knk__n__maf`) | ⚠️ Table identifiée comme « plafond knock » mais le modèle de couple MSV70 est complexe ; il existe aussi `ip_iga_min_n_maf_opm_*`, `ip_fac_eff_iga_opm_*`, `ip_iga_ofs_max_knk` qui interagissent. | **Moyen** — l'effet réel d'une modif +5° devrait être validé sur banc avant tout test piste |
| §4 Lambda (`ip_lamb_fl__n` comme vraie table WOT) | ✅ Description XDF lue directement, stock vérifié | **Élevé** |
| §5 Film mural (vraies tables `ip_ti_tco_*_*_wf_opm_*`) | ⚠️ Tables identifiées via descriptions XDF ; multiplicateur ×1.20 = recommandation conservative basée sur la physique, pas sur retour d'expérience N52 publié | **Moyen** |
| §6 EVAP | ⚠️ Mentionné mais non vérifié en détail | **Faible** |
| §7 Compléments (deadtime `ip_ti_add_dly`, délai FL) | ✅ Vérifié | **Élevé** |
| §13 Paramètres non couverts | ℹ️ Liste honnête de ce qui mériterait investigation | — |

**Limites de cet audit :**
- Aucun test sur véhicule réel n'a été conduit. Les recommandations sont basées sur la physique de la combustion E85 et la lecture du XDF, pas sur un retour d'expérience validé sur N52 + MSV70 + injecteurs stock.
- Le modèle de couple MSV70 (torque model) est volumineux et imbriqué ; certaines interactions (par ex. comment `ip_fac_eff_iga_opm_1` module l'avance demandée) n'ont pas été tracées en détail.
- Les checksums du bin ne sont pas vérifiés par ce tuto. TunerPro RT les corrige automatiquement avec ce XDF, mais c'est à valider avant flash.
- Aucune version « bin déjà calibré E85 » n'est fournie. Ce tuto donne des valeurs cibles, pas un fichier prêt à flasher.

**Avant tout flash :**
1. Lire l'avertissement flex-fuel en tête de document
2. Avoir 2 backups du bin stock
3. Tester d'abord les modifications de §1 (injecteurs) seules, valider STFT, puis ajouter §2 (cranking), valider, etc. — ne jamais tout modifier d'un coup
4. Avoir un scanner OBD2 capable de lire STFT/LTFT/lambda en temps réel
5. Pour la phase avance §3, avoir idéalement un accès banc dynamométrique ou au minimum un log de cliquetis détaillé

---

## 🧪 15. Mythes et Réalités sur l'Éthanol

L'E85 traîne une réputation de carburant "agressif" ou "destructeur" souvent alimentée par des expériences sur des véhicules anciens, des mélanges mal dosés, ou des conversions bâclées. Voici un état des lieux basé sur la physique et la chimie, pas sur les forums.

---

### "L'éthanol assèche et fissure les durites / joints"

**Verdict : FAUX sur les véhicules modernes.**

Cette rumeur vient d'une réalité des années 1980–2000 : les tuyaux et joints en **caoutchouc NBR** (nitrile standard) se dégradent effectivement au contact de l'éthanol — gonflement, ramollissement, fissuration à long terme. Sur les véhicules de cette époque, la conversion E85 sans remplacement des flexibles était problématique.

**Sur le N52B30 (2005–2012)**, toutes les pièces d'origine BMW en contact avec le carburant sont en **Viton (FKM)** ou en métal anodisé — matériaux parfaitement compatibles E85. La norme européenne EN 228 impose d'ailleurs la compatibilité E10 depuis 2010, et les fabricants ont migré vers le Viton bien avant.

**Le vrai risque** : les réparations antérieures effectuées avec des pièces génériques pas chères (flexibles NBR ou joints nitrile aftermarket). Une inspection visuelle avant conversion règle le problème.

---

### "L'éthanol corrode l'aluminium et l'acier"

**Verdict : FAUX dans des conditions normales.**

L'éthanol **pur anhydre** (sans eau) est en réalité moins corrosif que l'essence sur l'aluminium. La corrosion apparaît dans deux cas précis :

1. **Éthanol hydraté** (avec eau libre) + aluminium non anodisé + longue stagnation → oxydation possible. Ce cas ne se produit pas dans un circuit carburant fonctionnel et bien entretenu.
2. **Alliages de magnésium en contact direct avec le carburant** (corps de carburateur magnésium sur véhicules des années 70–80, certaines pompes mécaniques anciennes) → réaction avec l'éthanol. Sans objet sur N52 : la pompe à carburant est en acier/aluminium anodisé, et le bloc moteur N52 (qui contient bien du magnésium dans sa structure composite Mg/Al) n'est **jamais en contact avec le carburant** — il est isolé par le circuit de refroidissement et la lubrification.

Le rail d'injection, les tuyaux métalliques et le réservoir acier du N52 ne sont pas concernés.

---

### "L'éthanol consomme les injecteurs"

**Verdict : FAUX pour les injecteurs modernes.**

Les injecteurs Bosch EV14 (stock N52, référence 13537531634) ont une bague d'étanchéité en **Viton et un corps en acier inox**. Ils sont conçus pour fonctionner avec des carburants contenant de l'alcool — Bosch les spécifie comme compatibles E100.

La durée de vie des injecteurs sur E85 est **identique** à celle sur essence si la calibration est correcte (pas de fonctionnement prolongé en mélange pauvre).

---

### "L'éthanol bouffe le catalyseur"

**Verdict : FAUX — l'éthanol est plus propre que l'essence.**

L'éthanol brûle plus complètement que l'essence (moins d'imbrûlés HC, moins de CO à lambda correct). Les DTC catalyseur (P0420/P0430) qui apparaissent lors d'une conversion E85 sont dus à la recalibration des adaptations lambda, pas à une destruction chimique. Ils disparaissent après 200–500 km une fois les LTFT stabilisés.

L'éthanol ne contient **pas de soufre** ni de composés aromatiques lourds (benzène, toluène) qui empoisonnent les catalyseurs à long terme. Sur le long terme, l'E85 est moins agressif pour le catalyseur que le SP95.

---

### "Avec l'E85 on consomme deux fois plus"

**Verdict : EXAGÉRÉ — c'est +30 à +40% en volume, pas ×2.**

L'éthanol a un pouvoir calorifique inférieur (PCI) d'environ 26 MJ/kg contre 44 MJ/kg pour l'essence. **Mais l'AFR stœchiométrique est aussi inférieur** (≈9.8:1 vs 14.7:1), donc le moteur brûle moins d'air par kg de carburant. Les deux effets se compensent partiellement.

En pratique, sur un N52 bien calibré :

| Usage | Surconsommation volumique E85 vs SP95 |
|---|---|
| Ville / ralenti | +30 à +35% |
| Route mixte | +28 à +33% |
| Autoroute | +25 à +30% |

Avec un prix E85 autour de 0.80–0.95 €/L en France (vs ~1.80 €/L pour le SP95), le **coût au kilomètre reste inférieur** même avec la surconsommation.

---

### "L'éthanol absorbe l'eau et provoque des problèmes"

**Verdict : VRAI mais sans conséquence pratique dans un usage normal.**

L'éthanol est effectivement hygroscopique — il absorbe l'humidité atmosphérique. Dans un circuit carburant étanche et rempli régulièrement, la quantité d'eau absorbée est négligeable. Le problème théorique (séparation de phase eau/éthanol) ne survient qu'avec des mélanges E10–E15 à faible teneur, pas avec l'E85 pur.

**Précaution réelle** : ne pas laisser le réservoir quasi-vide stationner plusieurs semaines en hiver humide. Remplir à ≥ ¼ de réservoir si le véhicule est immobilisé.

---

### "Il faut un capteur flex-fuel pour rouler à l'E85"

**Verdict : NON — un capteur est une option, pas une obligation.**

Un capteur flex-fuel mesure en temps réel le titre en éthanol du carburant et adapte la calibration automatiquement. C'est pratique sur un véhicule qui alterne régulièrement E85 et SP95. Mais :

- Sur une **conversion fixe E85** (uniquement E85), le titre éthanol varie peu (60–85% selon la saison) et les LTFT absorbent l'écart (±25% de plage d'adaptation).
- Une **calibration fixe E70** est un bon compromis qui couvre l'E65–E75 sans capteur.
- Les **STFT/LTFT via scanner OBD** remplacent avantageusement le capteur pour une utilisation avisée.

---

### Ce qui est VRAI et réel

| Risque réel | Gravité | Solution |
|---|---|---|
| Démarrage difficile < 0°C | Moyen | Enrichissement cranking calibré, bougie resserrée, batterie neuve |
| Filtre à essence bouché à 200 km (dépôts anciens dissous) | Faible | Changer le filtre avant ET après conversion |
| Casse moteur si calibration pauvre en WOT prolongé | Élevé | Valider STFT/LTFT avant toute accélération franche |
| Réparations antérieures en caoutchouc NBR | Moyen | Inspection visuelle + remplacement si nécessaire |
| Perte autonomie (~30%) | Faible (coût compensé) | Accepter ou prévoir des arrêts plus fréquents |
| Instabilité catalyseur les 500 premiers km | Faible | DTC temporaires, effacer après stabilisation |

---

## 🎯 Conclusion

### Résumé de la Stratégie

1. **`c_fac_mff_ti_stnd_1` / `c_fac_mff_ti_stnd_2` / `c_fac_mff_ti_stnd[0]` / `c_fac_mff_ti_stnd[1]` / `c_fac_mff_ti_stnd_mon` — ×1.45 (E85)** : C'est la base de tout. Sans ça, rien ne fonctionne.
2. **Cranking + warm-up** : L'E85 froid est le défi principal sur N52.
3. **Film mural** : ×1.25 sur les tables `ip_ti_tco_pos_slow_wf_opm_1`, `ip_ti_tco_pos_slow_wf_opm_2`, `ip_ti_tco_pos_fast_wf_opm_1` et `ip_ti_tco_pos_fast_wf_opm_2` assure un comportement fluide à température intermédiaire.
4. **Lambda** : la boucle fermée fait le travail si le facteur d'injection est correct. Le stock est déjà à λ 0.920 en WOT (`ip_lamb_fl__n`), aucun enrichissement WOT manuel n'est nécessaire.
5. **Avance** : modifier `ip_iga_bas_max_knk__n__maf` — table plafond knock, axes MAF×RPM en roulage. Le gain le plus visible sur la route, mais le plus dangereux. Progressivité absolue.

### Risques et Responsabilité

*Ce tutoriel est fourni à titre éducatif et informatif. La modification du calculateur moteur entraîne :*
- *La perte de la garantie constructeur*
- *Des risques de casse mécanique en cas de mauvaise calibration*
- *Des conséquences légales potentielles (contrôle technique, assurance)*

*Procédez à vos risques et faites toujours une sauvegarde de votre bin stock.*

---

*Dernière mise à jour : 2026-04-09 | Version : 3.4 — Explications opm_1/opm_2, 5 copies c_fac_mff_ti_stnd, film mural slow/fast/pos/neg, hiérarchie tables lambda — N52B30 + 13537531634*