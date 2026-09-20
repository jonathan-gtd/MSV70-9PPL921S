# 00 — Installation

> État de l'installation **vérifié sur le poste le 18/09/2026**.  
> Tout est en place. Ce document sert à contrôler qu'une machine est prête, pas à réinstaller.

---

## 1. Inventaire vérifié

| Composant | Chemin | État |
|---|---|---|
| NCS Expert | `C:\NCSEXPER\BIN\NCSEXPER.exe` | ✅ v4, build 24/04/2013 |
| NCS Dummy | `C:\NCS Dummy\NcsDummy.exe` | ✅ |
| Tool32 | `C:\EDIABAS\Bin\Tool32.exe` | ✅ |
| INPA | `C:\EC-APPS\INPA\BIN\INPA.exe` | ✅ |
| DATEN E89 | `C:\NCSEXPER\DATEN\E89\` | ✅ 433 fichiers (contenu daté 23/07/2014) |
| SGDAT | `C:\NCSEXPER\SGDAT\` | ✅ 645 fichiers |
| Descriptions EDIABAS | `C:\EDIABAS\ECU\` | ✅ 1219 `.prg` — dont `CAS.PRG` |
| Profils | `C:\NCSEXPER\PFL\` | ✅ `NCSDUMMY4.PFL` + `01_Default.pfl` |
| Fichier de manipulation | `C:\NCSEXPER\WORK\FSW_PSW.MAN` | ✅ présent, **0 octet** |

**E89 = la série de données qui couvre E81/E82/E87/E88/E90/E91/E92/E93.** Il n'existe pas de dossier « E90 », c'est normal.

---

## 2. Les DATEN — ne pas y toucher

Les fichiers DATEN sont la base de données de codage de BMW : pour chaque module et chaque version matérielle, la liste exhaustive des fonctions codables et de leurs valeurs. NCS Expert et NCS Dummy ne savent rien par eux-mêmes, ils lisent ça.

La mise à jour a déjà été faite sur ce poste. Preuve : les dossiers de sauvegarde horodatés que **BMW Coding Tool** crée lui-même en s'exécutant —

```
C:\NCSEXPER\SGDAT_20240805_195649\
C:\NCSEXPER\DATEN\R50_20240805_195659\
```

🔶 **Ne relance pas BMW Coding Tool.** Le contenu daté de 2014 n'est pas de l'obsolescence : la production E9x s'est arrêtée en 2013, rien qui concerne une E90 de 2006 n'a bougé après. Une « mise à jour » ne peut que casser une installation qui fonctionne.

---

## 3. Les deux profils

Les profils sont dans `C:\NCSEXPER\PFL\`. La liste affichée par NCS Expert montre leur **description interne**, pas le nom de fichier :

| Affiché dans NCS Expert | Fichier | Usage |
|---|---|---|
| **NCS Dummy profile** | `NCSDUMMY4.PFL` | ✅ **codage manuel FSW/PSW** — celui de tous les jours |
| Default Profil | `01_Default.pfl` | 🔶 manipulation FSW/PSW désactivée ; peut coder depuis le FA |

Toute la différence tient à une ligne :

```ini
[FSWPSW]
FswPswManipulieren=1    ← NCSDUMMY4  : écriture depuis FSW_PSW.MAN autorisée
FswPswManipulieren=0    ← Default    : manipulation du .MAN désactivée
```

`FswPswManipulieren=0` ne rend pas le profil « lecture seule » : il désactive
l'utilisation du `.MAN`. Le job choisi détermine si l'on lit ou si l'on code.

⚠️ **NCS Expert ne charge aucun profil au démarrage.** C'est à refaire **à chaque lancement**, via la barre de menus en haut (`File → Load Profile`), pas via les boutons F1–F6. Sans profil chargé, les boutons restent grisés.

Le nom du profil actif s'affiche **en bas de la fenêtre**. C'est le seul témoin fiable.

---

## 4. Règle du `FSW_PSW.MAN` vide

Le fichier `C:\NCSEXPER\WORK\FSW_PSW.MAN` doit exister et faire **0 octet au lancement** de NCS Expert.

C'est le fichier que le job `SG_CODIEREN` applique à la voiture. S'il contient les restes d'une session précédente, tu risques d'écrire dans un module les valeurs destinées à un autre.

**Réflexe de fin de session :**
```
copy nul "C:\NCSEXPER\WORK\FSW_PSW.MAN"
```

---

## 5. Interface — le seul point à surveiller

Configuration EDIABAS en place :

```ini
C:\EDIABAS\BIN\EDIABAS.INI   →  Interface = STD:OBD
C:\EDIABAS\BIN\OBD.INI       →  [OBD] Port = Com1
```

🔶 **`OBD.INI` est réglé sur COM1.** Vérifie le port réel du câble K+DCAN :

```
Gestionnaire de périphériques → Ports (COM et LPT) → USB Serial Port (COMx)
```

Deux réglages à faire une fois pour toutes, dans **Propriétés → Paramètres du port → Avancé** :

| Réglage | Valeur | Pourquoi |
|---|---|---|
| Numéro de port COM | **COM1 à COM9** | EDIABAS ne sait pas adresser au-delà de COM9 |
| Latency Timer | **1 ms** | Valeur par défaut 16 ms → timeouts en codage |

Si le port réel n'est pas COM1, soit tu le forces à COM1 dans le Gestionnaire, soit tu corriges la ligne `Port=` de `OBD.INI`. **Les deux doivent concorder.**

---

## 6. Prérequis physiques — non négociables

| Point | Détail |
|---|---|
| **Chargeur de batterie** | Branché et actif. Pas « batterie chargée » : chargeur. Un codage dure plusieurs minutes contact mis, sans alternateur. |
| Contact | Position 2 (tout allumé, moteur coupé) |
| Moteur | **Coupé** |
| Consommateurs | Phares, clim, autoradio éteints |
| Portes | Fermées (évite les réveils de modules en cours d'écriture) |
| Ordinateur | Sur secteur, veille et mise en veille prolongée **désactivées** |

> Sur E9x, coder un module en sous-tension est la première cause de FRM briqué. Le chargeur n'est pas une précaution de confort.

---

## 7. Test à blanc, sans voiture

On peut lancer NCS Expert et explorer les menus au bureau. Dans ce cas :

- Toute tentative de lecture échoue — **c'est normal**, il n'y a personne au bout du câble
- NCS Expert retombe sur son jeu de démonstration interne (VIN `WBAGL41000LC00015O`, concessionnaire `12345`), défini dans `C:\NCSEXPER\CFGDAT\COAPI.INI` section `[Systemdaten]`
- Cette VIN de démo est structurellement invalide → erreur `COAPI-1085`

**Ce n'est pas une panne.** Voir [03 — Dépannage](03_DEPANNAGE.md) pour distinguer ce cas d'un vrai problème de liaison.

En revanche **NCS Dummy fonctionne parfaitement hors voiture** : il lit les DATEN sur disque. C'est le bon moment pour explorer le catalogue des options — voir [02 — NCS Dummy](02_NCS_DUMMY.md).
