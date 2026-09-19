# JBBF — Boitier de jonction

🟠 Module `JBBF` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `JBBFE.C05` |
| Fonctions codables | **34** |
| Avec valeurs extraites | 30 (88 %) |
| Avec explication rédigée | 1 |
| Fichiers DATEN analysés | 8 |

Variantes disponibles : `JBBFE.C04`, `JBBFE.C05`, `JBBFE.C06`, `JBBFE.C07`, `JBBFEII.C06`, `JBBFEII.C07`, `JBBFEII.C09`, `JBBFEII.C0A`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 34 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ANZ_EIN_N_BLOCK_WI_HECK` | nombre · marche · N · blocage · **WI** · arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `AUC_SENSOR` | **AUC** · capteur | `nicht_aktiv` · `aktiv` |  |
| `BISTABILES_RELAIS_ALT` | **BISTABILES** · **RELAIS** · **ALT** | `nicht_aktiv` · `aktiv` · `vorne_hinten` · `wert_01` · `zieltermin` · `wert_02` |  |
| `BLOCKERKENNUNG_WISCH_FRON` | **BLOCKERKENNUNG** · essuyage · **FRON** | — |  |
| `BLOCKERKENNUNG_WISCH_HECK` | **BLOCKERKENNUNG** · essuyage · arriere | — |  |
| `CCM_PANIC_MODE` | **CCM** · alarme panique · mode | `nicht_aktiv` · `aktiv` |  |
| `DRUCKSENSOR` | **DRUCKSENSOR** | `nicht_aktiv` · `aktiv` |  |
| `ENTSICHERN_CENTERLOCK` | deverrouillage · **CENTERLOCK** | `nicht_aktiv` · `aktiv` |  |
| `FENSTERHEBER_ELEKTRISCH` | leve-vitre · **ELEKTRISCH** | `nicht_aktiv` · `aktiv` |  |
| `FENSTERHEBER_MIT_EKS` | leve-vitre · avec · systeme electronique de cle | `nicht_aktiv` · `aktiv` |  |
| `FOND_SCHICHT_POT` | places arriere · **SCHICHT** · **POT** | `nicht_aktiv` · `aktiv` |  |
| `HECKSCHEIBE_VERBAUT` | lunette arriere · installe | `nicht_aktiv` · `aktiv` |  |
| `HECKWISCHER_EIN_RUECKWGAN` | **HECKWISCHER** · marche · **RUECKWGAN** | `nicht_aktiv` · `aktiv` |  |
| `HECKWI_STOP_NACH_KLR_EIN` | **HECKWI** · arret · apres · regulation de climatisation · marche | `nicht_aktiv` · `aktiv` |  |
| `INTERVAL_HECKWISCHER` | **INTERVAL** · **HECKWISCHER** | `nicht_aktiv` |  |
| `JBBFE_TUERAUF_STOP_MAUT` | **JBBFE** · porte ouverte · arret · peage | `nicht_aktiv` · `aktiv` |  |
| `KOMPRESSORVENTIL` | **KOMPRESSORVENTIL** | `nicht_aktiv` · `aktiv` |  |
| `KOMPRESSOR_GROSS` | compresseur · **GROSS** | `nicht_aktiv` · `aktiv` |  |
| `KURZ_REVERSIERWEG_EKS` | **KURZ** · **REVERSIERWEG** · systeme electronique de cle | `wert_01` |  |
| `LADERAUMABDK_BEI_OFFN_HKK` | **LADERAUMABDK** · lors de · **OFFN** · **HKK** | `nicht_aktiv` · `aktiv` |  |
| `PANIC_MODE` | alarme panique · mode | `nicht_aktiv` |  |
| `REGENSENSOR` | **REGENSENSOR** | `nicht_aktiv` · `aktiv` |  |
| `SCHEINWERFERREININUNG` | lave-phares | `nicht_aktiv` · `aktiv` | Lave-phares. Le nom comporte une coquille d'origine BMW (REININUNG). |
| `SITZHEIZUNG_LOW` | siege chauffant · bas | `nicht_aktiv` · `aktiv` |  |
| `SONNENROLLO_LADERAUMABDEC` | **SONNENROLLO** · **LADERAUMABDEC** | `nicht_aktiv` · `sonnenrollo` · `laderaumabdeckung` |  |
| `SPERRZ_N_BLOCK_WI_HECK` | **SPERRZ** · N · blocage · **WI** · arriere | — |  |
| `SPRITZDUESEN_SPIEGEL_HEIZ` | **SPRITZDUESEN** · retroviseur · chauffage | `nicht_aktiv` · `spritz` · `spritz_und_spiegel` |  |
| `STANDHEIZUNG` | **STANDHEIZUNG** | `nicht_aktiv` · `aktiv` |  |
| `STANDHEIZUNG_TIMEOUT` | **STANDHEIZUNG** · delai d'attente | `nicht_aktiv` |  |
| `TAKTWASCHEN_HECK` | **TAKTWASCHEN** · arriere | `nicht_aktiv` · `aktiv` |  |
| `WASSER_VENTIL` | eau · valve | `nicht_aktiv` · `aktiv` |  |
| `WISHENINTERVALL_STILL` | **WISHENINTERVALL** · **STILL** | — |  |
| `ZUSATZWASSERPUMPE` | **ZUSATZWASSERPUMPE** | `nicht_verbaut` · `verbaut` |  |
| `ZWP_EGS_STEUERN` | pompe a eau additionnelle · calculateur de boite · **STEUERN** | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
