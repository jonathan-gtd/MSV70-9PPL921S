# FZD — Module de toit

🟢 Module `FZD` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `FZD.C06` |
| FSW dans cette variante | **41** |
| FSW toutes variantes confondues | 41 |
| Fichiers DATEN analyses | 1 |

Variantes disponibles : `FZD.C06`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 41 fonctions codables

| FSW | Traduction |
|---|---|
| `ANTIWUMMER` | **ANTIWUMMER** |
| `BSS` | **BSS** |
| `DATA_TRANSFER` | **DATA** · **TRANSFER** |
| `E9X_CAR` | **E9X** · vehicule |
| `EC_SPIEGEL` | **EC** · retroviseur |
| `EC_SPIEGEL_KOMPASS_CABRIO` | **EC** · retroviseur · **KOMPASS** · **CABRIO** |
| `FZD_IGR` | module de toit · **IGR** |
| `FZD_KOMFORT_POS_AUF` | module de toit · confort · position · ouverture |
| `FZD_KOMFORT_POS_ZU` | module de toit · confort · position · fermeture |
| `FZD_MSA` | module de toit · Start & Stop |
| `HEBEN_AUS_GEOEFFNET` | **HEBEN** · arret · **GEOEFFNET** |
| `INNENLICHT` | **INNENLICHT** |
| `KLEMME_15_NOTWENDIG` | **KLEMME** · 15 · **NOTWENDIG** |
| `KOMFORT_HEBEN_FZD` | confort · **HEBEN** · module de toit |
| `KOMFORT_OEFFNEN_FZD` | confort · ouverture · module de toit |
| `KOMFORT_SCHLIESSEN_FZD` | confort · fermeture · module de toit |
| `LENGTH_NO_STOP` | **LENGTH** · non · arret |
| `LENGTH_NO_STOP_TILT` | **LENGTH** · non · arret · **TILT** |
| `MDS_CAR` | **MDS** · vehicule |
| `MOTOR_DIRECTION` | moteur · **DIRECTION** |
| `NO_RAIN_INTENSITY` | non · pluie · **INTENSITY** |
| `NO_SHD` | non · toit ouvrant |
| `OEFFNEN_AUS_GEHOBEN` | ouverture · arret · **GEHOBEN** |
| `PANIC_CLOSE` | alarme panique · fermeture |
| `PANIC_CLOSE_TILT` | alarme panique · fermeture · **TILT** |
| `PANIC_FREIGABE_CAS` | alarme panique · **FREIGABE** · **CAS** |
| `PANIC_FREIGABE_KL15` | alarme panique · **FREIGABE** · borne 15 (contact) |
| `POS_DIRECTION` | position · **DIRECTION** |
| `QUARTERTURN` | **QUARTERTURN** |
| `RAIN_CLOSE_INTENSITY` | pluie · fermeture · **INTENSITY** |
| `REGENSCHLIESSEN` | fermeture sous la pluie |
| `REGENSCHLIESSENTILT` | **REGENSCHLIESSENTILT** |
| `RLS` | capteur pluie/lumiere |
| `SCHIEBEHEBEDACH_TIPP_AUF` | toit ouvrant relevable · impulsion · ouverture |
| `SCHIEBEHEBEDACH_TIPP_HEB` | toit ouvrant relevable · impulsion · **HEB** |
| `SCHIEBEHEBEDACH_TIPP_SENK` | toit ouvrant relevable · impulsion · **SENK** |
| `SCHIEBEHEBEDACH_TIPP_ZU` | toit ouvrant relevable · impulsion · fermeture |
| `SHD_POS_QUARTERTURN` | toit ouvrant · position · **QUARTERTURN** |
| `TOLERANCE` | **TOLERANCE** |
| `T_PANIC1` | T · **PANIC1** |
| `T_PANIC2` | T · **PANIC2** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
