# Guide Flash ECU — Siemens MSV70 (BMW N52B30)

> Procédure complète : lecture, modification et reprogrammation du calculateur MSV70.  
> Véhicule cible : BMW E90/E91/E92/E93 330i — N52B30 — SW 9PPL921S

---

## Vue d'ensemble

```
BIN Stock (lecture/dump)
        │
        ▼
  TunerPro RT + XDF  ──► Édition des paramètres
        │
        ▼
  BIN Modifié (checksum corrigé)
        │
        ▼
  Flasher OBD  ──► MSV70 reprogrammé
```

---

## 1. Matériel Nécessaire

| Interface | Protocole | Compatible MSV70 (E9x) | Usage |
|:---|:---|:---|:---|
| **K+DCAN USB** (FTDI authentique) | K-Line + D-CAN (OBD) | ✅ Oui | Lecture/écriture OBD, diagnostics INPA/WinKFP |
| **ENET RJ45** | Ethernet OBD | ⚠️ Limité sur E-series | Plutôt F-series ; possible avec adaptateur sur certains E9x |
| **KTag / KESS V2** | OBD + Bench | ✅ Oui | Flash OBD et bench (JTAG/BDM optionnel) |
| **MPPS V21+** | OBD | ✅ Oui | Lecture/écriture bin via OBD — outil courant |
| **PCM Flash** | OBD | ✅ Oui (module 121) | Solution logicielle professionnelle, protocoles BMW |
| **Tactrix OpenPort 2.0** | OBD | ✅ Oui | Via EcuFlash/Tactrix |

> ⚠️ Évitez les clones K+DCAN avec puce CH340/CH341 — utilisez des versions FTDI FT232RL. Les clones provoquent des déconnexions en milieu de flash.

---

## 2. Lire / Dumper le Bin Stock

**Règle d'or : deux sauvegardes minimum** du bin stock dans des emplacements distincts (disque dur + clé USB + cloud). Un flash raté sans backup = ECU en limp mode ou mort.

**Via WinKFP (outil dealer BMW) :**
```
1. Connectez le K+DCAN USB (port OBD)
2. Contact sur ON, moteur éteint
3. WinKFP → "Expertmodus"
4. Sélectionnez : DME (MSV70) → "FA lesen" puis "SWE lesen"
5. Export bin : Menu → "Steuergerät sichern" → choisir un dossier
6. Fichier résultant : *.bin (2 560 Ko = 2 Mo)
```

**Via MPPS / KESS :**
```
1. Connectez l'interface OBD
2. Sélectionnez : BMW → Siemens MSV70 → Read
3. Sauvegardez le fichier .bin AVANT toute modification
4. Convention de nommage : "VB67774_921S_STOCK_YYYYMMDD.bin"
```

---

## 3. Modifier avec TunerPro RT

**Installation :**
1. TunerPro RT (gratuit) : [tunerpro.net](https://www.tunerpro.net)
2. `File → Open XDF` → `BMW_Siemens_MSV70_9PPL921S_2560K.xdf`
3. `File → Open BIN` → votre fichier `.bin` (une copie, jamais l'original)

**Interface :**
- **Tables** : double-clic sur un paramètre pour ouvrir la cartographie
- **Scale Selection** : pour multiplier/additionner une zone de cellules

**Points de vigilance :**
- Travailler sur une **copie** du bin stock — ne jamais modifier l'original
- Vérifier l'unité affichée (ms/mg, °CRK, lambda) — TunerPro applique l'équation XDF automatiquement
- La valeur saisie est en **unité physique réelle** ; TunerPro reconvertit en raw binaire à la sauvegarde

---

## 4. Checksum — Correction Obligatoire

Le MSV70 vérifie l'intégrité du bin à chaque démarrage via checksum. Un bin modifié sans correction provoque un refus de démarrage ou un mode dégradé.

**TunerPro RT** corrige le checksum automatiquement si le XDF contient la définition checksum (c'est le cas du XDF fourni dans ce projet).

**Vérification manuelle dans TunerPro :**
```
XDF → Checksum → "Verify" puis "Fix"
Si le statut passe de "FAIL" à "OK" → le bin est prêt à flasher
```

**Outils alternatifs :**
- **WinOLS** (commercial) — correction automatique avec plugin MSV70
- MSV70 Checksum Tool (standalone, communauté RomRaider/ECU Editing)

---

## 5. RSA — Signature Cryptographique

| Plateforme | ECU | Protection | Impact tuning |
|:---|:---|:---|:---|
| E46 / E39 (2000–2005) | MSS54 / MS43 | ❌ Aucune | Modification libre après checksum |
| **E90/E91/E92 N52 (2005–2012)** | **MSV70** | **⚠️ Checksum uniquement** | **Pas de RSA — modification possible** |
| E90 N54 (2006–2010) | MSD80 | ✅ RSA partiel | Contournement nécessaire |
| F-series (2012+) | MSD87 / DME8 | ✅ RSA complet | Protection forte — outil dédié requis |

**Bonne nouvelle pour MSV70 :** Pas de signature RSA. La seule protection est le checksum, corrigé automatiquement par TunerPro RT.

> Si vous utilisez un MSV70 de remplacement (occasion), vérifiez que la version logicielle correspond (`9PPL921S`). Certains MSV70 tardifs peuvent avoir une protection différente. Identifiez la version exacte via ISTA avant tout flash.

---

## 6. Flasher le Bin Modifié

**Précautions impératives :**
- **Batterie ≥ 12.5 V** pendant tout le flash — brancher un chargeur de maintien
- **Contact ON, moteur éteint** — ne jamais démarrer pendant un flash
- **Aucun consommateur électrique** allumé (clim, phares, sono)
- **Câble OBD fixe** — ne pas bouger l'interface pendant l'écriture
- En cas d'interruption → re-flasher immédiatement avec le bin stock

**Via WinKFP :**
```
1. Contact sur ON (moteur éteint, batterie ≥ 12.5 V)
2. WinKFP → Expertmodus → DME → "Steuergerät programmieren"
3. Sélectionner le .bin modifié (checksum corrigé)
4. Lancer la programmation — ne pas interrompre (~3–5 min)
5. Fin : "Programmierung erfolgreich" → redémarrer
```

**Via MPPS / KESS :**
```
1. BMW → Siemens MSV70 → Write
2. Charger le .bin modifié
3. Écriture → attendre confirmation
4. Déconnecter, redémarrer
```

---

## 7. Vérification Post-Flash

```
1. Premier démarrage : noter le comportement (STFT, régime, DTC éventuels)
2. ISTA / INPA : vérifier l'absence de codes défauts liés au DME
3. STFT/LTFT : doivent être proches de 0% après 5 min de chauffe
4. Si DTC P0600+ (communication ECU) : re-flasher immédiatement avec bin stock
```

---

## 8. Séquence d'Application Recommandée (E85)

Pour une conversion E85, ne tout modifier pas d'un coup. Procéder étape par étape :

```
Étape 1 — c_fac_mff_ti_stnd (×1.33–1.45, 5 copies)
  → Flasher → Valider STFT ±5%

Étape 2 — ip_mff_cst_opm_1/_2 + c_tco_n_mff_cst
  → Flasher → Valider démarrage froid

Étape 3 — ip_ti_tco_pos_*_wf_opm_* (×1.20)
  → Flasher → Valider transitions tiède

Étape 4 — ip_iga_bas_max_knk__n__maf (+2° à +5°)
  → Flasher → Valider pas de cliquetis
```

Voir [TUTO E85.md](../TUTO%20E85.md) pour le détail complet de chaque étape.
