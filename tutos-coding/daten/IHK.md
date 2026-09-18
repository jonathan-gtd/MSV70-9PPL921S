# IHK — Climatisation

🟢 Module `IHK` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `IHXXPL2.C02` |
| FSW dans cette variante | **74** |
| FSW toutes variantes confondues | 85 |
| Fichiers DATEN analyses | 3 |

Variantes disponibles : `IHXXPL2.C01`, `IHXXPL2.C02`, `IHKAE82E.C10`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 74 fonctions codables

| FSW | Traduction |
|---|---|
| `ABK_KLIMASTIL` | **ABK** · **KLIMASTIL** |
| `ALKS` | **ALKS** |
| `ANZEIGE_TIMER_STHZ_SA` | affichage · minuterie · **STHZ** · **SA** |
| `AUTO_BEI_KL15_EIN_0` | automatique · lors de · borne 15 (contact) · marche · 0 |
| `AUTO_BEI_KL15_EIN_1` | automatique · lors de · borne 15 (contact) · marche · 1 |
| `AUTO_BEI_KL15_EIN_2` | automatique · lors de · borne 15 (contact) · marche · 2 |
| `BAUKASTEN_PTC` | **BAUKASTEN** · **PTC** |
| `BAUREIHE` | **BAUREIHE** |
| `CC_MELDUNG_SH` | Check Control · message · **SH** |
| `C_CKM_D_AUTO_BEI_KL15_EIN` | C · memoire par cle (Car Key Memory) · D · automatique · lors de · borne 15 (contact) · marche |
| `C_CKM_D_GEBLAESE_OFFSET` | C · memoire par cle (Car Key Memory) · D · pulseur · decalage |
| `C_CKM_D_KLIMA_MIT_AUTO` | C · memoire par cle (Car Key Memory) · D · climatisation · avec · automatique |
| `C_CKM_D_TEMPERATUR_OFFSET` | C · memoire par cle (Car Key Memory) · D · temperature · decalage |
| `DEFROST_LEITSCHAUFELN` | **DEFROST** · **LEITSCHAUFELN** |
| `DELAY_SH_LED` | delai · **SH** · LED |
| `ECO_PRO_DEFAULT_START` | economie · **PRO** · **DEFAULT** · demarrage |
| `ECO_PRO_LAST_FU` | economie · **PRO** · charge · **FU** |
| `ECO_PRO_MODE` | economie · **PRO** · mode |
| `EINBAUORT` | **EINBAUORT** |
| `EL_ZUHEIZER_PTC` | **EL** · **ZUHEIZER** · **PTC** |
| `ERSTE_HEIZZEIT_HHS_A` | premier · duree de chauffage · _HHS_ · A |
| `ERSTE_HEIZZEIT_HHS_B` | premier · duree de chauffage · _HHS_ · B |
| `F25_BESCHL_VERM` | **F25** · **BESCHL** · **VERM** |
| `GBL_OFF` | **GBL** · arret |
| `GEBLAESE_AUTO_MIT_A` | pulseur · automatique · avec · A |
| `GEBLAESE_AUTO_MIT_BALKEN` | pulseur · automatique · avec · **BALKEN** |
| `GEBLAESE_MIT_LUEFTER_SYMB` | pulseur · avec · ventilateur · **SYMB** |
| `GEBLAESE_MIT_RAHMEN` | pulseur · avec · **RAHMEN** |
| `GEBLAESE_OFFSET_0` | pulseur · decalage · 0 |
| `GEBLAESE_OFFSET_1` | pulseur · decalage · 1 |
| `GEBLAESE_OFFSET_2` | pulseur · decalage · 2 |
| `HDC_ZYKLISCH` | assistance en descente · **ZYKLISCH** |
| `IGR` | **IGR** |
| `IHX_CBS` | **IHX** · entretien selon besoin (CBS) |
| `KLIMAGERAET_MIT_SCHICHTNG` | **KLIMAGERAET** · avec · **SCHICHTNG** |
| `KLIMA_MIT_AUTO_0` | climatisation · avec · automatique · 0 |
| `KLIMA_MIT_AUTO_1` | climatisation · avec · automatique · 1 |
| `KLIMA_MIT_AUTO_2` | climatisation · avec · automatique · 2 |
| `KLIMA_SPERREN` | climatisation · **SPERREN** |
| `KMK_ZUSCHALTSCHWELLE` | **KMK** · **ZUSCHALTSCHWELLE** |
| `KOMPRESSOR` | compresseur |
| `KOMPRESSOR_KUPPLUNG` | compresseur · embrayage |
| `KOMPRESSOR_MITTEL` | compresseur · moyen |
| `KOMP_AKUSTIK` | compensation · **AKUSTIK** |
| `KOREA_SYMBOL` | **KOREA** · **SYMBOL** |
| `LAENDERVARANTE` | **LAENDERVARANTE** |
| `MANUELLES_VERDECK_E88` | **MANUELLES** · **VERDECK** · **E88** |
| `MOTOR_IHX` | moteur · **IHX** |
| `MSA` | Start & Stop |
| `MSA_ANSTEUERLOGIK` | Start & Stop · **ANSTEUERLOGIK** |
| `MSA_KENNLINIENSATZ_US` | Start & Stop · **KENNLINIENSATZ** · marche americain |
| `MSA_MEMORY` | Start & Stop · memoire |
| `MSA_NOTSTRATEGIE` | Start & Stop · **NOTSTRATEGIE** |
| `NACHLAUFZEIT_UM_VENTIL` | **NACHLAUFZEIT** · **UM** · valve |
| `OFF_MEMORY` | arret · memoire |
| `PIA_PORTIERUNG` | **PIA** · **PORTIERUNG** |
| `PSEUDOZUHEIZEN` | **PSEUDOZUHEIZEN** |
| `REST_TASTE_VERBAUT` | chaleur residuelle · **TASTE** · installe |
| `RL_LL` | feu arriere · **LL** |
| `SH_ZUHEIZEN` | **SH** · **ZUHEIZEN** |
| `SOLARSENSORKENNLINIE` | **SOLARSENSORKENNLINIE** |
| `STANDHEIZUNG` | **STANDHEIZUNG** |
| `STANDHEIZUNG_SZ` | **STANDHEIZUNG** · **SZ** |
| `TAKTUNGSDAUER_HHS` | **TAKTUNGSDAUER** · _HHS_ |
| `TEILLASTBETRIEB_SH` | **TEILLASTBETRIEB** · **SH** |
| `TEMPERATUR_OFFSET_0` | temperature · decalage · 0 |
| `TEMPERATUR_OFFSET_1` | temperature · decalage · 1 |
| `TEMPERATUR_OFFSET_2` | temperature · decalage · 2 |
| `UEBERSETZUNG_KOMPRESSOR` | **UEBERSETZUNG** · compresseur |
| `UMLUFT_MEMORY` | recyclage d'air · memoire |
| `VERBRAUCHSOPTIMIERUNG` | **VERBRAUCHSOPTIMIERUNG** |
| `WARTUNGSINTERVALL_FZE` | **WARTUNGSINTERVALL** · **FZE** |
| `ZWEITE_HEIZZEIT_HHS` | **ZWEITE** · duree de chauffage · _HHS_ |
| `ZYLINDER_ZAHL` | **ZYLINDER** · **ZAHL** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
