# 03 — Dépannage

---

## 1. `COAPI-1085 : FGNR fehlerhaft`

*FGNR = Fahrgestellnummer = numéro de châssis.* NCS Expert refuse le numéro de châssis.

### Diagnostic en 3 questions

**1. Es-tu branché à la voiture, contact mis ?**

Si **non** → **c'est normal, il n'y a rien à réparer.** Hors voiture, aucune lecture n'est possible ; NCS Expert retombe sur son jeu de démonstration interne, dont la VIN est invalide par construction. Voir §2.

**2. As-tu fait `F2 CHASSIS → E89` AVANT de lire le FA ?**

Sans châssis sélectionné, NCS Expert n'a aucun référentiel pour valider ce qu'il lit. **C'est la cause n°1.** Refaire dans l'ordre : profil → CHASSIS → FA.

**3. As-tu saisi la VIN à la main ?**

NCS Expert attend la **VIN courte = les 7 derniers caractères**, pas les 17.

### Contournement

Récupérer le FA par un autre chemin, puis le saisir manuellement :

```
1. Tool32 (C:\EDIABAS\Bin\Tool32.exe) → File → Open → CAS.PRG
2. Job FA_LESEN   (ou C_FA_LESEN selon la version du CAS)
3. Execute → noter le type + les Salapa
4. NCS Expert : F1 VIN/ZCS/FA → F2 « Enter FA » → saisie manuelle
```

**Ce test tranche aussi le diagnostic :** si Tool32 ne répond pas non plus, le problème n'est pas NCS Expert mais la liaison OBD → §4.

---

## 2. Reconnaître le jeu de données de démonstration

Quand NCS Expert n'a parlé à personne, il utilise les valeurs codées en dur dans `C:\NCSEXPER\CFGDAT\COAPI.INI`, section `[Systemdaten]` :

```
FAHRGESTELL_NR    WBAGL41000LC00015O     ← 18 caractères, finit par la lettre O
HAENDLER_NR       12345
TEILENUMMER       1753452
```

**Si tu vois ces valeurs, la voiture n'a jamais été interrogée.** Cette VIN est structurellement invalide → `COAPI-1085` garanti.

---

## 3. Lire les traces : la voiture a-t-elle répondu ?

`C:\NCSEXPER\WORK\` contient des fichiers de trace réécrits à chaque session. Ils disent la vérité, contrairement aux messages d'erreur.

| Fichier | Contenu |
|---|---|
| `ABLAUF.TRC` | Journal des appels — **jusqu'où NCS Expert est allé** |
| `SYSDATEN.TRC` | Données système lues (ou celles de démo) |
| `FSW_PSW.TRC` | Codage lu dans le module |

### Le test décisif

Ouvrir `ABLAUF.TRC` :

```
Entry:coapiSetBaureihe     ← une seule ligne = arrêté à « définir la série »
                              AUCUNE communication avec la voiture
```

Si le fichier ne contient que ça, inutile de chercher du côté des DATEN, du profil ou de la VIN : **rien n'a été envoyé sur le bus.** Le problème est en amont — pas de voiture, ou pas de liaison.

---

## 4. Pas de communication avec la voiture

| # | Vérification |
|---|---|
| 1 | Câble branché **avant** le lancement du logiciel |
| 2 | Contact sur position 2 (pas seulement accessoires) |
| 3 | Port réel du FTDI (Gestionnaire de périphériques) = port de `C:\EDIABAS\BIN\OBD.INI` |
| 4 | Port COM entre **1 et 9** — EDIABAS ne va pas au-delà |
| 5 | Latency Timer du FTDI à **1 ms** |
| 6 | `EDIABAS.INI` → `Interface = STD:OBD` |
| 7 | Tension batterie suffisante — chargeur branché |

**Test de référence :** `Tool32 → CAS.PRG → FA_LESEN`. S'il répond, toute la chaîne EDIABAS est bonne et le problème est dans NCS Expert. S'il ne répond pas, c'est le matériel.

> 🔶 Le câble K+DCAN qui sert à flasher la DME en K-line est le même. S'il fonctionne pour le flash, il fonctionne pour le codage — chercher alors du côté du port COM ou du profil, pas du câble.

---

## 5. Aucun profil chargé / boutons grisés

**C'est le comportement normal au démarrage.** NCS Expert ne charge jamais de profil tout seul, à refaire à chaque lancement :

```
File → Load Profile → « NCS Dummy profile »
```

Dans la **barre de menus en haut**, pas dans les boutons F1–F6.

Contrôle : le nom du profil s'affiche **en bas de la fenêtre**.

**Si la liste des profils est vide** → problème de répertoire de démarrage. NCS Expert résout ses chemins en relatif (`..\PFL`, `..\daten`, `..\work`). Lancer directement `C:\NCSEXPER\BIN\NCSEXPER.exe`, en administrateur. Si les profils réapparaissent, corriger le champ « Démarrer dans » du raccourci.

---

## 6. `coding error` à l'écriture

| Cause | Vérification |
|---|---|
| Manipulation du `.MAN` désactivée | Le profil **Default** a `FswPswManipulieren=0` ; pour appliquer un `.MAN`, charger « NCS Dummy profile ». Ce réglage ne signifie pas « lecture seule ». |
| `.MAN` absent ou mal placé | Doit être `C:\NCSEXPER\WORK\FSW_PSW.MAN` |
| `.MAN` destiné à un autre module | Toujours régénérer le `.MAN` depuis le `.TRC` du module en cours |
| Tension insuffisante | Chargeur branché |
| Module endormi | Couper le contact, attendre 1 min, remettre |

Après toute erreur d'écriture : **relire le module** et comparer au `.TRC` d'origine avant de retenter quoi que ce soit.

---

## 7. Module qui ne répond plus après codage

1. Couper le contact, attendre **5 minutes** que tous les modules s'endorment
2. Remettre le contact, retenter une lecture
3. Si ça répond → recoder depuis le `.TRC` d'origine
4. Si ça ne répond toujours pas → **retour usine depuis le FA** ([01 — Procédure](01_PROCEDURE.md), méthode B)

C'est exactement la situation pour laquelle le FA doit être sauvegardé avant toute écriture.

---

## 8. Erreurs à ne pas « réparer »

| Symptôme | Réalité |
|---|---|
| DATEN datées de 2014 | Normal — production E9x terminée en 2013. **Ne pas mettre à jour.** |
| Pas de dossier `E90` dans `DATEN\` | Normal — la série s'appelle **E89** et couvre les E8x/E9x |
| Aucun profil au démarrage | Normal — voir §5 |
| `FSW_PSW.MAN` à 0 octet | Normal et **requis** au lancement |
