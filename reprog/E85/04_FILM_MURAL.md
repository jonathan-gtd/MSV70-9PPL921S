# §4 — Film Mural (Wall Film Correction)

> 💡 Le film de carburant sur les parois du collecteur est **~25–35% plus épais sur E85**. Le tip-in est une zone de boucle ouverte — film sous-compensé = lean transitoire = claquement d'admission. On calibre pour E85 (×1.25), safe même avec du E70 réel.

### 📋 Tables à modifier

| Paramètre | Adresse | Type | Modification |
|---|---|---|---|
| `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | 8×8, film lent, Valvetronic | **×1.25** |
| `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | 8×8, film lent, papillonné | **×1.25** |
| `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | 8×8, film rapide, Valvetronic | **×1.25** |
| `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | 8×8, film rapide, papillonné | **×1.25** |

> Tables `_neg_*` : non modifiées en première intention.

### 🔨 Procédure

```
TunerPro → chaque table _pos_* → Ctrl+A → Scale ×1.25 → répéter ×4
```

### Le vrai système de film mural du MSV70

#### Pourquoi lent ET rapide ?

Le film de carburant sur les parois du collecteur se comporte comme deux phénomènes superposés avec des constantes de temps très différentes :

- **Film lent (slow)** : la couche de carburant qui s'accumule progressivement et s'évapore entre les injections. Sa dynamique est de l'ordre de plusieurs secondes — c'est le film « résiduel » qui persiste entre les accélérations. Axes : TCO × RPM. **Impact E85 : l'éthanol s'évapore beaucoup moins vite que l'essence à basse température → ce film grossit.**

- **Film rapide (fast)** : la réponse instantanée lors d'une transition de charge (tip-in / tip-out brutal). Quand vous enfoncez la pédale d'un coup, le débit d'air augmente soudainement alors que le film mural n'a pas encore eu le temps de répondre — il faut injecter un surplus immédiat pour compenser. Axes : TCO × RPM, mêmes que le film lent, mais les valeurs sont environ ×0.4–0.6 plus petites car la composante rapide est une correction transitoire, pas un enrichissement continu.

#### Pourquoi positif ET négatif ?

- **Positif (pos)** : correction d'injection en **enrichissement** — s'applique quand le film mural absorbe du carburant (accélération, augmentation de charge). L'ECU injecte plus que la dose calculée pour « alimenter » le film.
- **Négatif (neg)** : correction d'injection en **appauvrissement** — s'applique quand le film restitue du carburant (décélération, levée de pied). L'ECU injecte moins car le film en cours d'évaporation fournit du carburant supplémentaire au cylindre.

**Pour E85 :** augmenter les tables `_pos_*` (le film est plus épais, il faut compenser plus). Les tables `_neg_*` ne sont **pas** modifiées en première intention — ce sont des tables statiques dans le bin, elles ne s'adaptent pas d'elles-mêmes. La raison de les laisser en attente : leur valeur stock est environ 6× plus faible que les tables positives correspondantes (exemple : pos_slow = 36.0 vs neg_slow = 6.0 à froid), donc l'erreur absolue d'un neg non corrigé est petite. De plus, la sous-correction en tip-out produit un mélange légèrement **riche** en décélération — sans risque. À l'inverse, une table pos incorrecte produirait un lean transitoire en tip-in — risque de raté ou de cliquetis. Corriger les neg en deuxième étape, une fois les pos validées.

Le MSV70 implémente ce modèle complet avec **deux composantes** (lente et rapide) et **deux directions** (positive et négative). Les tables principales sont :

