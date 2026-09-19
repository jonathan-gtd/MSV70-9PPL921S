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
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 19 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `BAUART_MK60` | type de construction · bloc hydraulique MK60 | `e81_4zyl` · `e88_4zyl` · `e87_4zyl` · `e90_4zyl` · `e91_4zyl` · `e92_4zyl` · `e93_4zyl` · `e90_4zyl_m_fw` … *(+9)* |  |
| `C03_UNTERSTEUERSCHWELLE_2` | [variante C03] · seuil de sous-virage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `CHECKSUM` | somme de controle | `grundcode_var` |  |
| `DBC_MIN_AUFBAUGRADIENT` | _DBC_ · minimum · gradient de montee en pression | `wert_00` |  |
| `DBC_MIN_AUSLOESEDRUCK` | _DBC_ · minimum · pression de declenchement | `wert_00` · `wert_01` · `wert_02` |  |
| `DELTA_PSI_BETA_LIMITKORR2` | ecart · psi (?) · _BETA_ · correction de limite 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `DRUCKMODELL_HINTERACHSE` | modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `DRUCKMODELL_VORDERACHSE` | modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `GEARMOMENTREGLER_MAX_HA_2` | regulateur de couple de boite · maximum · essieu arriere · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `GEARMOMENTREGLER_MIN_HA_2` | regulateur de couple de boite · minimum · essieu arriere · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `GETRIEBE` | boite de vitesses | `handschaltung` · `automatik` · `sequentiell_ag` |  |
| `LENKWINKELKENNLINIE_2` | courbe d'angle de braquage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_05` |  |
| `MOTOR_ART_DSC_2` | moteur · type · controle de stabilite · 2 | `n46b20` · `m47d20` · `n52b30` · `n54b30` |  |
| `PRE_MSR` | pre- · regulation du couple de frein moteur | `aktiv` · `nicht_aktiv` |  |
| `RPA` | indicateur de crevaison | `nicht_aktiv` · `aktiv` |  |
| `UEBERSTEUERN_MUE_0_2` | survirage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `UEBERSTEUERN_MUE_1_2` | survirage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* |  |
| `UNTERSTEUERN_MUE_0_2` | sous-virage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` |  |
| `UNTERSTEUERN_MUE_1_2` | sous-virage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
