# PDC — Radars de recul

🟢 Module `PDC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `PDC_E87.C03` |
| FSW dans cette variante | **45** |
| FSW toutes variantes confondues | 48 |
| Fichiers DATEN analyses | 3 |

Variantes disponibles : `PDC_E87.C03`, `PDC_E87.C04`, `PDC_E87.C05`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 45 fonctions codables

| FSW | Traduction |
|---|---|
| `ABSCHALTWEG` | **ABSCHALTWEG** |
| `ABSCHALT_GESCHWINDIGKEIT` | coupure · **GESCHWINDIGKEIT** |
| `ABSTAND_ECKE_MIT_VO` | distance · coin · avec · **VO** |
| `ABSTAND_ECK_MIT_HI` | distance · **ECK** · avec · haut |
| `ABSTAND_MIT_MIT_HI` | distance · avec · avec · haut |
| `ABSTAND_MIT_MIT_VO` | distance · avec · avec · **VO** |
| `AHK_OFFSET_FRONT` | **AHK** · decalage · avant |
| `AHK_OFFSET_HINTEN` | **AHK** · decalage · arriere |
| `AKUSTIK_RECHTS_LINKS` | **AKUSTIK** · droite · gauche |
| `AKUSTISCHE_FEHLERMELDUNG` | **AKUSTISCHE** · message d'erreur |
| `ANHAENGERKUPPLUNG` | **ANHAENGERKUPPLUNG** |
| `ANZAHL_WANDLER` | nombre · **WANDLER** |
| `DAUERTON` | **DAUERTON** |
| `DAUERTON_BEREICH_HINTEN` | **DAUERTON** · plage · arriere |
| `DAUERTON_BEREICH_VORNE` | **DAUERTON** · plage · avant |
| `EINSCHALTMELDUNG_IMMER` | **EINSCHALTMELDUNG** · toujours |
| `EINSCHALTVEROEGERUNG_ZEIT` | **EINSCHALTVEROEGERUNG** · duree |
| `EINSCHALTVERZOEGERUNG` | **EINSCHALTVERZOEGERUNG** |
| `FAHRZEUG_STILLSTAND` | **FAHRZEUG** · **STILLSTAND** |
| `LAUTSTAERKE_HINTEN` | **LAUTSTAERKE** · arriere |
| `LAUTSTAERKE_VORN` | **LAUTSTAERKE** · **VORN** |
| `MESSBEREICHE_ECKE_VORNE` | **MESSBEREICHE** · coin · avant |
| `MESSBEREICHE_MITTE_HINTEN` | **MESSBEREICHE** · **MITTE** · arriere |
| `MESSBEREICHE_MITTE_VORNE` | **MESSBEREICHE** · **MITTE** · avant |
| `MESSBEREICH_ECKE_HINTEN` | **MESSBEREICH** · coin · arriere |
| `OFFSET_ECKE_HINTEN` | decalage · coin · arriere |
| `OFFSET_ECKE_VORNE` | decalage · coin · avant |
| `OFFSET_MITTE_VORNE` | decalage · **MITTE** · avant |
| `OFFSET_MIT_HINTEN` | decalage · avec · arriere |
| `PDC_DEAKTIVIERUNG` | radars · **DEAKTIVIERUNG** |
| `PDC_SCHALTER` | radars · **SCHALTER** |
| `SC_FLAG_DT_FRONT` | _SC_ · indicateur · **DT** · avant |
| `SC_FLAG_DT_REAR` | _SC_ · indicateur · **DT** · arriere |
| `SC_FLAG_LP_FRONT` | _SC_ · indicateur · **LP** · avant |
| `SC_FLAG_LP_REAR` | _SC_ · indicateur · **LP** · arriere |
| `SC_FLAG_OE_FRONT` | _SC_ · indicateur · **OE** · avant |
| `SC_FLAG_OE_REAR` | _SC_ · indicateur · **OE** · arriere |
| `SC_FLAG_WD_FRONT` | _SC_ · indicateur · **WD** · avant |
| `SC_FLAG_WD_REAR` | _SC_ · indicateur · **WD** · arriere |
| `STILLSTAND_ZEIT` | **STILLSTAND** · duree |
| `TONAUSGABE_CAN` | **TONAUSGABE** · bus CAN |
| `TONAUSGABE_LSP` | **TONAUSGABE** · **LSP** |
| `TON_ANZEIGEDAUER` | **TON** · **ANZEIGEDAUER** |
| `WANDERKENNUNG` | **WANDERKENNUNG** |
| `WANDLER_STOERUNG` | **WANDLER** · **STOERUNG** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