| Paramètre | Adresse | Dim. / axes | Description XDF | Rôle |
|---|---|---|---|---|
| `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | 8×8, X=TCO, Y=RPM | total positive wall film factor | Film lent — enrichissement tip-in |
| `ip_ti_tco_neg_slow_wf_opm_1` | 0x4CAFC | 8×8, X=TCO, Y=RPM | total negative wall film factor | Film lent — décélération |
| `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | 8×8, X=TCO, Y=RPM | fast positive wall film factor | Film rapide — tip-in brutal |
| `ip_ti_tco_neg_fast_wf_opm_1` | 0x4437C | 8×8, X=TCO, Y=RPM | fast negative wall film factor | Film rapide — tip-out brutal |
| `ip_ti_fac_pos_fast_wf_opm_1` | 0x4425C | 8×8, X=charge, Y=RPM | load correction fast wall film positive | Correction charge pour film rapide pos |
| `ip_ti_fac_neg_fast_wf_opm_1` | 0x4415C | 8×8, X=charge, Y=RPM | load correction fast wall film negative | Correction charge pour film rapide neg |
| `ip_fac_ti_wf_opm_1` | 0x42CBA | 4×4, X=TCO, Y=TIA | temperature correction for total wallfilm | Facteur global T° |
| **Valvetronic-specific (N52)** | | | | |
| `ip_fac_ti_maf_sp_wf_pos_opm_1` | 0x42C5A | 1×8, X=TCO | coolant temperature correction for air mass flow setpoint triggered wallfilm - positive | Film mural déclenché par changement de levée Valvetronic |
| `ip_crlc_pos_maf_sp_wf_opm_1` | — | — | correlation constant for MAF-SP triggered wallfilm positive | Constante corrélation |
| `ip_ti_cor_tps_mod_wf` | — | — | Wall film injection time for VLFT change at TPS-mode | Correction TI pour changement levée valves |

Toutes ces tables existent aussi en version `_opm_2` (mode papillonné). Sur N52 Valvetronic, c'est `_opm_1` qui est actif la majorité du temps.

<a id="pencil-film"></a>

### ✏️ Avant / Après — les 4 tables de film mural positif

> Axes communs : X = TCO (°C), Y = RPM. Objectif E85 = stock × 1.25 (arrondi à la décimale).

**`ip_ti_tco_pos_slow_wf_opm_1` @ 0x4CBFC** (film lent, Valvetronic) — équation `0.100 × raw_u16be` :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   36.0   36.0   32.0   26.0   18.0    8.0    4.0    2.0
 896 rpm :   38.0   38.0   34.0   28.0   20.0   10.0    6.0    2.0
1216 rpm :   40.0   40.0   36.0   30.0   22.0   12.0    8.0    4.0
1600 rpm :   42.0   42.0   38.0   32.0   24.0   14.0   10.0    6.0
2208 rpm :   46.0   46.0   42.0   36.0   28.0   18.0   14.0   10.0
3008 rpm :   50.0   50.0   46.0   40.0   32.0   22.0   18.0   14.0
4192 rpm :   54.0   54.0   50.0   44.0   36.0   26.0   22.0   18.0
5600 rpm :   58.0   58.0   54.0   48.0   40.0   30.0   26.0   22.0
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   45.0   45.0   40.0   32.5   22.5   10.0    5.0    2.5
 896 rpm :   47.5   47.5   42.5   35.0   25.0   12.5    7.5    2.5
