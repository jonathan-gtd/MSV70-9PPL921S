# Catalogue des DATEN — ce véhicule uniquement

Les fonctions codables des 19 calculateurs de la voiture, dans leurs variantes réelles, traduites.

> **Variantes :** relevées par NCS Expert sur le véhicule
> **1 738 fonctions codables** · **1 692 avec leurs valeurs** (97,4 %)
> **1 365 avec une signification en anglais** · **1 479 avec une explication française**, dont **113 déductions du libellé**
> **259 fonctions sans explication** · décomposition disponible pour chaque ligne, avec des fragments encore ambigus

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
| DWAI | `DWA_IRS.C05` | 36 |
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
| **Explication (FR)** | Traduction française ou déduction explicitement signalée |

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
| **Explication (FR)** | Traduction française ou déduction signalée | 1 479 |

Les deux sont indépendantes : l'anglais reste toujours affiché, le français vient à côté.

### La source anglaise

Le fichier `Translations.csv` livré avec NCS Dummy contient **14 823 descriptions** de fonctions et de valeurs, écrites par la communauté — crédits inscrits dans le fichier : *revtor, IcemanBHE, joako, lolo, rdl, StefanL, rondo, LPCapital*. Le recoupement avec les FSW du véhicule donne **1 365 correspondances**.

Ces descriptions sont souvent plus précises que ce que j'aurais écrit, et détaillent parfois chaque valeur :

```
Alarm type [ece: 1 cycle Europe sound, us: 8 cycles USA sound,
            gb: 8 cycles Europe sound (United Kingdom)]
```

### La traduction française

**1 479 lignes expliquées en français : toute description anglaise disponible a sa traduction française.** 114 explications préexistantes ont le statut `maison`, 1 252 le statut `traduit`. La passe du 20/09/2026 ajoute **113 interprétations** au statut `deduction_libelle`, préfixées « Déduction du libellé : ».

La traduction est faite **à la main**, en quatre passes. Une substitution automatique mot à mot avait été implémentée puis mesurée et rejetée : couverture lexicale 54 %, et des sorties comme « Catalyseur elimination », « Cornering feux », « Automatique ceinture de securite hand-over uses ». L'ordre des mots diffère trop entre les deux langues pour une substitution terme à terme.

Les notations d'unités sont converties dans les deux colonnes : `[time_s=data/10]` devient `[durée s = valeur / 10]`, `[angle_°=data/50]` devient `[angle degrés = valeur / 50]`.

**Les 259 lignes restantes n'ont de description dans aucune des deux langues.** Elles affichent `—` dans les deux colonnes. Les nouvelles déductions ne remplissent pas la colonne anglaise et ne constituent pas des effets validés sur le véhicule. Voir [la méthode et les sources](TRADUCTIONS.md).

---

## 5. Les catalogues

| Module | Rôle | Fonctions | Valeurs | Expliquées | |
|---|---|---:|---:|---:|---|
| [FRM](FRM.md) | Éclairage, vitres, rétroviseurs | 449 | 448 | 409 | 🟠 |
| [CAPPL](CAPPL.md) | CCC — applications iDrive | 305 | 303 | 293 | 🟠 |
| [KOMBI](KOMBI.md) | Combiné | 295 | 295 | 216 | 🟢 |
| [CAS](CAS.md) | Accès et antidémarrage | 229 | 227 | 185 | 🔴 |
| [JBBF](JBBF.md) | Boîtier de jonction | 97 | 72 | 71 | 🟠 |
| [IHK](IHK.md) | Climatisation | 58 | 57 | 43 | 🟢 |
| [ABG](ABG.md) | Airbags | 53 | 53 | 43 | 🔴 |
| [PDC](PDC.md) | Radars de recul | 48 | 47 | 37 | 🟢 |
| [FZD](FZD.md) | Module de toit | 41 | 41 | 33 | 🟢 |
| [DWAI](DWAI.md) | Alarme — protection volumétrique | 36 | 36 | 35 | 🟠 |
| [EKP](EKP.md) | Pompe à carburant | 28 | 28 | 17 | 🔴 |
| [DWAS](DWAS.md) | Sirène d'alarme | 26 | 15 | 26 | 🟠 |
| [DSC](DSC.md) | Contrôle de stabilité / ABS | 19 | 19 | 18 | 🔴 |
| [CHOST](CHOST.md) | CCC — unité centrale | 19 | 18 | 19 | 🟠 |
| [RLS](RLS.md) | Capteur pluie / luminosité | 12 | 12 | 12 | 🟢 |
| [CAUDI](CAUDI.md) | CCC — audio | 7 | 5 | 7 | 🟠 |
| [CGATE](CGATE.md) | CCC — passerelle | 7 | 7 | 6 | 🟠 |
| [CTUNE](CTUNE.md) | CCC — tuner | 5 | 5 | 5 | 🟠 |
| [6BMOT](6BMOT.md) | **Calculateur moteur** | 4 | 4 | 4 | 🔴 |

Plus :

- **[LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md)** — 1 298 fragments traduits ou contextualisés + 126 lectures probables ; des fragments restent non résolus
- **[fsw_par_module.csv](fsw_par_module.csv)** — `module, variante, fsw, bloc, traduction, valeurs, signification_en, explication_fr, source`
- **[TRADUCTIONS.md](TRADUCTIONS.md)** — provenance des corrections et limites des déductions
- **[verifier_catalogue.py](verifier_catalogue.py)** — contrôle de cohérence du CSV, des fiches et des liens

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

**259 lignes n'ont aucune explication**, en anglais comme en français. Les **113 nouvelles déductions** sont distinguées des traductions de descriptions existantes.

**Toute ligne disposant d'une description anglaise a sa traduction française.** Il n'y a plus d'écart entre les deux colonnes.

**La décomposition n'est pas une garantie de compréhension complète.** Le lexique contient 1 298 fragments traduits ou contextualisés et 126 lectures probables. Le marqueur `(?)` signale une hypothèse ; les fragments entre underscores restent ambigus ou non résolus. Le contexte du module prime sur une substitution mot à mot.

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
