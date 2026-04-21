# TUTO VMAX : Modifier la Vitesse Maximale — Siemens MSV70 / N52B30

> **Véhicule ciblé :** BMW E90/E91/E92/E93 — Moteur N52B30 — Calculateur Siemens MSV70  
> **Fichier de base :** VB67774_921S_Full.bin  
> **Version :** 1.0 — Données réelles extraites du bin — 2026-04-08

---

## 📌 Contexte — Le Gentlemen's Agreement BMW

À partir de 1992, les constructeurs allemands (BMW, Mercedes, Audi, Volkswagen) ont signé un accord volontaire limitant leurs véhicules à **250 km/h** électroniquement. Ce bridage n'est pas mécanique — c'est le calculateur qui coupe l'injection (et parfois l'allumage) quand la vitesse dépasse le seuil programmé.

Sur le MSV70, ce seuil est stocké dans **`c_vs_max_2` = 250** (1 LSB = 1 km/h).

Le N52B30 330i peut atteindre ~260–270 km/h mécaniquement, selon la boîte et la transmission. Le suppression du bridage permet d'exploiter cette marge.

---

## ⚙️ Paramètres concernés

### Paramètres principaux à modifier

| Paramètre | Valeur stock | Description |
|---|---|---|
| **`c_vs_max_2`** | **250 km/h** | **Seuil principal — Gentlemen's Agreement** |
| `c_vs_max_1` | 235 km/h | Seuil secondaire condition 1 |
| `c_vs_max_3` | 243 km/h | Seuil secondaire condition 3 |
| `c_vs_max_0_dft` | 206 km/h | Seuil par défaut (mode dégradé) |
| `c_vs_max_hys` | 4 km/h | Hystérésis (ne pas modifier) |

### Paramètres informatifs (ne pas modifier)

| Paramètre | Valeur stock | Description |
|---|---|---|
| `c_vs_max_amt` | 80 km/h | Protection changement de rapport AMT |
| `c_vs_thd_n_max_h_mt` | 254 km/h | Seuil vitesse pour limite RPM haute (MT) |
| `c_vs_thd_n_max_h_at` | 254 km/h | Seuil vitesse pour limite RPM haute (AT) |

> `c_vs_thd_n_max_h_mt/at` à 254 km/h signifie que la limite RPM haute est pratiquement toujours active (le véhicule ne peut pas dépasser 254 km/h avec le bridage stock). Si tu supprimes le bridage VMAX et que le véhicule peut dépasser 254 km/h, la limite RPM haute s'activera — ce qui est le comportement souhaité.

---

## 🔧 Comment modifier dans TunerPro RT

### Méthode simple — Modifier uniquement c_vs_max_2

Pour la grande majorité des cas (débridage VMAX, ou abaissement pour circuit/usage commercial) :

1. Ouvrir TunerPro RT → charger XDF + BIN
2. Rechercher `c_vs_max_2` dans l'arbre des paramètres
3. Double-clic → valeur scalaire affichée en km/h
4. Modifier la valeur → sauvegarder
5. Corriger le checksum : `XDF → Checksum → Fix`
6. Flasher

> **Valeur 255** = limite pratiquement supprimée pour un uint8 (max physique de la variable). Pour une suppression complète, utiliser 255 ou chercher le flag `lc_inh_fcut_n_max`.

### Méthode complète — Cohérence tous les seuils

Si tu veux une cohérence parfaite entre tous les seuils (éviter que le seuil "condition 1" à 235 km/h ne se déclenche avant le seuil principal) :

| Paramètre | Stock | Débridage complet | Abaissement 230 km/h |
|---|---|---|---|
| `c_vs_max_0_dft` | 206 | 255 | 230 |
| `c_vs_max_1` | 235 | 255 | 230 |
| `c_vs_max_2` | 250 | 255 | 230 |
| `c_vs_max_3` | 243 | 255 | 230 |
| `c_vs_max_hys` | 4 | 4 | 4 |

> **Ne jamais modifier `c_vs_max_hys`** — l'hystérésis empêche le bridage de cycler rapidement autour du seuil (oscillations injection on/off). 4 km/h est la valeur correcte.

---

## 📐 Scénarios de modification

### Scénario 1 — Débridage complet (Autobahn, circuit fermé)

Mettre tous les seuils à 255 km/h. La limite physique du véhicule (environ 260–270 km/h selon rapport/pneumatiques) devient la seule contrainte.

```
c_vs_max_0_dft = 255
c_vs_max_1     = 255
c_vs_max_2     = 255
c_vs_max_3     = 255
c_vs_max_hys   = 4      ← ne pas toucher
```

### Scénario 2 — Abaissement pour usage commercial / véhicule de société

Limiter à 180 km/h (exemples : flotte d'entreprise, jeune conducteur) :

```
c_vs_max_0_dft = 180
c_vs_max_1     = 180
c_vs_max_2     = 180
c_vs_max_3     = 180
```

### Scénario 3 — Bridage piste à 200 km/h (trackday circuit)

Certains circuits imposent une limite vitesse. 200 km/h est une limite courante :

```
c_vs_max_0_dft = 200
c_vs_max_1     = 200
c_vs_max_2     = 200
c_vs_max_3     = 200
```

---

## 🔗 Interaction avec le limiteur RPM

Le bridage VMAX et le limiteur RPM sont **deux systèmes indépendants**. Supprimer le bridage VMAX ne supprime pas le limiteur RPM (6980 RPM MT stock). C'est le comportement voulu : même à vitesse libre, le moteur ne peut pas dépasser sa limite RPM.

Si tu veux aller au-delà de 260 km/h (par exemple avec une boîte longue ou une modification aérodynamique), il faut aussi adapter le limiteur RPM — consulter [TUTO RPM Protection.md](TUTO%20RPM%20Protection.md).

---

## ⚠️ Points de vigilance

| Risque | Précaution |
|---|---|
| Pneumatiques non homologués haute vitesse | Vérifier l'indice de vitesse (V=240, W=270, Y=300 km/h) avant toute modification |
| Freins insuffisants pour la VMAX réelle | Évaluer les freins avant un débridage — les freins stock sont dimensionnés pour 250 km/h |
| Légalité | La suppression du bridage peut être contraire au CT ou à la légalité selon le pays et l'usage |
| `c_vs_max_2` < `c_vs_max_0_dft` | Si le seuil default est plus élevé que le seuil principal, le bridage est inefficace dans certaines conditions — toujours mettre tous les seuils à la même valeur |
| Oublier de corriger le checksum | Flash refusé au démarrage — `Checksum → Fix` dans TunerPro avant flash |

---

## 🔗 Liens

- [TUTO E85.md](TUTO%20E85.md) — Conversion éthanol complète
- [TUTO RPM Protection.md](TUTO%20RPM%20Protection.md) — Modifier les limiteurs de régime
- [README.md](README.md) — Documentation générale MSV70
