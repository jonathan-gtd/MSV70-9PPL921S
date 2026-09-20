# DSC — Controle de stabilite / ABS

🔴 Module `DSC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`MK60_87.C03`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **19** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 19 |
| Signification (EN) | 18 |
| Explication (FR) | 18 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 19 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `BAUART_MK60` | type de construction · bloc hydraulique MK60 | `e81_4zyl` · `e88_4zyl` · `e87_4zyl` · `e90_4zyl` · `e91_4zyl` · `e92_4zyl` · `e93_4zyl` · `e90_4zyl_m_fw` … *(+9)* | Model variant | Variante de modele |
| `C03_UNTERSTEUERSCHWELLE_2` | [variante C03] · seuil de sous-virage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Understeer threshold 2 [data<128 ? coefficient=2.5+(data/64) : coefficient=2.5+((data-256)/64)] | Seuil de sous-virage 2 [valeur < 128 : coefficient = 2,5 + (valeur / 64), sinon 2,5 + ((valeur - 256) / 64)] |
| `CHECKSUM` | somme de controle | `grundcode_var` | Checksum | Somme de controle |
| `DBC_MIN_AUFBAUGRADIENT` | _DBC_ · minimum · gradient de montee en pression | `wert_00` | — | — |
| `DBC_MIN_AUSLOESEDRUCK` | _DBC_ · minimum · pression de declenchement | `wert_00` · `wert_01` · `wert_02` | Dynamic brake control (DBC) minimium release pressure | Pression minimale de relachement du controle dynamique de freinage (DBC) |
| `DELTA_PSI_BETA_LIMITKORR2` | ecart · psi (?) · _BETA_ · correction de limite 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` | Delta-psi-beta limit correction [value=1+(data/16)] | Correction de la limite delta-psi-beta [valeur = 1 + (valeur / 16)] |
| `DRUCKMODELL_HINTERACHSE` | modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` | Pressure model rear axle | Modele de pression, essieu arriere |
| `DRUCKMODELL_VORDERACHSE` | modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` | Pressure model front axle | Modele de pression, essieu avant |
| `GEARMOMENTREGLER_MAX_HA_2` | regulateur de couple de boite · maximum · essieu arriere · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` | Gear-torque control maximum rear axle [data<8 ? percentage_%=(data*4000)/1024 : percentage_%=((data-16)*4000)/1024] | Regulation de couple maximale, essieu arriere [valeur < 8 : % = (valeur x 4000) / 1024, sinon ((valeur - 16) x 4000) / 1024] |
| `GEARMOMENTREGLER_MIN_HA_2` | regulateur de couple de boite · minimum · essieu arriere · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` | Gear-torque control minimum rear axle [data<8 ? percentage_%=(data*4000)/1024 : percentage_%=((data-16)*4000)/1024] | Regulation de couple minimale, essieu arriere [valeur < 8 : % = (valeur x 4000) / 1024, sinon ((valeur - 16) x 4000) / 1024] |
| `GETRIEBE` | boite de vitesses | `handschaltung` · `automatik` · `sequentiell_ag` | Transmission | Boite de vitesses |
| `LENKWINKELKENNLINIE_2` | courbe d'angle de braquage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_05` | Steering angle characteristic curve 2 | Courbe caracteristique d'angle de braquage 2 |
| `MOTOR_ART_DSC_2` | moteur · type · controle de stabilite · 2 | `n46b20` · `m47d20` · `n52b30` · `n54b30` | Engine type | Type de moteur |
| `PRE_MSR` | pre- · regulation du couple de frein moteur | `aktiv` · `nicht_aktiv` | Pre engine drag-torque control | Pre-regulation du couple de frein moteur |
| `RPA` | indicateur de crevaison | `nicht_aktiv` · `aktiv` | Tire puncture warning (RPA) | Alerte de crevaison (RPA) |
| `UEBERSTEUERN_MUE_0_2` | survirage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` | Oversteer friction coefficient 0 2 [data<128 ? coefficient=1.5+(data/128) : coefficient=1.5+((data-256)/128)] | Coefficient d'adherence en survirage 0-2 [valeur < 128 : coefficient = 1,5 + (valeur / 128), sinon 1,5 + ((valeur - 256) / 128)] |
| `UEBERSTEUERN_MUE_1_2` | survirage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* | Oversteer friction coefficient 1 2 [data<128 ? coefficient=2.5+(data/64) : coefficient=2.5+((data-256)/64)] | Coefficient d'adherence en survirage 1-2 [valeur < 128 : coefficient = 2,5 + (valeur / 64), sinon 2,5 + ((valeur - 256) / 64)] |
| `UNTERSTEUERN_MUE_0_2` | sous-virage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` | Understeer friction coefficient 0 2 [data<128 ? coefficient=1.5+(data/128) : coefficient=1.5+((data-256)/128)] | Coefficient d'adherence en sous-virage 0-2 [valeur < 128 : coefficient = 1,5 + (valeur / 128), sinon 1,5 + ((valeur - 256) / 128)] |
| `UNTERSTEUERN_MUE_1_2` | sous-virage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` | Understeer friction coefficient 1 2 [data<128 ? coefficient=2.5+(data/64) : coefficient=2.5+((data-256)/64)] | Coefficient d'adherence en sous-virage 1-2 [valeur < 128 : coefficient = 2,5 + (valeur / 64), sinon 2,5 + ((valeur - 256) / 64)] |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française ou interprétation explicitement marquée
« Déduction du libellé ». Cette dernière reste à confirmer ; elle ne documente pas
les valeurs ni un effet testé sur le véhicule. Un tiret signifie qu’aucune explication
n’est établie. Voir [la méthode et les sources](TRADUCTIONS.md). Les notations d'unités sont converties : `[durée s = valeur / 10]`.
