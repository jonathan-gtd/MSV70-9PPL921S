# Catalogue des DATEN — traduit et expliqué

Le contenu réel de tes SP-Daten E89, module par module, traduit en français.

> **Source :** `C:\NCSEXPER\DATEN\E89` — 433 fichiers, contenu daté du 23/07/2014
> **Extrait le :** 19/09/2026
> **2 282 fonctions codables** réparties sur les 19 modules du véhicule

---

## 1. Ce qu'est le codage, concrètement

Un calculateur BMW contient deux choses distinctes :

| | Le programme | Les données de codage |
|---|---|---|
| **Quoi** | Le firmware — la logique | Une table de réglages |
| **Change avec** | La reprogrammation (WinKFP, flash) | Le codage (NCS Expert) |
| **Risque** | Module briqué | Module mal configuré |

Le codage ne modifie **que la seconde**. Le programme sait déjà faire rabattre les rétroviseurs — la donnée de codage dit simplement s'il doit le faire ou non.

C'est pour cette raison que le codage est réversible : la logique n'a pas bougé.

### La paire FSW / PSW

Chaque réglage est un couple :

```
FSW  =  Funktionsschlüsselwort  =  LA QUESTION   "rabat-on les rétroviseurs au verrouillage ?"
PSW  =  Parameterschlüsselwort  =  LA RÉPONSE    "aktiv" / "nicht_aktiv"
```

Ce catalogue liste **les questions** (FSW) de chaque module. Les réponses possibles (PSW) s'affichent dans NCS Dummy quand on ouvre la trace du module.

### Ce qui circule entre les outils

```
Voiture ──lecture──▶  FSW_PSW.TRC   (état actuel, en hexadécimal)
                            │
                   NCS Dummy + DATEN  (traduit l'hexadécimal en noms lisibles)
                            │
                      FSW_PSW.MAN   (uniquement les lignes à changer)
                            │
Voiture ◀──écriture── SG_CODIEREN
```

Les **DATEN** sont le dictionnaire qui rend les deux fichiers intelligibles. Sans eux, un `.TRC` n'est qu'une suite d'octets. C'est ce dictionnaire qui est extrait et traduit ici.

---

## 2. Les catalogues par module

| Module | Rôle | Fonctions codables | |
|---|---|---:|---|
| [FRM](FRM.md) | Module de plancher — éclairage, vitres, rétroviseurs | 438 | 🟠 |
| [CAS](CAS.md) | Accès et antidémarrage | 229 | 🔴 |
| [KOMBI](KOMBI.md) | Combiné | 237 | 🟢 |
| [CAPPL](CAPPL.md) | CCC — applications iDrive | 233 | 🟠 |
| [DSC](DSC.md) | Contrôle de stabilité / ABS | 761 | 🔴 |
| [ABG](ABG.md) | Airbags | 51 | 🔴 |
| [IHK](IHK.md) | Climatisation | 74 | 🟢 |
| [PDC](PDC.md) | Radars de recul | 45 | 🟢 |
| [FZD](FZD.md) | Module de toit | 41 | 🟢 |
| [DWAY](DWAY.md) | Alarme — composant à confirmer | 36 | 🟠 |
| [JBBF](JBBF.md) | Boîtier de jonction | 34 | 🟠 |
| [EKP](EKP.md) | Pompe à carburant | 28 | 🔴 |
| [DWAS](DWAS.md) | Sirène d'alarme | 26 | 🟠 |
| [CHOST](CHOST.md) | CCC — unité centrale | 18 | 🟠 |
| [RLS](RLS.md) | Capteur pluie / luminosité | 10 | 🟢 |
| [CGATE](CGATE.md) | CCC — passerelle | 8 | 🟠 |
| [CAUDI](CAUDI.md) | CCC — audio | 7 | 🟠 |
| [CTUNE](CTUNE.md) | CCC — tuner | 4 | 🟠 |
| [6BMOT](6BMOT.md) | **Calculateur moteur** | 2 | 🔴 |

Plus :

- **[LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md)** — les 499 fragments allemands traduits, la clé de lecture
- **[fsw_par_module.csv](fsw_par_module.csv)** — les 2 282 lignes en format exploitable

> 💡 `DSC` affiche 761 fonctions parce que la variante exacte n'a pas pu être déterminée : c'est l'union de 26 fichiers de variantes. La vraie liste de ton DSC est bien plus courte.

---

## 3. Comment ce catalogue a été construit

Il ne vient pas d'un forum. Il est extrait de **tes** fichiers DATEN.

### Le dictionnaire des mots-clés

`SWTFSW03.DAT` contient les 9 429 mots-clés FSW de la série E89, sous forme d'enregistrements :

```
<identifiant:2> 01 00 <clé:2> NOM_DU_FSW \0
```

### Les références dans chaque module

Les fichiers `.C0x` d'un module ne contiennent pas les noms, seulement des références numériques, encadrées par un motif constant :

```
12 00 01 ?? ?? 00 00 ?? 00 00 00 01 00 <clé:2>
```

Chaque clé trouvée est résolue via le dictionnaire. **Taux de résolution : 98,5 %** — sur 1 441 références testées, 21 seulement sont restées non résolues.

### La traduction

Les noms BMW sont des assemblages de fragments allemands séparés par `_`. Un dictionnaire de 499 fragments traduit chaque composant, puis les glose de gauche à droite. Voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).

---

## 4. Ce que ce catalogue ne fait pas

Trois limites à garder en tête. Elles ne le rendent pas inutile — elles disent où s'arrête sa fiabilité.

**La traduction est morphologique, pas sémantique.** Elle décompose le nom, elle n'explique pas l'effet réel sur la voiture. `TUERAUF_STOP_MAUT` se lit *porte ouverte · arrêt · péage* : il faut savoir que BMW appelle « fonction péage » la remontée de vitre d'une impulsion pour comprendre que ce code l'interrompt quand une portière est ouverte.

**Les valeurs (PSW) ne sont pas incluses.** Le catalogue liste les questions, pas les réponses possibles. NCS Dummy les affiche.

**La variante retenue peut ne pas être la tienne.** Chaque module existe en plusieurs versions matérielles (`.C06`, `.C08`…). La variante indiquée en tête de chaque fichier est celle déduite de `E89SGET.000`. Si NCS Dummy en annonce une autre en ouvrant ton `.TRC`, c'est **NCS Dummy qui a raison**.

> **Ce catalogue sert à chercher et à comprendre. Il ne remplace pas la lecture du module.**

---

## 5. Méthode de travail

1. **Ici**, au calme : trouver la fonction voulue, noter le FSW
2. **NCS Dummy** : ouvrir le `.TRC` du module, vérifier que le FSW existe dans ta variante et voir sa valeur actuelle
3. **NCS Expert** : coder, relire, vérifier

Un FSW présent dans ce catalogue mais absent de ton `.TRC` signifie simplement que ta variante ne l'expose pas. C'est normal et fréquent.

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
