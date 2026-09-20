# Sauvegardes d'origine — 19 modules

> Lectures `CODIERDATEN_LESEN` faites au premier passage.  
> **Ces fichiers ne s'écrasent jamais** : ce sont les références de retour en arrière.

Les variantes de l'inventaire ci-dessous sont celles relevées sur le véhicule dans
[le catalogue](../daten/README.md). Les anciennes correspondances génériques issues
de `E89SGET.000` et `E89SGFAM.DAT` ne suffisaient pas à établir l'index de codage réel.

Les 19 sauvegardes de modules sont des traces **NETTODAT** (lignes `B adresse,...`),
malgré leurs noms `<MODULE>.TRC`. Le fichier `fa.trc` contient le FA et n'est pas une
trace de codage FSW/PSW. Conserver ces contenus tels quels.

---

## 🔴 Le point important

**`6BMOT.TRC` est ta DME MSV70.**

`E89SGET.000` donne la correspondance générique `6BMOT → MSV70.C02` ; la variante relevée sur ce véhicule est `MSV70.C03`. Et `E89AEDOC.000` documente le découpage historique de la famille moteur : *« Familie MOT in 4DMOT, 4BMOT und 6BMOT aufgespalten »* — **6B**MOT = 6 cylindres **B**enzin (essence).

| | |
|---|---|
| **La lecture faite** | ✅ sans aucun risque, `CODIERDATEN_LESEN` ne fait qu'interroger |
| **Une écriture dessus** | 🔴 `SG_CODIEREN` réécrit la zone de codage de la DME. Avec `cal_cks_active = 0` et la calibration E85 custom, retour au flash complet |

**Ce module apparaît dans la liste de NCS Expert sous le nom `6BMOT`, pas « DME » ni « MSV70 ».** C'est sous ce nom-là qu'il faut le reconnaître pour ne pas le sélectionner.

---

## Inventaire des 19 modules

### 🔴 Ne jamais coder

| Fichier | DATEN | Nom allemand | Fonction |
|---|---|---|---|
| `6BMOT.TRC` | `MSV70.C03` | *Motor 6-Zylinder Benzin* | **La DME.** Voir l'encadré ci-dessus |
| `ABG.TRC` | `MRS5.C08` | *Airbag — Multiple Restraint System* | Airbags, prétensionneurs de ceinture |
| `CAS.TRC` | `CAS3.C06` | *Car Access System* | Accès, antidémarrage, clés. Erreur = ne démarre plus |
| `DSC.TRC` | `MK60_87.C03` | *Dynamische Stabilitäts-Control* | ESP / ABS |
| `EKP.TRC` | `EKPM60_2.C03` | *Elektrische Kraftstoffpumpe* | Pilotage de la pompe à carburant. Sur une voiture convertie E85, le dernier endroit où improviser |

### 🟠 Codables, mais sensibles

| Fichier | DATEN | Nom allemand | Fonction |
|---|---|---|---|
| `FRM.TRC` | `FRMFA.C12` | *Fußraummodul* | Éclairage, vitres, rétroviseurs, verrouillage. **80 % des codages intéressants sont ici** |
| `JBBF.TRC` | `JBBFE.C07` | *Junction Box Bordnetz* | Boîtier de jonction — centralise beaucoup de fonctions carrosserie |
| `DWAS.TRC` | `DWASINE.C04` | *DWA Sirene* | Sirène d'alarme autoalimentée |
| `DWAI.TRC` | `DWA_IRS.C05` | *Diebstahlwarnanlage — Innenraumschutz* | Alarme — protection volumétrique |
| `CHOST.TRC` | `CCC_HST.C06` | *CCC Host* | Unité centrale nav/multimédia |
| `CGATE.TRC` | `CCC_GW.C02` | *CCC Gateway* | Passerelle entre le CCC et les bus véhicule |
| `CAPPL.TRC` | `CCC_APP.C0B` | *CCC Application* | Applications : navigation, menus, réglages |
| `CAUDI.TRC` | `CCC_ASK.C09` | *Audio-System-Kontroller* | Partie audio / amplification |
| `CTUNE.TRC` | `CCC_ANT.C0B` | *Antenne / Tuner* | Tuner radio et antenne |

### 🟢 Terrain d'apprentissage

| Fichier | DATEN | Nom allemand | Fonction |
|---|---|---|---|
| `KOMBI.TRC` | `KMBI_PL2.C08` | *Kombiinstrument* | **Combiné. Commence par celui-là** — aucune fonction de sécurité |
| `IHK.TRC` | `IHXXPL2.C01` | *Integrierte Heizungs- und Klimaanlage* | Climatisation automatique |
| `PDC.TRC` | `PDC_E87.C05` | *Park Distance Control* | Radars de recul |
| `RLS.TRC` | `RLS_E65.C06` | *Regen-Licht-Sensor* | Capteur de pluie et de luminosité |
| `FZD.TRC` | `FZD.C06` | *Funktionszentrum Dach* | Module de toit — plafonnier, éclairage intérieur, toit ouvrant |

