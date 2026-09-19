# PDC — Radars de recul

🟢 Module `PDC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `PDC_E87.C03` |
| Fonctions codables | **45** |
| Avec valeurs extraites | 44 (97 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 3 |

Variantes disponibles : `PDC_E87.C03`, `PDC_E87.C04`, `PDC_E87.C05`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 45 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ABSCHALTWEG` | **ABSCHALTWEG** | `e87` · `e90` · `e91` · `e90_m` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `ABSCHALT_GESCHWINDIGKEIT` | coupure · **GESCHWINDIGKEIT** | `e87` · `e90` · `e90_m` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `ABSTAND_ECKE_MIT_VO` | distance · coin · avec · **VO** | `e84` · `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `ABSTAND_ECK_MIT_HI` | distance · **ECK** · avec · haut | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `ABSTAND_MIT_MIT_HI` | distance · avec · avec · haut | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `ABSTAND_MIT_MIT_VO` | distance · avec · avec · **VO** | `e84` · `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `AHK_OFFSET_FRONT` | **AHK** · decalage · avant | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `AHK_OFFSET_HINTEN` | **AHK** · decalage · arriere | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `AKUSTIK_RECHTS_LINKS` | **AKUSTIK** · droite · gauche | `nicht_aktiv` · `aktiv` |  |
| `AKUSTISCHE_FEHLERMELDUNG` | **AKUSTISCHE** · message d'erreur | `nicht_aktiv` · `aktiv` |  |
| `ANHAENGERKUPPLUNG` | **ANHAENGERKUPPLUNG** | `nicht_aktiv` · `aktiv` |  |
| `ANZAHL_WANDLER` | nombre · **WANDLER** | — |  |
| `DAUERTON` | **DAUERTON** | `nicht_aktiv` · `aktiv` |  |
| `DAUERTON_BEREICH_HINTEN` | **DAUERTON** · plage · arriere | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `DAUERTON_BEREICH_VORNE` | **DAUERTON** · plage · avant | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `EINSCHALTMELDUNG_IMMER` | **EINSCHALTMELDUNG** · toujours | `nicht_aktiv` · `aktiv` |  |
| `EINSCHALTVEROEGERUNG_ZEIT` | **EINSCHALTVEROEGERUNG** · duree | `e87` · `e90` · `e91` · `e90_m` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `EINSCHALTVERZOEGERUNG` | **EINSCHALTVERZOEGERUNG** | `nicht_aktiv` · `aktiv` |  |
| `FAHRZEUG_STILLSTAND` | **FAHRZEUG** · **STILLSTAND** | `nicht_aktiv` · `aktiv` |  |
| `LAUTSTAERKE_HINTEN` | **LAUTSTAERKE** · arriere | `e87` · `e90` · `e84` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+15)* |  |
| `LAUTSTAERKE_VORN` | **LAUTSTAERKE** · **VORN** | `e87` · `e90` · `e84` · `e90_m` · `e91` · `e92` · `e93` · `e81` … *(+13)* |  |
| `MESSBEREICHE_ECKE_VORNE` | **MESSBEREICHE** · coin · avant | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `MESSBEREICHE_MITTE_HINTEN` | **MESSBEREICHE** · **MITTE** · arriere | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `MESSBEREICHE_MITTE_VORNE` | **MESSBEREICHE** · **MITTE** · avant | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `MESSBEREICH_ECKE_HINTEN` | **MESSBEREICH** · coin · arriere | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `OFFSET_ECKE_HINTEN` | decalage · coin · arriere | `e84` · `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `OFFSET_ECKE_VORNE` | decalage · coin · avant | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `OFFSET_MITTE_VORNE` | decalage · **MITTE** · avant | `e84` · `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `OFFSET_MIT_HINTEN` | decalage · avec · arriere | `e84` · `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `PDC_DEAKTIVIERUNG` | radars · **DEAKTIVIERUNG** | `nicht_aktiv` · `aktiv` |  |
| `PDC_SCHALTER` | radars · **SCHALTER** | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_DT_FRONT` | _SC_ · indicateur · **DT** · avant | `nicht_aktiv` · `aktiv` · `e87` · `e84` · `e90` · `e91` · `e92` · `e93` … *(+19)* |  |
| `SC_FLAG_DT_REAR` | _SC_ · indicateur · **DT** · arriere | `nicht_aktiv` · `aktiv` · `e87` · `e90` · `e91` · `e92` · `e93` · `e81` … *(+15)* |  |
| `SC_FLAG_LP_FRONT` | _SC_ · indicateur · **LP** · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_LP_REAR` | _SC_ · indicateur · **LP** · arriere | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_OE_FRONT` | _SC_ · indicateur · **OE** · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_OE_REAR` | _SC_ · indicateur · **OE** · arriere | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_WD_FRONT` | _SC_ · indicateur · **WD** · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_WD_REAR` | _SC_ · indicateur · **WD** · arriere | `nicht_aktiv` · `aktiv` |  |
| `STILLSTAND_ZEIT` | **STILLSTAND** · duree | `e87` · `e90` · `e91` · `e92` · `e93` · `e81` · `e82` · `e84` … *(+13)* |  |
| `TONAUSGABE_CAN` | **TONAUSGABE** · bus CAN | `nicht_aktiv` · `aktiv` |  |
| `TONAUSGABE_LSP` | **TONAUSGABE** · **LSP** | `nicht_aktiv` · `aktiv` |  |
| `TON_ANZEIGEDAUER` | **TON** · **ANZEIGEDAUER** | `e87` · `e90` · `e84` · `e91` · `e92` · `e93` · `e81` · `e82` … *(+13)* |  |
| `WANDERKENNUNG` | **WANDERKENNUNG** | `nicht_aktiv` · `aktiv` |  |
| `WANDLER_STOERUNG` | **WANDLER** · **STOERUNG** | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
