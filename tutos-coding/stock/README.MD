# Sauvegardes d'origine — 19 modules

> Lectures `CODIERDATEN_LESEN` faites au premier passage.  
> **Ces fichiers ne s'écrasent jamais** : ce sont les références de retour en arrière.

Identification croisée avec `C:\NCSEXPER\DATEN\E89\E89SGET.000` et `E89SGFAM.DAT` — les noms de fichiers DATEN ci-dessous viennent de la table officielle, pas d'une supposition.

---

## 🔴 Le point important

**`6BMOT.TRC` est ta DME MSV70.**

`E89SGET.000` donne la correspondance : `6BMOT → MSV70.C02`. Et `E89AEDOC.000` documente le découpage historique de la famille moteur : *« Familie MOT in 4DMOT, 4BMOT und 6BMOT aufgespalten »* — **6B**MOT = 6 cylindres **B**enzin (essence).

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
| `6BMOT.TRC` | `MSV70.C02` | *Motor 6-Zylinder Benzin* | **La DME.** Voir l'encadré ci-dessus |
| `ABG.TRC` | `MRS5.C06/C07` | *Airbag — Multiple Restraint System* | Airbags, prétensionneurs de ceinture |
| `CAS.TRC` | `CAS3.C06/C08` | *Car Access System* | Accès, antidémarrage, clés. Erreur = ne démarre plus |
| `DSC.TRC` | `DSC_87` | *Dynamische Stabilitäts-Control* | ESP / ABS |
| `EKP.TRC` | `EKP` | *Elektrische Kraftstoffpumpe* | Pilotage de la pompe à carburant. Sur une voiture convertie E85, le dernier endroit où improviser |

### 🟠 Codables, mais sensibles

| Fichier | DATEN | Nom allemand | Fonction |
|---|---|---|---|
| `FRM.TRC` | `FRMFA.C10` | *Fußraummodul* | Éclairage, vitres, rétroviseurs, verrouillage. **80 % des codages intéressants sont ici** |
| `JBBF.TRC` | `JBBFE.C05` | *Junction Box Bordnetz* | Boîtier de jonction — centralise beaucoup de fonctions carrosserie |
| `DWAS.TRC` | `DWASINE.C03` | *DWA Sirene* | Sirène d'alarme autoalimentée |
| `DWAY.TRC` | *(voir note)* | *Diebstahlwarnanlage* | Alarme — second composant |
| `CHOST.TRC` | `CCC_HST.C02` | *CCC Host* | Unité centrale nav/multimédia |
| `CGATE.TRC` | `CCC_GW.C03` | *CCC Gateway* | Passerelle entre le CCC et les bus véhicule |
| `CAPPL.TRC` | `CCC_APP.C07` | *CCC Application* | Applications : navigation, menus, réglages |
| `CAUDI.TRC` | `CCC_ASK.C05` | *Audio-System-Kontroller* | Partie audio / amplification |
| `CTUNE.TRC` | `CCC_ANT.C05` | *Antenne / Tuner* | Tuner radio et antenne |

### 🟢 Terrain d'apprentissage

| Fichier | DATEN | Nom allemand | Fonction |
|---|---|---|---|
| `KOMBI.TRC` | `KMBI_PL2.C06` | *Kombiinstrument* | **Combiné. Commence par celui-là** — aucune fonction de sécurité |
| `IHK.TRC` | `IHXXPL2.C02` | *Integrierte Heizungs- und Klimaanlage* | Climatisation automatique |
| `PDC.TRC` | `PDC_E87.C03/C04` | *Park Distance Control* | Radars de recul |
| `RLS.TRC` | `RLS_E65.C06` / `RLS_PL2.C07` | *Regen-Licht-Sensor* | Capteur de pluie et de luminosité |
| `FZD.TRC` | `FZD.C06` | *Funktionszentrum Dach* | Module de toit — plafonnier, éclairage intérieur, toit ouvrant |

---

### Note sur `DWAY.TRC`

Le nom `DWAY` **n'existe pas** dans les DATEN E89. Les trois composants d'alarme déclarés dans `E89SGFAM.DAT` sont :

| Nom SG | DATEN | Rôle |
|---|---|---|
| `DWAS` | `DWASINE.C0x` | Sirène |
| `DWAI` | `DWA_IRS.C05` | *Innenraumschutz* — protection volumétrique habitacle |
| `DWA_63` | `DWA_63.C07` | Module d'alarme principal |

C'est donc soit une faute de frappe pour `DWAI`, soit le module principal renommé. **Ouvrir `DWAY.TRC` dans NCS Dummy tranchera** : il identifie la variante tout seul à partir du contenu.

---

## Ce que la liste dit de la voiture

| Observation | Déduction |
|---|---|
| Aucun module de boîte (EGS / VGSG) | **Boîte mécanique** — cohérent avec l'absence de `$205` au FA |
| 5 modules CCC (`CHOST`/`CGATE`/`CAPPL`/`CAUDI`/`CTUNE`) | Navigation Professional — cohérent avec `$609` |
| `PDC` présent | Radars de recul — `$507` |
| `RLS` présent | Capteur pluie + allumage auto — `$521` |
| `IHK` présent | Climatisation automatique — `$534` |
| `DWAS` + `DWAY` présents | Alarme — `$302` |

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
| `$302` | Alarme antivol | ✅ DWAS + DWAY |
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
