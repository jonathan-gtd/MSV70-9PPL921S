# Catalogue des DATEN — traduit et expliqué

Le contenu réel de tes SP-Daten E89, module par module, en français.

> **Source :** `C:\NCSEXPER\DATEN\E89` — 433 fichiers, contenu daté du 23/07/2014
> **2 282 fonctions codables** sur les 19 modules du véhicule
> **2 256 avec leurs valeurs possibles** (98,9 %) · **160 avec une explication rédigée**

---

## 1. Ce qu'est le codage, concrètement

Un calculateur BMW contient deux choses distinctes :

| | Le programme | Les données de codage |
|---|---|---|
| **Quoi** | Le firmware — la logique | Une table de réglages |
| **Change avec** | La reprogrammation (WinKFP, flash) | Le codage (NCS Expert) |
| **Risque** | Module briqué | Module mal configuré |

Le codage ne modifie **que la seconde**. Le programme sait déjà faire rabattre les rétroviseurs — la donnée de codage dit simplement s'il doit le faire. C'est pour ça que le codage est réversible : la logique n'a pas bougé.

### La paire FSW / PSW

Chaque réglage est un couple :

```
FSW  =  Funktionsschlüsselwort  =  LA QUESTION   SPIEGEL_KOMFORT_EINKLAPP
PSW  =  Parameterschlüsselwort  =  LA RÉPONSE    aktiv  /  nicht_aktiv
```

Ce catalogue donne **les deux** : la question et les réponses autorisées.

### Comment lire une ligne

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `TUERAUF_STOP_MAUT` | porte ouverte · arrêt · péage | `aktiv` · `nicht_aktiv` | Interrompt la remontée automatique de vitre (fonction péage) dès qu'une portière est ouverte. |

- **Traduction** — décomposition mot à mot du nom allemand
- **Valeurs possibles** — ce que le module accepte réellement, extrait des DATEN
- **Explication** — rédigée seulement pour les fonctions identifiées avec certitude

Trois formes de valeurs reviennent :

| Forme | Sens |
|---|---|
| `aktiv` / `nicht_aktiv` | Interrupteur |
| `wert_01` … `wert_08` | Valeur numérique dans une plage |
| `ece` / `us` / `gb` | Choix nommé — souvent une variante de marché |

### Ce qui circule entre les outils

```
Voiture ──lecture──▶  FSW_PSW.TRC   (état actuel, en hexadécimal)
                            │
                   NCS Dummy + DATEN  (traduit en noms lisibles)
                            │
                      FSW_PSW.MAN   (uniquement les lignes à changer)
                            │
Voiture ◀──écriture── SG_CODIEREN
```

Les DATEN sont le dictionnaire qui rend ces fichiers intelligibles. C'est lui qui est extrait ici.

---

## 2. Les catalogues par module

| Module | Rôle | Fonctions | Avec valeurs | Expliquées | |
|---|---|---:|---:|---:|---|
| [DSC](DSC.md) | Contrôle de stabilité / ABS | 761 | 761 | 47 | 🔴 |
| [FRM](FRM.md) | Éclairage, vitres, rétroviseurs | 438 | 437 | 75 | 🟠 |
| [KOMBI](KOMBI.md) | Combiné | 237 | 237 | 6 | 🟢 |
| [CAPPL](CAPPL.md) | CCC — applications iDrive | 233 | 231 | 7 | 🟠 |
| [CAS](CAS.md) | Accès et antidémarrage | 229 | 227 | 7 | 🔴 |
| [IHK](IHK.md) | Climatisation | 74 | 72 | 6 | 🟢 |
| [ABG](ABG.md) | Airbags | 51 | 51 | 2 | 🔴 |
| [PDC](PDC.md) | Radars de recul | 45 | 44 | 0 | 🟢 |
| [FZD](FZD.md) | Module de toit | 41 | 41 | 2 | 🟢 |
| [DWAY](DWAY.md) | Alarme — composant à confirmer | 36 | 36 | 2 | 🟠 |
| [JBBF](JBBF.md) | Boîtier de jonction | 34 | 30 | 1 | 🟠 |
| [EKP](EKP.md) | Pompe à carburant | 28 | 28 | 0 | 🔴 |
| [DWAS](DWAS.md) | Sirène d'alarme | 26 | 15 | 5 | 🟠 |
| [CHOST](CHOST.md) | CCC — unité centrale | 18 | 17 | 0 | 🟠 |
| [RLS](RLS.md) | Capteur pluie / luminosité | 10 | 10 | 0 | 🟢 |
| [CGATE](CGATE.md) | CCC — passerelle | 8 | 8 | 0 | 🟠 |
| [CAUDI](CAUDI.md) | CCC — audio | 7 | 5 | 0 | 🟠 |
| [CTUNE](CTUNE.md) | CCC — tuner | 4 | 4 | 0 | 🟠 |
| [6BMOT](6BMOT.md) | **Calculateur moteur** | 2 | 2 | 0 | 🔴 |

