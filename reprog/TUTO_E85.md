# TUTO E85 : Conversion Ethanol pour Siemens MSV70 — BMW 330i N52B30

> **Véhicule ciblé :** BMW E90/E91/E92/E93 330i — Moteur N52B30 — Calculateur Siemens MSV70  
> **Injecteurs :** BMW 13 53 7531634 (Bosch EV14, port injection, pression nominale rail 5.0 bar — `c_fup_nom` stock = 5000 hPa)  
> **Fichier de base :** VB67774_921S_Full.bin  
> **Stratégie de calibration :**
> - **Facteur injecteur → E85 (×1.45 via `ip_mff_cor_opm`)** : boucle ouverte (WOT) toujours riche, quelle que soit la saison. Note : `c_fac_mff_ti_stnd` reste au stock — l'enrichissement passe par les 4 maps `ip_mff_cor_opm_*` (raw 47 407, phys 1.473)
> - **Cranking → E70** : exception — trop riche au démarrage = noyage moteur
> - **Avance → E60** : pire octane légal français = zéro risque de cliquetis en toutes conditions
> - **Film mural → E85 (×1.25)** : tip-in riche = safe
> - **Carburant réel moyen attendu :** E70 (70% éthanol) — moyenne annuelle pondérée France  
> **Version :** 3.5 — Données réelles extraites du bin + descriptions XDF — 2026-04-10 — §13 complété avec valeurs stock vérifiées (13.1–13.9) + correction LTFT réel −8%/+12%

---

## 📌 Résumé des Paramètres Impactés par la Conversion E85

Avant d'entrer dans le détail de chaque section, voici l'ensemble des paramètres du bin MSV70 qui doivent être modifiés — ou surveillés — lors d'une conversion E85, avec la raison physique de chaque impact.

