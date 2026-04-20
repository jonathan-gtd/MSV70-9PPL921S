# XDF Analyzer — MSV70

Outils Python pour analyser les paramètres ECU d'un fichier XDF TunerPro, lire les valeurs depuis un BIN, et exposer tout ça via un serveur MCP (Model Context Protocol).

## Installation

```bash
cd xdf-analyzer
pip install -r requirements.txt
```

## Usage CLI

```bash
python main.py
```

Sortie : résumé console + export CSV horodaté dans `exports/`.

## Serveur MCP

Le serveur MCP permet d'interroger le calculateur directement depuis Claude.

```bash
python mcp_server.py
```

### Outils exposés

| Outil | Description |
|---|---|
| `list_params` | Liste les paramètres XDF avec filtres (search, catégorie, type, E85) |
| `get_param_info` | Définition complète d'un paramètre (métadonnées + documentation) |
| `read_bin_value` | Lit la valeur décodée d'un paramètre dans un fichier BIN |
| `compare_bins` | Compare un paramètre entre deux BIN et liste les différences |
| `diff_all_changed_params` | Scan tous les paramètres modifiés entre deux BIN |
| `list_bin_files` | Liste les fichiers BIN disponibles avec taille et existence |

### Configuration Claude Code

Le fichier `.mcp.json` à la racine du repo configure le serveur automatiquement pour Claude Code CLI.

### Configuration Claude Desktop

Ajouter ce bloc dans `%APPDATA%\Claude\claude_desktop_config.json` (Windows) ou `~/Library/Application Support/Claude/claude_desktop_config.json` (Mac) :

```json
{
  "mcpServers": {
    "msv70-xdf-analyzer": {
      "command": "python",
      "args": ["mcp_server.py"],
      "cwd": "C:/chemin/vers/MSV70-9PPL921S/xdf-analyzer"
    }
  }
}
```

## Structure

```
data/
  Full/
    BMW_Siemens_MSV70_9PPL921S_2560K.xdf   ← Définitions TunerPro full (2560 Ko, BASEOFFSET=0x40000)
    VB67774_921S_Full.bin                  ← Dump ECU voiture (2 Mo)
    VB67774_921S_Full_E85.bin              ← Bin E85 modifié (2 Mo)

  Partial/
    BMW_Siemens_MSV70_9PPL921S_128K.xdf    ← Définitions TunerPro partial (BASEOFFSET=0)
    S7581293_Partial.bin                   ← Stock référence 0BN1S (123 Ko)
    VB67774_921S_Partial.bin               ← Dump ECU partiel (123 Ko)
    VB67774_921S_Partial_E85.bin           ← Bin E85 partiel (123 Ko)

docs/
  01_injecteurs.json                       ← Scaling injecteurs
  02_demarrage_froid.json                  ← Cranking, after-start, seuils TCO
  03_chauffe.json                          ← Warm-up lambda, enrichissements post-démarrage
  04_allumage.json                         ← Tables avance, knock control
  05_film_mural.json                       ← Wall film (slow/fast), corrections Valvetronic
  06_lambda_richesse.json                  ← Tables lambda (bas_1-4, WOT ip_lamb_fl__n)
  07_adaptations_evap.json                 ← Purge canister, EVAP, corrections STFT/LTFT
  08_limiteurs.json                        ← RPM protection, VMAX, protections thermiques
  09_ralenti.json                          ← Régulation ralenti, cibles régime

exports/                                   ← Sorties CSV (générées par main.py, ignorées par git)
```

## Modules

| Fichier | Rôle |
|---|---|
| `main.py` | Point d'entrée CLI — orchestre parse + enrich + export |
| `xdf_parser.py` | Parse le fichier XDF (adresses, équations, axes) |
| `doc_manager.py` | Charge les JSON docs et enrichit chaque paramètre |
| `bin_reader.py` | Lit et décode les valeurs depuis un fichier BIN |
| `exporter.py` | Génère le CSV et affiche le résumé console |
| `mcp_server.py` | Serveur MCP — expose les outils ECU à Claude |

## Notes de calibration E85 — MSV70 N52B30

### Facteur injecteur — limitation XDF

Le paramètre `c_fac_mff_ti_stnd` existe en 5 copies dans le bin :

| Copie | Adresse | Équation XDF | Max encodable |
|---|---|---|---|
| `[0]` | 0x45AAC | `0.000012 × X` (uint16) | 0.7864 |
| `[1]` | 0x45AAE | `0.000012 × X` (uint16) | 0.7864 |
| `_1` | 0x44AC0 | `0.000006 × X` (uint16) | **0.3932** |
| `_2` | 0x44AC2 | `0.000006 × X` (uint16) | **0.3932** |
| `_mon` | 0x4958C | `0.000006 × X` (uint16) | **0.3932** |

La cible E85 pour le N52B30 est **0.491 ms/mg** (facteur ×1.447 vs stock 0.3394).

Les copies `_1`, `_2` et `_mon` utilisent l'équation `0.000006 × X` : le maximum physique encodable
est `65535 × 0.000006 = 0.3932`, soit **inférieur à la cible 0.491**. Il est impossible d'y écrire
la valeur E85 correcte avec ce XDF.

### Solution retenue : `ip_mff_cor_opm`

Au lieu de modifier `c_fac_mff_ti_stnd`, l'enrichissement E85 est appliqué via les maps de
correction multiplicative `ip_mff_cor_opm_*` (eq = `0.000031 × X`, max encodable = 2.031).

| Paramètre | Adresse | Taille | Stock | E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_1_1` | 0x4E3D4 | 12×16 | 1.0159 (flat) | **1.4730 (flat)** |
| `ip_mff_cor_opm_1_2` | 0x4E554 | 12×16 | 1.0159 (flat) | **1.4730 (flat)** |
| `ip_mff_cor_opm_2_1` | 0x4E6D4 | 10×12 | 1.0159 (flat) | **1.4730 (flat)** |
| `ip_mff_cor_opm_2_2` | 0x4E7C4 | 10×12 | 1.0159 (flat) | **1.4730 (flat)** |

`c_fac_mff_ti_stnd` reste au stock (0.3394) sur toutes les copies — aucun problème de monitoring.

**Effectif combiné :** `0.3394 × 1.4730 = 0.4999 ms/mg` → ratio ×1.450 vs stock (cible ×1.447, écart 0.2%).