Plus :

- **[LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md)** — les 499 fragments allemands traduits, la clé de lecture
- **[fsw_par_module.csv](fsw_par_module.csv)** — les 2 282 lignes exploitables : `module, fsw, traduction, valeurs, explication`

> 💡 `DSC` affiche 761 fonctions parce que sa variante exacte n'a pas pu être déterminée : c'est l'union de 26 fichiers. Ta vraie liste est bien plus courte.

---

## 3. Comment ce catalogue a été construit

Il ne vient pas d'un forum. Il est extrait de **tes** fichiers DATEN, par rétro-ingénierie de deux formats binaires.

### Les dictionnaires

```
SWTFSW03.DAT   9 429 mots-clés FSW de la série E89
SWTPSW03.DAT   2 229 valeurs PSW
```

Même structure d'enregistrement dans les deux :

```
<identifiant:2> 01 00 <clé:2> NOM \0
```

### Les références dans chaque module

Les `.C0x` ne contiennent pas les noms, seulement des clés numériques :

```
12 00 01 ?? ?? 00 00 ?? 00 00 00 01 00 <clé FSW:2>   ← début d'enregistrement
                                  10 00 <clé PSW:2>   ← une par valeur autorisée
```

Chaque clé est résolue via le dictionnaire correspondant.

**Taux de résolution : 98,5 %** pour les FSW, **98,9 %** des fonctions ont leurs valeurs.

### La traduction

Les noms BMW sont des assemblages de fragments allemands séparés par `_`. Un dictionnaire de 499 fragments traduit chaque composant. Voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).

---

## 4. Ce que ce catalogue ne fait pas

**La traduction est morphologique, pas sémantique.** Elle décompose le nom, elle n'explique pas l'effet. `TUERAUF_STOP_MAUT` se lit *porte ouverte · arrêt · péage* : il faut savoir que BMW appelle « fonction péage » la remontée de vitre d'une impulsion pour comprendre ce que fait ce code. C'est précisément pour ça que la colonne **Explication** existe — mais elle ne couvre que 160 lignes sur 2 282.

**Une case Explication vide ne veut pas dire que la fonction est sans intérêt.** Elle veut dire que son effet n'a pas été établi de source sûre. Mieux vaut une case vide qu'une invention.

**La variante retenue peut ne pas être la tienne.** Chaque module existe en plusieurs versions (`.C06`, `.C08`…). Celle indiquée en tête de fichier est déduite de `E89SGET.000`. Si NCS Dummy en annonce une autre en ouvrant ton `.TRC`, **c'est NCS Dummy qui a raison**.

**Les valeurs sont celles que le module accepte, pas la tienne.** Savoir que `TUERAUF_STOP_MAUT` accepte `aktiv` et `nicht_aktiv` ne dit pas laquelle est active sur ta voiture. Ça, seule la lecture du module le dit.

---

## 5. Méthode de travail

1. **Ici**, au calme : trouver la fonction, noter le FSW et les valeurs possibles
2. **NCS Dummy** : ouvrir le `.TRC`, vérifier que le FSW existe dans ta variante et voir sa valeur actuelle
3. **NCS Expert** : coder, relire, vérifier

Un FSW présent ici mais absent de ton `.TRC` signifie simplement que ta variante ne l'expose pas. C'est normal et fréquent.

---

## 6. Avant toute écriture

| | |
|---|---|
| 1 | **FA sauvegardé** — sans lui, aucun retour possible |
| 2 | **Chargeur branché** — sous-tension = module briqué |
| 3 | **`.TRC` d'origine** conservé, jamais écrasé |
| 4 | **Une modification à la fois**, relue et vérifiée |
| 5 | **Jamais `6BMOT`** — c'est la DME, calibration E85 custom |

Voir [la procédure complète](../01_PROCEDURE.md).
