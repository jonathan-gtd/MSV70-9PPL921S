# JBBF — Boitier de jonction

🟠 Module `JBBF` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `JBBFE.C05` |
| FSW dans cette variante | **34** |
| FSW toutes variantes confondues | 241 |
| Fichiers DATEN analyses | 8 |

Variantes disponibles : `JBBFE.C04`, `JBBFE.C05`, `JBBFE.C06`, `JBBFE.C07`, `JBBFEII.C06`, `JBBFEII.C07`, `JBBFEII.C09`, `JBBFEII.C0A`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 34 fonctions codables

| FSW | Traduction |
|---|---|
| `ANZ_EIN_N_BLOCK_WI_HECK` | nombre · marche · N · blocage · **WI** · arriere |
| `AUC_SENSOR` | **AUC** · capteur |
| `BISTABILES_RELAIS_ALT` | **BISTABILES** · **RELAIS** · **ALT** |
| `BLOCKERKENNUNG_WISCH_FRON` | **BLOCKERKENNUNG** · essuyage · **FRON** |
| `BLOCKERKENNUNG_WISCH_HECK` | **BLOCKERKENNUNG** · essuyage · arriere |
| `CCM_PANIC_MODE` | **CCM** · alarme panique · mode |
| `DRUCKSENSOR` | **DRUCKSENSOR** |
| `ENTSICHERN_CENTERLOCK` | deverrouillage · **CENTERLOCK** |
| `FENSTERHEBER_ELEKTRISCH` | leve-vitre · **ELEKTRISCH** |
| `FENSTERHEBER_MIT_EKS` | leve-vitre · avec · systeme electronique de cle |
| `FOND_SCHICHT_POT` | places arriere · **SCHICHT** · **POT** |
| `HECKSCHEIBE_VERBAUT` | lunette arriere · installe |
| `HECKWISCHER_EIN_RUECKWGAN` | **HECKWISCHER** · marche · **RUECKWGAN** |
| `HECKWI_STOP_NACH_KLR_EIN` | **HECKWI** · arret · apres · regulation de climatisation · marche |
| `INTERVAL_HECKWISCHER` | **INTERVAL** · **HECKWISCHER** |
| `JBBFE_TUERAUF_STOP_MAUT` | **JBBFE** · porte ouverte · arret · peage |
| `KOMPRESSORVENTIL` | **KOMPRESSORVENTIL** |
| `KOMPRESSOR_GROSS` | compresseur · **GROSS** |
| `KURZ_REVERSIERWEG_EKS` | **KURZ** · **REVERSIERWEG** · systeme electronique de cle |
| `LADERAUMABDK_BEI_OFFN_HKK` | **LADERAUMABDK** · lors de · **OFFN** · **HKK** |
| `PANIC_MODE` | alarme panique · mode |
| `REGENSENSOR` | **REGENSENSOR** |
| `SCHEINWERFERREININUNG` | lave-phares |
| `SITZHEIZUNG_LOW` | siege chauffant · bas |
| `SONNENROLLO_LADERAUMABDEC` | **SONNENROLLO** · **LADERAUMABDEC** |
| `SPERRZ_N_BLOCK_WI_HECK` | **SPERRZ** · N · blocage · **WI** · arriere |
| `SPRITZDUESEN_SPIEGEL_HEIZ` | **SPRITZDUESEN** · retroviseur · chauffage |
| `STANDHEIZUNG` | **STANDHEIZUNG** |
| `STANDHEIZUNG_TIMEOUT` | **STANDHEIZUNG** · delai d'attente |
| `TAKTWASCHEN_HECK` | **TAKTWASCHEN** · arriere |
| `WASSER_VENTIL` | eau · valve |
| `WISHENINTERVALL_STILL` | **WISHENINTERVALL** · **STILL** |
| `ZUSATZWASSERPUMPE` | **ZUSATZWASSERPUMPE** |
| `ZWP_EGS_STEUERN` | pompe a eau additionnelle · calculateur de boite · **STEUERN** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
