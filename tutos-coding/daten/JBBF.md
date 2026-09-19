# JBBF — Boitier de jonction

🟠 Module `JBBF` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`JBBFE.C07`** |
| Identifiée par | recoupement avec `JBBF.TRC` lu sur le véhicule |
| Fonctions retenues | **97** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 72 |
| Avec explication rédigée | 1 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 97 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ANSTEUERZEIT_HK_MOTOR` | duree de commande · hayon · moteur | `tabelle_1` |  |
| `ANSTEUERZEIT_HS_MOTOR` | duree de commande · lunette arriere (?) · moteur | `tabelle_1` |  |
| `ANSTEUERZEIT_ZV_MOTOREN` | duree de commande · verrouillage centralise · moteurs | `tabelle_1` |  |
| `ANZAHL_NACHWISCHZ_FRONT` | nombre · duree de balayage differe · avant | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `ANZAHL_NACHWISCHZ_HECK` | nombre · duree de balayage differe · arriere | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `ANZAHL_SPRITZIMPULSE` | nombre · impulsions de lave-glace | `wert_02` · `wert_03` |  |
| `ANZAHL_WASCHBET_ZUR_SRA` | nombre · cycles de lavage · vers · lave-phares | `wert_05` · `wert_10` · `wert_01` |  |
| `ANZ_EIN_N_BLOCK_WI_FRONT` | nombre · marche · N · blocage · essuyage · avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `ANZ_EIN_N_BLOCK_WI_HECK` | nombre · marche · N · blocage · essuyage · arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `AUC_SENSOR` | capteur de qualite d'air · capteur | `nicht_aktiv` · `aktiv` |  |
| `AUS_BEI_NICHT_EINSCHAF` | arret · lors de · non · activation (?) | `nicht_aktiv` · `aktiv` |  |
| `AUS_BEI_SVAUS` | arret · lors de · surveillance desactivee (?) | `nicht_aktiv` · `aktiv` |  |
| `AUS_BEI_WECKER` | arret · lors de · reveil | `nicht_aktiv` · `aktiv` |  |
| `BISTABILES_RELAIS_ALT` | bistable · relais · ancien | `nicht_aktiv` · `aktiv` · `vorne_hinten` · `vorne` |  |
| `BISTABREL_WECKER_ANZ` | relais bistable · reveil · nombre | `wert_30` |  |
| `BLOCKERKENNUNG_WISCH_FRON` | detection de blocage · essuyage · avant (?) | — |  |
| `BLOCKERKENNUNG_WISCH_HECK` | detection de blocage · essuyage · arriere | — |  |
| `BREMSZEIT_ZV_MOTOREN` | duree de freinage · verrouillage centralise · moteurs | — |  |
| `CCM_PANIC_MODE` | message Check Control · alarme panique · mode | `nicht_aktiv` · `aktiv` |  |
| `CC_F_KUEHLMITTELSTAND` | Check Control · F · niveau de liquide de refroidissement | — |  |
| `CC_F_WASCHWASSERSTAND` | Check Control · F · niveau de lave-glace | — |  |
| `CC_I_ZEITZELLE` | Check Control · I · cellule temporelle | — |  |
| `CODIERUNG_ERFOLGT_BLK0` | codage · effectue · clignotant | `codierung_erfolgt` · `wert_01` · `zieltermin` · `wert_02` |  |
| `CODIERUNG_ERFOLGT_BLK1` | codage · effectue · clignotant 1 | `codierung_erfolgt` |  |
| `DAUER_SPRITZIMPULSE_JBBF` | duree · impulsions de lave-glace · boitier de jonction | — |  |
| `DRUCKSENSOR` | capteur de pression | `nicht_aktiv` · `aktiv` |  |
| `ENDABSCHALTUNG_STROM` | coupure finale · courant | `wert_20` · `wert_25` · `wert_30` · `wert_35` · `wert_40` · `wert_45` · `wert_50` · `wert_55` |  |
| `ENTPRELLZEIT_TASTER_HK` | duree d'anti-rebond · bouton · hayon | `wert_30` |  |
| `ENTSICHERN_CENTERLOCK` | deverrouillage · verrouillage centralise | `nicht_aktiv` · `aktiv` |  |
| `FEMODE_NOTLAUF_FRONTWISCH` | mode · mode degrade · essuyage avant | `nicht_aktiv` · `aktiv` |  |
| `FENSTERHEBER_ELEKTRISCH` | leve-vitre · electrique | `nicht_aktiv` · `aktiv` |  |
| `FENSTERHEBER_FAHRZEUGTYP` | leve-vitre · type de vehicule | `wert_01` · `wert_02` |  |
| `FENSTERHEBER_FREIG_CA` | leve-vitre · autorisation · acces confort (?) | `aktiv` · `nicht_aktiv` |  |
| `FENSTERHEBER_FREIG_DELAY` | leve-vitre · autorisation · delai | — |  |
| `FENSTERHEBER_MIT_EKS` | leve-vitre · avec · systeme electronique de cle | `nicht_aktiv` · `aktiv` |  |
| `FOND_SCHICHT_POT` | places arriere · couche · potentiometre | `nicht_aktiv` · `aktiv` |  |
| `FUELTANK_PWM_ACT_TIME` | reservoir · rapport cyclique (PWM) · actif · duree | `wert_01` |  |
| `FUELTANK_PWM_PERIOD` | reservoir · rapport cyclique (PWM) · periode | `wert_01` |  |
| `FUELTANK_PWM_SUPPLY` | reservoir · rapport cyclique (PWM) · alimentation | `nicht_aktiv` · `aktiv` |  |
| `HANDSCHUHKASTEN_REPBLK` | boite a gants · blocage de repetition (?) | `nicht_aktiv` · `aktiv` |  |
| `HECKKLAPPE_OEFFNEN` | hayon · ouverture | `nicht_aktiv` · `aktiv` |  |
| `HECKSCHEIBE_VERBAUT` | lunette arriere · installe | `nicht_aktiv` · `aktiv` |  |
| `HECKWASCH_START_DELAY` | lavage de lunette arriere · demarrage · delai | — |  |
| `HECKWASCH_STOP_TIME` | lavage de lunette arriere · arret · duree | — |  |
| `HECKWISCHER_EIN_RUECKWGAN` | essuie-glace arriere · marche · marche arriere engagee | `nicht_aktiv` · `aktiv` |  |
| `HECKWI_STOP_NACH_KLR_EIN` | essuie-glace arriere · arret · apres · regulation de climatisation · marche | `nicht_aktiv` · `aktiv` |  |
| `HSK_RPBLCK_LOWER_THRSHKLD` | chauffage de lunette arriere (?) · blocage de repetition (?) · inferieur · seuil | `wert_10` |  |
| `HSK_RPBLCK_UPPER_THRSHKLD` | chauffage de lunette arriere (?) · blocage de repetition (?) · superieur · seuil | `wert_15` |  |
| `HSK_TMER` | chauffage de lunette arriere (?) · minuteur | — |  |
| `HSK_TMSTART` | chauffage de lunette arriere (?) · demarrage du minuteur | — |  |
| `HSK_TPAUSE` | chauffage de lunette arriere (?) · duree de pause | — |  |
| `INTERVAL_HECKWISCHER` | intervalle · essuie-glace arriere | `nicht_aktiv` |  |
| `JBBFE_TUERAUF_STOP_MAUT` | boitier de jonction · porte ouverte · arret · peage | `nicht_aktiv` · `aktiv` |  |
| `KOMPRESSORVENTIL` | electrovanne de compresseur | `nicht_aktiv` · `aktiv` |  |
| `KOMPRESSOR_GROSS` | compresseur · grand | `nicht_aktiv` · `aktiv` |  |
| `KURZ_REVERSIERWEG_EKS` | court · course d'inversion · systeme electronique de cle | `wert_01` |  |
| `LADERAUMABDECK_AKTIV` | cache-bagages · actif | `keine` · `heckklappe` · `heckscheibe` · `beide` |  |
| `LADERAUMABDK_BEI_OFFN_HKK` | cache-bagages · lors de · ouverture · contacteur de hayon | `nicht_aktiv` · `aktiv` |  |
| `NACHLAUFZEIT_SITZHEIZUNG` | duree de temporisation · siege chauffant | — |  |
| `NACHLZ_BISTABR_TRANSPM` | duree de temporisation · bistable · mode transport | `nicht_aktiv` |  |
| `NACHL_KL30F` | temporisation · borne 30F | — |  |
| `PANIC_MODE` | alarme panique · mode | `nicht_aktiv` |  |
| `PAUSE_SPRITZIMPULSE` | pause · impulsions de lave-glace | — |  |
| `POWERDOWN_WECKER_ANZ` | mise hors tension · reveil · nombre | `nicht_aktiv` |  |
| `PWRDWN_TIME_NICHT_EINSCHL` | mise hors tension · duree · non · activation | `nicht_aktiv` · `wert_01` · `wert_02` · `wert_04` |  |
| `REGENSENSOR` | capteur de pluie | `nicht_aktiv` · `aktiv` |  |
| `REVERSIERWEG_EKS` | course d'inversion · systeme electronique de cle | `wert_210` · `wert_070` |  |
| `RUECKSCHALTUNG_AKT_KMH` | retrogradage · actif · km/h | `nicht_aktiv` |  |
| `SCHEIBWA_BEI_WASSER_NIEDR` | lave-glace · lors de · eau · bas | `nicht_aktiv` · `aktiv` |  |
| `SCHEINWERFERREININUNG` | lave-phares | `nicht_aktiv` · `aktiv` | Lave-phares. Le nom comporte une coquille d'origine BMW (REININUNG). |
| `SERVICEPOSITION` | position d'entretien | `nicht_aktiv` · `aktiv` |  |
| `SITZHEIZUNG_LOW` | siege chauffant · bas | `nicht_aktiv` · `aktiv` |  |
| `SOROLLO_ENTPRELL_STROM` | chaine non identifiee (?) · anti-rebond · courant | — |  |
| `SOROLLO_I_MAX_C` | chaine non identifiee (?) · I · maximum · C | `wert_07` · `wert_08` · `wert_09` · `wert_10` · `wert_11` · `wert_12` · `wert_13` · `wert_15` |  |
| `SOROLLO_LADEABDEC_HANDS_I` | chaine non identifiee (?) · cache-bagages · mains libres · I | `nicht_aktiv` · `sonnenrollo` · `laderaumabdeckung` · `handschuhkasten` |  |
| `SOROLLO_T_MAX` | chaine non identifiee (?) · T · maximum | `wert_08` · `wert_10` · `wert_12` · `wert_14` |  |
| `SPERRZ_N_BLOCK_WI_FRONT` | duree de blocage · N · blocage · essuyage · avant | — |  |
| `SPERRZ_N_BLOCK_WI_HECK` | duree de blocage · N · blocage · essuyage · arriere | — |  |
| `SPRITZDUESEN_SPIEGEL_HEIZ` | gicleurs · retroviseur · chauffage | `nicht_aktiv` · `spritz` · `spritz_und_spiegel` |  |
| `SRA_SPERRZEIT` | lave-phares · duree de blocage | — |  |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` |  |
| `STANDHEIZUNG_TIMEOUT` | chauffage stationnaire · delai d'attente | `nicht_aktiv` |  |
| `STANDRUECKSCHALTUNG` | retrogradage a l'arret | `nicht_aktiv` · `aktiv` |  |
| `TAKTWASCHEN_HECK` | lavage cadence · arriere | `nicht_aktiv` · `aktiv` |  |
| `THERMOSCHUTZ_AUSSENTEMP` | protection thermique · temperature exterieure | — |  |
| `TIME_AUS_NICHT_EINSCHLAF` | duree · arret · non · mise en veille | `nicht_aktiv` |  |
| `VERZOEGERUNG_HK_ANSTEUER` | temporisation · hayon · commande | — |  |
| `VERZOEGERUNG_MOT_ZS_VR` | temporisation · moteur · centrale · avant droit | `tabelle_1` |  |
| `WASSER_VENTIL` | eau · valve | `nicht_aktiv` · `aktiv` |  |
| `WIEDEREINSCH_TEMP` | reactivation · temperature | — |  |
| `WIEDERHOLSPERRE_ZV` | blocage de repetition · verrouillage centralise | `nicht_aktiv` · `aktiv` |  |
| `WISCH_ERST_XXS_WASCH_FRON` | essuyage · premier · tres petit (?) · lavage · avant (?) | — |  |
| `WISCH_ERST_XXS_WASCH_H_07` | essuyage · premier · tres petit (?) · lavage · _H_ · 07 | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `WISHENINTERVALL_STILL` | intervalle d'essuyage · silencieux | — |  |
| `ZUSATZWASSERPUMPE` | pompe a eau additionnelle | `nicht_verbaut` · `verbaut` |  |
| `ZWP_EGS_STEUERN` | pompe a eau additionnelle · calculateur de boite · commander | `nicht_aktiv` · `aktiv` |  |
| `ZYCLUS_VOLLEND_KLR_AUS` | cycle · achevement · regulation de climatisation · arret | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
