# Catalogue des DATEN — ce véhicule uniquement

Les fonctions codables des 19 calculateurs de la voiture, dans leurs variantes réelles, traduites.

> **Variantes :** relevées par NCS Expert sur le véhicule
> **1 738 fonctions codables** · **1 692 avec leurs valeurs** (97,4 %)
> **1 365 avec une signification en anglais** (78,5 %) · **851 traduites en français** (49 %)
> **100 % des lignes traduites** — 1 294 fragments établis + 130 lectures probables

---

## 1. Les variantes du véhicule

Liste relevée sur la voiture. C'est elle qui fait foi — rien n'est déduit ici.

| Module | Variante | Fonctions |
|---|---|---:|
| FRM | `FRMFA.C12` | 449 |
| CAPPL | `CCC_APP.C0B` | 305 |
| KOMBI | `KMBI_PL2.C08` | 295 |
| CAS | `CAS3.C06` | 229 |
| JBBF | `JBBFE.C07` | 97 |
| IHK | `IHXXPL2.C01` | 58 |
| ABG | `MRS5.C08` | 53 |
| PDC | `PDC_E87.C05` | 48 |
| FZD | `FZD.C06` | 41 |
| DWAY | `DWA_IRS.C05` | 36 |
| EKP | `EKPM60_2.C03` | 28 |
| DWAS | `DWASINE.C04` | 26 |
| DSC | `MK60_87.C03` | 19 |
| CHOST | `CCC_HST.C06` | 19 |
| RLS | `RLS_E65.C06` | 12 |
| CAUDI | `CCC_ASK.C09` | 7 |
| CGATE | `CCC_GW.C02` | 7 |
| CTUNE | `CCC_ANT.C0B` | 5 |
| 6BMOT | `MSV70.C03` | 4 |

**Contrôle croisé : les 19 variantes sont à 100 % de cohérence avec les `.TRC`** — chaque fonction tombe dans un bloc d'adresses que le module a réellement renvoyé. C'est la vérification, pas la source.

### Pourquoi la déduction automatique ne suffisait pas

Une version précédente de ce catalogue déduisait les variantes en testant chaque candidate contre les `.TRC`. Le critère — 100 % des fonctions dans les blocs observés — est **nécessaire mais pas suffisant** : plusieurs variantes le satisfont. L'arbitrage retenu (prendre la plus fournie) s'est avéré faux sur **11 modules sur 19**.

| Module | Variante réelle | Ce que la déduction donnait |
|---|---|---|
| DSC | `MK60_87.C03` — 19 fonctions | `MK60_87.C0A` — 86 |
| ABG | `MRS5.C08` — 53 | `MRS5CD.C22` — 67 |
| CAS | `CAS3.C06` — 229 | `CAS3.C09` — 255 |
| IHK | `IHXXPL2.C01` — 58 | `IHXXPL2.C02` — 74 |
| CAUDI | `CCC_ASK.C09` — 7 | `CCC_ASK.C04` — 13 |
| CGATE | `CCC_GW.C02` — 7 | `CCC_GW.C03` — 8 |
| RLS | `RLS_E65.C06` | `RLS_PL2.C08` |
| CHOST | `CCC_HST.C06` | `CCC_HST.C05` |
| CTUNE | `CCC_ANT.C0B` | `CCC_ANT.C07` |
| DWAS | `DWASINE.C04` | `DWASINE.C02` |
| 6BMOT | `MSV70.C03` | `MSV70.C01` |

Le biais allait systématiquement vers des variantes **plus grosses** que la réalité. Le catalogue contenait donc des fonctions inexistantes sur la voiture. C'est corrigé.

---

## 2. Conséquence majeure : le DSC

**19 fonctions, pas 86.** Et surtout : **aucun** des codes DSC des listes PDF n'existe sur ce véhicule.

| Code du PDF | Présent sur `MK60_87.C03` ? |
|---|---|
| `C0C_DIFF_LOCK` | ❌ absent |
| `C0C_FLR` | ❌ absent |
| `C0C_TB_BDD` | ❌ absent |
| `C0C_FBS` | ❌ absent |
| `C0C_HSA` | ❌ absent |
| `C0C_BB_RAB` | ❌ absent |
| `C0C_HBA` | ❌ n'existe nulle part |