1216 rpm :   50.0   50.0   45.0   37.5   27.5   15.0   10.0    5.0
1600 rpm :   52.5   52.5   47.5   40.0   30.0   17.5   12.5    7.5
2208 rpm :   57.5   57.5   52.5   45.0   35.0   22.5   17.5   12.5
3008 rpm :   62.5   62.5   57.5   50.0   40.0   27.5   22.5   17.5
4192 rpm :   67.5   67.5   62.5   55.0   45.0   32.5   27.5   22.5
5600 rpm :   72.5   72.5   67.5   60.0   50.0   37.5   32.5   27.5
```

---

**`ip_ti_tco_pos_slow_wf_opm_2` @ 0x4CC7C** (film lent, papillonné) — même équation :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  224.7  180.1  134.1  114.0   90.0   26.0   20.0   20.0
 896 rpm :  224.7  180.1  132.7  113.5   88.5   28.0   20.0   20.0
1216 rpm :  224.7  180.1  132.9  107.0   81.0   28.0   20.0   20.0
1600 rpm :  224.7  180.1  133.5   76.5   57.1   30.0   20.0   18.0
2208 rpm :  224.7  180.3  134.6   76.7   57.6   32.0   22.0   18.0
3008 rpm :  224.7  185.4  144.7  105.0   78.5   36.0   26.0   26.0
4192 rpm :  234.9  194.5  155.0  127.7   96.6   52.3   37.8   33.6
5600 rpm :  259.4  215.5  172.3  142.7  112.5   69.8   52.9   50.7
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  280.9  225.1  167.6  142.5  112.5   32.5   25.0   25.0
 896 rpm :  280.9  225.1  165.9  141.9  110.6   35.0   25.0   25.0
1216 rpm :  280.9  225.1  166.1  133.8  101.3   35.0   25.0   25.0
1600 rpm :  280.9  225.1  166.9   95.6   71.4   37.5   25.0   22.5
2208 rpm :  280.9  225.4  168.3   95.9   72.0   40.0   27.5   22.5
3008 rpm :  280.9  231.8  180.9  131.3   98.1   45.0   32.5   32.5
4192 rpm :  293.6  243.1  193.8  159.6  120.8   65.4   47.3   42.0
5600 rpm :  324.3  269.4  215.4  178.4  140.6   87.3   66.1   63.4
```

---

**`ip_ti_tco_pos_fast_wf_opm_1` @ 0x443FC** (film rapide, Valvetronic) — équation `0.500 × raw_u8` :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   22.0   20.5   18.5   15.0   11.0    7.5    6.0    4.5
 896 rpm :   23.5   21.5   19.5   16.0   12.0    8.0    6.5    5.0
1216 rpm :   25.5   23.0   21.0   17.5   13.5    9.0    7.0    5.5
1600 rpm :   28.0   25.5   23.0   19.5   15.5   10.5    8.0    6.5
2208 rpm :   32.5   29.5   26.5   23.0   18.5   13.0   10.5    9.0
3008 rpm :   39.0   35.5   32.0   28.0   23.0   17.0   14.0   12.5
4192 rpm :   52.5   47.5   43.0   38.0   32.0   25.0   21.5   19.5
5600 rpm :   72.5   66.5   60.0   53.5   45.5   37.0   33.0   31.0
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   27.5   25.6   23.1   18.8   13.8    9.4    7.5    5.6
 896 rpm :   29.4   26.9   24.4   20.0   15.0   10.0    8.1    6.3
1216 rpm :   31.9   28.8   26.3   21.9   16.9   11.3    8.8    6.9
1600 rpm :   35.0   31.9   28.8   24.4   19.4   13.1   10.0    8.1
2208 rpm :   40.6   36.9   33.1   28.8   23.1   16.3   13.1   11.3
3008 rpm :   48.8   44.4   40.0   35.0   28.8   21.3   17.5   15.6
4192 rpm :   65.6   59.4   53.8   47.5   40.0   31.3   26.9   24.4
5600 rpm :   90.6   83.1   75.0   66.9   56.9   46.3   41.3   38.8
```

---

**`ip_ti_tco_pos_fast_wf_opm_2` @ 0x4443C** (film rapide, papillonné) — même équation :

STOCK :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   80.0   66.0   46.5   42.0   24.5   12.0   10.0   10.0
 896 rpm :   80.0   66.0   46.0   41.0   26.0   14.5   11.5   11.5
1216 rpm :   80.0   66.0   47.0   37.0   25.5   16.5   13.0   13.0
1600 rpm :   80.0   66.0   48.0   34.5   25.0   19.0   14.5   14.5
2208 rpm :   80.0   66.0   50.5   35.0   27.5   22.0   16.0   16.0
3008 rpm :   81.5   68.5   52.5   42.5   33.0   26.5   17.0   17.0
4192 rpm :   88.0   73.0   57.0   49.0   40.0   30.5   23.0   20.5
5600 rpm :   94.0   78.5   61.5   54.5   47.5   34.5   28.5   26.0
```

