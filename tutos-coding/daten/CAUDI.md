# CAUDI — CCC - audio

🟠 Module `CAUDI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_ASK.C05` |
| Fonctions codables | **7** |
| Avec valeurs extraites | 5 (71 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 7 |

Variantes disponibles : `CCC_ASK.C04`, `CCC_ASK.C05`, `CCC_ASK.C06`, `CCC_ASK.C07`, `CCC_ASK.C08`, `CCC_ASK.C09`, `CCC_ASK.C0A`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 7 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `AUDIO_SYSTEM` | audio · **SYSTEM** | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` · `e70_top_hifi` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · **SOURCE** | — |  |
| `INPUT_SCALING_SUB2` | entree · **SCALING** · **SUB2** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `deny_never` · `e87` · `no_aux` · `e6x_hifi` … *(+16)* |  |
| `PHONE_VERSION` | **PHONE** · **VERSION** | — |  |
| `VOICE_PRE_PROC_MIC_DIST` | **VOICE** · pre- · **PROC** · **MIC** · **DIST** | `par_vpp_144mm` · `par_vpp_210mm` |  |
| `VOICE_PRE_PROC_STEER_SIDE` | **VOICE** · pre- · **PROC** · **STEER** · **SIDE** | `par_vpp_left_steering` · `par_vpp_right_steering` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
