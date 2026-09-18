# Codage véhicule — BMW E90 330i (N52B30)

> Codage des modules électroniques via **NCS Expert** + **NCS Dummy**.  
> Véhicule : BMW E90/E91/E92/E93 330i — série de données **E89**  
> Interface : K+DCAN USB (FTDI)

---

## ⚠️ Codage ≠ Reprogrammation

Deux mondes distincts, à ne jamais confondre :

| | Reprogrammation (`tutos-reprog/`) | Codage (ce dossier) |
|---|---|---|
| **Quoi** | Cartographie moteur (injection, allumage, lambda) | Options et comportements des modules |
| **Où** | Flash de la DME MSV70 | FSW/PSW dans CAS, FRM, KOMBI, JBBF… |
| **Outils** | TunerPro + XDF + flasher | NCS Expert + NCS Dummy + DATEN |
| **Exemple** | `c_ti_as_global = 1.450` | Rétroviseurs qui se rabattent au verrouillage |

🔴 **La DME ne se code pas.** Ton MSV70 tourne avec une calibration E85 custom et `cal_cks_active = 0`. Un `SG_CODIEREN` sur la DME réécrit sa zone de codage et peut invalider le tune. **Ne sélectionne jamais la DME dans NCS Expert.**

---

## Documents

| Document | Contenu |
|---|---|
| [00 — Installation](00_INSTALLATION.md) | État vérifié du poste, chemins, port COM, ce qu'il ne faut pas toucher |
| [01 — Procédure](01_PROCEDURE.md) | La marche à suivre A→Z, menu par menu, lecture puis écriture |
| [02 — NCS Dummy](02_NCS_DUMMY.md) | Lister toutes les options codables, éditer, générer le `.MAN` |
| [03 — Dépannage](03_DEPANNAGE.md) | COAPI-1085 et les autres erreurs, lecture des traces |
| [04 — Modules](04_MODULES.md) | Modules de l'E90, niveaux de risque, codages courants |
| [Lexique](LEXIQUE_CODING.md) | FSW, PSW, FA, ZCS, Salapa, SG, DATEN… |
| [daten/](daten/README.md) | **Catalogue des DATEN traduit** — 2 282 fonctions codables, module par module |
| [Sauvegardes](../data/Coding/README.md) | Convention de nommage des backups `.TRC` / `.MAN` |

---

## Démarrage rapide

```
1. Chargeur de batterie branché · contact position 2 · moteur coupé
2. NCS Expert → File → Load Profile → « NCS Dummy profile »
3. F2 CHASSIS      → E89
4. F1 VIN/ZCS/FA   → F3 « ZCS/FA f. ECU »   → sauvegarder le FA
5. F4 Process ECU  → module → F4 Read ECU   → FSW_PSW.TRC
6. NCS Dummy       → éditer → FSW_PSW.MAN
7. F2 Change job   → SG_CODIEREN → F3 Execute job
```

Détail complet dans [01 — Procédure](01_PROCEDURE.md).

---

## Les 5 règles

1. **L'ordre est CHASSIS → FA → ECU.** L'inverser produit une erreur de validation VIN.
2. **Le FA d'abord, toujours.** C'est le seul moyen de recoder un module d'origine. Pas de FA sauvegardé = pas d'écriture.
3. **Chargeur obligatoire.** Coder en sous-tension est la cause n°1 de module briqué sur E9x.
4. **Un module à la fois**, lu et sauvegardé avant d'être écrit.
5. **Jamais la DME.** Voir l'encadré ci-dessus.

---

> Le codage modifie des modules de sécurité du véhicule. Aucune écriture sans sauvegarde préalable.
