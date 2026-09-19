# Catalogue des DATEN — ciblé sur ce véhicule

Uniquement les fonctions que **ta** voiture expose réellement, traduites.

> **Source :** `C:\NCSEXPER\DATEN\E89` croisé avec les 19 traces `.TRC` lues sur le véhicule
> **1 779 fonctions codables** · **1 732 avec leurs valeurs** (97 %) · **127 avec explication rédigée**
> **787 fragments allemands traduits** · 58 % des lignes entièrement traduites

---

## 1. Comment le ciblage a été fait

C'est le point qui change tout par rapport à une liste de forum.

Chaque fonction, dans les DATEN, porte l'adresse du bloc de codage où elle vit. Chaque `.TRC` lu sur la voiture liste les blocs que le module a **réellement renvoyés**. En croisant les deux :

```
fonction dans un bloc présent dans le TRC   →   elle existe sur ta voiture
fonction dans un bloc absent du TRC         →   écartée
```

### Bénéfice inattendu : l'identification des variantes

En testant **chaque variante** de chaque module contre son `.TRC`, celle dont 100 % des fonctions tombent dans les blocs observés est la bonne. **18 modules sur 19 identifiés de façon certaine.**

| Module | Variante réelle | Ce que supposait `E89SGET.000` |
|---|---|---|
| FRM | **`FRMFA.C12`** | C10 |
| KOMBI | **`KMBI_PL2.C08`** | C06 |
| CAS | **`CAS3.C09`** | C06 |
| ABG | **`MRS5CD.C22`** | MRS5.C06 |
| JBBF | **`JBBFE.C07`** | C05 |
| PDC | **`PDC_E87.C05`** | C03 |
| RLS | **`RLS_PL2.C08`** | C07 |
| DSC | **`MK60_87.C0A`** | indéterminé |
| DWAY | **`DWA_IRS.C05`** | indéterminé |

Deux mystères résolus au passage :

**`DWAY` est bien `DWA_IRS`** — la protection volumétrique de l'habitacle (*Innenraumschutz*). 36 fonctions, correspondance exacte.

**Le DSC passe de 761 à 86 fonctions.** Et ses préfixes réels sont `C0A` (60 fonctions), `C0B` (5) et `C09` (1). **`C0C_` n'existe pas sur ce véhicule** : tous les codes DSC des listes PDF sont inapplicables tels quels — il faut lire `C0A_DIFF_LOCK` et non `C0C_DIFF_LOCK`.

---

## 2. Comment lire une ligne

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `TUERAUF_STOP_MAUT` | porte ouverte · arrêt · péage | `aktiv` · `nicht_aktiv` | Interrompt la remontée automatique de vitre (fonction péage) dès qu'une portière est ouverte. |

- **Traduction** — décomposition du nom allemand, fragment par fragment
- **Valeurs** — ce que le module accepte, extrait des DATEN
- **Explication** — rédigée seulement quand l'effet est établi de source sûre

Trois formes de valeurs :

| Forme | Sens |
|---|---|
| `aktiv` / `nicht_aktiv` | Interrupteur |
| `wert_00` … `wert_08` | Valeur dans une plage |
| `ece` / `us` / `gb` | Choix nommé, souvent une variante de marché |

---

## 3. Les catalogues