OBJECTIF E85 (×1.25) :
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  100.0   82.5   58.1   52.5   30.6   15.0   12.5   12.5
 896 rpm :  100.0   82.5   57.5   51.3   32.5   18.1   14.4   14.4
1216 rpm :  100.0   82.5   58.8   46.3   31.9   20.6   16.3   16.3
1600 rpm :  100.0   82.5   60.0   43.1   31.3   23.8   18.1   18.1
2208 rpm :  100.0   82.5   63.1   43.8   34.4   27.5   20.0   20.0
3008 rpm :  101.9   85.6   65.6   53.1   41.3   33.1   21.3   21.3
4192 rpm :  110.0   91.3   71.3   61.3   50.0   38.1   28.8   25.6
5600 rpm :  117.5   98.1   76.9   68.1   59.4   43.1   35.6   32.5
```

### ✅ Vérification — test transition tiède (~50°C)

| Test | ✅ OK | ⚠️ Si raté |
|---|---|---|
| Accélération 30%, moteur à ~50°C | Lisse, aucun trou | Trou → `ip_ti_tco_pos_fast_wf_opm_1` ligne TCO **+10%** |
| Accélération 80% | Réponse nette, sans latence | À-coup riche → `ip_ti_tco_pos_slow_wf_opm_1` **−5%** |

### Stratégie film mural — calibré E85 pour la sécurité en open loop transitoire

Le tip-in (enfoncement rapide de la pédale) est une zone de **boucle ouverte transitoire** : la sonde lambda ne réagit pas assez vite pour corriger. Un film sous-compensé donne un mélange lean transitoire — raté ou claquement d'admission. Un film sur-compensé donne un bref enrichissement — sans conséquence.

> **Choix retenu : calibrer le film mural pour E85 (×1.25)**, même si le carburant réel est E70. Avec du E70 réel, la compensation sera légèrement excessive → tip-in légèrement riche → safe. C'est la même logique que pour le facteur injecteur : mieux vaut être riche dans les zones où rien ne corrige.

Sur E85, le film mural est physiquement plus épais (environ +25% à +35% par rapport à l'essence) et plus lent à se dissiper. Les modifications à appliquer :

1. **Approche retenue** : multiplier les tables `ip_ti_tco_pos_slow_wf_opm_1`, `ip_ti_tco_pos_slow_wf_opm_2`, `ip_ti_tco_pos_fast_wf_opm_1` et `ip_ti_tco_pos_fast_wf_opm_2` par **×1.25** globalement. Ce facteur couvre l'E85 pur avec une légère sur-compensation sécuritaire pour l'E70 réel.

2. **Ajustement fin** : si des « trous » d'accélération persistent à température intermédiaire (30-60°C), augmenter sélectivement les colonnes correspondantes de +10% supplémentaires.

3. **Tables `_neg_*_wf`** : non modifiées en première intention — elles gèrent la récupération de film en tip-out (levée de pied) et sont statiques dans le bin. La sous-correction produit un mélange légèrement riche en décélération (sans risque), contrairement aux tables pos où la sous-correction produirait un lean transitoire. À corriger en deuxième étape (×1.25 identique aux pos), une fois les tables pos validées sur route.

4. **Valvetronic — particularité N52** : les tables `ip_fac_ti_maf_sp_wf_*_opm_1` et `ip_ti_cor_tps_mod_wf` gèrent le film mural induit par les changements de levée Valvetronic. Sur E85, elles peuvent nécessiter un +15% à +20% pour compenser le film plus épais lors des transitions Valvetronic rapides. À régler seulement si les tests pratiques montrent des ratés lors d'accélérations progressives.

### Test pratique « transition tiède »

La zone critique reste **30-60°C** (moteur juste monté en température) :

```
Test :
1. Démarrez moteur froid
2. Conduisez 10 min à vitesse modérée (moteur monte à ~50°C)
3. À feu rouge (moteur ~50°C, ralenti) : accélérez progressivement 30%
   → Doit être LISSE, aucun "trou" ni hésitation