| # | Paramètre(s) | Section | Pourquoi c'est impacté |
|---|---|---|---|
| 1 | `ip_mff_cor_opm_1_1` `ip_mff_cor_opm_1_2` `ip_mff_cor_opm_2_1` `ip_mff_cor_opm_2_2` | §1 — Injecteurs [✏️](#pencil-injecteurs) | Maps de correction multiplicative d'injection, calibrées à **×1.45 (E85, raw 47 407, phys 1.473)** sur toutes les cellules. `c_fac_mff_ti_stnd` reste au stock — les copies `_1/_2/_mon` ne peuvent pas encoder 0.491 (max physique = 0.393 avec leur équation ×0.000006). En boucle ouverte (WOT), aucune correction lambda n'intervient : ip_mff_cor garantit systématiquement la richesse E85. En boucle fermée, le LTFT compense le titre réel. **4 maps à modifier simultanément.** |
| 2 | `c_tco_n_mff_cst` | §2 — Démarrage froid [✏️](#pencil-tco) | Seuil en-dessous duquel les enrichissements de cranking s'activent. Stock : 17 °C. À relever à 25 °C car l'éthanol a besoin d'enrichissement à des températures ambiantes que l'essence gère sans aide. |
| 3 | `ip_mff_cst_opm_1` `ip_mff_cst_opm_2` | §2 — Démarrage froid [✏️](#pencil-cranking) | **Exception à la règle "E85 partout" :** le cranking est boucle ouverte mais trop riche = noyage moteur (le carburant liquide étouffe la bougie). On calibre pour E70 réel — ni trop, ni trop peu. L'enrichissement E85 de `ip_mff_cor_opm` ne s'applique pas ici : ces tables pilotent la dose directement en boucle ouverte cranking. |
| 4 | `ip_fac_lamb_wup` | §2 — Démarrage froid [✏️](#pencil-wup) | Facteur multiplicateur sur la consigne lambda après démarrage. Stock : 1.000 partout. **Ses axes sont X = MAF (65–500 mg/stk), Y = RPM (704–3008 tr/min)** — ce n'est pas une table température moteur, c'est une table charge×régime. Elle permet d'enrichir les zones basse charge / bas régime où la sonde lambda n'est pas encore opérationnelle. L'enrichissement en fonction de la TCO n'existe pas dans cette table — il est géré par `ip_mff_cst_opm_*` (cranking) et la boucle lambda (chauffe). |
| 5 | `ip_iga_bas_max_knk__n__maf` (+ `ip_iga_min_n_maf_opm_1` / `ip_iga_min_n_maf_opm_2`) | §3 — Avance [✏️](#pencil-avance) | L'avance est calibrée pour **E60 (plancher légal hivernal, ~101 RON)** — le pire carburant que vous pouvez légalement avoir à la pompe. Raison : si on cale l'avance sur E70 ou E85 et que la station délivre du E60 en hiver, on risque le cliquetis. En calibrant sur E60, on est safe quelle que soit la saison. Gain de puissance légèrement réduit (+2.5° max vs +4.5° pour E70), mais zéro risque moteur. |
| 6 | `ip_ti_tco_pos_slow_wf_opm_1` / `ip_ti_tco_pos_slow_wf_opm_2` + `ip_ti_tco_pos_fast_wf_opm_1` / `ip_ti_tco_pos_fast_wf_opm_2` | §5 — Film mural [✏️](#pencil-film) | Le film mural s'applique lors des transitions de charge (tip-in) — une zone de boucle ouverte transitoire. Calibré pour **E85 (×1.25)** : si le carburant réel est E70, on sur-compense légèrement le film → mélange légèrement riche sur tip-in → safe. Sous-compenser serait lean transitoire → risque de claquement ou raté. |
| 7 | `c_t_ti_dly_fl_1` `c_t_ti_dly_fl_2` | §7 — Complémentaires [✏️](#pencil-dly) | Délai entre détection de pleine charge et application de l'enrichissement WOT. À réduire à 0 ms pour que la richesse cible soit appliquée instantanément lors d'une accélération franche sur E85. |
| — | — | — | **— Optionnels —** |
| 8 | `ip_lamb_fl__n` | §4 — Lambda WOT [✏️](#pencil-lambda-wot) | **Vraie table de richesse pleine charge** (1×12 f(RPM)). Stock déjà à λ 0.920 (et 0.871 à 6500 rpm) — l'enrichissement WOT essence est **déjà présent**. Sur E85, on peut soit laisser tel quel, soit dé-enrichir légèrement (0.94-0.95) puisque la chaleur de vaporisation E85 protège mécaniquement contre la détonation. |
| 9 | `c_iga_ini` | §7 — Complémentaires [✏️](#pencil-iga) | Avance d'allumage initiale lors du cranking. Si le démarrage reste difficile après ajustement des tables cranking, +1° à +2° ici facilite l'inflammation du mélange E85 froid. |

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

**Si vous avez remplacé les injecteurs :** Recalculez le facteur ip_mff_cor avec la formule de la section 1 (dépend du titre éthanol réel + débit des nouveaux injecteurs) plutôt qu'un facteur fixe.

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

## 🔧 1. Mise à l'échelle des injecteurs — Paramètre CRITIQUE

**Explication :** L'E85 a un AFR stœchiométrique de 9.55:1 (vs 14.7:1 essence) — le moteur doit injecter ~45% de masse carburant de plus pour le même débit d'air. Sur le MSV70, l'enrichissement passe par les maps `ip_mff_cor_opm_*` : multiplicateurs d'injection appliqués après `c_fac_mff_ti_stnd`, dont 3 copies sur 5 ne peuvent pas encoder la cible E85 (overflow XDF, max physique = 0.393 ms/mg < 0.491 cible). `c_fac_mff_ti_stnd` reste au stock sur toutes les copies.

**Tables concernées :**

| Paramètre | Adresse | Structure | Équation | Max encodable |
|---|---|---|---|---|
| `ip_mff_cor_opm_1_1` | 0x4E3D4 | 12×16 flat | 0.000031 × X | 2.031 |
| `ip_mff_cor_opm_1_2` | 0x4E554 | 12×16 flat | 0.000031 × X | 2.031 |
| `ip_mff_cor_opm_2_1` | 0x4E6D4 | 10×12 flat | 0.000031 × X | 2.031 |
| `ip_mff_cor_opm_2_2` | 0x4E7C4 | 10×12 flat | 0.000031 × X | 2.031 |

**Procédure :**
1. Ouvrir chaque map dans TunerPro (`BMW_Siemens_MSV70_9PPL921S_2560K.xdf`)
2. Sélectionner toutes les cellules (Ctrl+A)
3. Saisir le raw : **47 407** (phys 1.473, ×1.45 E85)
4. Répéter pour les 4 maps
5. `c_fac_mff_ti_stnd` : **ne pas toucher** (toutes copies restent au stock)

<a id="pencil-injecteurs"></a>
✏️ **Avant / Après :**

| Paramètre | Adresse | Structure | Raw stock | Phys stock | Raw E85 | Phys E85 |
|---|---|---|---|---|---|---|
| `ip_mff_cor_opm_1_1` | 0x4E3D4 | 12×16 flat | 32 770 | **1.016** | **47 407** | **1.473** |
| `ip_mff_cor_opm_1_2` | 0x4E554 | 12×16 flat | 32 770 | **1.016** | **47 407** | **1.473** |
| `ip_mff_cor_opm_2_1` | 0x4E6D4 | 10×12 flat | 32 770 | **1.016** | **47 407** | **1.473** |
| `ip_mff_cor_opm_2_2` | 0x4E7C4 | 10×12 flat | 32 770 | **1.016** | **47 407** | **1.473** |

> Effectif combiné : `0.3394 × 1.473 = 0.500 ms/mg` (ratio ×1.450 vs stock, cible ×1.447, écart 0.2%)

**Vérification :**
- Moteur chaud (TCO > 80°C), ralenti stable
- Lire **STFT** via ISTA / INPA / Torque Pro
- Cible : **−5% à +5%**
- STFT > +15% → `ip_mff_cor_opm_*` trop petit — augmenter de +2–3%
- STFT < −15% → `ip_mff_cor_opm_*` trop grand — diminuer de −2–3%

---

### Quel facteur selon le titre éthanol réel ?

La formule correcte dérive de l'AFR stœchiométrique du mélange :

```
AFR_blend = 1 / ( E_fraction/9.0 + (1−E_fraction)/14.7 )

Facteur_injection = (14.7 / AFR_blend) × 0.94
                    ↑ ratio AFR       ↑ correction densité éthanol/essence
```

| Teneur éthanol | AFR stœchio | Facteur vs stock | `ip_mff_cor_opm_*` phys cible | `ip_mff_cor_opm_*` raw cible |
|---|---|---|---|---|
| E65 (65%) | 10.41:1 | **×1.33** | **1.352** | **43 613** |
| E70 (70%) | 10.18:1 | **×1.36** | **1.382** | **44 581** |
| E75 (75%) | 9.97:1 | **×1.39** | **1.412** | **45 548** |
| **E85 (85%) ← CIBLE INJECTEUR** | **9.55:1** | **×1.45** | **1.473** | **47 407** |

> **Pourquoi E85 comme cible du facteur injecteur alors que le carburant réel est E70 ?**
> En boucle fermée, le LTFT corrige automatiquement l'excès de richesse : avec du E70 réel, le LTFT se stabilise à environ −6% — dans la plage −8%/+12% du MSV70, aucun problème.
> En boucle ouverte (WOT, transitions), aucune correction n'intervient. En calibrant sur E85, on garantit qu'on est toujours du côté riche en open loop, quelle que soit la teneur réelle de la pompe (E60 à E85). C'est le choix sécuritaire pour une carto de rue.

> Si vous ne savez pas exactement quel titre vous avez, commencez avec E70 (raw 44 581) — les STFT vous diront si vous montez ou descendez.

### Formule pour injecteurs remplacés

```
ip_mff_cor_nouveau = ip_mff_cor_STOCK × Facteur_Ethanol × (Débit_stock / Débit_nouveaux)
                   = 1.016 × Facteur_Ethanol × (Débit_stock / Débit_nouveaux)

Exemple — injecteurs N54 (débit ~30% supérieur aux N52 stock) sur E70 :
  ip_mff_cor = 1.016 × 1.36 × (1 / 1.30) = 1.016 × 1.046 ≈ 1.063  → raw ip_mff_cor ≈ 34 290
```

### Durée de test au ralenti et risques

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

**Explication :** L'éthanol s'évapore difficilement sous 20°C (ébullition à 78°C vs −40°C pour l'essence). Le moteur a besoin de 1.5× à 2× plus de masse carburant au cranking pour créer un mélange inflammable. L'after-start enrichit la mixture pendant que la sonde lambda monte en température.

---

### 2.1 — Tables de cranking : `ip_mff_cst_opm_1` et `ip_mff_cst_opm_2`

**Tables concernées :**

| Paramètre | Adresse | Structure | Équation | Axes |
|---|---|---|---|---|
| `ip_mff_cst_opm_1` | 0x437DC | 3×8 | 0.021195 × X | X = TCO (°C), Y = RPM |
| `ip_mff_cst_opm_2` | 0x4380C | 3×8 | 0.021195 × X | X = TCO (°C), Y = RPM |

**Procédure :** Multiplier chaque colonne TCO par le facteur E70 correspondant :

```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
Facteur  :  ×2.00   ×1.80   ×1.65  ×1.55  ×1.35  ×1.20  ×1.10  ×1.05
```

> Sur E70 (30% essence), la volatilité à froid est améliorée — les facteurs sont légèrement inférieurs à un E85 pur.

<a id="pencil-cranking"></a>
✏️ **Avant / Après :**

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

**Vérification :**
- Moteur froid (TCO < 20°C), sans appuyer sur la pédale
- Démarrage en **≤ 3 tours** de vilebrequin
- Ralenti initial stable (800–1200 tr/min), descend progressivement
- Si > 5 tours → +15% sur toutes les colonnes cranking, reflasher

---

### 2.2 — Seuil de cranking : `c_tco_n_mff_cst`

**Tables concernées :**

| Paramètre | Adresse | Équation | Note |
|---|---|---|---|
| `c_tco_n_mff_cst` | 0x44F2F | 0.75 × X − 48 | Seuil TCO activation enrichissements cranking |

**Procédure :** Raw 87 → **97** (via TunerPro, scalaire).

<a id="pencil-tco"></a>
✏️ **Avant / Après :**

| | Raw | Valeur |
|---|---|---|
| **AVANT (stock)** | 87 | **17.25 °C** |
| **APRÈS (E85)** | **97** | **25.00 °C** |

**Vérification :** Les enrichissements cranking restent actifs jusqu'à 25°C de TCO (observable en simulant via ISTA la lecture TCO pendant le warm-up).

---

### 2.3 — Warm-up lambda : `ip_fac_lamb_wup`

**Tables concernées :**

| Paramètre | Adresse | Structure | Axes | Équation |
|---|---|---|---|---|
| `ip_fac_lamb_wup` | 0x42764 | 6×6 | X = MAF (65–500 mg/stk), Y = RPM (704–3008 tr/min) | direct (facteur −) |

> Attention : axes MAF × RPM, **pas** TCO. Enrichit les basses charges pendant que la sonde lambda monte en température.

**Procédure :** Augmenter les cellules basse charge / bas régime selon la table E85 ci-dessous. Cellules hautes charges (MAF > 400) restent à 1.000.

<a id="pencil-wup"></a>
✏️ **Avant / Après :**

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

**Vérification :**
- STFT les 30 premières secondes après démarrage : entre −10% et +15% (normal à froid)
- Si STFT oscillent → augmenter de +0.02 sur les cellules basses charges et itérer
- Cible après 2 min de warm-up : STFT entre −5% et +5%

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

**Explication :** L'E85/E70 a un indice d'octane de ~104–108 RON (vs 95 SP95) — il résiste mieux au cliquetis, ce qui permet d'avancer l'allumage et d'augmenter la pression cylindre. La cible est calibrée pour **E60 (101 RON)** : le pire carburant légal en France, pour garantir zéro cliquetis quelle que soit la saison. Gain réaliste : +5 à +12% de puissance au frein.

| Carburant | Octane RON | Auto-inflammation |
|---|---|---|
| Essence 95 | 95 | ~280°C |
| E70 | ~104 | ~400°C |
| E85 | ~108 | ~420°C |

**Tables concernées :**

| Paramètre | Adresse | Structure | Axes | Rôle | Priorité |
|---|---|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | 0x4323A | 8×8 | X = MAF (mg/stk), Y = RPM | Plafond knock — **table principale à modifier** | CRITIQUE |
| `ip_iga_st_bas_opm_1` / `_opm_2` | 0x43586 / 0x435B6 | 6×8 | X = TCO, Y = RPM cranking | Avance cranking uniquement | Optionnel |

**Procédure :** Ajouter les incréments E60 (colonne droite = haute charge) progressivement, palier par palier. Voir tableau de modification §3.2.

### 3.1 — Tables d'avance à identifier dans le MSV70

Le MSV70 gère l'allumage par **modèle de couple** (torque model). Il n'y a pas une seule « table de base » qui sort directement l'avance appliquée : l'avance effective résulte d'un calcul borné entre un plafond (knock-limited) et un plancher. Les tables réellement utilisées **en roulage** sont :

| Paramètre | Adresse | Dimensions / axes | Rôle | Sensibilité E85 |
|---|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | **0x4323A** | 8×8, X=MAF 0.55–2.25 mg/stk, Y=RPM 608–7008 | **Plafond knock — référence MBT.** C'est la table principale à modifier pour gagner de l'avance en E85. | **CRITIQUE** |
| `ip_iga_min_n_maf_opm_1` | 0x4347A | 8×6, X=MAF 0.36–2.14, Y=RPM 320–6528 | Plancher d'avance (mode normal) | Aucune |
| `ip_iga_min_n_maf_opm_2` | 0x434AA | 8×6, X=MAF 0.28–2.14, Y=RPM 320–6528 | Plancher d'avance (mode throttled) | Aucune |
| `ip_fac_eff_iga_opm_1` / `ip_fac_eff_iga_opm_2` | 0x4A5D4 | 16×16, X=couple %, Y=RPM | Facteur d'efficacité avance pour le modèle de couple | Faible/Modérée |
| `ip_iga_st_bas_opm_1` / `ip_iga_st_bas_opm_2` | 0x43586 / 0x435B6 | 6×8, X=TCO, Y=RPM cranking | Avance uniquement pendant la phase de démarrage (cranking) — axes TCO et RPM 80–920 tr/min uniquement | Optionnel (démarrage froid) |

<a id="pencil-avance"></a>
✏️ **Avant / Après — `ip_iga_bas_max_knk__n__maf` (°CRK avant PMH) :**

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

**Vérification :**
- Rouler **50 km** après chaque palier d'avance avant d'aller plus loin
- Scanner OBD2 : aucun recel d'avance par le knock control (LTFT avance stable)
- Aucun cliquetis perceptible (son métallique bref à pleine charge) → si oui, revenir −1° immédiatement
- Pleine charge sécurisée uniquement après validation à charge partielle

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

**Explication :** Le N52B30 est déjà enrichi à λ 0.920 en pleine charge (stock). Sur E85, la chaleur de vaporisation élevée protège mécaniquement — rien d'obligatoire à modifier. En option, dé-enrichir légèrement (0.940–0.950) pour gagner un peu de puissance. La charge partielle est gérée automatiquement par la boucle fermée lambda si `ip_mff_cor_opm` est correctement calibré.

**Tables concernées :**

| Paramètre | Adresse | Structure | Axes | Modification |
|---|---|---|---|---|
| `ip_lamb_fl__n` | 0x436A2 | Courbe 1×12 | X = RPM (608–6496) | **Optionnel** — laisser stock ou monter à 0.940–0.950 |

**Procédure :**
- **Option A :** Ne rien modifier — le stock λ 0.920 est suffisant et sécurisé sur E85
- **Option B :** Monter les cellules 608–4800 rpm à 0.940–0.950, conserver 5504 et 6496 rpm bas (protection thermique soupapes)

**Valeurs STOCK `ip_lamb_fl__n` @0x436A2 :**

```
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
  1. ip_mff_cor_opm est correctement scalé (×1.33 à ×1.45 selon votre titre éthanol)
  2. La sonde lambda lit correctement (lambda reste relative, pas absolue)
  3. La boucle fermée se régule sur la sonde lambda
```

Le système de boucle fermée se corrige automatiquement si `ip_mff_cor_opm` est correctement calibré.

### Quand (et comment) modifier le lambda WOT sur E85

Le stock est déjà à λ 0.920 en WOT, ce qui est **assez riche**. Sur E85 pur, la chaleur de vaporisation élevée (~840 kJ/kg vs 305 kJ/kg essence) refroidit fortement la chambre et protège mécaniquement contre la détonation. Vous avez donc deux options :

**Option A — Garder `ip_lamb_fl__n` stock.** Le λ 0.920 stock injecte ~9% de carburant en plus qu'à stœchio. Sur E85, cette richesse protège toujours — aucun problème de fiabilité.

**Option B — Dé-enrichir légèrement pour gagner un peu de puissance.**
Sur E85, λ 0.94–0.96 en WOT est un bon compromis (plus proche de stoechio = couple maxi, mais encore assez riche pour une marge de sécurité). Modification proposée sur `ip_lamb_fl__n` :

<a id="pencil-lambda-wot"></a>
✏️ **Avant / Après (option B — dé-enrichissement WOT) :**

```
RPM    : 608   992   1216   1600   2016   2496   3008   3520   4128   4800   5504   6496
Stock  : 0.920 0.920 0.913  0.920  0.920  0.920  0.920  0.920  0.920  0.920  0.901  0.871
E85    : 0.950 0.950 0.945  0.950  0.950  0.945  0.945  0.940  0.935  0.930  0.920  0.900
```

> Conserver les cellules 5504 et 6496 rpm à une valeur basse (0.900–0.920) : protection thermique des soupapes d'échappement à très haut régime.

**Vérification :**
- Sonde large bande (Innovate LC-2 ou équivalent) : lambda WOT entre 0.90 et 0.95
- À défaut : STFT stable, aucune remontée LTFT en roulage WOT
- LTFT > +10% → `ip_mff_cor_opm_*` trop petit (pas lambda WOT)
- LTFT < −10% → `ip_mff_cor_opm_*` trop grand (pas lambda WOT)

### Corrections LTFT persistants

Si les LTFT restent décalés après stabilisation :
- **LTFT > +10%** : mélange trop pauvre → `ip_mff_cor_opm_*` trop petit, augmenter
- **LTFT < −10%** : mélange trop riche → `ip_mff_cor_opm_*` trop grand, réduire

Dans les deux cas, la correction se fait sur le facteur injecteur, pas sur les consignes lambda.

---

## 🚿 5. Film Mural (Wall Film Correction)

**Explication :** En injection indirecte (port), une partie du carburant s'adhère aux parois du collecteur et s'évapore lentement. L'éthanol adhère plus et s'évapore moins vite que l'essence → le film est ~25–35% plus épais sur E85. Le tip-in (enfoncement rapide de la pédale) est une zone de boucle ouverte transitoire où aucune correction lambda n'intervient : un film sous-compensé donne un mélange lean → claquement d'admission. On calibre pour E85 (×1.25) même si le carburant réel est E70 : légère sur-compensation en tip-in = safe.

**Tables concernées :**

| Paramètre | Adresse | Structure | Axes | Modification |
|---|---|---|---|---|
| `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | 8×8 | X = TCO, Y = RPM | **×1.25** — film lent, mode Valvetronic |
| `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | 8×8 | X = TCO, Y = RPM | **×1.25** — film lent, mode papillonné |
| `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | 8×8 | X = TCO, Y = RPM | **×1.25** — film rapide, mode Valvetronic |
| `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | 8×8 | X = TCO, Y = RPM | **×1.25** — film rapide, mode papillonné |

> Tables `_neg_*` non modifiées en première intention (erreur → mélange riche en décélération = sans risque vs tables pos où erreur → lean transitoire).

**Procédure :**
1. Ouvrir chaque table dans TunerPro
2. Sélectionner toutes les cellules (Ctrl+A)
3. Multiplier par **1.25** (fonction "Scale" ou recalcul manuel)
4. Répéter pour les 4 tables `_pos_*`

### Le vrai système de film mural du MSV70

#### Pourquoi lent ET rapide ?

Le film de carburant sur les parois du collecteur se comporte comme deux phénomènes superposés avec des constantes de temps très différentes :

- **Film lent (slow)** : la couche de carburant qui s'accumule progressivement et s'évapore entre les injections. Sa dynamique est de l'ordre de plusieurs secondes — c'est le film « résiduel » qui persiste entre les accélérations. Axes : TCO × RPM. **Impact E85 : l'éthanol s'évapore beaucoup moins vite que l'essence à basse température → ce film grossit.**

- **Film rapide (fast)** : la réponse instantanée lors d'une transition de charge (tip-in / tip-out brutal). Quand vous enfoncez la pédale d'un coup, le débit d'air augmente soudainement alors que le film mural n'a pas encore eu le temps de répondre — il faut injecter un surplus immédiat pour compenser. Axes : TCO × RPM, mêmes que le film lent, mais les valeurs sont environ ×0.4–0.6 plus petites car la composante rapide est une correction transitoire, pas un enrichissement continu.

#### Pourquoi positif ET négatif ?

- **Positif (pos)** : correction d'injection en **enrichissement** — s'applique quand le film mural absorbe du carburant (accélération, augmentation de charge). L'ECU injecte plus que la dose calculée pour « alimenter » le film.
- **Négatif (neg)** : correction d'injection en **appauvrissement** — s'applique quand le film restitue du carburant (décélération, levée de pied). L'ECU injecte moins car le film en cours d'évaporation fournit du carburant supplémentaire au cylindre.

**Pour E85 :** augmenter les tables `_pos_*` (le film est plus épais, il faut compenser plus). Les tables `_neg_*` ne sont **pas** modifiées en première intention — ce sont des tables statiques dans le bin, elles ne s'adaptent pas d'elles-mêmes. La raison de les laisser en attente : leur valeur stock est environ 6× plus faible que les tables positives correspondantes (exemple : pos_slow = 36.0 vs neg_slow = 6.0 à froid), donc l'erreur absolue d'un neg non corrigé est petite. De plus, la sous-correction en tip-out produit un mélange légèrement **riche** en décélération — sans risque. À l'inverse, une table pos incorrecte produirait un lean transitoire en tip-in — risque de raté ou de cliquetis. Corriger les neg en deuxième étape, une fois les pos validées.

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

<a id="pencil-film"></a>
✏️ **Avant / Après — les 4 tables de film mural positif :**

> Axes communs : X = TCO (°C), Y = RPM. Objectif E85 = stock × 1.25 (arrondi à la décimale).

**`ip_ti_tco_pos_slow_wf_opm_1` @ 0x4CBFC** (film lent, Valvetronic) — équation `0.100 × raw_u16be` :

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

**`ip_ti_tco_pos_slow_wf_opm_2` @ 0x4CC7C** (film lent, papillonné) — même équation :

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

**`ip_ti_tco_pos_fast_wf_opm_1` @ 0x443FC** (film rapide, Valvetronic) — équation `0.500 × raw_u8` :

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

**`ip_ti_tco_pos_fast_wf_opm_2` @ 0x4443C** (film rapide, papillonné) — même équation :

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

**Vérification :**
- Moteur à ~50°C (après 10 min de conduite modérée)
- Accélération progressive à 30% : doit être lisse, aucun trou
- Accélération franche à 80% : réponse nette, sans temps de latence
- Trou/hésitation → `ip_ti_tco_pos_fast_wf_opm_1` ligne TCO correspondante +10%
- À-coup riche → `ip_ti_tco_pos_slow_wf_opm_1` ligne correspondante −5%

### Stratégie film mural — calibré E85 pour la sécurité en open loop transitoire

Le tip-in (enfoncement rapide de la pédale) est une zone de **boucle ouverte transitoire** : la sonde lambda ne réagit pas assez vite pour corriger. Un film sous-compensé donne un mélange lean transitoire — raté ou claquement d'admission. Un film sur-compensé donne un bref enrichissement — sans conséquence.

> **Choix retenu : calibrer le film mural pour E85 (×1.25)**, même si le carburant réel est E70. Avec du E70 réel, la compensation sera légèrement excessive → tip-in légèrement riche → safe. C'est la même logique que pour le facteur injecteur : mieux vaut être riche dans les zones où rien ne corrige.

Sur E85, le film mural est physiquement plus épais (environ +25% à +35% par rapport à l'essence) et plus lent à se dissiper. Les modifications à appliquer :

1. **Approche retenue** : multiplier les tables `ip_ti_tco_pos_slow_wf_opm_1`, `ip_ti_tco_pos_slow_wf_opm_2`, `ip_ti_tco_pos_fast_wf_opm_1` et `ip_ti_tco_pos_fast_wf_opm_2` par **×1.25** globalement. Ce facteur couvre l'E85 pur avec une légère sur-compensation sécuritaire pour l'E70 réel.

2. **Ajustement fin** : si des « trous » d'accélération persistent à température intermédiaire (30-60°C), augmenter sélectivement les colonnes correspondantes de +10% supplémentaires.

3. **Tables `_neg_*_wf`** : non modifiées en première intention — elles gèrent la récupération de film en tip-out (levée de pied) et sont statiques dans le bin. La sous-correction produit un mélange légèrement riche en décélération (sans risque), contrairement aux tables pos où la sous-correction produirait un lean transitoire. À corriger en deuxième étape (×1.25 identique aux pos), une fois les tables pos validées sur route.

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

**Explication :** Le système EVAP purge les vapeurs du réservoir dans le collecteur. Sur E85, ces vapeurs contiennent plus d'éthanol → enrichissements transitoires lors des purges. La boucle fermée MSV70 corrige automatiquement si l'écart reste raisonnable.

**Tables concernées :**

| Paramètre | Rôle | Modification |
|---|---|---|
| `ip_crlc_mff_buf_cp` | Buffer MFF purge canister | **Ne pas modifier** — observer STFT d'abord |
| `ip_maf_kgh_pred_cor_map_tia` | Correction masse air purge | **Ne pas modifier** en première intention |

**Procédure :** Aucune modification en première intention. Observer les STFT pendant la purge (charge partielle, ~50 km/h).

**Vérification :**
- STFT stables pendant la purge : **aucune action**
- STFT oscillent dans ±15% : **acceptable** — la boucle fermée compense
- STFT oscillent > ±15% de manière persistante : réduire le débit maxi canister

---

## 🔩 7. Paramètres Complémentaires

**Explication :** Trois paramètres mineurs qui complètent la conversion. Le délai WOT (`c_t_ti_dly_fl_*`) et l'avance cranking (`c_iga_ini`) sont optionnels mais améliorent le comportement. Le temps mort injecteur (`ip_ti_add_dly`) ne se modifie que si les injecteurs sont remplacés.

---

### 7.1 — Temps mort injecteur : `ip_ti_add_dly`

**Tables concernées :**

| Paramètre | Structure | Axes | Modification |
|---|---|---|---|
| `ip_ti_add_dly` | Courbe 1×8 | X = tension batterie (0–24.9V) | **Ne pas modifier** sur injecteurs stock |

> Propriété électrique de l'injecteur — indépendante du carburant. Si changement d'injecteurs : recalculer avec les caractéristiques deadtime du fabricant (courbe f(VB)).

---

### 7.2 — Délai enrichissement WOT : `c_t_ti_dly_fl_1` et `c_t_ti_dly_fl_2`

**Tables concernées :**

| Paramètre | Note |
|---|---|
| `c_t_ti_dly_fl_1` | Délai avant application enrichissement pleine charge |
| `c_t_ti_dly_fl_2` | Idem — second paramètre |

**Procédure :** Passer les deux à **0 ms** (enrichissement WOT instantané).

<a id="pencil-dly"></a>
✏️ **Avant / Après :**

| | Valeur |
|---|---|
| **AVANT (stock)** | à extraire du bin |
| **APRÈS (E85)** | **0 ms** |

> Adresses à localiser dans le bin — non extraites à ce jour.

**Vérification :** Comportement à la pleine charge — l'enrichissement WOT doit être immédiat (pas de délai perceptible à l'accélération).

---

### 7.3 — Avance cranking : `c_iga_ini` *(optionnel)*

**Tables concernées :**

| Paramètre | Équation | Modification |
|---|---|---|
| `c_iga_ini` | 0.375 × X − 35.625 (°CRK) | stock + 1° à +2° si démarrage difficile |

**Procédure :** À tester seulement si le démarrage reste difficile après calibration §2.

<a id="pencil-iga"></a>
✏️ **Avant / Après :**

| | Raw | Valeur |
|---|---|---|
| **AVANT (stock)** | à extraire | à extraire |
| **APRÈS (E85)** | stock + delta | **stock + 1° à + 2°** |

> Adresse à localiser dans le bin.

**Vérification :** Démarrage à froid < 3 tours après calibration §2. `c_iga_ini` n'intervient que si le démarrage reste > 5 tours malgré les tables cranking correctement ajustées.

---

## ⚠️ 8. Avertissements et Maintenance

### A. Surveillance Permanente des Fuel Trims

| Indicateur | Normal | Acceptable | PROBLÈME |
|---|---|---|---|
| STFT | ±5% | ±10% | > ±15% |
| LTFT | ±5% | ±10% | > ±15% |

```
STFT > +15% = Le calculateur rajoute du carburant en permanence
  → ip_mff_cor_opm trop petit, ou lambda trop pauvre

STFT < −15% = Le calculateur enlève du carburant
  → ip_mff_cor_opm trop grand, ou lambda trop riche

LTFT élevé en permanence = Votre calibration a une dérive systématique
  → Ajustez ip_mff_cor_opm (trop petit si LTFT positif, trop grand si négatif)
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
| 1 | `ip_mff_cor_opm_1_1` | 0x4E3D4 | 32 770 | **1.016** | **47 407** | **1.473** | — | E85 — multiplicateur injection opm1 banc1 (12×16 flat) |
| 2 | `ip_mff_cor_opm_1_2` | 0x4E554 | 32 770 | **1.016** | **47 407** | **1.473** | — | E85 — idem opm1 banc2 (12×16 flat) |
| 3 | `ip_mff_cor_opm_2_1` | 0x4E6D4 | 32 770 | **1.016** | **47 407** | **1.473** | — | E85 — idem opm2 banc1 (10×12 flat) |
| 4 | `ip_mff_cor_opm_2_2` | 0x4E7C4 | 32 770 | **1.016** | **47 407** | **1.473** | — | E85 — idem opm2 banc2 (10×12 flat) |
| — | `c_fac_mff_ti_stnd_*` (×5) | — | stock | **0.3394** | **stock** | **0.3394** | ms/mg | **NE PAS MODIFIER** — copies _1/_2/_mon ne peuvent pas encoder 0.491 (overflow XDF) |
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
| 18 | `c_t_ti_dly_fl_1` | — | non extrait | — | — | **0 ms** | ms | Enrichissement WOT instantané |
| 22 | `c_t_ti_dly_fl_2` | — | non extrait | — | — | **0 ms** | ms | Enrichissement WOT instantané |
| 23 | `c_iga_ini` | — | non extrait | — | — | **stock +1° à +2°** si démarrage difficile | °CRK | Optionnel — allumage cranking |

### Ordre de Priorité d'Application

```
ÉTAPE 1 — OBLIGATOIRE :
  ✅ ip_mff_cor_opm_1_1, ip_mff_cor_opm_1_2, ip_mff_cor_opm_2_1, ip_mff_cor_opm_2_2 → raw 47 407 (phys 1.473, ×1.45 E85 — open loop toujours riche)
     c_fac_mff_ti_stnd reste au stock (toutes copies — NE PAS MODIFIER)
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
- `ip_mff_cor_opm_1_1`, `ip_mff_cor_opm_1_2`, `ip_mff_cor_opm_2_1`, `ip_mff_cor_opm_2_2` → **raw 47 407 (phys 1.473, ×1.45 E85 — open loop safe)**
- `c_fac_mff_ti_stnd` : **NE PAS MODIFIER** — reste au stock sur toutes les copies
- c_tco_n_mff_cst → 25°C

**Test immédiat :**
1. Démarrez moteur (si tiède ou chaud)
2. Attendez ralenti stable (1–2 min)
3. Lisez STFT → doivent être entre −10% et +10%
4. Si STFT > +12% : augmentez `ip_mff_cor_opm` (toutes les 4 maps) de +3% supplémentaire
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
   - Si LTFT dérive → ajustez `ip_mff_cor_opm` (toutes les 4 maps)

2. Roulage 30 km mixte :
   - Log continu STFT par zone de charge
   - Si correction persistante > ±10% : ajuster `ip_mff_cor_opm` (toutes les 4 maps)

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
| STFT > +15% en permanence | ip_mff_cor_opm trop faible | +3–5% sur les 4 maps |
| STFT < −15% en permanence | ip_mff_cor_opm trop élevé | −3–5% sur les 4 maps |
| Ralenti instable moteur chaud | EVAP ou STFT oscillant | Vérifier canister purge, STFT en temps réel |
| Cliquetis pleine charge | Avance trop haute | −1° à −2° immédiatement |
| Perte puissance progressive | Filtre bouché ou pompe fatiguée | Changez filtre / testez pompe |
| Fumée noire échappement | ip_fac_lamb_wup trop riche | −0.05 à 0.08 |
| LTFT monte sur autoroute | ip_mff_cor_opm trop faible | +3–5% sur les 4 maps |
| Odeur forte éthanol à l'arrêt | Fuite carburant (joints) | Inspection immédiate obligatoire |

---

## 📚 13. Paramètres E85 NON couverts par ce tuto (à investiguer)

Ce tuto couvre les modifications **essentielles et validées** pour une carto E85 de base sur MSV70 N52B30. Plusieurs sous-systèmes du MSV70 ont aussi un impact sur le comportement E85 mais ne sont pas traités ici en détail, soit parce qu'ils sont complexes, soit parce qu'ils dépassent le scope d'une conversion grand public, soit parce que leur impact réel reste à valider sur banc.

Les données ci-dessous sont **extraites du bin stock** `VB67774_921S_Full.bin` via le XDF `BMW_Siemens_MSV70_9PPL921S_2560K.xdf`. Les valeurs stock sont vérifiées.

---

### 13.1 — Enrichissement transitoire pleine charge

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

Sur E85, si vous constatez des **trous d'accélération brefs uniquement lors de kickdown** (et non lors d'une accélération progressive), c'est l'indice que ces tables manquent d'enrichissement transitoire. Augmenter les cellules de mi-charge de `KF_FTRANSVL` de +10 à +20% dans la zone 0.393–0.786 peut corriger ce symptôme. **Le film mural (§5) couvre déjà une partie de ce phénomène** — diagnostiquer d'abord lequel des deux est en cause avant de modifier `KF_FTRANSVL`.

---

### 13.2 — Warm-up lambda au ralenti

**Paramètre :** `ip_fac_lamb_wup_is` @ 0x42788 — **i**dle **s**peed warm-up lambda factor

| Structure | Axe X | Axe Y | Unité |
|---|---|---|---|
| 3×4, uint8, ×0.007813 | MAF : 65 / 100 / 200 / 300 mg/stk | RPM : 704 / 1216 / 1760 tr/min | facteur (-) |

**Valeurs stock :**
```
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.00   1.00   1.00   1.00
1216 rpm :       1.00   1.00   1.00   1.00
1760 rpm :       1.00   1.00   1.00   1.00
```
> Stock : 1.000 partout — aucun enrichissement spécifique au ralenti pendant le warm-up.

**Distinction avec `ip_fac_lamb_wup` (§2.3) :**

- `ip_fac_lamb_wup` : facteur warm-up général, actif pendant toute la phase de chauffe, axé sur MAF × RPM tout régime
- `ip_fac_lamb_wup_is` : uniquement actif au **ralenti** (idle speed) pendant le warm-up — zone basse charge, bas régime

**Impact E85 :**

Sur E85 froid (<10°C), le ralenti peut être instable les 30 premières secondes après démarrage : oscillations RPM, hésitations. Si `ip_mff_cst_opm_*` (cranking, §2.1) est déjà bien calibré et que le problème persiste en maintien de ralenti (pas au démarrage lui-même), c'est ici qu'intervenir.

**Modification suggérée :**
```
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.05   1.03   1.00   1.00   ← enrichir basse charge ralenti froid
1216 rpm :       1.03   1.02   1.00   1.00   ← légèrement
1760 rpm :       1.00   1.00   1.00   1.00   ← ne pas toucher
```
> Encode en uint8 : valeur = target / 0.007813 → 1.05 = raw 134, 1.03 = raw 132, 1.02 = raw 131.

**Condition d'intervention :** uniquement si ralenti instable pendant warm-up après correction du cranking. Ne pas modifier si `ip_fac_lamb_wup` est déjà enrichi.

---

### 13.3 — Stratégie de chauffe catalyseur (cold start retard)

**Paramètres identifiés :**

| Paramètre | Adresse | Structure | Description XDF |
|---|---|---|---|
| `ip_fac_eff_iga_ch_cold_opm_1` | 0x4A444 | 10×10, uint8, ×0.044 | Factor for catalyst heating cold engine — mode normal |
| `ip_fac_eff_iga_ch_cold_opm_2` | 0x4A4A8 | 10×10, uint8, ×0.044 | Factor for catalyst heating cold engine — mode papillonné |
| `ip_lamb_sawup` | 0x4BBDC | 8×8, uint16, ×0.000977 | Basic lambda correction for rich catalyst heating |
| `ip_lamb_sawup_is` | 0x4BC5C | 8×8, uint16, ×0.000977 | Idem au ralenti (idle speed) |

**Valeurs stock `ip_fac_eff_iga_ch_cold_opm_1`** — axe X (TPS 10–65%), axe Y (RPM 704–5888) :
```
        TPS% →  10.0  15.0  20.0  25.0  30.0  35.0  40.0  45.0  50.0  65.0
 704 rpm :       5.98  6.95  6.51  6.51  5.98  5.50  4.58  4.49  4.36  4.40
 896 rpm :       5.98  7.30  6.86  6.69  6.25  5.54  4.93  4.58  4.49  4.49
1056 rpm :       6.12  7.48  7.48  6.25  5.76  5.24  5.02  4.88  4.62  4.62
1216 rpm :       6.20  7.48  7.26  5.98  5.50  5.02  4.80  4.80  4.75  4.75
1504 rpm :       5.28  7.35  6.25  5.50  5.24  4.75  4.75  4.75  4.53  4.75
1760 rpm :       4.22  7.00  5.63  4.88  4.71  4.75  4.80  4.80  4.80  4.88
2016 rpm :       3.83  5.94  4.44  4.14  3.92  4.53  4.58  4.80  4.80  4.66
2304 rpm :       3.39  4.71  3.08  3.30  2.99  3.65  4.31  4.58  4.80  4.40
2848 rpm :       3.30  4.14  2.82  3.30  3.78  4.40  4.71  5.10  5.37  5.06
5888 rpm :       5.76  4.84  4.53  4.88  4.84  5.72  6.03  5.02  4.40  4.75
```
> Ces valeurs sont des **facteurs de retard d'allumage** (en degrés × 0.044) appliqués spécifiquement pendant la phase de chauffe catalyseur. Le retard maximum atteint ~7.5° (raw 170 × 0.044), ce qui génère des EGT élevées — c'est voulu pour chauffer rapidement le catalyseur froid.

**Valeurs stock `ip_lamb_sawup`** — plage ~0.935 à 1.008 :
> Lambda cible légèrement riche à froid pour la chauffe catalyseur. Avec pompe à air secondaire (SAP), ces valeurs sont actives et poussent le catalyseur à monter en température rapidement.

**Impact E85 :**

L'éthanol brûle à une température de combustion légèrement plus basse que l'essence (chaleur de vaporisation élevée, meilleur refroidissement chambre). Résultat : les EGT de démarrage sur E85 sont ~30–50°C inférieures. La stratégie de retard d'allumage est moins efficace car elle génère moins de chaleur dans l'échappement.

**Conséquences pratiques :**
- Le catalyseur met légèrement plus longtemps à atteindre sa température de light-off (~350°C)
- DTC P0420/P0430 (efficacité catalyseur) peuvent apparaître temporairement les premiers 500 km
- Ces DTC disparaissent habituellement une fois les adaptations recalculées

**Ne pas modifier `ip_fac_eff_iga_ch_cold_opm_*` en première intention.** Intervenir uniquement si P0420/P0430 persistent au-delà de 500 km après effacement des adaptations via ISTA.

---

### 13.4 — Protection température échappement (EGT)

**Paramètre :** `c_teg_max_iga` @ 0x44F54

| Valeur stock | Unité | Description XDF |
|---|---|---|
| **865 °C** | °C | Maximum allowable exhaust gas temperature for spark retard control |

Au-delà de ce seuil, le MSV70 retire automatiquement l'avance à l'allumage pour protéger le catalyseur et les turbines (sur turbo) / soupapes (sur N52).

**Sur E85, l'EGT est typiquement 30–50 °C plus basse à puissance équivalente**, grâce à la chaleur de vaporisation élevée de l'éthanol qui refroidit la chambre de combustion. Conséquence directe :

| Situation | Essence | E85 |
|---|---|---|
| EGT WOT 6500 rpm | ~800–850°C | ~750–800°C |
| Déclenchement protection | Fréquent à haute charge | Rare |
| Marge disponible pour +avance | Réduite | Augmentée |

**Ne pas modifier ce seuil.** Il est calibré pour protéger le moteur en cas de panne sonde lambda ou de carburant hors spec. Le réduire n'apporte rien sur E85 ; l'augmenter est dangereux.

**Information utile pour §3 (avance) :** la marge EGT plus large sur E85 justifie physiquement les +2.5° d'avance de la stratégie E60-safe — non seulement l'octane le permet, mais l'EGT inférieur laisse de la marge thermique.

---

### 13.5 — Film mural induit par Valvetronic (spécifique N52)

**Paramètres identifiés :**

| Paramètre | Adresse | Structure | Description XDF |
|---|---|---|---|
| `ip_fac_ti_maf_sp_wf_pos_opm_1` | 0x42C5A | 1×8, uint8, ×0.007813 | Correction TCO pour film mural déclenché par ↑MAF setpoint (Valvetronic levée croissante) |
| `ip_fac_ti_maf_sp_wf_pos_opm_2` | 0x42C62 | 1×8, uint8, ×0.007813 | Idem mode papillonné |
| `ip_fac_ti_maf_sp_wf_neg_opm_1` | 0x42C4A | 1×8, uint8, ×0.007813 | Correction TCO pour film mural déclenché par ↓MAF setpoint (Valvetronic levée décroissante) |
| `ip_fac_ti_maf_sp_wf_neg_opm_2` | 0x42C52 | 1×8, uint8, ×0.007813 | Idem mode papillonné |
| `ip_crlc_pos_maf_sp_wf_opm_1` | 0x4234C | 8×3, uint16, ×0.000015 | Constante de corrélation film mural MAF-SP déclenché — positif |
| `ip_crlc_pos_maf_sp_wf_opm_2` | 0x4237C | 8×3, uint16, ×0.000015 | Idem mode papillonné |
| `ip_ti_cor_tps_mod_wf` | 0x4C93C | 8×8, uint16, ×0.004 ms | Temps d'injection film mural pour changement de levée en mode TPS |
| `ip_fac_tps_mod_wf` | 0x42CF4 | scalaire | Pondération film mural déclenché par changement levée en TPS-mode |

**Valeurs stock extraites du bin :**

`ip_fac_ti_maf_sp_wf_pos_opm_1` — axe X (TCO : -30 à 115°C) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.3   65.3   84.8  114.8
facteur :    0.414  0.399  0.383  0.359  0.313  0.234  0.211  0.195
```

`ip_fac_ti_maf_sp_wf_neg_opm_1` — même axe X :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.3   65.3   84.8  114.8
facteur :    0.406  0.399  0.375  0.344  0.274  0.125  0.047  0.008
```
> Le facteur négatif (levée décroissante) tombe quasi à zéro à chaud (0.008 @ 115°C) — normal car à chaud le film mural sur collecteur est minimal.

`ip_fac_tps_mod_wf` = **−2.0** (scalaire) — ce scalaire négatif indique que la pondération du film mural en mode TPS est inactive ou inversée par design.

**Physique du phénomène (spécifique N52 Valvetronic) :**

Sur un moteur conventionnel, le film mural se constitue lors des variations du papillon. Sur le N52, c'est la levée Valvetronic qui joue ce rôle : chaque changement de levée modifie le débit d'air d'admission, ce qui crée une variation de MAF setpoint (`MAF_SP`). Le MSV70 a une compensation spécifique pour ce phénomène — distincte du film mural TCO/RPM couvert en §5.

**Impact E85 :**

L'éthanol a une chaleur de vaporisation ~2.7× supérieure à l'essence. Le film mural sur les parois du collecteur s'évapore plus lentement sur E85, surtout à froid. La correction Valvetronic de `ip_fac_ti_maf_sp_wf_pos_opm_1` devrait théoriquement être augmentée de ~15–25% aux températures froides (TCO < 15°C) pour compenser le film plus persistant.

**Symptôme d'inadéquation :** couple instable lors de changements rapides de pédale (pas en tip-in brusque, mais en modulation douce de la charge) — spécifiquement les 5 premières minutes de conduite sur E85 froid.

**À ne modifier qu'après** avoir validé et stabilisé les tables de film mural TCO/RPM de §5.

---

### 13.6 — Coupure d'injection en décélération (Schubabschaltung / fuel cutoff)

**Paramètre :** `id_maf_n_min_fcut_fast` @ 0x41E1C

| Structure | Axe X (RPM) | Axe Y (MAF mg/stk) |
|---|---|---|
| 4×4, uint8 (booléen) | 1200 / 2400 / 3600 / 4800 | 100 / 125 / 225 / 375 |

**Valeurs stock :**
```
        MAF →  100   125   225   375 mg/stk
1200 rpm :      0     0     0     0
2400 rpm :      0     0     0     1
3600 rpm :      0     0     1     1
4800 rpm :      0     1     1     1
```
> `1` = fuel cutoff rapide activé dans cette zone. Le cutoff est actif au-dessus de ~3600 rpm à charge significative (>225 mg/stk), ou à partir de 2400 rpm à très haute charge (>375 mg/stk).

**Impact E85 :** aucune modification nécessaire. Le cutoff décélération fonctionne identiquement sur E85 — la coupure d'injection est indépendante du type de carburant.

**Information utile :** lors de la reprise après un cutoff, l'allumage du mélange E85 est légèrement plus lent qu'avec l'essence (surtout à froid). Si vous constatez des à-coups à la reprise après décélération, vérifier d'abord `ip_fac_lamb_wup` et le cranking plutôt que le cutoff lui-même.

---

### 13.7 — Limites de fuel trims (STFT/LTFT clamps)

**Paramètres localisés dans le bin :**

| Paramètre | Adresse | Valeur stock | Description |
|---|---|---|---|
| `c_fac_max_h_rng_lam_ad` | 0x47F4C | **+12.0 %** | Limite haute de l'adaptation lambda — zone haute charge |
| `c_fac_max_l_rng_lam_ad` | 0x47F4E | **+12.0 %** | Limite haute de l'adaptation lambda — zone basse charge |
| `c_fac_min_h_rng_lam_ad` | 0x47F50 | **92.0 %** | Limite basse (100%−92% = −8%) — zone haute charge |
| `c_fac_min_l_rng_lam_ad` | 0x47F52 | **92.0 %** | Limite basse — zone basse charge |
| `c_lam_mv_dyw_dly` | 0x44B3E | **7.7 %** | Seuil dynamique lambda (STFT window) |

**Correction de la valeur ±25 % souvent citée :**

Le tuto mentionnait que « le LTFT peut absorber ±25 % » — **c'est inexact sur ce bin MSV70.** Les paramètres réels lus dans le bin sont :

| Trim | Valeur réelle (bin stock) | Note |
|---|---|---|
| LTFT limite positive | **+12 %** | `c_fac_max_*_rng_lam_ad` |
| LTFT limite négative | **−8 %** | 100% − 92% = −8% |
| STFT fenêtre dynamique | **±7.7 %** | `c_lam_mv_dyw_dly` |

> La plage asymétrique (−8% / +12%) est intentionnelle : le MSV70 est plus tolérant à un mélange trop riche (LTFT positif = ECU réduit l'injection) qu'à un mélange trop pauvre.

**Zones d'activation de l'adaptation LTFT :**

| Zone | RPM | MAF seuil |
|---|---|---|
| Basse charge | 1056–3296 rpm | < 5.5 mg/stk |
| Haute charge | 2016–6016 rpm | < 55 mg/stk |

**Implication pour la conversion E85 :**

Avec `ip_mff_cor_opm` calibré pour E85 (raw 47 407, phys 1.473, effectif ×1.45) mais un carburant réel E70, l'ECU doit réduire l'injection de ~7% en boucle fermée → LTFT attendu : **−7 à −8%**. Cela rentre exactement dans la limite de −8% — on est à la limite du plafond. Si vous avez du E60 hivernal (calibration E85 alors que vous avez E60), le LTFT devrait se caler à **−12 à −13%**, ce qui dépasse le plafond → l'ECU ne pourra pas compenser complètement → mélange légèrement riche persistant en boucle fermée (acceptable, mais pas parfait).

**Conclusion pratique :** si vos LTFT plafonnent en négatif à −8% de manière permanente, c'est que votre facteur ip_mff_cor est trop élevé pour le carburant réel en station. Affiner `ip_mff_cor_opm` (toutes les 4 maps) selon le titre éthanol réel (§1).

---

### 13.8 — Phasage injection EOI

**Paramètre :** `ip_eoi_1_bas` @ 0x4E914 — End Of Injection angle (premier injecteur)

| Structure | Axe X (durée injection) | Axe Y (RPM) | Unité Z |
|---|---|---|---|
| 8×6, uint16, ×0.375 | 0.4 / 2.0 / 3.7 / 6.0 / 12.8 / 14.0 ms | 512 / 704 / 992 / 1504 / 2016 / 3008 / 4512 / 6496 rpm | °CRK (degrees crankshaft) |

**Valeurs stock (°CRK après PMH) :**
```
        TI (ms) →   0.4    2.0    3.7    6.0   12.8   14.0
 512 rpm :          213.0  214.9  225.8  234.0  197.6  199.9
 704 rpm :          216.8  217.5  228.0  234.8  199.1  199.9
 992 rpm :          223.1  225.4  234.0  241.9  171.8  169.5
1504 rpm :          232.1  232.1  235.1  240.8   90.0   85.1
2016 rpm :          238.5  237.4  235.9  238.1   82.1   66.4
3008 rpm :          238.9  241.1  236.6  230.6  102.8   79.5
4512 rpm :          235.1  240.8  240.0  209.3   97.5   70.1
6496 rpm :          225.8  230.3  225.4  205.5   97.5   70.5
```

**Physique de l'EOI sur port injection N52 :**

L'angle EOI définit précisément *quand* l'injection se termine par rapport au PMH. Sur port injection, l'optimum est d'avoir fini l'injection avant que la soupape d'admission s'ouvre (~140–160°CRK après PMH d'allumage, soit ~200–220° après PMH d'admission). Les valeurs stock autour de 200–240°CRK correspondent à cette fenêtre optimale aux régimes normaux.

Aux longues durées d'injection (12.8–14.0 ms — cas WOT), l'EOI tombe à ~66–100°CRK : l'injection finit beaucoup plus tôt, parfois avec soupape d'admission déjà ouverte — c'est inévitable à haute charge car l'injection doit commencer très tôt pour finir à temps.

**Impact E85 (+45% de durée d'injection) :**

Sur E85, la durée d'injection augmente de ~45%. Pour le même angle EOI, l'injection démarre ~45% plus tôt. À bas régime et faible charge, cela reste dans la fenêtre acceptable (soupape fermée). À haute charge (longues TI), l'injection peut finir encore plus tardivement — mais le MSV70 gère cela automatiquement via la même table EOI sans modification.

**En pratique :** la table `ip_eoi_1_bas` n'a pas besoin d'être modifiée pour E85 sur injecteurs stock. Si les injecteurs sont remplacés par des injecteurs à plus haut débit (durées d'injection réduites), la table devient pertinente — c'est une modification avancée hors scope de ce tuto.

---

### 13.9 — Pression de rail / `ip_fup_cor`

**Paramètres :**

| Paramètre | Adresse | Valeur stock | Unité |
|---|---|---|---|
| `c_fup_nom` | 0x44B0C | **5000 hPa** (5.0 bar) | hPa |
| `ip_fup_cor` | 0x4AF44 | Table 6×6 | hPa (correction) |

**Valeurs stock `ip_fup_cor`** — axe X (débit L/h : 50 à 140), axe Y (tension batterie : 0 à 5V) :
```
        débit (L/h) →    50     80    100    110    120    140
toutes tensions :      +0.06  -17.0  -34.0  -46.0  -63.0  -101.0 hPa
```
> Toutes les lignes de tension sont identiques — la tension batterie n'influe pas sur la correction de pression rail dans ce bin. Seul le débit compte.

**Interprétation :**

À fort débit (140 L/h), la pompe stock génère une chute de pression de −101 hPa (−1.01 bar) par rapport à la pression nominale. La pression rail effective serait alors : 5000 − 101 = **4899 hPa (4.9 bar)** — soit une chute de ~2%. C'est acceptable avec l'essence.

**Sur E85 (+30% de débit) :**

En WOT prolongé sur E85, le débit requis est ~130 L/h (vs ~100 L/h essence). La correction `ip_fup_cor` à 130 L/h interpolée est environ −80 hPa → pression effective ~4920 hPa. La pompe stock reste dans les limites.

Cependant, si la pompe est en fin de vie (>150 000 km ou débit <2.0 L/30s au test), la chute réelle peut dépasser ces valeurs calculées :

| Symptôme | Cause probable | Solution |
|---|---|---|
| Perte de couple progressive sur accélération longue (>10 sec WOT) | Chute de pression rail | Remplacer pompe (→ N54 swap ou Walbro 255) |
| STFT riche en WOT (lambda amont < 0.90 log) | Pression trop haute (rare) | Vérifier régulateur pression |
| DTC P0087 (fuel pressure low) | Pompe insuffisante | Remplacement pompe obligatoire |

**La correction `ip_fup_cor` est une table de lecture seule pour diagnostic — ne pas modifier.** Si la pression rail s'effondre sur E85, la solution est mécanique (pompe à plus grand débit), pas logicielle.

---

## 🔬 14. État de Vérification du Tuto

Ce tuto est issu d'un audit du XDF `BMW_Siemens_MSV70_9PPL921S_2560K.xdf` et du bin `VB67774_921S_Full.bin` du dépôt. Voici l'état honnête de chaque section :

| Section | Statut | Niveau de confiance |
|---|---|---|
| §1 Injecteurs (`ip_mff_cor_opm_*`) | ✅ Adresses + valeurs vérifiées — c_fac reste stock, overflow documenté | **Élevé** |
| §2 Cranking (`c_tco_n_mff_cst`, `ip_mff_cst_opm_*`) | ✅ Vérifié | **Élevé** |
| §2.3 `ip_fac_lamb_wup` | ⚠️ Adresse corrigée (0x42764), axes confirmés (MAF×RPM) | **Moyen** — la stratégie d'utilisation reste à valider en pratique |
| §3 Avance (`ip_iga_bas_max_knk__n__maf`) | ⚠️ Table identifiée comme « plafond knock » mais le modèle de couple MSV70 est complexe ; il existe aussi `ip_iga_min_n_maf_opm_*`, `ip_fac_eff_iga_opm_*`, `ip_iga_ofs_max_knk` qui interagissent. | **Moyen** — l'effet réel d'une modif +5° devrait être validé sur banc avant tout test piste |
| §4 Lambda (`ip_lamb_fl__n` comme vraie table WOT) | ✅ Description XDF lue directement, stock vérifié | **Élevé** |
| §5 Film mural (vraies tables `ip_ti_tco_*_*_wf_opm_*`) | ⚠️ Tables identifiées via descriptions XDF ; multiplicateur ×1.20 = recommandation conservative basée sur la physique, pas sur retour d'expérience N52 publié | **Moyen** |
| §6 EVAP | ⚠️ Mentionné mais non vérifié en détail | **Faible** |
| §7 Compléments (deadtime `ip_ti_add_dly`, délai FL) | ✅ Vérifié | **Élevé** |
| §13 Paramètres non couverts | ✅ Adresses + valeurs stock vérifiées au bit près pour 13.1–13.9. Correction apportée : LTFT réel = −8%/+12% (pas ±25%). Recommandations E85 basées sur la physique et les valeurs bin. | **Moyen** — impact réel à valider sur banc |

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

- Sur une **conversion fixe E85** (uniquement E85), le titre éthanol varie peu (60–85% selon la saison) et les LTFT absorbent l'écart (plage réelle : −8%/+12% sur MSV70).
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

1. **`ip_mff_cor_opm_1_1` / `ip_mff_cor_opm_1_2` / `ip_mff_cor_opm_2_1` / `ip_mff_cor_opm_2_2` — raw 47 407 (phys 1.473, ×1.45 E85)** : C'est la base de tout. `c_fac_mff_ti_stnd` reste au stock (overflow XDF empêche d'y encoder ×1.45). Sans l'enrichissement ip_mff_cor, rien ne fonctionne.
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

*Dernière mise à jour : 2026-04-11 | Version : 3.6 — Enrichissement E85 via ip_mff_cor_opm (raw 47 407) — c_fac_mff_ti_stnd NE PAS MODIFIER (overflow XDF), LTFT réel −8%/+12% — N52B30 + 13537531634*