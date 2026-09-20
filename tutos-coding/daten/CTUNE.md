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
| Signification (EN) | 5 |
| Explication (FR) | 5 |
| Blocs d'adresses du TRC | 1 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 5 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `IBOC_CCC` | radio numerique HD · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | In-band on-channel (IBOC) digital radio (HD radio) | Radio numerique IBOC (HD Radio) |
| `LAENDERVARIANTE_TUNER_ANT` | variante pays · tuner · antenne | `ece` · `us` · `japan` · `oceanien` · `no_area` | Country variant tuner | Variante pays du tuner |
| `RDS_AF_FUNKTION_CCC` | RDS · frequences alternatives · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` | Radio data system (RDS) alternative frequencies (AF) (switch to alternative frequency for the same station when the signal becomes to weak) | Frequences alternatives (AF) du RDS : bascule sur une autre frequence de la meme station quand le signal faiblit |
| `REGIONALISIERUNG` | regionalisation | `reg_aus` · `reg_ein` | Radio data system (RDS) regional (REG) | Mode regional (REG) du RDS |
| `TP_FUNKTION_ANT` | point de test (?) · fonction · antenne | `nicht_aktiv` · `tp` | Radio data system (RDS) traffic programme (TP) | Programme trafic (TP) du RDS |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française ou interprétation explicitement marquée
« Déduction du libellé ». Cette dernière reste à confirmer ; elle ne documente pas
les valeurs ni un effet testé sur le véhicule. Un tiret signifie qu’aucune explication
n’est établie. Voir [la méthode et les sources](TRADUCTIONS.md). Les notations d'unités sont converties : `[durée s = valeur / 10]`.
