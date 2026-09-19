# CAUDI — CCC - audio

🟠 Module `CAUDI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_ASK.C09`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **7** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 5 |
| Signification (EN) | 7 |
| Explication (FR) | 7 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 7 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `AUDIO_SYSTEM` | audio · systeme | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+3)* | Audio system | Systeme audio |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` | Coding completed | Codage termine |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · source | — | Digital audio source | Source audio numerique |
| `INPUT_SCALING_SUB2` | entree · mise a l'echelle · sous-ensemble 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `deny_never` · `e87` · `no_aux` · `e6x_hifi` … *(+32)* | Auxiliary input gain [data<128 ? gain_dB=data : gain_dB=data-256] | Gain de l'entree auxiliaire [valeur < 128 : gain dB = valeur, sinon gain dB = valeur - 256] |
| `PHONE_VERSION` | telephone · version | — | Telephone type | Type de telephone |
| `VOICE_PRE_PROC_MIC_DIST` | voix · pre- · processus · microphone · distance | `par_vpp_144mm` · `par_vpp_210mm` | Microphone distance for voice pre-processing | Distance du microphone pour le pretraitement vocal |
| `VOICE_PRE_PROC_STEER_SIDE` | voix · pre- · processus · direction · lateral | `par_vpp_left_steering` · `par_vpp_right_steering` | Steering type for voice pre-processing | Cote de conduite pour le pretraitement vocal |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