4. Accélérez 80% :
   → Réponse nette, pas de temps de latence

Résultat :
  Trou/hésitation → Film insuffisant → +10% sur ip_ti_tco_pos_fast_wf_opm_1 ligne correspondante
  À-coup riche → Film trop fort → −5% sur ip_ti_tco_pos_slow_wf_opm_1 ligne correspondante
```

### ⚠️ Zone d'incertitude honnête

Le modèle de film mural du MSV70 combine plusieurs paramètres qui interagissent (film lent, film rapide, correction charge, correction température, corrections Valvetronic). **Il n'existe pas de « réglage E85 prêt à l'emploi » pour le film mural N52 publié ouvertement**. Les recommandations ci-dessus (multiplicateur ×1.25 global) sont un point de départ conservatif basé sur les différences physiques essence/E85, pas sur un retour d'expérience validé sur N52. En pratique, l'enrichissement cranking (§2) + le facteur d'échelle injecteur (§1) + la boucle fermée lambda suffisent pour une conversion E85 de base ; le film mural est une optimisation secondaire.

---


---

## Récapitulatif — Valeurs Avant / Après

### ⑦ ip_ti_tco_pos_slow_wf_opm_1 — Film lent Valvetronic @ 0x4CBFC (8×8, TCO × RPM)

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   36.0   36.0   32.0   26.0   18.0    8.0    4.0    2.0
 896 rpm :   38.0   38.0   34.0   28.0   20.0   10.0    6.0    2.0
1216 rpm :   40.0   40.0   36.0   30.0   22.0   12.0    8.0    4.0
1600 rpm :   42.0   42.0   38.0   32.0   24.0   14.0   10.0    6.0
2208 rpm :   46.0   46.0   42.0   36.0   28.0   18.0   14.0   10.0
3008 rpm :   50.0   50.0   46.0   40.0   32.0   22.0   18.0   14.0
4192 rpm :   54.0   54.0   50.0   44.0   36.0   26.0   22.0   18.0
5600 rpm :   58.0   58.0   54.0   48.0   40.0   30.0   26.0   22.0

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   45.0   45.0   40.0   32.5   22.5   10.0    5.0    2.5
 896 rpm :   47.5   47.5   42.5   35.0   25.0   12.5    7.5    2.5
1216 rpm :   50.0   50.0   45.0   37.5   27.5   15.0   10.0    5.0
1600 rpm :   52.5   52.5   47.5   40.0   30.0   17.5   12.5    7.5
2208 rpm :   57.5   57.5   52.5   45.0   35.0   22.5   17.5   12.5
3008 rpm :   62.5   62.5   57.5   50.0   40.0   27.5   22.5   17.5
4192 rpm :   67.5   67.5   62.5   55.0   45.0   32.5   27.5   22.5
5600 rpm :   72.5   72.5   67.5   60.0   50.0   37.5   32.5   27.5
```

---

### ⑧ ip_ti_tco_pos_slow_wf_opm_2 — Film lent papillonné @ 0x4CC7C (8×8)

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  224.7  180.1  134.1  114.0   90.0   26.0   20.0   20.0
 896 rpm :  224.7  180.1  132.7  113.5   88.5   28.0   20.0   20.0
1216 rpm :  224.7  180.1  132.9  107.0   81.0   28.0   20.0   20.0
1600 rpm :  224.7  180.1  133.5   76.5   57.1   30.0   20.0   18.0
2208 rpm :  224.7  180.3  134.6   76.7   57.6   32.0   22.0   18.0
3008 rpm :  224.7  185.4  144.7  105.0   78.5   36.0   26.0   26.0
4192 rpm :  234.9  194.5  155.0  127.7   96.6   52.3   37.8   33.6
5600 rpm :  259.4  215.5  172.3  142.7  112.5   69.8   52.9   50.7

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  280.9  225.1  167.6  142.5  112.5   32.5   25.0   25.0
 896 rpm :  280.9  225.1  165.9  141.9  110.6   35.0   25.0   25.0
