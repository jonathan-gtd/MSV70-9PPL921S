# Sauvegardes de codage

> Tout ce qui est lu dans la voiture atterrit ici, **avant** toute écriture.  
> Procédure : [tutos-coding/](../../tutos-coding/README.md)

---

## Arborescence

```
data/Coding/
├── README.md
├── FA_<AAAAMMJJ>.txt          ← le bon de commande usine
└── backup/
    ├── <MODULE>_stock_<AAAAMMJJ>.TRC     ← état d'origine (à ne jamais écraser)
    ├── <MODULE>_<AAAAMMJJ>.MAN           ← ce qui a été écrit
    └── <MODULE>_apres_<AAAAMMJJ>.TRC     ← relecture de contrôle
```

---

## Convention de nommage

| Fichier | Format | Exemple |
|---|---|---|
| Bon de commande | `FA_<AAAAMMJJ>.txt` | `FA_20260918.txt` |
| État d'origine | `<MODULE>_stock_<AAAAMMJJ>.TRC` | `KOMBI_stock_20260918.TRC` |
| Modification appliquée | `<MODULE>_<AAAAMMJJ>.MAN` | `KOMBI_20260918.MAN` |
| Relecture après écriture | `<MODULE>_apres_<AAAAMMJJ>.TRC` | `KOMBI_apres_20260918.TRC` |

Le nom du module est celui affiché par NCS Expert.

---

## Règles

1. **Le FA en premier.** Aucune écriture dans la voiture tant que `FA_*.txt` n'est pas committé. C'est le seul moyen de recoder un module d'origine.

2. **Un `_stock_` ne s'écrase jamais.** C'est la référence de retour en arrière. Un seul par module, celui de la toute première lecture — même dix sessions plus tard.

3. **Renommer immédiatement après la lecture.** NCS Expert écrit toujours dans le même fichier `C:\NCSEXPER\WORK\FSW_PSW.TRC`, écrasé à chaque lecture. Deux lectures d'affilée sans renommage = la première est perdue.

4. **Committer avant de coder, pas après.** Une sauvegarde restée sur le disque du portable ne sert à rien le jour où le portable est le problème.

5. **Vider le `.MAN` en fin de session :**
   ```
   copy nul "C:\NCSEXPER\WORK\FSW_PSW.MAN"
   ```
   Ce fichier doit faire 0 octet au lancement de NCS Expert, sinon `SG_CODIEREN` peut appliquer les restes d'une session précédente au mauvais module.

---

## Le fichier FA

Copier le texte tel qu'affiché par NCS Expert, sans reformater :

```
FA_20260918.txt
-----------------------------------------
Date de lecture : 18/09/2026
Lu via         : NCS Expert F1 → F3 ZCS/FA f. ECU
VIN            : ...
Type           : ...
Salapa         : $205 $494 $8SP ...
```

En cas de doute sur une lecture, la recouper avec `Tool32 → CAS.PRG → FA_LESEN`.
