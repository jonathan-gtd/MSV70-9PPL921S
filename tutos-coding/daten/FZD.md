# FZD — Module de toit

🟢 Module `FZD` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `FZD.C06` |
| Fonctions codables | **41** |
| Avec valeurs extraites | 41 (100 %) |
| Avec explication rédigée | 2 |
| Fichiers DATEN analysés | 1 |

Variantes disponibles : `FZD.C06`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 41 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ANTIWUMMER` | **ANTIWUMMER** | `aktiv` · `nicht_aktiv` |  |
| `BSS` | **BSS** | `aktiv` · `nicht_aktiv` |  |
| `DATA_TRANSFER` | **DATA** · **TRANSFER** | `aktiv` · `nicht_aktiv` |  |
| `E9X_CAR` | **E9X** · vehicule | `aktiv` · `nicht_aktiv` |  |
| `EC_SPIEGEL` | **EC** · retroviseur | `aktiv` · `nicht_aktiv` |  |
| `EC_SPIEGEL_KOMPASS_CABRIO` | **EC** · retroviseur · **KOMPASS** · **CABRIO** | `aktiv` · `nicht_aktiv` |  |
| `FZD_IGR` | module de toit · **IGR** | `aktiv` · `nicht_aktiv` |  |
| `FZD_KOMFORT_POS_AUF` | module de toit · confort · position · ouverture | `aktiv` · `nicht_aktiv` |  |
| `FZD_KOMFORT_POS_ZU` | module de toit · confort · position · fermeture | `aktiv` · `nicht_aktiv` |  |
| `FZD_MSA` | module de toit · Start & Stop | `aktiv` · `nicht_aktiv` |  |
| `HEBEN_AUS_GEOEFFNET` | **HEBEN** · arret · **GEOEFFNET** | `aktiv` · `nicht_aktiv` |  |
| `INNENLICHT` | **INNENLICHT** | `aktiv` · `nicht_aktiv` |  |
| `KLEMME_15_NOTWENDIG` | **KLEMME** · 15 · **NOTWENDIG** | `aktiv` · `nicht_aktiv` |  |
| `KOMFORT_HEBEN_FZD` | confort · **HEBEN** · module de toit | `wert_01` · `wert_02` · `wert_03` |  |
| `KOMFORT_OEFFNEN_FZD` | confort · ouverture · module de toit | `wert_01` · `wert_02` · `wert_03` |  |
| `KOMFORT_SCHLIESSEN_FZD` | confort · fermeture · module de toit | `wert_01` · `wert_02` · `wert_03` |  |
| `LENGTH_NO_STOP` | **LENGTH** · non · arret | `wert_01` |  |
| `LENGTH_NO_STOP_TILT` | **LENGTH** · non · arret · **TILT** | `wert_01` |  |
| `MDS_CAR` | **MDS** · vehicule | `aktiv` · `nicht_aktiv` |  |
| `MOTOR_DIRECTION` | moteur · **DIRECTION** | `aktiv` · `nicht_aktiv` |  |
| `NO_RAIN_INTENSITY` | non · pluie · **INTENSITY** | `wert_01` |  |
| `NO_SHD` | non · toit ouvrant | `aktiv` · `nicht_aktiv` |  |
| `OEFFNEN_AUS_GEHOBEN` | ouverture · arret · **GEHOBEN** | `aktiv` · `nicht_aktiv` |  |
| `PANIC_CLOSE` | alarme panique · fermeture | `aktiv_2p` · `aktiv_1p` · `nicht_aktiv` |  |
| `PANIC_CLOSE_TILT` | alarme panique · fermeture · **TILT** | `aktiv_2p` · `aktiv_1p` · `nicht_aktiv` |  |
| `PANIC_FREIGABE_CAS` | alarme panique · **FREIGABE** · **CAS** | `aktiv` · `nicht_aktiv` |  |
| `PANIC_FREIGABE_KL15` | alarme panique · **FREIGABE** · borne 15 (contact) | `aktiv` · `nicht_aktiv` |  |
| `POS_DIRECTION` | position · **DIRECTION** | `aktiv` · `nicht_aktiv` |  |
| `QUARTERTURN` | **QUARTERTURN** | `aktiv` · `nicht_aktiv` |  |
| `RAIN_CLOSE_INTENSITY` | pluie · fermeture · **INTENSITY** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `basisparameter` · `nfthub` · `asthub` · `nftschieb` · `astschieb` |  |
| `REGENSCHLIESSEN` | fermeture sous la pluie | `aktiv` · `nicht_aktiv` | Fermeture automatique des vitres en cas de pluie. |
| `REGENSCHLIESSENTILT` | **REGENSCHLIESSENTILT** | `aktiv` · `nicht_aktiv` | Fermeture automatique du toit ouvrant en cas de pluie. |
| `RLS` | capteur pluie/lumiere | `aktiv` · `nicht_aktiv` |  |
| `SCHIEBEHEBEDACH_TIPP_AUF` | toit ouvrant relevable · impulsion · ouverture | `aktiv` · `nicht_aktiv` |  |
| `SCHIEBEHEBEDACH_TIPP_HEB` | toit ouvrant relevable · impulsion · **HEB** | `aktiv` · `nicht_aktiv` |  |
| `SCHIEBEHEBEDACH_TIPP_SENK` | toit ouvrant relevable · impulsion · **SENK** | `aktiv` · `nicht_aktiv` |  |
| `SCHIEBEHEBEDACH_TIPP_ZU` | toit ouvrant relevable · impulsion · fermeture | `aktiv` · `nicht_aktiv` |  |
| `SHD_POS_QUARTERTURN` | toit ouvrant · position · **QUARTERTURN** | `wert_01` · `shdparameter` · `codierung` · `kennlinien` |  |
| `TOLERANCE` | **TOLERANCE** | `wert_01` |  |
| `T_PANIC1` | T · **PANIC1** | `wert_01` |  |
| `T_PANIC2` | T · **PANIC2** | `wert_01` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