Le seul FSW à préfixe de variante est `C03_UNTERSTEUERSCHWELLE_2` — seuil de sous-virage. **La page entière « codes DSC » est inapplicable à cette voiture.**

Deux confirmations utiles trouvées dans ce même module :

```
MOTOR_ART_DSC_2   →   n46b20 · m47d20 · n52b30 · n54b30
GETRIEBE          →   handschaltung · automatik · sequentiell_ag
```

Le DSC connaît le `n52b30` et la `handschaltung` — cohérent avec le moteur et la boîte mécanique.

---

## 3. Comment lire une ligne

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `TUERAUF_STOP_MAUT` | porte ouverte · arrêt · péage | `aktiv` · `nicht_aktiv` | Opening door aborts power window one-touch movement | Interrompt la remontée automatique de vitre (fonction péage) dès qu'une portière est ouverte. |

| Colonne | Ce que c'est |
|---|---|
| **FSW** | Le nom du réglage, tel qu'il apparaît dans NCS Dummy |
| **Décomposition** | Le nom allemand, fragment par fragment |
| **Valeurs** | Ce que le module accepte, extrait des DATEN |
| **Signification (EN)** | Description NCS Dummy, verbatim |
| **Explication (FR)** | Traduction française, ou `—` si pas encore traduite |

Trois formes de valeurs :

| Forme | Sens |
|---|---|
| `aktiv` / `nicht_aktiv` | Interrupteur |
| `wert_00` … `wert_10` | Valeur dans une plage |
| `ece` / `us` / `gb` | Choix nommé, souvent une variante de marché |

---

## 4. Les deux colonnes de sens

| Colonne | Contenu | Lignes |
|---|---|---:|
| **Signification (EN)** | Description verbatim de la base NCS Dummy | 1 365 |
| **Explication (FR)** | Traduction française | 851 |

Les deux sont indépendantes : l'anglais reste toujours affiché, le français vient à côté.

### La source anglaise

Le fichier `Translations.csv` livré avec NCS Dummy contient **14 823 descriptions** de fonctions et de valeurs, écrites par la communauté — crédits inscrits dans le fichier : *revtor, IcemanBHE, joako, lolo, rdl, StefanL, rondo, LPCapital*. Le recoupement avec les FSW du véhicule donne **1 365 correspondances**.

Ces descriptions sont souvent plus précises que ce que j'aurais écrit, et détaillent parfois chaque valeur :

```
Alarm type [ece: 1 cycle Europe sound, us: 8 cycles USA sound,
            gb: 8 cycles Europe sound (United Kingdom)]
```

### La traduction française

**851 lignes sur 1 738**, dont 114 rédigées directement en français et 737 traduites depuis l'anglais.

La traduction est faite **à la main**. Une substitution automatique mot à mot a été implémentée puis rejetée : couverture lexicale 54 %, et des sorties comme « Catalyseur elimination », « Cornering feux », « Automatique ceinture de securite hand-over uses ». L'ordre des mots diffère trop entre les deux langues.

Les notations d'unités sont converties dans les deux colonnes : `[time_s=data/10]` devient `[durée s = valeur / 10]`.

**Reste à traduire : 514 lignes**, correspondant à 543 descriptions distinctes. Un tiret dans la colonne FR signale une ligne non encore traduite — l'anglais reste disponible juste à côté.

---

## 5. Les catalogues

