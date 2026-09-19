# DWAY — Alarme - protection volumetrique

🟠 Module `DWAY` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`DWA_IRS.C05`** |
| Identifiée par | recoupement avec `DWAY.TRC` lu sur le véhicule |
| Fonctions retenues | **36** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 36 |
| Avec explication rédigée | 2 |
| Blocs d'adresses du TRC | 8 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 36 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ALARM_OPT_FERNLICHT` | alarme · option · **FERNLICHT** | `nicht_aktiv` · `aktiv` |  |
| `ALARM_OPT_WARNBLINKER` | alarme · option · feux de detresse | `nicht_aktiv` · `aktiv` |  |
| `ANFRAGE_FENSTERPOSITIONEN` | **ANFRAGE** · **FENSTERPOSITIONEN** | `nicht_aktiv` · `aktiv` |  |
| `CHECKSUM_0` | somme de controle · 0 | `grundcode_var` · `wert_01` · `ewp` · `wert_02` · `wert_03` · `wert_04` · `e92_m` · `instanceid_0x12` … *(+12)* |  |
| `CHECKSUM_1` | somme de controle · 1 | `grundcode_var` |  |
| `CHECKSUM_2` | somme de controle · 2 | `grundcode_var` |  |
| `CHECKSUM_3` | somme de controle · 3 | `grundcode_var` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `codierung_erfolgt` |  |
| `CODIERUNG_ERFOLGT_1` | codage · effectue · 1 | `codierung_erfolgt` |  |
| `DEF_QUIT_AKUST_ENTSCH` | defaut · acquittement sonore · sonore · desarmement | `nicht_aktiv` · `aktiv` |  |
| `DEF_QUIT_AKUST_SCHAERF` | defaut · acquittement sonore · sonore · armement | `nicht_aktiv` · `aktiv` |  |
| `DEF_QUIT_AKUST_SCHAERF_KL` | defaut · acquittement sonore · sonore · armement · borne | `nicht_aktiv` · `aktiv` |  |
| `DEF_QUIT_OPT_ENTSCH` | defaut · acquittement sonore · option · desarmement | `nicht_aktiv` · `aktiv` |  |
| `DEF_QUIT_OPT_SCHAERF` | defaut · acquittement sonore · option · armement | `nicht_aktiv` · `aktiv` |  |
| `DEF_QUIT_OPT_SCHAERF_KLAP` | defaut · acquittement sonore · option · armement · **KLAP** | `nicht_aktiv` · `aktiv` |  |
| `FUNK_INNENRAUMSCHUTZ` | **FUNK** · **INNENRAUMSCHUTZ** | `aktiv` · `nicht_aktiv` |  |
| `HOTELSTELLUNG` | **HOTELSTELLUNG** | `nicht_aktiv` · `aktiv` |  |
| `LADEEINRICHTUNG` | **LADEEINRICHTUNG** | `nicht_aktiv` · `aktiv` |  |
| `LAUTSTAERKE` | volume sonore | `nicht_aktiv` · `aktiv` |  |
| `MOTORFERNSTART` | demarrage moteur a distance | `nicht_aktiv` · `aktiv` |  |
| `NEIGUNGSGEBER` | **NEIGUNGSGEBER** | `aktiv` · `nicht_aktiv` |  |
| `NOTLAUFFUNKTION_HUPE` | **NOTLAUFFUNKTION** · **HUPE** | `nicht_aktiv` · `aktiv` |  |
| `PANIK_ALARM` | mode panique · alarme | `nicht_aktiv` · `aktiv` |  |
| `PIAU_AKUST_ENTSCH` | _PIAU_ · sonore · desarmement | `nicht_aktiv` · `aktiv` |  |
| `PIAU_AKUST_SCHAERF_KLAPPE` | _PIAU_ · sonore · armement · trappe | `nicht_aktiv` · `aktiv` |  |
| `QUIT_AKUST_ENTSCH` | acquittement sonore · sonore · desarmement | `nicht_aktiv` · `aktiv` | Bip sonore au desarmement de l'alarme. |
| `QUIT_AKUST_SCHAERF` | acquittement sonore · sonore · armement | `nicht_aktiv` · `aktiv` | Bip sonore a l'armement de l'alarme. |
| `QUIT_AKUST_SCHAERF_KLAPPE` | acquittement sonore · sonore · armement · trappe | `nicht_aktiv` · `aktiv` |  |
| `QUIT_OPT_ENTSCH` | acquittement sonore · option · desarmement | `nicht_aktiv` · `aktiv` |  |
| `QUIT_OPT_SCHAERF` | acquittement sonore · option · armement | `nicht_aktiv` · `aktiv` |  |
| `QUIT_OPT_SCHAERF_KLAPPE` | acquittement sonore · option · armement · trappe | `nicht_aktiv` · `aktiv` |  |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable | `nicht_aktiv` · `aktiv` |  |
| `SIGNAL_HK` | signal · hayon | `nicht_aktiv` · `aktiv` |  |
| `TIMEOUT_DEAKT_IRNE` | delai d'attente · **DEAKT** · **IRNE** | `wert_01` · `wert_02` · `wert_03` · `wert_05` · `wert_10` · `wert_15` |  |
| `TRIGGERANZAHL_KLAPPEN` | nombre de declenchements · **KLAPPEN** | `wert_01` · `wert_02` |  |
| `TRIGGERANZAHL_SENSOREN` | nombre de declenchements · **SENSOREN** | `wert_01` · `wert_02` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
