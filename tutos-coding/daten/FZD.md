# FZD — Module de toit

🟢 Module `FZD` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`FZD.C06`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **41** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 41 |
| Signification (EN) | 31 |
| Explication (FR) | 3 |
| Blocs d'adresses du TRC | 7 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 41 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ANTIWUMMER` | anti-bourdonnement | `aktiv` · `nicht_aktiv` | Comfort position (reduces wind noise) enabled | — |
| `BSS` | capteur (?) | `aktiv` · `nicht_aktiv` | — | — |
| `DATA_TRANSFER` | donnees · transfert | `aktiv` · `nicht_aktiv` | — | — |
| `E9X_CAR` | serie E9x · vehicule | `aktiv` · `nicht_aktiv` | E9x (3 Series) | — |
| `EC_SPIEGEL` | electrochrome (?) · retroviseur | `aktiv` · `nicht_aktiv` | Automatic electrochromatic dimming mirror | Retroviseur photosensible automatique |
| `EC_SPIEGEL_KOMPASS_CABRIO` | electrochrome (?) · retroviseur · boussole · cabriolet | `aktiv` · `nicht_aktiv` | Automatic electrochromatic dimming mirror with compass (convertible) | — |
| `FZD_IGR` | module de toit · regulation (?) | `aktiv` · `nicht_aktiv` | — | — |
| `FZD_KOMFORT_POS_AUF` | module de toit · confort · position · ouverture | `aktiv` · `nicht_aktiv` | Stop at comfort position when opening sunroof (reduces wind noise) | — |
| `FZD_KOMFORT_POS_ZU` | module de toit · confort · position · fermeture | `aktiv` · `nicht_aktiv` | Stop at comfort position when closing sunroof (reduces wind noise) | — |
| `FZD_MSA` | module de toit · Start & Stop | `aktiv` · `nicht_aktiv` | — | — |
| `HEBEN_AUS_GEOEFFNET` | levee · arret · ouvert | `aktiv` · `nicht_aktiv` | Allow raising sunroof from opened position | — |
| `INNENLICHT` | eclairage interieur | `aktiv` · `nicht_aktiv` | Interior lights | — |
| `KLEMME_15_NOTWENDIG` | borne · 15 · necessaire | `aktiv` · `nicht_aktiv` | Sunroof only with ignition on | — |
| `KOMFORT_HEBEN_FZD` | confort · levee · module de toit | `wert_01` · `wert_02` · `wert_03` | Comfort raising sunroof | — |
| `KOMFORT_OEFFNEN_FZD` | confort · ouverture · module de toit | `wert_01` · `wert_02` · `wert_03` | Comfort opening sunroof | — |
| `KOMFORT_SCHLIESSEN_FZD` | confort · fermeture · module de toit | `wert_01` · `wert_02` · `wert_03` | Comfort closing sunroof | — |
| `LENGTH_NO_STOP` | longueur · non · arret | `wert_01` | — | — |
| `LENGTH_NO_STOP_TILT` | longueur · non · arret · entrebaillement | `wert_01` | — | — |
| `MDS_CAR` | systeme de diagnostic (?) · vehicule | `aktiv` · `nicht_aktiv` | — | — |
| `MOTOR_DIRECTION` | moteur · direction | `aktiv` · `nicht_aktiv` | Engine direction | — |
| `NO_RAIN_INTENSITY` | non · pluie · intensite | `wert_01` | Rain intensity lower limit | — |
| `NO_SHD` | non · toit ouvrant | `aktiv` · `nicht_aktiv` | — | — |
| `OEFFNEN_AUS_GEHOBEN` | ouverture · arret · haut de gamme | `aktiv` · `nicht_aktiv` | Allow opening sunroof from raised position | — |
| `PANIC_CLOSE` | alarme panique · fermeture | `aktiv_2p` · `aktiv_1p` · `nicht_aktiv` | Panic close | — |
| `PANIC_CLOSE_TILT` | alarme panique · fermeture · entrebaillement | `aktiv_2p` · `aktiv_1p` · `nicht_aktiv` | Panic close tilted sunroof | — |
| `PANIC_FREIGABE_CAS` | alarme panique · autorisation · systeme d'acces | `aktiv` · `nicht_aktiv` | Panic close only when authenticated by car access system (CAS) | — |
| `PANIC_FREIGABE_KL15` | alarme panique · autorisation · borne 15 (contact) | `aktiv` · `nicht_aktiv` | Panic close only with ignition on | — |
| `POS_DIRECTION` | position · direction | `aktiv` · `nicht_aktiv` | — | — |
| `QUARTERTURN` | quart de tour | `aktiv` · `nicht_aktiv` | — | — |
| `RAIN_CLOSE_INTENSITY` | pluie · fermeture · intensite | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `basisparameter` · `nfthub` · `asthub` · `nftschieb` · `astschieb` | Rain intesity upper limit for automatic closing | — |
| `REGENSCHLIESSEN` | fermeture sous la pluie | `aktiv` · `nicht_aktiv` | Close sunroof with rain sensor (ignition must be switched on and windscreen wipers set to automatic) | Fermeture automatique des vitres en cas de pluie. |
| `REGENSCHLIESSENTILT` | fermeture du toit sous la pluie | `aktiv` · `nicht_aktiv` | Close tilted sunroof with rain sensor (ignition must be switched on and windscreen wipers set to automatic) | Fermeture automatique du toit ouvrant en cas de pluie. |
| `RLS` | capteur pluie/lumiere | `aktiv` · `nicht_aktiv` | Rain and light sensor (RLS) | — |
| `SCHIEBEHEBEDACH_TIPP_AUF` | toit ouvrant relevable · impulsion · ouverture | `aktiv` · `nicht_aktiv` | One-touch opening sunroof | — |
| `SCHIEBEHEBEDACH_TIPP_HEB` | toit ouvrant relevable · impulsion · levee | `aktiv` · `nicht_aktiv` | One-touch raise sunroof | — |
| `SCHIEBEHEBEDACH_TIPP_SENK` | toit ouvrant relevable · impulsion · abaissement | `aktiv` · `nicht_aktiv` | One-touch lower sunroof | — |
| `SCHIEBEHEBEDACH_TIPP_ZU` | toit ouvrant relevable · impulsion · fermeture | `aktiv` · `nicht_aktiv` | One-touch closing sunroof | — |
| `SHD_POS_QUARTERTURN` | toit ouvrant · position · quart de tour | `wert_01` · `shdparameter` · `codierung` · `kennlinien` | Jamming protection move back distance for sliding/tilting sunroof | — |
| `TOLERANCE` | tolerance | `wert_01` | Target position tolerance | — |
| `T_PANIC1` | T · alarme panique 1 | `wert_01` | Timer panic close phase 1 | — |
| `T_PANIC2` | T · alarme panique 2 | `wert_01` | Timer panic close phase 2 | — |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
