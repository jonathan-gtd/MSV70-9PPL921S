# §1 — Prérequis Avant Toute Conversion E85

Avant d'ouvrir TunerPro ou de toucher le moindre paramètre, vous devez valider chaque point ci-dessous. Une conversion E85 réussie dépend autant de l'état mécanique du véhicule que de la calibration.

---

### 📋 Checklist Avant Conversion

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

