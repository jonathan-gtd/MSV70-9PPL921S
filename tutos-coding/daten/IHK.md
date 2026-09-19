# IHK — Climatisation

🟢 Module `IHK` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`IHXXPL2.C02`** |
| Identifiée par | recoupement avec `IHK.TRC` lu sur le véhicule |
| Fonctions retenues | **74** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 72 |
| Avec explication rédigée | 6 |
| Blocs d'adresses du TRC | 1 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 74 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ABK_KLIMASTIL` | couvercle (?) · style de climatisation | — |  |
| `ALKS` | systeme d'assistance (?) | `nicht_aktiv` · `aktiv` |  |
| `ANZEIGE_TIMER_STHZ_SA` | affichage · minuterie · chauffage stationnaire · equipement optionnel | `nicht_aktiv` · `aktiv` |  |
| `AUTO_BEI_KL15_EIN_0` | automatique · lors de · borne 15 (contact) · marche · 0 | `aktiv` · `nicht_aktiv` |  |
| `AUTO_BEI_KL15_EIN_1` | automatique · lors de · borne 15 (contact) · marche · 1 | `nicht_aktiv` · `aktiv` |  |
| `AUTO_BEI_KL15_EIN_2` | automatique · lors de · borne 15 (contact) · marche · 2 | `aktiv` · `nicht_aktiv` |  |
| `BAUKASTEN_PTC` | modulaire · chauffage PTC | `verbaut` · `nicht_verbaut` |  |
| `BAUREIHE` | serie | `e81` · `e82` · `e87` · `e84` · `e88` · `e90` · `e91` · `e92` · `e93` · `e89` |  |
| `CC_MELDUNG_SH` | Check Control · message · toit ouvrant (?) | `nicht_aktiv` · `aktiv` | Message Check Control pour cette lampe. |
| `C_CKM_D_AUTO_BEI_KL15_EIN` | C · memoire par cle (Car Key Memory) · D · automatique · lors de · borne 15 (contact) · marche | `nicht_aktiv` · `aktiv` |  |
| `C_CKM_D_GEBLAESE_OFFSET` | C · memoire par cle (Car Key Memory) · D · pulseur · decalage | `kein_offset` · `geblaese_p_erhoehen` · `geblaese_p_verringern` |  |
| `C_CKM_D_KLIMA_MIT_AUTO` | C · memoire par cle (Car Key Memory) · D · climatisation · avec · automatique | `aktiv` · `nicht_aktiv` |  |
| `C_CKM_D_TEMPERATUR_OFFSET` | C · memoire par cle (Car Key Memory) · D · temperature · decalage | `kein_offset` · `offset_plus_1k` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_2k` · `offset_minus_3k` · `offset_minus_1k` |  |
| `DEFROST_LEITSCHAUFELN` | degivrage · volets directeurs | `nicht_aktiv` · `aktiv` |  |
| `DELAY_SH_LED` | delai · toit ouvrant (?) · LED | `nicht_aktiv` |  |
| `ECO_PRO_DEFAULT_START` | economie · professionnel · valeur par defaut · demarrage | `aktiv` · `nicht_aktiv` |  |
| `ECO_PRO_LAST_FU` | economie · professionnel · charge · radio (?) | `aktiv` · `nicht_aktiv` |  |
| `ECO_PRO_MODE` | economie · professionnel · mode | `aktiv` · `nicht_aktiv` |  |
| `EINBAUORT` | emplacement de montage | `unten` · `oben` |  |
| `EL_ZUHEIZER_PTC` | electrique · chauffage d'appoint · chauffage PTC | `verbaut` · `nicht_verbaut` |  |
| `ERSTE_HEIZZEIT_HHS_A` | premiere · duree de chauffage · _HHS_ · A | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` |  |
| `ERSTE_HEIZZEIT_HHS_B` | premiere · duree de chauffage · _HHS_ · B | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` |  |
| `F25_BESCHL_VERM` | serie F25 · acceleration · reduction (?) | `aktiv` · `nicht_aktiv` |  |
| `GBL_OFF` | global (?) · arret | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_AUTO_MIT_A` | pulseur · automatique · avec · A | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_AUTO_MIT_BALKEN` | pulseur · automatique · avec · barre | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_MIT_LUEFTER_SYMB` | pulseur · avec · ventilateur · symbole | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_MIT_RAHMEN` | pulseur · avec · cadre | `aktiv` · `nicht_aktiv` |  |
| `GEBLAESE_OFFSET_0` | pulseur · decalage · 0 | `geblaese_p_verringern` · `kein_geblaese_offset` · `geblaese_p_erhoehen` |  |
| `GEBLAESE_OFFSET_1` | pulseur · decalage · 1 | `kein_geblaese_offset` · `geblaese_p_verringern` · `geblaese_p_erhoehen` |  |
| `GEBLAESE_OFFSET_2` | pulseur · decalage · 2 | `kein_geblaese_offset` · `geblaese_p_verringern` · `geblaese_p_erhoehen` |  |
| `HDC_ZYKLISCH` | assistance en descente · cyclique | `aktiv` · `nicht_aktiv` |  |
| `IGR` | regulation (?) | `nicht_aktiv` · `aktiv` |  |
| `IHX_CBS` | climatisation (?) · entretien selon besoin (CBS) | `nicht_aktiv` · `aktiv` |  |
| `KLIMAGERAET_MIT_SCHICHTNG` | groupe de climatisation · avec · stratification | `nicht_aktiv` · `aktiv` |  |
| `KLIMA_MIT_AUTO_0` | climatisation · avec · automatique · 0 | `nicht_aktiv` · `aktiv` | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_MIT_AUTO_1` | climatisation · avec · automatique · 1 | `nicht_aktiv` · `aktiv` | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_MIT_AUTO_2` | climatisation · avec · automatique · 2 | `aktiv` · `nicht_aktiv` | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_SPERREN` | climatisation · blocage | `aktiv` · `nicht_aktiv` |  |
| `KMK_ZUSCHALTSCHWELLE` | constante (?) · seuil d'enclenchement | `hoch` · `niedrig` |  |
| `KOMPRESSOR` | compresseur | `klein` · `gross` |  |
| `KOMPRESSOR_KUPPLUNG` | compresseur · embrayage | `nicht_aktiv` · `aktiv` |  |
| `KOMPRESSOR_MITTEL` | compresseur · moyen | `aktiv` · `nicht_aktiv` |  |
| `KOMP_AKUSTIK` | compensation · acoustique | `vdw_hoch` · `vdw_niedrig` |  |
| `KOREA_SYMBOL` | Coree · symbole | `nicht_aktiv` · `aktiv` |  |
| `LAENDERVARANTE` | variante pays | `heissland` · `ece` · `us` · `japan` |  |
| `MANUELLES_VERDECK_E88` | manuel · capote · serie E88 | `verbaut` · `nicht_verbaut` |  |
| `MOTOR_IHX` | moteur · climatisation (?) | `benzin_normal` · `diesel_normal` |  |
| `MSA` | Start & Stop | `nicht_aktiv` · `aktiv` |  |
| `MSA_ANSTEUERLOGIK` | Start & Stop · logique de commande | `ihxx` · `can` |  |
| `MSA_KENNLINIENSATZ_US` | Start & Stop · jeu de courbes · marche americain | `aktiv` · `nicht_aktiv` |  |
| `MSA_MEMORY` | Start & Stop · memoire | `nicht_aktiv` · `aktiv` | Memorise le dernier etat du Start & Stop — sans objet avant 2008. |
| `MSA_NOTSTRATEGIE` | Start & Stop · strategie de secours | `nicht_aktiv` · `aktiv` |  |
| `NACHLAUFZEIT_UM_VENTIL` | duree de temporisation · vers · valve | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `OFF_MEMORY` | arret · memoire | `aktiv` · `nicht_aktiv` |  |
| `PIA_PORTIERUNG` | interface d'affichage (?) · portage | `datenportierung_aktiv` · `datenportierung_inaktiv` |  |
| `PSEUDOZUHEIZEN` | chauffage d'appoint simule | `aktiv` · `nicht_aktiv` |  |
| `REST_TASTE_VERBAUT` | chaleur residuelle · touche · installe | `nicht_aktiv` · `aktiv` | Remplace la touche ALL par la touche REST (chaleur residuelle). |
| `RL_LL` | feu arriere · ralenti (?) | `rl` · `ll` |  |
| `SH_ZUHEIZEN` | toit ouvrant (?) · chauffage d'appoint | `aktiv` · `nicht_aktiv` |  |
| `SOLARSENSORKENNLINIE` | courbe du capteur solaire | `kennlinie_02` · `kennlinie_01` |  |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` |  |
| `STANDHEIZUNG_SZ` | chauffage stationnaire · centre de commande (?) | `nicht_aktiv` · `aktiv` |  |
| `TAKTUNGSDAUER_HHS` | duree de cadencement · _HHS_ | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e93` · `e92` |  |
| `TEILLASTBETRIEB_SH` | fonctionnement en charge partielle · toit ouvrant (?) | `aktiv` · `nicht_aktiv` |  |
| `TEMPERATUR_OFFSET_0` | temperature · decalage · 0 | `offset_plus_1k` · `kein_offset` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` |  |
| `TEMPERATUR_OFFSET_1` | temperature · decalage · 1 | `kein_offset` · `offset_plus_1k` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` |  |
| `TEMPERATUR_OFFSET_2` | temperature · decalage · 2 | `offset_plus_1k` · `offset_plus_3k` · `offset_plus_2k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` · `kein_offset` |  |
| `UEBERSETZUNG_KOMPRESSOR` | rapport de demultiplication · compresseur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `UMLUFT_MEMORY` | recyclage d'air · memoire | `nicht_aktiv` · `aktiv` |  |
| `VERBRAUCHSOPTIMIERUNG` | optimisation de consommation | `nicht_aktiv` · `aktiv` |  |
| `WARTUNGSINTERVALL_FZE` | intervalle d'entretien · vehicule | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `ZWEITE_HEIZZEIT_HHS` | deuxieme · duree de chauffage · _HHS_ | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` |  |
| `ZYLINDER_ZAHL` | cylindres · nombre | — |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
