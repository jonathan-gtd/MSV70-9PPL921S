# VMAX — Siemens MSV70 / N52B30

> **Véhicule :** BMW E90/E91/E92/E93 — N52B30 — Siemens MSV70  
> **Bin de référence :** VB67774_921S_Full.bin — SW 9PPL921S

---

## Contexte

Depuis 1992, BMW, Mercedes, Audi et Volkswagen brident électroniquement leurs véhicules à **250 km/h** via un accord volontaire (Gentlemen's Agreement). Ce bridage est logiciel : le calculateur coupe l'injection quand la vitesse mesurée dépasse le seuil programmé.

Sur le MSV70, ce seuil est contrôlé par **quatre paramètres** dont le principal est `c_vs_max_2` = 250 km/h (1 LSB = 1 km/h). Le N52B30 330i est mécaniquement capable d'atteindre ~260–270 km/h selon la boîte et les pneumatiques.

---

## Paramètres

### À modifier

| Paramètre | Stock | Description |
|---|---|---|
| `c_vs_max_2` | 250 km/h | Seuil principal — Gentlemen's Agreement |
| `c_vs_max_1` | 235 km/h | Seuil condition 1 |
| `c_vs_max_3` | 243 km/h | Seuil condition 3 |
| `c_vs_max_0_dft` | 206 km/h | Seuil mode dégradé |

Les quatre seuils doivent être mis à la **même valeur cible**. Si `c_vs_max_1` reste à 235 km/h alors que `c_vs_max_2` est à 255, le bridage "condition 1" se déclenche avant le seuil principal.

### À ne pas modifier

| Paramètre | Stock | Pourquoi |
|---|---|---|
| `c_vs_max_hys` | 4 km/h | Hystérésis — empêche l'oscillation injection on/off autour du seuil |
| `c_vs_max_amt` | 80 km/h | Protection changement de rapport AMT |
| `c_vs_thd_n_max_h_mt/at` | 254 km/h | Seuil d'activation de la limite RPM haute — cohérent avec le bridage stock |

---

## Procédure TunerPro RT

1. Charger XDF + BIN
2. Rechercher `c_vs_max_2` → valeur scalaire en km/h
3. Modifier
4. **Répercuter la même valeur** sur `c_vs_max_0_dft`, `c_vs_max_1`, `c_vs_max_3`
5. `Checksum → Fix`
6. Flasher

---

## Scénarios

### Débridage complet

```
c_vs_max_0_dft = 255
c_vs_max_1     = 255
c_vs_max_2     = 255
c_vs_max_3     = 255
c_vs_max_hys   = 4     ← ne pas toucher
```

255 est le maximum d'un uint8. La limite physique du véhicule (~260–270 km/h) devient la seule contrainte.

### Abaissement à 180 km/h (flotte, jeune conducteur)

```
c_vs_max_0_dft = 180
c_vs_max_1     = 180
c_vs_max_2     = 180
c_vs_max_3     = 180
```

### Bridage piste à 200 km/h

```
c_vs_max_0_dft = 200
c_vs_max_1     = 200
c_vs_max_2     = 200
c_vs_max_3     = 200
```

---

## Interaction avec le limiteur RPM

VMAX et limiteur RPM sont deux systèmes indépendants. Supprimer le bridage VMAX ne modifie pas le limiteur RPM (6980 RPM stock MT). Si le véhicule est mécaniquement capable de dépasser 260 km/h (boîte longue, modification aérodynamique), il faut aussi adapter le limiteur RPM — voir [TUTO RPM Protection](../rpm-protection/TUTO_RPM_Protection.md).

Note sur `c_vs_thd_n_max_h_mt/at` (254 km/h) : ce seuil active la limite RPM haute au-delà de 254 km/h. Avec le bridage stock à 250 km/h, ce seuil n'est jamais atteint. Après débridage, si le véhicule dépasse 254 km/h, la limite RPM haute s'active — ce qui est le comportement attendu.

---

## Points de vigilance

| Risque | Précaution |
|---|---|
| Pneumatiques non homologués haute vitesse | Vérifier l'indice (V=240, W=270, Y=300) avant de supprimer le bridage |
| Freins dimensionnés pour 250 km/h | Évaluer la capacité de freinage pour la VMAX réelle |
| `c_vs_max_0_dft` plus haut que `c_vs_max_2` | Le bridage mode dégradé peut dépasser le seuil principal |
| Légalité | Suppression du bridage peut invalider le CT selon le pays et l'usage |
| Oublier le checksum | Flash refusé — `Checksum → Fix` avant flash |

---

## Liens

- [TUTO RPM Protection](../rpm-protection/TUTO_RPM_Protection.md)
- [TUTO E85 — Sommaire](../E85/00_SOMMAIRE.md)
