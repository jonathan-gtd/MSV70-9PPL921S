# §24 — Mythes et Réalités sur l'Éthanol

L'E85 traîne une réputation de carburant "agressif" ou "destructeur" souvent alimentée par des expériences sur des véhicules anciens, des mélanges mal dosés, ou des conversions bâclées. Voici un état des lieux basé sur la physique et la chimie, pas sur les forums.

---

### "L'éthanol assèche et fissure les durites / joints"

**Verdict : FAUX sur les véhicules modernes.**

Cette rumeur vient d'une réalité des années 1980–2000 : les tuyaux et joints en **caoutchouc NBR** (nitrile standard) se dégradent effectivement au contact de l'éthanol — gonflement, ramollissement, fissuration à long terme. Sur les véhicules de cette époque, la conversion E85 sans remplacement des flexibles était problématique.

**Sur le N52B30 (2005–2012)**, toutes les pièces d'origine BMW en contact avec le carburant sont en **Viton (FKM)** ou en métal anodisé — matériaux parfaitement compatibles E85. La norme européenne EN 228 impose d'ailleurs la compatibilité E10 depuis 2010, et les fabricants ont migré vers le Viton bien avant.

**Le vrai risque** : les réparations antérieures effectuées avec des pièces génériques pas chères (flexibles NBR ou joints nitrile aftermarket). Une inspection visuelle avant conversion règle le problème.

---

### "L'éthanol corrode l'aluminium et l'acier"

**Verdict : FAUX dans des conditions normales.**

L'éthanol **pur anhydre** (sans eau) est en réalité moins corrosif que l'essence sur l'aluminium. La corrosion apparaît dans deux cas précis :

1. **Éthanol hydraté** (avec eau libre) + aluminium non anodisé + longue stagnation → oxydation possible. Ce cas ne se produit pas dans un circuit carburant fonctionnel et bien entretenu.
2. **Alliages de magnésium en contact direct avec le carburant** (corps de carburateur magnésium sur véhicules des années 70–80, certaines pompes mécaniques anciennes) → réaction avec l'éthanol. Sans objet sur N52 : la pompe à carburant est en acier/aluminium anodisé, et le bloc moteur N52 (qui contient bien du magnésium dans sa structure composite Mg/Al) n'est **jamais en contact avec le carburant** — il est isolé par le circuit de refroidissement et la lubrification.

Le rail d'injection, les tuyaux métalliques et le réservoir acier du N52 ne sont pas concernés.

---

### "L'éthanol consomme les injecteurs"

**Verdict : FAUX pour les injecteurs modernes.**

Les injecteurs Bosch EV14 (stock N52, référence 13537531634) ont une bague d'étanchéité en **Viton et un corps en acier inox**. Ils sont conçus pour fonctionner avec des carburants contenant de l'alcool — Bosch les spécifie comme compatibles E100.

La durée de vie des injecteurs sur E85 est **identique** à celle sur essence si la calibration est correcte (pas de fonctionnement prolongé en mélange pauvre).

---

### "L'éthanol bouffe le catalyseur"

**Verdict : FAUX — l'éthanol est plus propre que l'essence.**

L'éthanol brûle plus complètement que l'essence (moins d'imbrûlés HC, moins de CO à lambda correct). Les DTC catalyseur (P0420/P0430) qui apparaissent lors d'une conversion E85 sont dus à la recalibration des adaptations lambda, pas à une destruction chimique. Ils disparaissent après 200–500 km une fois les LTFT stabilisés.

L'éthanol ne contient **pas de soufre** ni de composés aromatiques lourds (benzène, toluène) qui empoisonnent les catalyseurs à long terme. Sur le long terme, l'E85 est moins agressif pour le catalyseur que le SP95.

---

### "Avec l'E85 on consomme deux fois plus"

**Verdict : EXAGÉRÉ — c'est +30 à +40% en volume, pas ×2.**

L'éthanol a un pouvoir calorifique inférieur (PCI) d'environ 26 MJ/kg contre 44 MJ/kg pour l'essence. **Mais l'AFR stœchiométrique est aussi inférieur** (≈9.8:1 vs 14.7:1), donc le moteur brûle moins d'air par kg de carburant. Les deux effets se compensent partiellement.

En pratique, sur un N52 bien calibré :

| Usage | Surconsommation volumique E85 vs SP95 |
|---|---|
| Ville / ralenti | +30 à +35% |
| Route mixte | +28 à +33% |
| Autoroute | +25 à +30% |

Avec un prix E85 autour de 0.80–0.95 €/L en France (vs ~1.80 €/L pour le SP95), le **coût au kilomètre reste inférieur** même avec la surconsommation.

---

### "L'éthanol absorbe l'eau et provoque des problèmes"

**Verdict : VRAI mais sans conséquence pratique dans un usage normal.**

L'éthanol est effectivement hygroscopique — il absorbe l'humidité atmosphérique. Dans un circuit carburant étanche et rempli régulièrement, la quantité d'eau absorbée est négligeable. Le problème théorique (séparation de phase eau/éthanol) ne survient qu'avec des mélanges E10–E15 à faible teneur, pas avec l'E85 pur.

**Précaution réelle** : ne pas laisser le réservoir quasi-vide stationner plusieurs semaines en hiver humide. Remplir à ≥ ¼ de réservoir si le véhicule est immobilisé.

---

### "Il faut un capteur flex-fuel pour rouler à l'E85"

**Verdict : NON — un capteur est une option, pas une obligation.**

Un capteur flex-fuel mesure en temps réel le titre en éthanol du carburant et adapte la calibration automatiquement. C'est pratique sur un véhicule qui alterne régulièrement E85 et SP95. Mais :

- Sur une **conversion fixe E85** (uniquement E85), le titre éthanol varie peu (60–85% selon la saison) et les LTFT absorbent l'écart (plage réelle : −8%/+12% sur MSV70).
- Une **calibration fixe E70** est un bon compromis qui couvre l'E65–E75 sans capteur.
- Les **STFT/LTFT via scanner OBD** remplacent avantageusement le capteur pour une utilisation avisée.

---

### Ce qui est VRAI et réel

| Risque réel | Gravité | Solution |
|---|---|---|
| Démarrage difficile < 0°C | Moyen | Enrichissement cranking calibré, bougie resserrée, batterie neuve |
| Filtre à essence bouché à 200 km (dépôts anciens dissous) | Faible | Changer le filtre avant ET après conversion |
| Casse moteur si calibration pauvre en WOT prolongé | Élevé | Valider STFT/LTFT avant toute accélération franche |
| Réparations antérieures en caoutchouc NBR | Moyen | Inspection visuelle + remplacement si nécessaire |
| Perte autonomie (~30%) | Faible (coût compensé) | Accepter ou prévoir des arrêts plus fréquents |
| Instabilité catalyseur les 500 premiers km | Faible | DTC temporaires, effacer après stabilisation |

---

