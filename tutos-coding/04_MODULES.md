# 04 — Modules de l'E90 et niveaux de risque

> Liste établie à partir des **DATEN E89 réellement présentes** sur le poste  
> (`C:\NCSEXPER\DATEN\E89`, 433 fichiers).

---

## ⚠️ Lire d'abord

Cette liste est celle des modules **que les DATEN savent coder pour la famille E8x/E9x** — pas celle des modules installés sur ta voiture. Beaucoup n'existent pas sur une 330i (modules M3, diesel, cabriolet, E70…).

**La liste réelle de ta voiture s'affiche dans NCS Expert après `F4 Process ECU`**, une fois le FA chargé. C'est elle qui fait foi.

Plusieurs modules existent en variantes (`CAS2` / `CAS3`, `FRM2_E89` / `FRM3_E89`…).
Utiliser la variante relevée sur le véhicule dans le [catalogue](daten/README.md) :
le filtrage d'une trace par NCS Dummy peut laisser plusieurs candidats compatibles.

---

## 🔴 Interdits — ne jamais sélectionner

| Module | Fonction | Pourquoi |
|---|---|---|
| **`MSV70`** | **Ta DME** | 🔴 **Le plus important.** Calibration E85 custom, `cal_cks_active = 0`. Un `SG_CODIEREN` réécrit sa zone de codage et peut invalider le tune — retour au flash complet. **Le fichier `MSV70.C0x` est présent dans les DATEN : il apparaîtra dans la liste. Passe ton chemin.** |
| `CAS2` / `CAS3` | Accès et antidémarrage | Erreur = voiture qui ne démarre plus, clés à réapparier |
| `MRS5` / `MRS7` / `MRS8` | Airbags | Organe de sécurité passive |
| `ACSM2` | Gestion crash | Organe de sécurité passive |
| `DSC_89` / `MK60_87` | ESP / ABS | Organe de sécurité active |
| `EPS_E89` | Direction assistée | Organe de sécurité active |
| `VGSG90` | Boîte automatique | Risque de casse mécanique |

> Ces modules se codent, dans l'absolu. Mais ils se codent avec un plan de secours, du temps, et une raison. Pas en découverte.

---

## 🟠 Prudence — utiles mais sensibles

| Module | Fonction | Remarque |
|---|---|---|
| `FRM2_E89` / `FRM3_E89` | Éclairage, vitres, rétroviseurs, verrouillage | **C'est là que vivent 80 % des codages intéressants** — et c'est le module réputé fragile de l'E9x. Le bricage concerne surtout la *reprogrammation* (WinKFP), pas le codage ; le facteur commun reste la **sous-tension**. Chargeur obligatoire, `.TRC` sauvegardé. |
| `JBBFE` / `JBBFEII` / `JBBFE3R` | Boîtier de jonction — centralise beaucoup de fonctions carrosserie | Plusieurs centaines de FSW. Effets de bord fréquents. |
| `CCC_*` / `CIC` / `RAD1` / `RAD2` | Autoradio / navigation | Codage inoffensif mécaniquement, mais un module multimédia mal codé peut boucler au démarrage |
| `DWA_63` / `DWASINE` | Alarme | Déclenchements intempestifs si mal codé |

---

## 🟢 Sûrs pour débuter

| Module | Fonction | Pourquoi c'est un bon terrain |
|---|---|---|
| **`KMBI_PL2`** | **Combiné (tableau de bord)** | ✅ **Commence par celui-là.** Aucune fonction de sécurité, aucun antidémarrage. Une erreur n'immobilise rien : au pire un affichage bizarre. |
| `IHKX_E89` / `IHXXPL2` | Climatisation | Confort pur |
| `PDC_E87` / `PDC_E84` | Radars de recul | Équipement périphérique |
| `RLS_PL2` | Capteur pluie / luminosité | Équipement périphérique |
| `AHM_E60` / `AHM_E70` | Attelage | Uniquement si équipé |
| `FZD` | Module de toit (plafonnier, toit ouvrant) | Confort |
| `RDCLCE89` | Contrôle pression pneus | Uniquement si équipé |

---

## Correspondance des sigles

| Sigle | Allemand | Français |
|---|---|---|
| **CAS** | *Car Access System* | Accès et antidémarrage |
| **FRM** | *Fußraummodul* | Module de plancher — éclairage, vitres, rétros |
| **JBBF** | *Junction Box Bordnetz* | Boîtier de jonction |
| **KMBI / KOMBI** | *Kombiinstrument* | Combiné, tableau de bord |
| **IHKA** | *Integrierte Heizungs- und Klimaanlage* | Climatisation automatique |
| **DSC** | *Dynamische Stabilitäts-Control* | ESP |
| **MRS** | *Multiple Restraint System* | Airbags |
| **ACSM** | *Advanced Crash Safety Module* | Gestion de crash |
| **EKPM** | *Elektrische Kraftstoffpumpe Modul* | Pompe à carburant |
| **PDC** | *Park Distance Control* | Radars de recul |
| **RLS** | *Regen-Licht-Sensor* | Capteur pluie/lumière |
| **AHM** | *Anhängermodul* | Module attelage |
| **FZD** | *Funktionszentrum Dach* | Module de toit |
| **DWA** | *Diebstahlwarnanlage* | Alarme antivol |
| **RDC** | *Reifendruck-Control* | Pression des pneus |
| **EPS** | *Elektrische Servolenkung* | Direction assistée |
| **VGSG** | *Getriebesteuergerät* | Calculateur de boîte auto |

---

## Codages courants sur E90

Fonctions fréquemment recherchées, avec le module concerné. **Les noms de FSW exacts sont à retrouver dans NCS Dummy** — ils varient selon la variante du module, et un nom recopié d'un forum peut ne pas exister sur ton exemplaire.

| Fonction souhaitée | Module |
|---|---|
| Vitres ouverture/fermeture à la clé | FRM |
| Rétroviseurs rabattus au verrouillage | FRM |
| Durée de l'éclairage d'accompagnement | FRM |
| Feux de jour / anneaux en feux diurnes | FRM |
| Clignotants — nombre de flashs « confort » | FRM |
| Flash des feux stop en freinage d'urgence | FRM |
| Bip de verrouillage | FRM / JBBF |
| Suppression du rappel de ceinture | KOMBI |
| Affichage vitesse numérique / menus OBC | KOMBI |
| Vidéo en roulant | CCC / CIC |

> 💡 Point de départ connu : le tutoriel de bmw-serie3.com utilise `TUERAUF_STOP_MAUT` dans le FRM pour la remontée des vitres. C'est un bon exemple pour se faire la main sur la recherche dans NCS Dummy.

---

## Méthode

1. Partir de **la fonction voulue**, pas du catalogue
2. Identifier le module concerné (tableau ci-dessus)
3. Lire son `.TRC`, le sauvegarder, l'ouvrir dans NCS Dummy
4. Chercher par mot-clé allemand (`TUER`, `SPIEGEL`, `LICHT`, `WARN`…)
5. Une modification à la fois, relue et vérifiée avant la suivante

Coder cinq choses d'un coup et découvrir un effet de bord, c'est cinq candidats et aucun coupable.
