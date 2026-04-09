# XDF Analyzer — MSV70

Extrait et corrèle les paramètres d'un fichier XDF TunerPro avec la documentation JSON.

## Usage

```bash
cd xdf-analyzer
python3 main.py
```

**Prérequis :** Python 3.6+ — bibliothèques standard uniquement (pas de `pip install`).  
**Sortie :** résumé console + export CSV horodaté dans `exports/`.

## Structure

```
data/
  BMW_Siemens_MSV70_9PPL921S_2560K.xdf   ← Définitions TunerPro (axes, équations, adresses)
  VB67774_921S_Full.bin                  ← Bin stock de référence (2 Mo)

docs/
  01_injecteurs.json                     ← Scaling injecteurs
  02_demarrage_froid.json                ← Cranking, after-start, seuils TCO
  03_chauffe.json                        ← Warm-up lambda, enrichissements post-démarrage
  04_allumage.json                       ← Tables avance, knock control
  05_film_mural.json                     ← Wall film (slow/fast), corrections Valvetronic
  06_lambda_richesse.json                ← Tables lambda (bas_1-4, WOT ip_lamb_fl__n)
  07_adaptations_evap.json               ← Purge canister, EVAP, corrections STFT/LTFT
  08_limiteurs.json                      ← RPM protection, VMAX, protections thermiques
  09_ralenti.json                        ← Régulation ralenti, cibles régime

exports/                                 ← Sorties CSV (générées par main.py)
```

## Modules

| Fichier | Rôle |
|---|---|
| `main.py` | Point d'entrée — orchestre les 3 étapes |
| `xdf_parser.py` | Parse le fichier XDF (adresses, équations, axes) |
| `doc_manager.py` | Charge les JSON et enrichit chaque paramètre |
| `exporter.py` | Génère le CSV et affiche le résumé console |
