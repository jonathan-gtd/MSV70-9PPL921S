# 02 — NCS Dummy : lister et éditer les options

> `C:\NCS Dummy\NcsDummy.exe`  
> **Fonctionne hors voiture.** Il lit les DATEN sur le disque — c'est l'outil à utiliser au bureau pour préparer une session.

---

## 1. À quoi il sert

NCS Expert sait lire et écrire dans les modules, mais affiche tout en **hexadécimal brut** : des paires `FSW / PSW` illisibles.

NCS Dummy fait la traduction. Il lit les mêmes fichiers DATEN que NCS Expert et donne :

- la **liste exhaustive** des fonctions codables d'un module
- pour chaque fonction, **toutes les valeurs possibles**, en clair
- la **valeur actuelle** de la voiture si on lui fournit un `.TRC`
- la génération du `FSW_PSW.MAN` prêt à écrire

C'est donc **lui** qui répond à la question « quelles options sont codables sur ma voiture ».

---

## 2. Configuration initiale (une seule fois)

Au premier lancement, il demande où se trouve NCS Expert :

```
C:\NCSEXPER
```

Il en déduit seul les sous-dossiers `DATEN\`, `SGDAT\` et `WORK\`. Si une série de données est demandée explicitement :

```
C:\NCSEXPER\DATEN\E89
```

---

## 3. Deux modes de lecture

### Mode A — Catalogue complet (sans la voiture)

On ouvre directement un module depuis les DATEN. On obtient **tout ce qui est codable**, indépendamment de la voiture.

```
Série   →  E89
Module  →  ex. KOMBI
```

C'est le mode exploration. Idéal au bureau pour repérer les fonctions intéressantes avant d'aller à la voiture.

⚠️ Un module existe en plusieurs versions matérielles (`.C04`, `.C06`, `.C08`…). Sans trace de la voiture, on ne sait pas laquelle est installée — donc **certaines fonctions listées peuvent ne pas exister sur ton exemplaire**. C'est de la reconnaissance, pas une vérité.

### Mode B — Avec le fichier de la voiture (recommandé)

On charge le `.TRC` produit par NCS Expert (`F4 Read ECU`) :

```
data/Coding/backup/KOMBI_stock_20260918.TRC
```

NCS Dummy identifie alors **la bonne version du module** et affiche la valeur réellement en place pour chaque fonction. C'est la seule vue fiable.

---

## 4. Comment lire l'affichage

| Terme | Signification |
|---|---|
| **FSW** | *Funktionsschlüsselwort* — la **fonction** (« que fait-on ? ») |
| **PSW** | *Parameterschlüsselwort* — la **valeur** (« comment ? ») |

Un codage = un couple **FSW + PSW**.

Les libellés d'origine sont en **allemand**, souvent abrégés. Quelques clés de lecture :

| Fragment | Sens |
|---|---|
| `aktiv` / `nicht_aktiv` | activé / désactivé |
| `mit` / `ohne` | avec / sans |
| `TUER` | porte |
| `SPIEGEL` | rétroviseur |
| `LICHT` | lumière |
| `WARN` | alerte |
| `ZEIT` / `_S_` | durée / secondes |
| `US` / `ECE` / `EU` | marché destinataire |

> 💡 Ne jamais modifier une fonction dont on ne comprend pas le libellé. Le fait qu'elle soit codable ne veut pas dire qu'elle est sans effet ailleurs.

---

## 5. Générer le fichier de modification

1. Trouver la fonction (recherche sur le nom FSW)
2. Changer sa valeur
3. Exporter le fichier de manipulation vers :

```
C:\NCSEXPER\WORK\FSW_PSW.MAN
```

### ⚠️ Le `.MAN` ne doit contenir que les lignes modifiées

`SG_CODIEREN` applique **tout** ce que contient le `.MAN`. Un `.MAN` contenant l'intégralité du module réécrit l'intégralité du module — inutilement risqué.

**Un bon `.MAN` fait quelques lignes.** S'il en fait des centaines, il y a une erreur d'export.

---

## 6. Méthode recommandée : le diff

```
1. Lire le module     → KOMBI_stock.TRC        (avant)
2. Éditer, générer le .MAN, coder
3. Relire le module   → KOMBI_20260918.TRC     (après)
4. Comparer les deux .TRC dans NCS Dummy
```

La comparaison doit montrer **exactement** les lignes voulues, ni plus ni moins. Toute ligne modifiée non prévue est un signal d'alarme : relire, comprendre, restaurer depuis le `.TRC` d'origine avant d'aller plus loin.

---

## 7. Méthode de travail conseillée

| Où | Quoi |
|---|---|
| **Au bureau** | Mode A — explorer le catalogue, noter les FSW intéressants |
| **À la voiture** | Lire les `.TRC` des modules visés, les sauvegarder, rentrer |
| **Au bureau** | Mode B — préparer les `.MAN` tranquillement |
| **À la voiture** | Coder, relire, vérifier |

Coder et découvrir en même temps, contact mis, est la meilleure façon de faire une erreur.

---

## 8. Ordre de grandeur

Un module comme le JBBF expose **plusieurs centaines de FSW**. Le catalogue complet de la voiture se compte en milliers de lignes.

N'essaie pas de tout lire. Pars d'une fonction que tu veux obtenir, cherche-la, ignore le reste.
