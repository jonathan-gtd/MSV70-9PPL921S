# 01 — Procédure de codage A→Z

> Marche à suivre complète, menu par menu.  
> Libellés de la version **anglaise** de NCS Expert 4.

---

## Vue d'ensemble

```
        Profil chargé  (File → Load Profile)
                 │
                 ▼
        F2  CHASSIS  →  E89
                 │
                 ▼
        F1  VIN/ZCS/FA  →  F3 ZCS/FA f. ECU     ◄── SAUVEGARDER LE FA
                 │
                 ▼
        F4  Process ECU  →  module
                 │
                 ▼
        F4  Read ECU  →  C:\NCSEXPER\WORK\FSW_PSW.TRC   ◄── SAUVEGARDER
                 │
                 ▼
        NCS Dummy  →  édition  →  FSW_PSW.MAN
                 │
                 ▼
        F2  Change job  →  SG_CODIEREN
                 │
                 ▼
        F3  Execute job  →  « coding ended »
```

🔑 **L'ordre CHASSIS → FA → ECU n'est pas négociable.** Sans châssis sélectionné, NCS Expert n'a aucun référentiel pour valider ce qu'il lit. Sans FA, la liste des modules est vide ou fausse.

---

## Étape 0 — Préparation

Voir [00 — Installation §6](00_INSTALLATION.md). En résumé : chargeur branché, contact position 2, moteur coupé, PC sur secteur, veille désactivée.

Vérifier aussi que `C:\NCSEXPER\WORK\FSW_PSW.MAN` fait bien **0 octet** avant de lancer NCS Expert.

---

## Étape 1 — Charger le profil

Dans la **barre de menus en haut** — pas dans les boutons F1–F6 :

```
File  →  Load Profile  →  « NCS Dummy profile »  →  OK
```

✅ **Contrôle :** le nom du profil apparaît en bas de la fenêtre. S'il n'apparaît pas, rien ne sera écrivable.

---

## Étape 2 — Le châssis, en premier

```
Écran principal  →  F2 CHASSIS  →  E89  →  OK
```

E89 couvre toute la famille E8x/E9x. Il n'y a pas d'entrée « E90 ».

---

## Étape 3 — Lire et sauvegarder le FA

```
F1 VIN/ZCS/FA  →  F3 « ZCS/FA f. ECU »
```

Contenu du sous-menu F1 :

| Bouton | Rôle | Usage |
|---|---|---|
| F1 Enter ZCS | Saisie manuelle du ZCS (ancien système E36/E46) | ❌ inutile sur E9x |
| F2 Enter FA | Saisie manuelle du bon de commande | 🔶 plan B si F3 échoue |
| **F3 ZCS/FA f. ECU** | **Lit le FA dans le CAS de la voiture** | ✅ **c'est celui-là** |
| F5 open file | Recharge un FA sauvegardé sur disque | ✅ pour restaurer plus tard |

Résultat attendu : le type (ex. `VB31`) suivi de la liste des Salapa — `$205`, `$494`, `$8SP`, `+ALx`…

### 🔒 Le FA est le filet de sécurité

> **Tant que le FA n'est pas sauvegardé, aucune écriture.**
>
> C'est le bon de commande usine. Avec lui, n'importe quel module se recode d'origine, même après une erreur. Sans lui, un module mal codé se répare à l'aveugle.

Copier le FA dans `data/Coding/FA_<date>.txt` et le committer **avant de continuer**.

Si `ZCS/FA f. ECU` échoue → voir [03 — Dépannage](03_DEPANNAGE.md).

---

## Étape 4 — Choisir le module

```
F4 Process ECU  →  sélectionner le module
```

Pour une première fois : **KOMBI** (combiné). Aucune fonction de sécurité, aucune gestion d'antidémarrage — une erreur n'immobilise rien.

Choix des modules et niveaux de risque : [04 — Modules](04_MODULES.md).

---

## Étape 5 — Lire le codage actuel

```
F4 Read ECU
```
(job `CODIERDATEN_LESEN`)

Produit `C:\NCSEXPER\WORK\FSW_PSW.TRC`.

⚠️ **Ce fichier est écrasé à chaque lecture.** Le renommer immédiatement :

```
data/Coding/backup/KOMBI_stock_20260918.TRC
```

Convention complète : [data/Coding/README.md](../data/Coding/README.md).

**À ce stade tout est réversible et rien n'a été écrit dans la voiture.** On peut s'arrêter ici, rentrer, et travailler tranquillement dans NCS Dummy.

---

## Étape 6 — Éditer dans NCS Dummy

Voir [02 — NCS Dummy](02_NCS_DUMMY.md).

Sortie attendue : un fichier `FSW_PSW.MAN` déposé dans `C:\NCSEXPER\WORK\`, **ne contenant que les lignes à modifier**.

---

## Étape 7 — Écrire dans la voiture

```
F2 Change job   →  SG_CODIEREN
F3 Execute job
```

Attendre le message **« coding ended »**. Ne rien toucher pendant l'opération : ni contact, ni portes, ni câble.

| Message | Signification |
|---|---|
| `coding ended` | ✅ Écriture réussie |
| `coding error` | ❌ Rien n'a été écrit dans la plupart des cas — relire le module pour vérifier |

### Contrôle après écriture

1. Relancer `F4 Read ECU`
2. Ouvrir le nouveau `.TRC` dans NCS Dummy
3. Vérifier que les valeurs modifiées sont bien celles attendues
4. Couper le contact, attendre ~1 min que les modules s'endorment, remettre le contact
5. Tester la fonction

---

## Étape 8 — Fin de session

```
copy nul "C:\NCSEXPER\WORK\FSW_PSW.MAN"
```

Puis committer dans le repo : le `.TRC` d'origine, le `.MAN` appliqué, le `.TRC` d'après-écriture.

---

## Revenir en arrière

Deux chemins, selon ce qu'on veut retrouver.

### A. Annuler une modification précise

1. Ouvrir le `.TRC` **d'origine** sauvegardé à l'étape 5 dans NCS Dummy
2. Régénérer un `FSW_PSW.MAN` avec les valeurs d'origine
3. `SG_CODIEREN` → `Execute job`

C'est la méthode normale, chirurgicale.

### B. Retour usine complet du module

Recoder le module depuis le bon de commande :

```
File → Load Profile → « Default Profil »
F1 VIN/ZCS/FA → F3 ZCS/FA f. ECU   (ou F5 open file → le FA sauvegardé)
F4 Process ECU → le module
F2 Change job → SG_CODIEREN
F3 Execute job
```

Avec le profil **Default** et un `FSW_PSW.MAN` vide, `SG_CODIEREN` recode le module **intégralement d'après le FA** — c'est-à-dire tel qu'il sortait d'usine.

⚠️ **Cette méthode écrase tous les codages manuels du module**, pas seulement le dernier. C'est le reset complet, à garder pour les situations bloquées.

---

## Rappels critiques

| 🔴 | Règle |
|---|---|
| 1 | **Jamais la DME** — calibration E85 custom, `cal_cks_active = 0`. Un `SG_CODIEREN` dessus peut invalider le tune. |
| 2 | **CAS** = antidémarrage. Une erreur et la voiture ne démarre plus. |
| 3 | **FRM** = le module le plus fragile de l'E9x en sous-tension. |
| 4 | Un seul module à la fois, lu et sauvegardé avant d'être écrit. |
| 5 | Jamais d'écriture sans le FA en sécurité. |
