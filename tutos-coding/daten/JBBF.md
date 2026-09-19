# JBBF — Boitier de jonction

🟠 Module `JBBF` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`JBBFE.C07`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **97** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 72 |
| Signification (EN) | 50 |
| Explication (FR) | 50 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 97 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ANSTEUERZEIT_HK_MOTOR` | duree de commande · hayon · moteur | `tabelle_1` | Control time trunk lid release motor | Duree de commande du moteur d'ouverture de coffre |
| `ANSTEUERZEIT_HS_MOTOR` | duree de commande · lunette arriere (?) · moteur | `tabelle_1` | Control time touring rear window release motor | Duree de commande du moteur d'ouverture de lunette arriere |
| `ANSTEUERZEIT_ZV_MOTOREN` | duree de commande · verrouillage centralise · moteurs | `tabelle_1` | Control time central locking motors | Duree de commande des moteurs de verrouillage centralise |
| `ANZAHL_NACHWISCHZ_FRONT` | nombre · duree de balayage differe · avant | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Number of after-wiping cycles for windscreen washing [count=data] | Nombre de cycles de balayage apres lavage du pare-brise [nombre = valeur] |
| `ANZAHL_NACHWISCHZ_HECK` | nombre · duree de balayage differe · arriere | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Number of after-wiping cycles for rear window washing [count=data] | Nombre de cycles de balayage apres lavage de la lunette arriere [nombre = valeur] |
| `ANZAHL_SPRITZIMPULSE` | nombre · impulsions de lave-glace | `wert_02` · `wert_03` | Number of spray impulses | Nombre d'impulsions de lave-glace |
| `ANZAHL_WASCHBET_ZUR_SRA` | nombre · cycles de lavage · vers · lave-phares | `wert_05` · `wert_10` · `wert_01` | Number of wash operations with headlights on until headlight cleaning | Nombre de lavages feux allumes avant declenchement du lave-phares |
| `ANZ_EIN_N_BLOCK_WI_FRONT` | nombre · marche · N · blocage · essuyage · avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` | Number of switch-on requests after windscreen wipers blocked | Nombre de tentatives de redemarrage apres blocage des essuie-glaces avant |
| `ANZ_EIN_N_BLOCK_WI_HECK` | nombre · marche · N · blocage · essuyage · arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` | Number of switch-on requests after rear window wiper blocked | Nombre de tentatives de redemarrage apres blocage de l'essuie-glace arriere |
| `AUC_SENSOR` | capteur de qualite d'air · capteur | `nicht_aktiv` · `aktiv` | Automatic recirculated air (AUC) sensor | Capteur de qualite d'air pour le recyclage automatique (AUC) |
| `AUS_BEI_NICHT_EINSCHAF` | arret · lors de · non · activation (?) | `nicht_aktiv` · `aktiv` | — | — |
| `AUS_BEI_SVAUS` | arret · lors de · surveillance desactivee (?) | `nicht_aktiv` · `aktiv` | — | — |
| `AUS_BEI_WECKER` | arret · lors de · reveil | `nicht_aktiv` · `aktiv` | — | — |
| `BISTABILES_RELAIS_ALT` | bistable · relais · ancien | `nicht_aktiv` · `aktiv` · `vorne_hinten` · `vorne` | — | — |
| `BISTABREL_WECKER_ANZ` | relais bistable · reveil · nombre | `wert_30` | — | — |
| `BLOCKERKENNUNG_WISCH_FRON` | detection de blocage · essuyage · avant (?) | — | Windscreen wipers blocking detection (no feedback from reset contact) | Detection de blocage des essuie-glaces avant (absence de retour du contact de repos) |
| `BLOCKERKENNUNG_WISCH_HECK` | detection de blocage · essuyage · arriere | — | Rear window wiper blocking detection (no feedback from reset contact) | Detection de blocage de l'essuie-glace arriere (absence de retour du contact de repos) |
| `BREMSZEIT_ZV_MOTOREN` | duree de freinage · verrouillage centralise · moteurs | — | — | — |
| `CCM_PANIC_MODE` | message Check Control · alarme panique · mode | `nicht_aktiv` · `aktiv` | — | — |
| `CC_F_KUEHLMITTELSTAND` | Check Control · F · niveau de liquide de refroidissement | — | Check control time constant for coolant level [time_s=data/5] | Constante de temps Check Control pour le niveau de liquide de refroidissement [duree s = valeur / 5] |
| `CC_F_WASCHWASSERSTAND` | Check Control · F · niveau de lave-glace | — | Check control time constant for low washer fluid level [time_s=data/5] | Constante de temps Check Control pour le niveau de lave-glace bas [duree s = valeur / 5] |
| `CC_I_ZEITZELLE` | Check Control · I · cellule temporelle | — | — | — |
| `CODIERUNG_ERFOLGT_BLK0` | codage · effectue · clignotant | `codierung_erfolgt` · `wert_01` · `zieltermin` · `wert_02` | Coding completed block 0 | Codage termine, bloc 0 |
| `CODIERUNG_ERFOLGT_BLK1` | codage · effectue · clignotant 1 | `codierung_erfolgt` | Coding completed block 1 | Codage termine, bloc 1 |
| `DAUER_SPRITZIMPULSE_JBBF` | duree · impulsions de lave-glace · boitier de jonction | — | Spray impulse duration | Duree d'une impulsion de lave-glace |
| `DRUCKSENSOR` | capteur de pression | `nicht_aktiv` · `aktiv` | Pressure sensor | Capteur de pression |
| `ENDABSCHALTUNG_STROM` | coupure finale · courant | `wert_20` · `wert_25` · `wert_30` · `wert_35` · `wert_40` · `wert_45` · `wert_50` · `wert_55` | — | — |
| `ENTPRELLZEIT_TASTER_HK` | duree d'anti-rebond · bouton · hayon | `wert_30` | — | — |
| `ENTSICHERN_CENTERLOCK` | deverrouillage · verrouillage centralise | `nicht_aktiv` · `aktiv` | Central locking button unlocks passenger door and trunk lid when only driver door is unlocked and opened | Le bouton de verrouillage central ouvre la porte passager et le coffre lorsque seule la porte conducteur est deverrouillee et ouverte |
| `FEMODE_NOTLAUF_FRONTWISCH` | mode · mode degrade · essuyage avant | `nicht_aktiv` · `aktiv` | Emergency operation windscreen wipers in manufacturing mode | Fonctionnement de secours des essuie-glaces en mode fin de chaine |
| `FENSTERHEBER_ELEKTRISCH` | leve-vitre · electrique | `nicht_aktiv` · `aktiv` | Electric power windows | Leve-vitres electriques |
| `FENSTERHEBER_FAHRZEUGTYP` | leve-vitre · type de vehicule | `wert_01` · `wert_02` | — | — |
| `FENSTERHEBER_FREIG_CA` | leve-vitre · autorisation · acces confort (?) | `aktiv` · `nicht_aktiv` | — | — |
| `FENSTERHEBER_FREIG_DELAY` | leve-vitre · autorisation · delai | — | — | — |
| `FENSTERHEBER_MIT_EKS` | leve-vitre · avec · systeme electronique de cle | `nicht_aktiv` · `aktiv` | Power windows with jamming protection | Leve-vitres avec protection anti-pincement |
| `FOND_SCHICHT_POT` | places arriere · couche · potentiometre | `nicht_aktiv` · `aktiv` | — | — |
| `FUELTANK_PWM_ACT_TIME` | reservoir · rapport cyclique (PWM) · actif · duree | `wert_01` | — | — |
| `FUELTANK_PWM_PERIOD` | reservoir · rapport cyclique (PWM) · periode | `wert_01` | — | — |
| `FUELTANK_PWM_SUPPLY` | reservoir · rapport cyclique (PWM) · alimentation | `nicht_aktiv` · `aktiv` | — | — |
| `HANDSCHUHKASTEN_REPBLK` | boite a gants · blocage de repetition (?) | `nicht_aktiv` · `aktiv` | — | — |
| `HECKKLAPPE_OEFFNEN` | hayon · ouverture | `nicht_aktiv` · `aktiv` | Open trunk lid with remote control | Ouverture du coffre par telecommande |
| `HECKSCHEIBE_VERBAUT` | lunette arriere · installe | `nicht_aktiv` · `aktiv` | Touring rear window installed | Lunette arriere ouvrante de break installee |
| `HECKWASCH_START_DELAY` | lavage de lunette arriere · demarrage · delai | — | — | — |
| `HECKWASCH_STOP_TIME` | lavage de lunette arriere · arret · duree | — | — | — |
| `HECKWISCHER_EIN_RUECKWGAN` | essuie-glace arriere · marche · marche arriere engagee | `nicht_aktiv` · `aktiv` | Rear window wiper activates in reverse with windscreen wipers on | L'essuie-glace arriere se declenche en marche arriere si les essuie-glaces avant fonctionnent |
| `HECKWI_STOP_NACH_KLR_EIN` | essuie-glace arriere · arret · apres · regulation de climatisation · marche | `nicht_aktiv` · `aktiv` | Rear window wiper stops after switching off ignition | L'essuie-glace arriere s'arrete a la coupure du contact |
| `HSK_RPBLCK_LOWER_THRSHKLD` | chauffage de lunette arriere (?) · blocage de repetition (?) · inferieur · seuil | `wert_10` | — | — |
| `HSK_RPBLCK_UPPER_THRSHKLD` | chauffage de lunette arriere (?) · blocage de repetition (?) · superieur · seuil | `wert_15` | — | — |
| `HSK_TMER` | chauffage de lunette arriere (?) · minuteur | — | — | — |
| `HSK_TMSTART` | chauffage de lunette arriere (?) · demarrage du minuteur | — | — | — |
| `HSK_TPAUSE` | chauffage de lunette arriere (?) · duree de pause | — | — | — |
| `INTERVAL_HECKWISCHER` | intervalle · essuie-glace arriere | `nicht_aktiv` | Rear window wiper interval | Intervalle de l'essuie-glace arriere |
| `JBBFE_TUERAUF_STOP_MAUT` | boitier de jonction · porte ouverte · arret · peage | `nicht_aktiv` · `aktiv` | — | — |
| `KOMPRESSORVENTIL` | electrovanne de compresseur | `nicht_aktiv` · `aktiv` | Air conditioning compressor valve | Electrovanne du compresseur de climatisation |
| `KOMPRESSOR_GROSS` | compresseur · grand | `nicht_aktiv` · `aktiv` | Large air conditioning compressor | Compresseur de climatisation grande capacite |
| `KURZ_REVERSIERWEG_EKS` | court · course d'inversion · systeme electronique de cle | `wert_01` | — | — |
| `LADERAUMABDECK_AKTIV` | cache-bagages · actif | `keine` · `heckklappe` · `heckscheibe` · `beide` | — | — |
| `LADERAUMABDK_BEI_OFFN_HKK` | cache-bagages · lors de · ouverture · contacteur de hayon | `nicht_aktiv` · `aktiv` | — | — |
| `NACHLAUFZEIT_SITZHEIZUNG` | duree de temporisation · siege chauffant | — | — | — |
| `NACHLZ_BISTABR_TRANSPM` | duree de temporisation · bistable · mode transport | `nicht_aktiv` | — | — |
| `NACHL_KL30F` | temporisation · borne 30F | — | — | — |
| `PANIC_MODE` | alarme panique · mode | `nicht_aktiv` | Panic mode (trigger alarm and close power windows by holding remote control button 3) | Mode panique : declenche l'alarme et ferme les vitres par appui maintenu sur le bouton 3 |
| `PAUSE_SPRITZIMPULSE` | pause · impulsions de lave-glace | — | Pause between spray impulses | Pause entre impulsions de lave-glace |
| `POWERDOWN_WECKER_ANZ` | mise hors tension · reveil · nombre | `nicht_aktiv` | — | — |
| `PWRDWN_TIME_NICHT_EINSCHL` | mise hors tension · duree · non · activation | `nicht_aktiv` · `wert_01` · `wert_02` · `wert_04` | — | — |
| `REGENSENSOR` | capteur de pluie | `nicht_aktiv` · `aktiv` | Rain Sensor | Capteur de pluie |
| `REVERSIERWEG_EKS` | course d'inversion · systeme electronique de cle | `wert_210` · `wert_070` | Jamming protection move back distance | Distance de recul de la protection anti-pincement |
| `RUECKSCHALTUNG_AKT_KMH` | retrogradage · actif · km/h | `nicht_aktiv` | Speed to step-down windscreen wiping speed from stage 2 to stage 1 | Vitesse de passage du balayage rapide au balayage lent |
| `SCHEIBWA_BEI_WASSER_NIEDR` | lave-glace · lors de · eau · bas | `nicht_aktiv` · `aktiv` | — | — |
| `SCHEINWERFERREININUNG` | lave-phares | `nicht_aktiv` · `aktiv` | Headlight cleaning | Lave-phares. Le nom comporte une coquille d'origine BMW (REININUNG). |
| `SERVICEPOSITION` | position d'entretien | `nicht_aktiv` · `aktiv` | Windscreen wiper service position to facilitate wiper blade change | Position d'entretien des essuie-glaces pour faciliter le changement des balais |
| `SITZHEIZUNG_LOW` | siege chauffant · bas | `nicht_aktiv` · `aktiv` | — | — |
| `SOROLLO_ENTPRELL_STROM` | chaine non identifiee (?) · anti-rebond · courant | — | — | — |
| `SOROLLO_I_MAX_C` | chaine non identifiee (?) · I · maximum · C | `wert_07` · `wert_08` · `wert_09` · `wert_10` · `wert_11` · `wert_12` · `wert_13` · `wert_15` | — | — |
| `SOROLLO_LADEABDEC_HANDS_I` | chaine non identifiee (?) · cache-bagages · mains libres · I | `nicht_aktiv` · `sonnenrollo` · `laderaumabdeckung` · `handschuhkasten` | — | — |
| `SOROLLO_T_MAX` | chaine non identifiee (?) · T · maximum | `wert_08` · `wert_10` · `wert_12` · `wert_14` | — | — |
| `SPERRZ_N_BLOCK_WI_FRONT` | duree de blocage · N · blocage · essuyage · avant | — | Overload protection reset time for blocked windscreen wipers | Delai de reinitialisation de la protection de surcharge apres blocage des essuie-glaces avant |
| `SPERRZ_N_BLOCK_WI_HECK` | duree de blocage · N · blocage · essuyage · arriere | — | Overload protection reset time for blocked rear window wiper | Delai de reinitialisation de la protection de surcharge apres blocage de l'essuie-glace arriere |
| `SPRITZDUESEN_SPIEGEL_HEIZ` | gicleurs · retroviseur · chauffage | `nicht_aktiv` · `spritz` · `spritz_und_spiegel` | Heated windscreen washer jets and mirrors | Gicleurs de lave-glace et retroviseurs chauffants |
| `SRA_SPERRZEIT` | lave-phares · duree de blocage | — | Lockout time headlight cleaning | Temps de blocage du lave-phares |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` | Independent park heating (note: aktiv/nicht_aktiv inverted for chassis E31) | Chauffage stationnaire (attention : aktiv/nicht_aktiv sont inverses sur E31) |
| `STANDHEIZUNG_TIMEOUT` | chauffage stationnaire · delai d'attente | `nicht_aktiv` | Independent park heating timeout | Delai du chauffage stationnaire |
| `STANDRUECKSCHALTUNG` | retrogradage a l'arret | `nicht_aktiv` · `aktiv` | Windscreen wipers return to interval at standstill (without rain sensor) | Retour des essuie-glaces en mode intermittent a l'arret (sans capteur de pluie) |
| `TAKTWASCHEN_HECK` | lavage cadence · arriere | `nicht_aktiv` · `aktiv` | — | — |
| `THERMOSCHUTZ_AUSSENTEMP` | protection thermique · temperature exterieure | — | — | — |
| `TIME_AUS_NICHT_EINSCHLAF` | duree · arret · non · mise en veille | `nicht_aktiv` | — | — |
| `VERZOEGERUNG_HK_ANSTEUER` | temporisation · hayon · commande | — | — | — |
| `VERZOEGERUNG_MOT_ZS_VR` | temporisation · moteur · centrale · avant droit | `tabelle_1` | — | — |
| `WASSER_VENTIL` | eau · valve | `nicht_aktiv` · `aktiv` | Water valve | Vanne d'eau |
| `WIEDEREINSCH_TEMP` | reactivation · temperature | — | — | — |
| `WIEDERHOLSPERRE_ZV` | blocage de repetition · verrouillage centralise | `nicht_aktiv` · `aktiv` | Central locking overload protection | Protection de surcharge du verrouillage centralise |
| `WISCH_ERST_XXS_WASCH_FRON` | essuyage · premier · tres petit (?) · lavage · avant (?) | — | Delay wiping after windscreen washing | Temporisation de balayage apres lavage du pare-brise |
| `WISCH_ERST_XXS_WASCH_H_07` | essuyage · premier · tres petit (?) · lavage · _H_ · 07 | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Delay wiping after rear window washing | Temporisation de balayage apres lavage de la lunette arriere |
| `WISHENINTERVALL_STILL` | intervalle d'essuyage · silencieux | — | Windscreen wipers interval at standstill | Intervalle des essuie-glaces a l'arret |
| `ZUSATZWASSERPUMPE` | pompe a eau additionnelle | `nicht_verbaut` · `verbaut` | Auxiliary water pump | Pompe a eau additionnelle |
| `ZWP_EGS_STEUERN` | pompe a eau additionnelle · calculateur de boite · commander | `nicht_aktiv` · `aktiv` | — | — |
| `ZYCLUS_VOLLEND_KLR_AUS` | cycle · achevement · regulation de climatisation · arret | `nicht_aktiv` · `aktiv` | Wipers complete cycle after turning off ignition | Les essuie-glaces terminent leur cycle apres coupure du contact |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
