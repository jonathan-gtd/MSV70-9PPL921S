# MCP Server — MSV70 XDF/BIN Analyzer

Serveur [Model Context Protocol](https://modelcontextprotocol.io) qui expose les outils d'analyse ECU directement dans Claude Code et Claude Desktop.

## Démarrage

```bash
pip install -r mcp-claude/requirements.txt
```

La configuration `.mcp.json` à la racine du projet charge le serveur automatiquement dans **Claude Code** au démarrage.

Pour **Claude Desktop**, ajouter dans `claude_desktop_config.json` :

```json
{
  "mcpServers": {
    "msv70-xdf-analyzer": {
      "command": "C:\\Python312\\python.exe",
      "args": ["C:\\chemin\\vers\\MSV70-9PPL921S\\mcp-claude\\mcp_server.py"]
    }
  }
}
```

## Outils exposés

| Outil | Description |
|---|---|
| `list_params` | Liste les paramètres XDF avec filtres (nom, catégorie, type, E85 only) |
| `get_param_info` | Définition complète d'un paramètre (métadonnées XDF + documentation) |
| `read_bin_value` | Lit la valeur décodée d'un paramètre dans un BIN |
| `compare_bins` | Compare un paramètre entre deux BINs et rapporte les écarts |
| `list_bin_files` | Liste les BINs disponibles avec leur taille |
| `diff_all_changed_params` | Scanne tous les paramètres et retourne ceux qui diffèrent entre deux BINs |
| `verify_e85_bin` | Vérifie un BIN E85 contre la base de connaissance (ok / manquant / inattendu) |

## BINs disponibles

| Clé | Fichier |
|---|---|
| `stock` | `bins/Full/VB67774_921S_Full.bin` |
| `stock_e85` | `bins/Full/VB67774_921S_Full_E85.bin` |
| `partial` | `bins/Partial/VB67774_921S_Partial.bin` |
| `s7_partial` | `bins/Partial/S7581293_Partial.bin` |

Les outils acceptent aussi un chemin absolu vers n'importe quel `.bin`.

## Structure

```
mcp-claude/
  mcp_server.py     ← point d'entrée, expose les 7 outils via FastMCP
  requirements.txt  ← mcp[cli]>=1.0.0

xdf-analyzer/       ← bibliothèques Python importées par le serveur
  xdf_parser.py     ← parse le fichier XDF TunerPro
  bin_reader.py     ← lit et décode les valeurs depuis un BIN
  doc_manager.py    ← enrichit les paramètres avec la base de connaissance
  knowledge/        ← JSON de documentation (e85_knowledge.json, xdf_catalog.json)

bins/               ← dumps ECU (Full 2560K et Partial 128K)
  Full/
  Partial/
```
