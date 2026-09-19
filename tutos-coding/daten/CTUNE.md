# CTUNE — CCC - tuner

🟠 Module `CTUNE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_ANT.C0B`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **5** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 5 |
| Avec explication rédigée | 5 |
| Blocs d'adresses du TRC | 1 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 5 fonctions codables

| FSW | Décomposition | Valeurs | Signification |
|---|---|---|---|
| `IBOC_CCC` | radio numerique HD · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | *In-band on-channel (IBOC) digital radio (HD radio)* |
| `LAENDERVARIANTE_TUNER_ANT` | variante pays · tuner · antenne | `ece` · `us` · `japan` · `oceanien` · `no_area` | *Country variant tuner* |
| `RDS_AF_FUNKTION_CCC` | RDS · frequences alternatives · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` | *Radio valeur system (RDS) alternative frequencies (AF) (switch to alternative frequency for the same station when the signal becomes to weak)* |
| `REGIONALISIERUNG` | regionalisation | `reg_aus` · `reg_ein` | *Radio valeur system (RDS) regional (REG)* |
| `TP_FUNKTION_ANT` | point de test (?) · fonction · antenne | `nicht_aktiv` · `tp` | *Radio valeur system (RDS) traffic programme (TP)* |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification** — texte en clair. En romain : rédigé à la main. *En italique* : description
de la base communautaire NCS Dummy, en anglais, non encore traduite.
Les notations d'unités (`[durée s = valeur / 10]`) sont traduites.