1216 rpm :  280.9  225.1  166.1  133.8  101.3   35.0   25.0   25.0
1600 rpm :  280.9  225.1  166.9   95.6   71.4   37.5   25.0   22.5
2208 rpm :  280.9  225.4  168.3   95.9   72.0   40.0   27.5   22.5
3008 rpm :  280.9  231.8  180.9  131.3   98.1   45.0   32.5   32.5
4192 rpm :  293.6  243.1  193.8  159.6  120.8   65.4   47.3   42.0
5600 rpm :  324.3  269.4  215.4  178.4  140.6   87.3   66.1   63.4
```

---

### ⑨ ip_ti_tco_pos_fast_wf_opm_1 — Film rapide Valvetronic @ 0x443FC (8×8)

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   22.0   20.5   18.5   15.0   11.0    7.5    6.0    4.5
 896 rpm :   23.5   21.5   19.5   16.0   12.0    8.0    6.5    5.0
1216 rpm :   25.5   23.0   21.0   17.5   13.5    9.0    7.0    5.5
1600 rpm :   28.0   25.5   23.0   19.5   15.5   10.5    8.0    6.5
2208 rpm :   32.5   29.5   26.5   23.0   18.5   13.0   10.5    9.0
3008 rpm :   39.0   35.5   32.0   28.0   23.0   17.0   14.0   12.5
4192 rpm :   52.5   47.5   43.0   38.0   32.0   25.0   21.5   19.5
5600 rpm :   72.5   66.5   60.0   53.5   45.5   37.0   33.0   31.0

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   27.5   25.6   23.1   18.8   13.8    9.4    7.5    5.6
 896 rpm :   29.4   26.9   24.4   20.0   15.0   10.0    8.1    6.3
1216 rpm :   31.9   28.8   26.3   21.9   16.9   11.3    8.8    6.9
1600 rpm :   35.0   31.9   28.8   24.4   19.4   13.1   10.0    8.1
2208 rpm :   40.6   36.9   33.1   28.8   23.1   16.3   13.1   11.3
3008 rpm :   48.8   44.4   40.0   35.0   28.8   21.3   17.5   15.6
4192 rpm :   65.6   59.4   53.8   47.5   40.0   31.3   26.9   24.4
5600 rpm :   90.6   83.1   75.0   66.9   56.9   46.3   41.3   38.8
```

---

### ⑩ ip_ti_tco_pos_fast_wf_opm_2 — Film rapide papillonné @ 0x4443C (8×8)

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   80.0   66.0   46.5   42.0   24.5   12.0   10.0   10.0
 896 rpm :   80.0   66.0   46.0   41.0   26.0   14.5   11.5   11.5
1216 rpm :   80.0   66.0   47.0   37.0   25.5   16.5   13.0   13.0
1600 rpm :   80.0   66.0   48.0   34.5   25.0   19.0   14.5   14.5
2208 rpm :   80.0   66.0   50.5   35.0   27.5   22.0   16.0   16.0
3008 rpm :   81.5   68.5   52.5   42.5   33.0   26.5   17.0   17.0
4192 rpm :   88.0   73.0   57.0   49.0   40.0   30.5   23.0   20.5
5600 rpm :   94.0   78.5   61.5   54.5   47.5   34.5   28.5   26.0

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  100.0   82.5   58.1   52.5   30.6   15.0   12.5   12.5
 896 rpm :  100.0   82.5   57.5   51.3   32.5   18.1   14.4   14.4
1216 rpm :  100.0   82.5   58.8   46.3   31.9   20.6   16.3   16.3
1600 rpm :  100.0   82.5   60.0   43.1   31.3   23.8   18.1   18.1
2208 rpm :  100.0   82.5   63.1   43.8   34.4   27.5   20.0   20.0
3008 rpm :  101.9   85.6   65.6   53.1   41.3   33.1   21.3   21.3
4192 rpm :  110.0   91.3   71.3   61.3   50.0   38.1   28.8   25.6
5600 rpm :  117.5   98.1   76.9   68.1   59.4   43.1   35.6   32.5
```

---

