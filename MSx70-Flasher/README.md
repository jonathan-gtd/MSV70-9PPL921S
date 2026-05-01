# MSx7x Flasher — MSV70 / MSS70

Outil Windows pour lire et flasher les calculateurs Siemens MSV70 et MSS70 via K-Line OBD.  
Corrige automatiquement les checksums et contourne la vérification de signature.

---

## Prérequis

| | |
|---|---|
| Runtime | .NET Framework 4.5.2 |
| Interface OBD | K+DCAN USB (FTDI FT232RL authentique recommandé) |
| EDIABAS | Installation standard `C:\Ediabas\` **ou** fichier `MSV70.prg` / `MSS70.prg` en standalone |

> ⚠️ Éviter les clones K+DCAN avec puce CH340/CH341 — ils provoquent des déconnexions en milieu de flash, ce qui peut rendre l'ECU non-démarrable.

Pour compiler depuis les sources : `EdiabasLib.dll` est nécessaire (non inclus).

---

## Configuration

Éditer `MSx7x Flasher.exe.config` avant la première utilisation :

| Paramètre | Défaut | Description |
|---|---|---|
| Port | `FTDI0` | Port de l'interface K+DCAN |
| SGBD directory | `C:\Ediabas\ECU` | Dossier contenant les fichiers `.prg` |
| SGBD | `MSV70.prg` | Fichier de définition protocole ECU |

Si EDIABAS n'est pas installé, pointer le paramètre `SGBD directory` vers le dossier contenant `MSV70.prg`.

---

## Utilisation

### Lire le bin (dump stock)

1. Contact sur **ON**, moteur **éteint**
2. Brancher le K+DCAN sur le port OBD
3. Lancer **MSx7x Flasher.exe**
4. Sélectionner l'ECU cible (MSV70 ou MSS70) → **Read**
5. Choisir **Full** (2 560 Ko) ou **Partial** (128 Ko)
6. Sauvegarder le `.bin` — **deux copies minimum** dans des emplacements distincts

> Règle d'or : ne jamais modifier l'original. Travailler uniquement sur des copies.

### Flasher un bin modifié

1. Brancher un **chargeur de maintien** (batterie ≥ 12,5 V obligatoire)
2. Contact sur **ON**, moteur **éteint**
3. Éteindre tous les consommateurs électriques (clim, phares, sono)
4. Dans MSx7x Flasher → **Write** → charger le `.bin` modifié (checksum corrigé)
5. Ne pas toucher le câble OBD pendant l'écriture (~3–5 min)
6. Attendre la confirmation → redémarrer

> En cas d'interruption pendant le flash → re-flasher **immédiatement** avec le bin stock.

---

## Checksum

Le MSV70 vérifie l'intégrité du bin à chaque démarrage. Un bin modifié sans checksum corrigé provoque un refus de démarrage ou un mode dégradé (limp mode).

**MSx7x Flasher corrige le checksum automatiquement** avant d'écrire.

Si vous modifiez le bin avec TunerPro RT, la correction se fait aussi automatiquement via le XDF fourni dans ce projet (`bins/Full/BMW_Siemens_MSV70_9PPL921S_2560K.xdf`) :

```
TunerPro → XDF → Checksum → "Verify" puis "Fix"
Statut "OK" → bin prêt à flasher
```

---

## Protection RSA

| ECU | Protection | Conséquence |
|---|---|---|
| MSV70 (E9x N52) | **Checksum uniquement** | Modification libre après correction checksum |
| MSD80 (E9x N54) | RSA partiel | Contournement nécessaire |
| MSD87 / DME8 (F-series) | RSA complet | Outil dédié requis |

**Le MSV70 n'a pas de signature RSA.** MSx7x Flasher contourne uniquement la vérification de checksum embarquée — aucune clé cryptographique à gérer.

> Si vous utilisez un MSV70 de remplacement (occasion), vérifiez que la version logicielle correspond (`9PPL921S`) via ISTA/D avant tout flash. Certains MSV70 de numéros de série tardifs peuvent avoir un comportement différent.

---

## Full vs Partial

| Mode | Taille | Contenu |
|---|---|---|
| **Full** | 2 560 Ko | ROM complète — à utiliser pour toute modification de calibration |
| **Partial** | 128 Ko | Zone calibration seule — flash plus rapide, mais ne contient pas le code programme |

Pour le tuning E85, toujours travailler en mode **Full**.

---

## Vérification post-flash

```
1. Premier démarrage : observer le comportement (régime, STFT)
2. ISTA/INPA : vérifier l'absence de codes défaut DME (P0600+)
3. STFT/LTFT proches de 0% après 5 min de chauffe → flash OK
4. DTC P0600+ ou refus de démarrage → re-flasher immédiatement avec bin stock
```

---

## Licence

GNU General Public License v3.0 — voir [LICENSE](LICENSE).

**Avertissement :** Cet outil est invasif par nature. Un flash raté peut rendre l'ECU non-démarrable. Aucun auteur ou contributeur ne peut être tenu responsable des dommages résultant de son utilisation.
