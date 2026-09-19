# IHK — Climatisation

🟢 Module `IHK` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`IHXXPL2.C01`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **58** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 57 |
| Signification (EN) | 26 |
| Explication (FR) | 27 |
| Blocs d'adresses du TRC | 1 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 58 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ALKS` | systeme d'assistance (?) | `nicht_aktiv` · `aktiv` | Radiator shutter flaps | Volets de calandre |
| `ANZEIGE_TIMER_STHZ_SA` | affichage · minuterie · chauffage stationnaire · equipement optionnel | `nicht_aktiv` · `aktiv` | — | — |
| `AUTO_BEI_KL15_EIN_0` | automatique · lors de · borne 15 (contact) · marche · 0 | `aktiv` · `nicht_aktiv` | — | — |
| `AUTO_BEI_KL15_EIN_1` | automatique · lors de · borne 15 (contact) · marche · 1 | `nicht_aktiv` · `aktiv` | — | — |
| `AUTO_BEI_KL15_EIN_2` | automatique · lors de · borne 15 (contact) · marche · 2 | `aktiv` · `nicht_aktiv` | — | — |
| `BAUREIHE` | serie | `e81` · `e82` · `e84` · `e87` · `e90` · `e91` · `e92` · `e93` · `e88` · `e89` | Chassis | Serie du vehicule |
| `C_CKM_D_AUTO_BEI_KL15_EIN` | C · memoire par cle (Car Key Memory) · D · automatique · lors de · borne 15 (contact) · marche | `nicht_aktiv` · `aktiv` | — | — |
| `C_CKM_D_GEBLAESE_OFFSET` | C · memoire par cle (Car Key Memory) · D · pulseur · decalage | `kein_offset` · `geblaese_p_erhoehen` · `geblaese_p_verringern` | — | — |
| `C_CKM_D_KLIMA_MIT_AUTO` | C · memoire par cle (Car Key Memory) · D · climatisation · avec · automatique | `aktiv` · `nicht_aktiv` | — | — |
| `C_CKM_D_TEMPERATUR_OFFSET` | C · memoire par cle (Car Key Memory) · D · temperature · decalage | `kein_offset` · `offset_plus_1k` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_2k` · `offset_minus_3k` · `offset_minus_1k` | — | — |
| `DEFROST_LEITSCHAUFELN` | degivrage · volets directeurs | `nicht_aktiv` · `aktiv` | — | — |
| `EINBAUORT` | emplacement de montage | `unten` · `oben` | Installation location | Emplacement de montage |
| `EL_ZUHEIZER_PTC` | electrique · chauffage d'appoint · chauffage PTC | `verbaut` · `nicht_verbaut` | Electric auxiliary heater with positive temperature coefficient (PTC) | Chauffage d'appoint electrique a coefficient de temperature positif (PTC) |
| `ERSTE_HEIZZEIT_HHS_A` | premiere · duree de chauffage · _HHS_ · A | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` | — | — |
| `ERSTE_HEIZZEIT_HHS_B` | premiere · duree de chauffage · _HHS_ · B | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` | — | — |
| `GEBLAESE_AUTO_MIT_A` | pulseur · automatique · avec · A | `aktiv` · `nicht_aktiv` | Display "AUTO" in automatic mode | Afficher AUTO en mode automatique |
| `GEBLAESE_AUTO_MIT_BALKEN` | pulseur · automatique · avec · barre | `aktiv` · `nicht_aktiv` | Display blower speed bars in automatic mode | Afficher les barres de vitesse du pulseur en mode automatique |
| `GEBLAESE_MIT_LUEFTER_SYMB` | pulseur · avec · ventilateur · symbole | `aktiv` · `nicht_aktiv` | Display blower symbol in automatic mode | Afficher le symbole du pulseur en mode automatique |
| `GEBLAESE_MIT_RAHMEN` | pulseur · avec · cadre | `aktiv` · `nicht_aktiv` | — | — |
| `GEBLAESE_OFFSET_0` | pulseur · decalage · 0 | `geblaese_p_verringern` · `kein_geblaese_offset` · `geblaese_p_erhoehen` | — | — |
| `GEBLAESE_OFFSET_1` | pulseur · decalage · 1 | `kein_geblaese_offset` · `geblaese_p_verringern` · `geblaese_p_erhoehen` | — | — |
| `GEBLAESE_OFFSET_2` | pulseur · decalage · 2 | `kein_geblaese_offset` · `geblaese_p_verringern` · `geblaese_p_erhoehen` | — | — |
| `IGR` | regulation (?) | `nicht_aktiv` · `aktiv` | Intelligent alternator control (IGR) | Gestion intelligente de l'alternateur (IGR) |
| `IHX_CBS` | climatisation (?) · entretien selon besoin (CBS) | `nicht_aktiv` · `aktiv` | — | — |
| `KLIMAGERAET_MIT_SCHICHTUN` | groupe de climatisation · avec · stratification | `nicht_aktiv` · `aktiv` | — | — |
| `KLIMA_MIT_AUTO_0` | climatisation · avec · automatique · 0 | `nicht_aktiv` · `aktiv` | Air conditioning compressor on in automatic mode | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_MIT_AUTO_1` | climatisation · avec · automatique · 1 | `nicht_aktiv` · `aktiv` | Air conditioning compressor on in automatic mode | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_MIT_AUTO_2` | climatisation · avec · automatique · 2 | `aktiv` · `nicht_aktiv` | Air conditioning compressor on in automatic mode | Coupe la climatisation lors de l'appui sur la touche AUTO. |
| `KLIMA_SPERREN` | climatisation · blocage | `aktiv` · `nicht_aktiv` | Disable air conditioning compressor | Desactiver le compresseur de climatisation |
| `KMK_ZUSCHALTSCHWELLE` | constante (?) · seuil d'enclenchement | `hoch` · `niedrig` | — | — |
| `KOMPRESSOR` | compresseur | `klein` · `gross` | Air conditioning compressor | Compresseur de climatisation |
| `KOMPRESSOR_KUPPLUNG` | compresseur · embrayage | `nicht_aktiv` · `aktiv` | Air conditioning compressor clutch | Embrayage du compresseur de climatisation |
| `KOMPRESSOR_MITTEL` | compresseur · moyen | `aktiv` · `nicht_aktiv` | Air conditioning compressor medium | Fluide du compresseur de climatisation |
| `LAENDERVARANTE` | variante pays | `heissland` · `ece` · `us` · `japan` | Country variant | Variante pays |
| `MANUELLES_VERDECK_E88` | manuel · capote · serie E88 | `verbaut` · `nicht_verbaut` | Manual convertible top E88 (1 Series convertible) | Capote manuelle E88 (Serie 1 cabriolet) |
| `MOTOR_IHX` | moteur · climatisation (?) | `benzin_normal` · `diesel_normal` | — | — |
| `MSA` | Start & Stop | `nicht_aktiv` · `aktiv` | Automatic start/stop (MSA) | Systeme Start & Stop (MSA) |
| `MSA_ANSTEUERLOGIK` | Start & Stop · logique de commande | `ihxx` · `can` | — | — |
| `MSA_MEMORY` | Start & Stop · memoire | `nicht_aktiv` · `aktiv` | — | Memorise le dernier etat du Start & Stop — sans objet avant 2008. |
| `MSA_NOTSTRATEGIE` | Start & Stop · strategie de secours | `nicht_aktiv` · `aktiv` | — | — |
| `NACHLAUFZEIT_UM_VENTIL` | duree de temporisation · vers · valve | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` | — | — |
| `OFF_MEMORY` | arret · memoire | `aktiv` · `nicht_aktiv` | Car/key memory for climate control off | Memoire par cle pour la climatisation coupee |
| `PIA_PORTIERUNG` | interface d'affichage (?) · portage | `datenportierung_aktiv` · `datenportierung_inaktiv` | — | — |
| `PSEUDOZUHEIZEN` | chauffage d'appoint simule | `aktiv` · `nicht_aktiv` | Pseudo heater for independent park heating | Chauffage simule pour le chauffage stationnaire |
| `RL_LL` | feu arriere · ralenti (?) | `rl` · `ll` | Right-hand drive / left-hand drive | Conduite a droite / conduite a gauche |
| `SOLARSENSORKENNLINIE` | courbe du capteur solaire | `kennlinie_02` · `kennlinie_01` | — | — |
| `STANDHEIZER_ALS_ZUHEIZER` | chauffage stationnaire · en tant que · chauffage d'appoint | `nicht_aktiv` · `aktiv` | Independent park heating as auxiliary heating | Chauffage stationnaire utilise en chauffage d'appoint |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` | Independent park heating (note: aktiv/nicht_aktiv inverted for chassis E31) | Chauffage stationnaire (attention : aktiv/nicht_aktiv sont inverses sur E31) |
| `STANDHEIZUNG_SZ` | chauffage stationnaire · centre de commande (?) | `nicht_aktiv` · `aktiv` | — | — |
| `TAKTUNGSDAUER_HHS` | duree de cadencement · _HHS_ | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e93` · `e92` | Duty cycle of rear window defroster | Rapport cyclique du degivrage de lunette arriere |
| `TEMPERATUR_OFFSET_0` | temperature · decalage · 0 | `offset_plus_1k` · `kein_offset` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` | — | — |
| `TEMPERATUR_OFFSET_1` | temperature · decalage · 1 | `kein_offset` · `offset_plus_1k` · `offset_plus_2k` · `offset_plus_3k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` | — | — |
| `TEMPERATUR_OFFSET_2` | temperature · decalage · 2 | `offset_plus_1k` · `offset_plus_3k` · `offset_plus_2k` · `offset_minus_1k` · `offset_minus_2k` · `offset_minus_3k` · `kein_offset` | — | — |
| `UEBERSETZUNG_KOMPRESSOR` | rapport de demultiplication · compresseur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` | — | — |
| `UMLUFT_MEMORY` | recyclage d'air · memoire | `nicht_aktiv` · `aktiv` | Recall last state of recirculating air after switching on ignition | Rappeler le dernier etat du recyclage d'air a la mise du contact |
| `WARTUNGSINTERVALL_FZE` | intervalle d'entretien · vehicule | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | — | — |
| `ZWEITE_HEIZZEIT_HHS` | deuxieme · duree de chauffage · _HHS_ | `e81` · `e82` · `e84` · `e87` · `e88` · `e90` · `e90b` · `e91` · `e92` · `e93` | — | — |
| `ZYLINDER_ZAHL` | cylindres · nombre | — | Cylinder count | Nombre de cylindres |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
