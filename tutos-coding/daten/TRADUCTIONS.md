# Traductions : méthode et suivi

## État au 20 septembre 2026

Le catalogue contient **1 738 lignes**, dont **1 365 avec une description anglaise**
et **1 479 avec une explication française**. Parmi ces dernières, **113 sont des
déductions du libellé**, explicitement signalées. **259 lignes restent sans explication**.

Cette passe corrige **94 décompositions** et ajoute **113 explications**, dans CAPPL,
CAS, FRM, FZD, IHK, JBBF, KOMBI et PDC. Les noms FSW, variantes, adresses, valeurs PSW
et descriptions anglaises sont conservés. Le module de protection volumétrique est
désormais nommé **DWAI**, avec sa variante déjà relevée `DWA_IRS.C05`.

## Ce que signifie la source du CSV

| `source` | Origine de l'explication française |
|---|---|
| `traduit` | Traduction de la description anglaise NCS Dummy — 1 252 lignes |
| `maison` | Explication française antérieure à cette passe — 114 lignes ; ce statut ne prouve pas une validation sur véhicule |
| `deduction_libelle` | Interprétation du nom FSW et du contexte du module — 113 lignes, préfixées « Déduction du libellé : » |
| vide | Aucune explication française retenue — 259 lignes |

La colonne anglaise conserve sa provenance NCS Dummy. Une déduction française ne
crée pas de traduction anglaise prétendument issue de cette base. Les notations
d'unités déjà adaptées en français dans certaines descriptions anglaises restent
un écart historique au « verbatim » annoncé ; elles ne sont pas modifiées ici.

## Corrections étayées par la base locale

Référence consultée : `C:\NCS Dummy\Translations.csv`, champ `LASTMODIFIED=20141115`.
Les descriptions peuvent être retrouvées à partir des clés ci-dessous, y compris
sur une autre installation disposant de cette même base.

| Fragment ou contexte | Correction | Clés de contrôle dans NCS Dummy |
|---|---|---|
| `EKS` dans les lève-vitres | Protection antipincement | `FENSTERHEBER_MIT_EKS`, `REVERSIERWEG_EKS` |
| `KLR`, `KLRAUS` | Borne accessoires R, coupure de cette borne | `KLR_AUS_TIMEOUT`, `SLEEPTIMER_FH_KLRAUS` |
| `QUIT`, `QUITT` | Acquittement, sans préjuger du mode sonore ou visuel | `QUIT_AKUST_ENTSCH`, `QUIT_OPT_ENTSCH` |
| `OPT` dans les alarmes/acquittements | Optique, donc visuel | `ALARM_OPT_FERNLICHT`, `QUIT_OPT_SCHAERF` |
| `PIA`, `PIAU` | Gestion des profils, mémoire non personnalisée pour l'alarme | `PIA`, `PIAU_AKUST_ENTSCH` |
| `CBS_INTD`, `CBS_INTW` | Intervalle temporel, intervalle de distance | `CBS_INTD_03H`, `CBS_INTW_11H` |
| `VERST` pour les sièges | Réglage de position | `CA107_AUT_VERST_ENTR` |
| `MIT`, `VO`, `HI` dans les distances PDC | Centre, avant, arrière | `ABSTAND_ECKE_MIT_VO`, `ABSTAND_MIT_MIT_HI` |
| `WANDLER` dans le PDC | Transducteur/capteur ultrasonique | `ANZAHL_WANDLER` |

Les autres corrections sont linguistiques et contextuelles : `WO` signifie
« without » dans `CLIMATE_WO_PREVIEW`, `TREN` reprend « Trend » dans la famille ALC,
et `MEM_POS_ANFAHREN` désigne le déplacement vers une position mémorisée.
L'expansion d'`IRNE` en « infrarouge » a été retirée faute de justification.

## Limites des ajouts sémantiques

Les descriptions préfixées « Déduction du libellé » expriment seulement ce que le nom
et son contexte permettent de lire. Elles n'établissent ni une unité, ni un seuil,
ni une correspondance `wert_*`, ni un comportement vérifié sur la voiture.
Les indices `_0`, `_1`, `_2` ne sont pas attribués arbitrairement à des clés.
Les identifiants CBS inconnus restent des identifiants, sans inventer l'opération
d'entretien. La présence d'une fonction dans les DATEN ne prouve pas l'installation
de l'équipement correspondant.

Les familles opaques `AMS_*`, `SC_FLAG_*`, `PA_*` restent à documenter. Certaines
lectures probables historiques peuvent encore être erronées selon le contexte :
elles ne doivent pas servir seules à décider d'un codage.

## Vérification et suite du travail

Depuis la racine du dépôt :

```console
python tutos-coding/daten/verifier_catalogue.py
```

Le contrôle compare les fiches Markdown au CSV, les compteurs et les liens locaux.
Il ne valide pas l'extraction binaire ni les effets d'un réglage. Le script
d'extraction historique des DATEN n'est pas présent dans le dépôt : sa conservation
avec la version et les empreintes des sources reste nécessaire pour rendre
l'ensemble de la construction du catalogue reproductible.

Pour les formats de traces et le filtrage des variantes, référence :
[manuel de l'auteur de NCS Dummy, sections 3.2 et 3.6](https://revtor.be/ncsdummy/ncsdummy.pdf).