| Module | Rôle | Variante | Fonctions | Valeurs | Expliquées | |
|---|---|---|---:|---:|---:|---|
| [FRM](FRM.md) | Éclairage, vitres, rétroviseurs | `FRMFA.C12` | 449 | 448 | 75 | 🟠 |
| [KOMBI](KOMBI.md) | Combiné | `KMBI_PL2.C08` | 295 | 295 | 9 | 🟢 |
| [CAS](CAS.md) | Accès et antidémarrage | `CAS3.C09` | 255 | 253 | 7 | 🔴 |
| [CAPPL](CAPPL.md) | CCC — applications iDrive | `CCC_APP.C0B` | 216 | 214 | 7 | 🟠 |
| [JBBF](JBBF.md) | Boîtier de jonction | `JBBFE.C07` | 97 | 72 | 1 | 🟠 |
| [DSC](DSC.md) | Contrôle de stabilité / ABS | `MK60_87.C0A` | 86 | 86 | 8 | 🔴 |
| [IHK](IHK.md) | Climatisation | `IHXXPL2.C02` | 74 | 72 | 6 | 🟢 |
| [ABG](ABG.md) | Airbags | `MRS5CD.C22` | 67 | 67 | 5 | 🔴 |
| [PDC](PDC.md) | Radars de recul | `PDC_E87.C05` | 48 | 47 | 0 | 🟢 |
| [FZD](FZD.md) | Module de toit | `FZD.C06` | 41 | 41 | 2 | 🟢 |
| [DWAY](DWAY.md) | Alarme — protection volumétrique | `DWA_IRS.C05` | 36 | 36 | 2 | 🟠 |
| [EKP](EKP.md) | Pompe à carburant | `EKPM60_2.C03` | 28 | 28 | 0 | 🔴 |
| [DWAS](DWAS.md) | Sirène d'alarme | `DWASINE.C02` | 26 | 15 | 5 | 🟠 |
| [CHOST](CHOST.md) | CCC — unité centrale | `CCC_HST.C05` | 19 | 18 | 0 | 🟠 |
| [CAUDI](CAUDI.md) | CCC — audio | `CCC_ASK.C04` | 13 | 11 | 0 | 🟠 |
| [RLS](RLS.md) | Capteur pluie / luminosité | `RLS_PL2.C08` | 12 | 12 | 0 | 🟢 |
| [CGATE](CGATE.md) | CCC — passerelle | `CCC_GW.C03` | 8 | 8 | 0 | 🟠 |
| [CTUNE](CTUNE.md) | CCC — tuner | `CCC_ANT.C07` | 5 | 5 | 0 | 🟠 |
| [6BMOT](6BMOT.md) | **Calculateur moteur** | `MSV70.C01` | 4 | 4 | 0 | 🔴 |

Plus :

- **[LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md)** — les 787 fragments traduits
- **[fsw_par_module.csv](fsw_par_module.csv)** — `module, variante, fsw, bloc, traduction, valeurs, explication`

---

## 4. Le seul module incertain : CAPPL

**89 fonctions écartées sur 305.** C'est le seul module où aucune variante n'atteint 100 % — `CCC_APP.C0B` plafonne à 71 %.

Deux explications possibles : la lecture du `.TRC` était partielle, ou la variante exacte du CCC n'est pas dans les DATEN E89. Les 216 fonctions retenues sont fiables (leur bloc est bien présent) ; ce sont les 89 écartées qui méritent une relecture du module si tu veux en avoir le cœur net.

Tous les autres modules sont à 0 écartée.

---

## 5. Méthode d'extraction

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

L'adresse du bloc se reconstruit en `0x00<adr_hi><adr_lo>00`. C'est elle qui permet le croisement avec les `.TRC`.

Taux de résolution : **98,5 %** des références FSW, **97 %** des fonctions ont leurs valeurs.

---

## 6. Ce que ce catalogue ne fait pas

**La traduction est morphologique, pas sémantique.** Elle décompose le nom, elle n'explique pas l'effet. D'où la colonne Explication — qui ne couvre que 127 lignes sur 1 779.

**Une case Explication vide n'est pas un désaveu de la fonction.** Elle veut dire que son effet n'est pas établi. Mieux vaut vide qu'inventé.

**Environ 760 fragments restent non traduits** (en **gras**), pour 930 occurrences. Ce sont majoritairement des sigles internes BMW apparaissant une ou deux fois. 58 % des lignes sont entièrement traduites ; sur le reste, le fragment manquant est généralement un suffixe technique, pas le cœur du sens.

**Les valeurs listées sont celles que le module accepte, pas la tienne.** Savoir que `TUERAUF_STOP_MAUT` accepte `aktiv` et `nicht_aktiv` ne dit pas laquelle est active chez toi. Seul NCS Dummy, en ouvrant le `.TRC`, le dit.

---

## 7. Avant toute écriture

| | |
|---|---|
| 1 | **FA sauvegardé** — sans lui, aucun retour possible |
| 2 | **Chargeur branché** — sous-tension = module briqué |
| 3 | **`.TRC` d'origine** conservé, jamais écrasé |
| 4 | **Une modification à la fois**, relue et vérifiée |
| 5 | **Jamais `6BMOT`** — c'est la DME, calibration E85 custom |

Voir [la procédure complète](../01_PROCEDURE.md).