---

### Identification de `DWAI.TRC`

Le nom de la sauvegarde a été corrigé en **`DWAI.TRC`**, sans modification de son
contenu. Le catalogue relevé sur le véhicule identifie la variante **`DWA_IRS.C05`**.
Les composants d'alarme sont distincts :

| Nom SG | DATEN | Rôle |
|---|---|---|
| `DWAS` | `DWASINE.C0x` | Sirène |
| `DWAI` | `DWA_IRS.C05` | *Innenraumschutz* — protection volumétrique habitacle |
| `DWA_63` | `DWA_63.C07` | Module d'alarme principal |

Le filtrage d'une trace dans NCS Dummy peut donner plusieurs variantes compatibles.
Il faut conserver la variante relevée sur le véhicule ; le seul contenu du fichier
ne garantit pas une identification unique.

---

## Ce que la liste dit de la voiture

| Observation | Déduction |
|---|---|
| Aucun module de boîte (EGS / VGSG) | **Boîte mécanique** — cohérent avec l'absence de `$205` au FA |
| 5 modules CCC (`CHOST`/`CGATE`/`CAPPL`/`CAUDI`/`CTUNE`) | Navigation Professional — cohérent avec `$609` |
| `PDC` présent | Radars de recul — `$507` |
| `RLS` présent | Capteur pluie + allumage auto — `$521` |
| `IHK` présent | Climatisation automatique — `$534` |
| `DWAS` + `DWAI` présents | Alarme — `$302` |

Le FA et les modules lus se confirment mutuellement.

---

## Le bon de commande — `fa.trc`

```
E90_#0906&LCSW%0300*VB31$1CA$240$249$2CZ$302$321$428$431$465$493$497
$4AE$4AG$507$520$521$534$544$548$563$609$694$7RS$854$863$877$881$8S3-B070
```

### Structure

| Champ | Valeur | Sens |
|---|---|---|
| Série | `E90_` | Berline E90 |
| Date de construction | `#0906` | **09/2006** — cohérent avec la DME MSV70 (remplacée par MSV80 en 09/2007) |
| Sellerie | `&LCSW` | `L` = *Leder* (cuir), `SW` = *schwarz* (noir) |
| Teinte | `%0300` | Code peinture 300 |
| Type | `*VB31` | Clé de type — 330i |
| Options | `$...` | Codes Salapa |
| Fin | `-B070` | Champ de fin de FA, non utilisé en codage |

> 💡 Ces données vivent dans le CAS. Dans `CAS.TRC`, le bloc `00101200` contient `30,33,30,30,4C,43,53,57` — soit en ASCII **`0300LCSW`** : la teinte et la sellerie, stockées telles quelles.

### Codes option — décodage usuel

⚠️ Il n'existe pas de table de correspondance des codes Salapa dans les DATEN locales. Le décodage ci-dessous est celui couramment admis, **non vérifié contre une source BMW officielle**. Les six premiers sont corroborés par la présence du module correspondant dans la liste lue.

| Code | Signification | Corroboré |
|---|---|---|
| `$302` | Alarme antivol | ✅ DWAS + DWAI |
| `$507` | Park Distance Control | ✅ PDC |
| `$521` | Capteur de pluie + allumage automatique | ✅ RLS |
| `$534` | Climatisation automatique | ✅ IHK |
| `$609` | Navigation Professional | ✅ 5 modules CCC |
| *(absence de `$205`)* | Pas de boîte automatique | ✅ aucun module de boîte |
| `$249` | Volant multifonction | — |
| `$428` | Triangle de signalisation | — |
| `$431` | Rétroviseur intérieur photosensible | — |
| `$465` | Trappe à skis / système de chargement | — |
| `$493` | Pack rangements | — |
| `$520` | Antibrouillards | — |
| `$544` | Régulateur de vitesse | — |
| `$548` | Compteur en km/h | — |
| `$563` | Pack lumière | — |

**Non décodés** — à vérifier avant de s'en servir :
`$1CA` `$240` `$2CZ` `$321` `$497` `$4AE` `$4AG` `$694` `$7RS` `$854` `$863` `$877` `$881` `$8S3`

---

## Rappel

Ces 19 fichiers sont l'état d'origine. Le jour où un module part en vrille, c'est **ici** qu'est la solution — à condition de ne jamais les avoir écrasés.

Procédure de restauration : [01 — Procédure](../01_PROCEDURE.md), section « Revenir en arrière ».