| Module | Rôle | Fonctions | Valeurs | Expliquées | |
|---|---|---:|---:|---:|---|
| [FRM](FRM.md) | Éclairage, vitres, rétroviseurs | 449 | 448 | 75 | 🟠 |
| [CAPPL](CAPPL.md) | CCC — applications iDrive | 305 | 303 | 7 | 🟠 |
| [KOMBI](KOMBI.md) | Combiné | 295 | 295 | 9 | 🟢 |
| [CAS](CAS.md) | Accès et antidémarrage | 229 | 227 | 7 | 🔴 |
| [JBBF](JBBF.md) | Boîtier de jonction | 97 | 72 | 1 | 🟠 |
| [IHK](IHK.md) | Climatisation | 58 | 57 | 4 | 🟢 |
| [ABG](ABG.md) | Airbags | 53 | 53 | 2 | 🔴 |
| [PDC](PDC.md) | Radars de recul | 48 | 47 | 0 | 🟢 |
| [FZD](FZD.md) | Module de toit | 41 | 41 | 2 | 🟢 |
| [DWAY](DWAY.md) | Alarme — protection volumétrique | 36 | 36 | 2 | 🟠 |
| [EKP](EKP.md) | Pompe à carburant | 28 | 28 | 0 | 🔴 |
| [DWAS](DWAS.md) | Sirène d'alarme | 26 | 15 | 5 | 🟠 |
| [DSC](DSC.md) | Contrôle de stabilité / ABS | 19 | 19 | 0 | 🔴 |
| [CHOST](CHOST.md) | CCC — unité centrale | 19 | 18 | 0 | 🟠 |
| [RLS](RLS.md) | Capteur pluie / luminosité | 12 | 12 | 0 | 🟢 |
| [CAUDI](CAUDI.md) | CCC — audio | 7 | 5 | 0 | 🟠 |
| [CGATE](CGATE.md) | CCC — passerelle | 7 | 7 | 0 | 🟠 |
| [CTUNE](CTUNE.md) | CCC — tuner | 5 | 5 | 0 | 🟠 |
| [6BMOT](6BMOT.md) | **Calculateur moteur** | 4 | 4 | 0 | 🔴 |

Plus :

- **[LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md)** — 1 294 fragments traduits + 130 lectures probables
- **[fsw_par_module.csv](fsw_par_module.csv)** — `module, variante, fsw, bloc, traduction, valeurs, signification_en, explication_fr, source`

---

## 6. Méthode d'extraction

Rétro-ingénierie de deux formats binaires des DATEN.

```
SWTFSW03.DAT   9 429 mots-clés FSW      <id:2> 01 00 <clé:2> NOM \0
SWTPSW03.DAT   2 229 valeurs PSW        même structure
```

Dans les `.C0x`, les noms sont absents — seulement des clés numériques :

```
12 00 01 <adr_lo> <adr_hi> 00 00 <offset> 00 00 00 01 00 <clé FSW:2>
                                                   10 00 <clé PSW:2>   ← une par valeur
```

L'adresse du bloc se reconstruit en `0x00<adr_hi><adr_lo>00`. Le recoupement avec les `.TRC` se fait sur les **trois octets de poids fort** : un module répond souvent à plusieurs sous-adresses dans une même région.

Taux de résolution : **98,5 %** des références FSW.

---

## 7. Ce que ce catalogue ne fait pas

**La colonne Décomposition est morphologique, pas sémantique.** Elle découpe le nom allemand, elle n'explique pas l'effet. C'est le rôle des colonnes Signification et Explication.

**373 lignes n'ont aucune signification**, en anglais comme en français : ni source communautaire, ni certitude de ma part. Mieux vaut vide qu'inventé.

**514 lignes ont l'anglais mais pas encore le français.** La traduction est en cours.

**La traduction est complète mais à deux niveaux.** 1 294 fragments ont une traduction établie ; 130 sont des abréviations BMW sans documentation publique, rendues avec la lecture la plus probable **suivie de `(?)`**. Ce marqueur signale une déduction, pas une source.

**Les valeurs listées sont celles que le module accepte, pas la tienne.** Savoir que `TUERAUF_STOP_MAUT` accepte `aktiv` et `nicht_aktiv` ne dit pas laquelle est active chez toi. Seul NCS Dummy, en ouvrant le `.TRC`, le dit.

---

## 8. Avant toute écriture

| | |
|---|---|
| 1 | **FA sauvegardé** — sans lui, aucun retour possible |
| 2 | **Chargeur branché** — sous-tension = module briqué |
| 3 | **`.TRC` d'origine** conservé, jamais écrasé |
| 4 | **Une modification à la fois**, relue et vérifiée |
| 5 | **Jamais `6BMOT`** — c'est la DME, calibration E85 custom |

Voir [la procédure complète](../01_PROCEDURE.md).
