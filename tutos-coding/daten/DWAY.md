# DWAY — Alarme - protection volumetrique

🟠 Module `DWAY` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`DWA_IRS.C05`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **36** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 36 |
| Signification (EN) | 35 |
| Explication (FR) | 35 |
| Blocs d'adresses du TRC | 4 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 36 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ALARM_OPT_FERNLICHT` | alarme · option · feux de route | `nicht_aktiv` · `aktiv` | Visual alarm (high beams flash) | Alarme visuelle (flashs de feux de route) |
| `ALARM_OPT_WARNBLINKER` | alarme · option · feux de detresse | `nicht_aktiv` · `aktiv` | Visual alarm (hazard lights flash) | Alarme visuelle (flashs de feux de detresse) |
| `ANFRAGE_FENSTERPOSITIONEN` | requete · positions de vitre | `nicht_aktiv` · `aktiv` | Query window positions | Interrogation des positions de vitres |
| `CHECKSUM_0` | somme de controle · 0 | `grundcode_var` · `wert_01` · `ewp` · `wert_02` · `wert_03` · `wert_04` · `e92_m` · `instanceid_0x12` … *(+12)* | Checksum 0 | Somme de controle 0 |
| `CHECKSUM_1` | somme de controle · 1 | `grundcode_var` | Checksum 1 | Somme de controle 1 |
| `CHECKSUM_2` | somme de controle · 2 | `grundcode_var` | Checksum 2 | Somme de controle 2 |
| `CHECKSUM_3` | somme de controle · 3 | `grundcode_var` | Checksum 3 | Somme de controle 3 |
| `CODIERUNG_ERFOLGT` | codage · effectue | `codierung_erfolgt` | Coding completed | Codage termine |
| `CODIERUNG_ERFOLGT_1` | codage · effectue · 1 | `codierung_erfolgt` | Coding completed 1 | Codage termine 1 |
| `DEF_QUIT_AKUST_ENTSCH` | defaut · acquittement sonore · sonore · desarmement | `nicht_aktiv` · `aktiv` | Default car/key memory for acoustic unlocking acknowledge (two alarm beeps) | Valeur par defaut de l'acquittement sonore au deverrouillage (deux bips) |
| `DEF_QUIT_AKUST_SCHAERF` | defaut · acquittement sonore · sonore · armement | `nicht_aktiv` · `aktiv` | Default car/key memory for acoustic locking acknowledge (single alarm beep) | Valeur par defaut de l'acquittement sonore au verrouillage (un bip) |
| `DEF_QUIT_AKUST_SCHAERF_KL` | defaut · acquittement sonore · sonore · armement · borne | `nicht_aktiv` · `aktiv` | Default car/key memory for for acoustic locking acknowledge only when all doors and lids closed | Valeur par defaut de l'acquittement sonore au verrouillage, uniquement si tout est ferme |
| `DEF_QUIT_OPT_ENTSCH` | defaut · acquittement sonore · option · desarmement | `nicht_aktiv` · `aktiv` | Default car/key memory for visual unlocking acknowledge (two turn signal flashes) | Valeur par defaut de l'acquittement visuel au deverrouillage (deux flashs) |
| `DEF_QUIT_OPT_SCHAERF` | defaut · acquittement sonore · option · armement | `nicht_aktiv` · `aktiv` | Default car/key memory for visual locking acknowledge (single turn signal flash) | Valeur par defaut de l'acquittement visuel au verrouillage (un flash) |
| `DEF_QUIT_OPT_SCHAERF_KLAP` | defaut · acquittement sonore · option · armement · volet | `nicht_aktiv` · `aktiv` | Default car/key memory for visual locking acknowledge only when all doors and lids closed | Valeur par defaut de l'acquittement visuel au verrouillage, uniquement si tout est ferme |
| `FUNK_INNENRAUMSCHUTZ` | radiofrequence · protection volumetrique | `aktiv` · `nicht_aktiv` | Ultrasonic interior monitoring | Surveillance ultrasonique de l'habitacle |
| `HOTELSTELLUNG` | position hotel | `nicht_aktiv` · `aktiv` | Hotel mode | Mode hotel |
| `LADEEINRICHTUNG` | dispositif de charge | `nicht_aktiv` · `aktiv` | With touring cargo bay | Avec coffre de break |
| `LAUTSTAERKE` | volume sonore | `nicht_aktiv` · `aktiv` | Sound volume | Volume sonore |
| `MOTORFERNSTART` | demarrage moteur a distance | `nicht_aktiv` · `aktiv` | Remote engine start | Demarrage moteur a distance |
| `NEIGUNGSGEBER` | capteur d'inclinaison | `aktiv` · `nicht_aktiv` | Tilt sensor | Capteur d'inclinaison |
| `NOTLAUFFUNKTION_HUPE` | fonction de secours · avertisseur sonore | `nicht_aktiv` · `aktiv` | Use horn when alarm siren defective | Utiliser l'avertisseur sonore si la sirene d'alarme est defaillante |
| `PANIK_ALARM` | mode panique · alarme | `nicht_aktiv` · `aktiv` | Panic alarm | Alarme panique |
| `PIAU_AKUST_ENTSCH` | _PIAU_ · sonore · desarmement | `nicht_aktiv` · `aktiv` | Unpersonalized car/key memory for acoustic unlocking acknowledge (two alarm beeps) | Memoire par cle non personnalisee pour l'acquittement sonore au deverrouillage (deux bips) |
| `PIAU_AKUST_SCHAERF_KLAPPE` | _PIAU_ · sonore · armement · trappe | `nicht_aktiv` · `aktiv` | Unpersonalized car/key memory for acoustic locking acknowledge only when all doors and lids closed | Memoire par cle non personnalisee pour l'acquittement sonore au verrouillage, uniquement si tout est ferme |
| `QUIT_AKUST_ENTSCH` | acquittement sonore · sonore · desarmement | `nicht_aktiv` · `aktiv` | Acoustic unlocking acknowledge (two alarm beeps) | Bip sonore au desarmement de l'alarme. |
| `QUIT_AKUST_SCHAERF` | acquittement sonore · sonore · armement | `nicht_aktiv` · `aktiv` | Acoustic locking acknowledge (single alarm beep) | Bip sonore a l'armement de l'alarme. |
| `QUIT_AKUST_SCHAERF_KLAPPE` | acquittement sonore · sonore · armement · trappe | `nicht_aktiv` · `aktiv` | Acoustic locking acknowledge only when all doors and lids closed | Acquittement sonore au verrouillage uniquement si toutes les portes et trappes sont fermees |
| `QUIT_OPT_ENTSCH` | acquittement sonore · option · desarmement | `nicht_aktiv` · `aktiv` | Visual unlocking acknowledge (two turn signal flashes) | Acquittement visuel au deverrouillage (deux flashs de clignotants) |
| `QUIT_OPT_SCHAERF` | acquittement sonore · option · armement | `nicht_aktiv` · `aktiv` | Visual locking acknowledge (single turn signal flash) | Acquittement visuel au verrouillage (un flash de clignotants) |
| `QUIT_OPT_SCHAERF_KLAPPE` | acquittement sonore · option · armement · trappe | `nicht_aktiv` · `aktiv` | Visual locking acknowledge only when all doors and lids closed | Acquittement visuel au verrouillage uniquement si toutes les portes et trappes sont fermees |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable | `nicht_aktiv` · `aktiv` | Sliding/tilting sunroof | Toit ouvrant coulissant/entrebaillant |
| `SIGNAL_HK` | signal · hayon | `nicht_aktiv` · `aktiv` | Trunk lid signal | Signal de coffre |
| `TIMEOUT_DEAKT_IRNE` | delai d'attente · desactivation · infrarouge (?) | `wert_01` · `wert_02` · `wert_03` · `wert_05` · `wert_10` · `wert_15` | — | — |
| `TRIGGERANZAHL_KLAPPEN` | nombre de declenchements · volets | `wert_01` · `wert_02` | Trigger count door, hood and trunk lid contacts | Nombre de declenchements des contacteurs de porte, capot et coffre |
| `TRIGGERANZAHL_SENSOREN` | nombre de declenchements · capteurs | `wert_01` · `wert_02` | Trigger count sensors | Nombre de declenchements des capteurs |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
