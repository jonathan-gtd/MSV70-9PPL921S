# CAUDI — CCC - audio

🟠 Module `CAUDI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_ASK.C05` |
| FSW dans cette variante | **7** |
| FSW toutes variantes confondues | 13 |
| Fichiers DATEN analyses | 7 |

Variantes disponibles : `CCC_ASK.C04`, `CCC_ASK.C05`, `CCC_ASK.C06`, `CCC_ASK.C07`, `CCC_ASK.C08`, `CCC_ASK.C09`, `CCC_ASK.C0A`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 7 fonctions codables

| FSW | Traduction |
|---|---|
| `AUDIO_SYSTEM` | audio · **SYSTEM** |
| `CODIERUNG_ERFOLGT` | codage · effectue |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · **SOURCE** |
| `INPUT_SCALING_SUB2` | entree · **SCALING** · **SUB2** |
| `PHONE_VERSION` | **PHONE** · **VERSION** |
| `VOICE_PRE_PROC_MIC_DIST` | **VOICE** · pre- · **PROC** · **MIC** · **DIST** |
| `VOICE_PRE_PROC_STEER_SIDE` | **VOICE** · pre- · **PROC** · **STEER** · **SIDE** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
