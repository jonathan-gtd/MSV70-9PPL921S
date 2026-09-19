# CAUDI — CCC - audio

🟠 Module `CAUDI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_ASK.C04`** |
| Identifiée par | recoupement avec `CAUDI.TRC` lu sur le véhicule |
| Fonctions retenues | **13** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 11 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 13 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `AUDIO_SYSTEM` | audio · **SYSTEM** | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · source | — |  |
| `INPUT_SCALING_SUB2` | entree · **SCALING** · **SUB2** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `deny_never` · `e61` · `no_aux` · `e64` … *(+17)* |  |
| `OUTPUT_GAIN_FRONTLEFT` | sortie · gain · **FRONTLEFT** | `wert_01` · `e61_stereo` · `e61_hifi` · `e64_hifi` · `e87_stereo` · `e87_hifi` |  |
| `OUTPUT_GAIN_FRONTRIGHT` | sortie · gain · **FRONTRIGHT** | `wert_01` · `e61_stereo` · `e61_hifi` · `e64_hifi` · `e87_stereo` · `e87_hifi` |  |
| `OUTPUT_GAIN_REARLEFT` | sortie · gain · arriere gauche | `wert_01` · `e61_stereo` · `e61_hifi` · `e64_hifi` · `e87_stereo` · `e87_hifi` |  |
| `OUTPUT_GAIN_REARRIGHT` | sortie · gain · arriere droit | `wert_01` · `e61_stereo` · `e61_hifi` · `e64_hifi` · `e87_stereo` · `e87_hifi` |  |
| `OUTPUT_GAIN_SUBWOOFERL` | sortie · gain · caisson de basses | `wert_01` · `e61_stereo` · `e61_hifi` · `e64_hifi` · `e87_stereo` |  |
| `OUTPUT_GAIN_SUBWOOFERR` | sortie · gain · **SUBWOOFERR** | `wert_01` · `e61_stereo` · `e61_hifi` · `e64_hifi` · `e87_stereo` |  |
| `PHONE_VERSION` | telephone · version | — |  |
| `VOICE_PRE_PROC_MIC_DIST` | voix · pre- · **PROC** · **MIC** · **DIST** | `par_vpp_144mm` · `par_vpp_210mm` |  |
| `VOICE_PRE_PROC_STEER_SIDE` | voix · pre- · **PROC** · direction · lateral | `par_vpp_left_steering` · `par_vpp_right_steering` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
