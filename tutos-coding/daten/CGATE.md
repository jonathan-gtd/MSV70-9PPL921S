# CGATE — CCC - passerelle

🟠 Module `CGATE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_GW.C02`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **7** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 7 |
| Signification (EN) | 6 |
| Explication (FR) | 6 |
| Blocs d'adresses du TRC | 4 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 7 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `AUDIO_SYSTEM` | audio · systeme | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+37)* | Audio system | Systeme audio |
| `PDC` | radars | `nicht_aktiv` · `aktiv` | Park distance control (PDC) | Radars de stationnement (PDC) |
| `PDC_SIGNAL` | radars · signal | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` | Park distance control (PDC) acoustic signal generation | Generation du signal sonore des radars de stationnement |
| `PDC_TYPE` | radars · type | `rear` · `full` | Park distance control (PDC) type | Type de radars de stationnement (PDC) |
| `REVERSE_GONG` | inversion · gong | `nicht_aktiv` · `aktiv` | Reverse gear warning | Alerte de marche arriere engagee |
| `RING_CCC` | anneau · systeme multimedia CCC | `aktiv` · `nicht_aktiv` | — | — |
| `SPEED_MODE` | vitesse · mode | `speed_mode_intern` · `speed_mode_extern` | Speed signal source | Source du signal de vitesse |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française ou interprétation explicitement marquée
« Déduction du libellé ». Cette dernière reste à confirmer ; elle ne documente pas
les valeurs ni un effet testé sur le véhicule. Un tiret signifie qu’aucune explication
n’est établie. Voir [la méthode et les sources](TRADUCTIONS.md). Les notations d'unités sont converties : `[durée s = valeur / 10]`.
