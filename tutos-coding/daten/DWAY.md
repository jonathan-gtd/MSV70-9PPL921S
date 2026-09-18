# DWAY — Alarme (composant a confirmer)

🟠 Module `DWAY` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `DWA_IRS.C05` |
| FSW dans cette variante | **36** |
| FSW toutes variantes confondues | 46 |
| Fichiers DATEN analyses | 2 |

Variantes disponibles : `DWA_IRS.C05`, `DWA_63.C07`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 36 fonctions codables

| FSW | Traduction |
|---|---|
| `ALARM_OPT_FERNLICHT` | alarme · option · **FERNLICHT** |
| `ALARM_OPT_WARNBLINKER` | alarme · option · **WARNBLINKER** |
| `ANFRAGE_FENSTERPOSITIONEN` | **ANFRAGE** · **FENSTERPOSITIONEN** |
| `CHECKSUM_0` | somme de controle · 0 |
| `CHECKSUM_1` | somme de controle · 1 |
| `CHECKSUM_2` | somme de controle · 2 |
| `CHECKSUM_3` | somme de controle · 3 |
| `CODIERUNG_ERFOLGT` | codage · effectue |
| `CODIERUNG_ERFOLGT_1` | codage · effectue · 1 |
| `DEF_QUIT_AKUST_ENTSCH` | defaut · acquittement sonore · sonore · desarmement |
| `DEF_QUIT_AKUST_SCHAERF` | defaut · acquittement sonore · sonore · armement |
| `DEF_QUIT_AKUST_SCHAERF_KL` | defaut · acquittement sonore · sonore · armement · borne |
| `DEF_QUIT_OPT_ENTSCH` | defaut · acquittement sonore · option · desarmement |
| `DEF_QUIT_OPT_SCHAERF` | defaut · acquittement sonore · option · armement |
| `DEF_QUIT_OPT_SCHAERF_KLAP` | defaut · acquittement sonore · option · armement · **KLAP** |
| `FUNK_INNENRAUMSCHUTZ` | **FUNK** · **INNENRAUMSCHUTZ** |
| `HOTELSTELLUNG` | **HOTELSTELLUNG** |
| `LADEEINRICHTUNG` | **LADEEINRICHTUNG** |
| `LAUTSTAERKE` | **LAUTSTAERKE** |
| `MOTORFERNSTART` | **MOTORFERNSTART** |
| `NEIGUNGSGEBER` | **NEIGUNGSGEBER** |
| `NOTLAUFFUNKTION_HUPE` | **NOTLAUFFUNKTION** · **HUPE** |
| `PANIK_ALARM` | **PANIK** · alarme |
| `PIAU_AKUST_ENTSCH` | _PIAU_ · sonore · desarmement |
| `PIAU_AKUST_SCHAERF_KLAPPE` | _PIAU_ · sonore · armement · trappe |
| `QUIT_AKUST_ENTSCH` | acquittement sonore · sonore · desarmement |
| `QUIT_AKUST_SCHAERF` | acquittement sonore · sonore · armement |
| `QUIT_AKUST_SCHAERF_KLAPPE` | acquittement sonore · sonore · armement · trappe |
| `QUIT_OPT_ENTSCH` | acquittement sonore · option · desarmement |
| `QUIT_OPT_SCHAERF` | acquittement sonore · option · armement |
| `QUIT_OPT_SCHAERF_KLAPPE` | acquittement sonore · option · armement · trappe |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable |
| `SIGNAL_HK` | **SIGNAL** · hayon |
| `TIMEOUT_DEAKT_IRNE` | delai d'attente · **DEAKT** · **IRNE** |
| `TRIGGERANZAHL_KLAPPEN` | **TRIGGERANZAHL** · **KLAPPEN** |
| `TRIGGERANZAHL_SENSOREN` | **TRIGGERANZAHL** · **SENSOREN** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
