# IHK — Climatisation

🟢 Module `IHK` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `IHXXPL2.C02` |
| Fonctions codables | **74** |
| Avec valeurs extraites | 72 (97 %) |
| Avec explication rédigée | 6 |
| Fichiers DATEN analysés | 3 |

Variantes disponibles : `IHXXPL2.C01`, `IHXXPL2.C02`, `IHKAE82E.C10`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 74 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ABK_KLIMASTIL` | **ABK** · **KLIMASTIL** | — |  |
| `ALKS` | **ALKS** | `nicht_aktiv` · `aktiv` |  |
| `ANZEIGE_TIMER_STHZ_SA` | affichage · minuterie · **STHZ** · **SA** | `nicht_aktiv` · `aktiv` |  |
| `AUTO_BEI_KL15_EIN_0` | automatique · lors de · borne 15 (contact) · marche · 0 | `aktiv` · `nicht_aktiv` |  |
| `AUTO_BEI_KL15_EIN_1` | automatique · lors de · borne 15 (contact) · marche · 1 | `nicht_aktiv` · `aktiv` |  |
| `AUTO_BEI_KL15_EIN_2` | automatique · lors de · borne 15 (contact) · marche · 2 | `aktiv` · `nicht_aktiv` |  |
| `BAUKASTEN_PTC` | **BAUKASTEN** · **PTC** | `verbaut` · `nicht_verbaut` |  |
| `BAUREIHE` | **BAUREIHE** | `e81` · `e82` · `e87` · `e84` · `e88` · `e90` · `e91` · `e92` · `e93` · `e89` |  |
| `CC_MELDUNG_SH` | Check Control · message · **SH** | `nicht_aktiv` · `aktiv` | Message Check Control pour cette lampe. |
| `C_CKM_D_AUTO_BEI_KL15_EIN` | C · memoire par cle (Car Key Memory) · D · automatique · lors de · borne 15 (contact) · marche | `nicht_aktiv` · `aktiv` |  |
| `C_CKM_D_GEBLAESE_OFFSET` | C · memoire par cle (Car Key Memory) · D · pulseur · decalage | `kein_offset` · `geblaese_p_erhoehen` · `geblaese_p_verringern` |  |
| `C_CKM_D_KLIMA_MIT_AUTO` | C · memoire par cle (Car Key Memory) · D · climatisation · avec · automatique | `aktiv` · `nicht_aktiv` |  |
| `C_CKM_D_TEMPERATUR_OFFSET` | C · memoire par cle (Car Key Memory) · D · temperature · decalage | `kein_offset` · `offset_plus_1k` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_2k` · `offset_minus_3k` · `offset_minus_1k` |  |
| `DEFROST_LEITSCHAUFELN` | **DEFROST** · **LEITSCHAUFELN** | `nicht_aktiv` · `aktiv` |  |
| `DELAY_SH_LED` | delai · **SH** · LED | `nicht_aktiv` |  |
| `ECO_PRO_DEFAULT_START` | economie · **PRO** · **DEFAULT** · demarrage | `aktiv` · `nicht_aktiv` |  |
| `ECO_PRO_LAST_FU` | economie · **PRO** · charge · **FU** | `aktiv` · `nicht_aktiv` |  |
| `ECO_PRO_MODE` | economie · **PRO** · mode | `aktiv` · `nicht_aktiv` |  |
| `EINBAUORT` | **EINBAUORT** | `unten` · `oben` |  |
| `EL_ZUHEIZER_PTC` | **EL** · **ZUHEIZER** · **PTC** | `verbaut` · `nicht_verbaut` |  |
| `ERSTE_HEIZZEIT_HHS_A` | premier · duree de chauffage · _HHS_ · A | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` |  |
| `ERSTE_HEIZZEIT_HHS_B` | premier · duree de chauffage · _HHS_ · B | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` |  |
| `F25_BESCHL_VERM` | **F25** · **BESCHL** · **VERM** | `aktiv` · `nicht_aktiv` |  |
| `GBL_OFF` | **GBL** · arret | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_AUTO_MIT_A` | pulseur · automatique · avec · A | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_AUTO_MIT_BALKEN` | pulseur · automatique · avec · **BALKEN** | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_MIT_LUEFTER_SYMB` | pulseur · avec · ventilateur · **SYMB** | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_MIT_RAHMEN` | pulseur · avec · **RAHMEN** | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_OFFSET_0` | pulseur · decalage · 0 | `geblaese_p_verringern` · `kein_geblaese_offset` · `geblaese_p_erhoehen` |  |
| `GEBLAESE_OFFSET_1` | pulseur · decalage · 1 | `kein_geblaese_offset` · `geblaese_p_verringern` · `geblaese_p_erhoehen` |  |
| `GEBLAESE_OFFSET_2` | pulseur · decalage · 2 | `kein_geblaese_offset` · `geblaese_p_verringern` · `geblaese_p_erhoehen` |  |
| `HDC_ZYKLISCH` | assistance en descente · **ZYKLISCH** | `aktiv` · `nicht_aktiv` |  |
| `IGR` | **IGR** | `nicht_aktiv` · `aktiv` |  |
| `IHX_CBS` | **IHX** · entretien selon besoin (CBS) | `nicht_aktiv` · `aktiv` |  |
| `KLIMAGERAET_MIT_SCHICHTNG` | **KLIMAGERAET** · avec · **SCHICHTNG** | `nicht_aktiv` · `aktiv` |  |
| `KLIMA_MIT_AUTO_0` | climatisation · avec · automatique · 0 | `nicht_aktiv` · `aktiv` | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_MIT_AUTO_1` | climatisation · avec · automatique · 1 | `nicht_aktiv` · `aktiv` | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_MIT_AUTO_2` | climatisation · avec · automatique · 2 | `aktiv` · `nicht_aktiv` | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_SPERREN` | climatisation · **SPERREN** | `aktiv` · `nicht_aktiv` |  |
| `KMK_ZUSCHALTSCHWELLE` | **KMK** · **ZUSCHALTSCHWELLE** | `hoch` · `niedrig` |  |
| `KOMPRESSOR` | compresseur | `klein` · `gross` |  |
| `KOMPRESSOR_KUPPLUNG` | compresseur · embrayage | `nicht_aktiv` · `aktiv` |  |
| `KOMPRESSOR_MITTEL` | compresseur · moyen | `aktiv` · `nicht_aktiv` |  |
| `KOMP_AKUSTIK` | compensation · **AKUSTIK** | `vdw_hoch` · `vdw_niedrig` |  |
| `KOREA_SYMBOL` | **KOREA** · **SYMBOL** | `nicht_aktiv` · `aktiv` |  |
| `LAENDERVARANTE` | **LAENDERVARANTE** | `heissland` · `ece` · `us` · `japan` |  |
| `MANUELLES_VERDECK_E88` | **MANUELLES** · **VERDECK** · **E88** | `verbaut` · `nicht_verbaut` |  |
| `MOTOR_IHX` | moteur · **IHX** | `benzin_normal` · `diesel_normal` |  |
| `MSA` | Start & Stop | `nicht_aktiv` · `aktiv` |  |
| `MSA_ANSTEUERLOGIK` | Start & Stop · **ANSTEUERLOGIK** | `ihxx` · `can` |  |
| `MSA_KENNLINIENSATZ_US` | Start & Stop · **KENNLINIENSATZ** · marche americain | `aktiv` · `nicht_aktiv` |  |
| `MSA_MEMORY` | Start & Stop · memoire | `nicht_aktiv` · `aktiv` | Memorise le dernier etat du Start & Stop — sans objet avant 2008. |
| `MSA_NOTSTRATEGIE` | Start & Stop · **NOTSTRATEGIE** | `nicht_aktiv` · `aktiv` |  |
| `NACHLAUFZEIT_UM_VENTIL` | **NACHLAUFZEIT** · **UM** · valve | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `OFF_MEMORY` | arret · memoire | `aktiv` · `nicht_aktiv` |  |
| `PIA_PORTIERUNG` | **PIA** · **PORTIERUNG** | `datenportierung_aktiv` · `datenportierung_inaktiv` |  |
| `PSEUDOZUHEIZEN` | **PSEUDOZUHEIZEN** | `aktiv` · `nicht_aktiv` |  |
| `REST_TASTE_VERBAUT` | chaleur residuelle · **TASTE** · installe | `nicht_aktiv` · `aktiv` | Remplace la touche ALL par la touche REST (chaleur residuelle). |
| `RL_LL` | feu arriere · **LL** | `rl` · `ll` |  |
| `SH_ZUHEIZEN` | **SH** · **ZUHEIZEN** | `aktiv` · `nicht_aktiv` |  |
| `SOLARSENSORKENNLINIE` | **SOLARSENSORKENNLINIE** | `kennlinie_02` · `kennlinie_01` |  |
| `STANDHEIZUNG` | **STANDHEIZUNG** | `nicht_aktiv` · `aktiv` |  |
| `STANDHEIZUNG_SZ` | **STANDHEIZUNG** · **SZ** | `nicht_aktiv` · `aktiv` |  |
| `TAKTUNGSDAUER_HHS` | **TAKTUNGSDAUER** · _HHS_ | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e93` · `e92` |  |
| `TEILLASTBETRIEB_SH` | **TEILLASTBETRIEB** · **SH** | `aktiv` · `nicht_aktiv` |  |
| `TEMPERATUR_OFFSET_0` | temperature · decalage · 0 | `offset_plus_1k` · `kein_offset` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` |  |
| `TEMPERATUR_OFFSET_1` | temperature · decalage · 1 | `kein_offset` · `offset_plus_1k` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` |  |
| `TEMPERATUR_OFFSET_2` | temperature · decalage · 2 | `offset_plus_1k` · `offset_plus_3k` · `offset_plus_2k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` · `kein_offset` |  |
| `UEBERSETZUNG_KOMPRESSOR` | **UEBERSETZUNG** · compresseur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `UMLUFT_MEMORY` | recyclage d'air · memoire | `nicht_aktiv` · `aktiv` |  |
| `VERBRAUCHSOPTIMIERUNG` | **VERBRAUCHSOPTIMIERUNG** | `nicht_aktiv` · `aktiv` |  |
| `WARTUNGSINTERVALL_FZE` | **WARTUNGSINTERVALL** · **FZE** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `ZWEITE_HEIZZEIT_HHS` | **ZWEITE** · duree de chauffage · _HHS_ | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` |  |
| `ZYLINDER_ZAHL` | **ZYLINDER** · **ZAHL** | — |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
