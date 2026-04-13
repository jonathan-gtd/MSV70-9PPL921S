# §22 — Avertissements et Maintenance

### A. Surveillance Permanente des Fuel Trims

| Indicateur | Normal | Acceptable | PROBLÈME |
|---|---|---|---|
| STFT | ±5% | ±10% | > ±15% |
| LTFT | ±5% | ±10% | > ±15% |

```
STFT > +15% = Le calculateur rajoute du carburant en permanence
  → ip_mff_cor_opm trop petit, ou lambda trop pauvre

STFT < −15% = Le calculateur enlève du carburant
  → ip_mff_cor_opm trop grand, ou lambda trop riche

LTFT élevé en permanence = Votre calibration a une dérive systématique
  → Ajustez ip_mff_cor_opm (trop petit si LTFT positif, trop grand si négatif)
```

**Scanner recommandé pour N52 :** ISTA/D via interface ENET ou K+DCAN (accès à tous les paramètres BMW).

### B. Pompe à Essence

Le N52 consomme environ 60 L/h en fonctionnement normal. Avec E70 (+36% de masse carburant), il faut ~82 L/h effectivement consommés.

> **Pourquoi le test ci-dessous donne 240 L/h alors que le moteur n'en consomme que 82 ?** Le test est réalisé circuit ouvert (retour carburant déconnecté, sans contre-pression du rail) : le débit mesuré est le débit max de la pompe à basse pression, bien supérieur au débit réel sous pression rail. Le seuil de 2.0 L/30 sec garantit une marge suffisante une fois le rail sous pression nominale (5 bar) en charge élevée.

**Test pompe :**
```
Déconnectez le retour carburant et laissez s'accumuler 30 sec :
  Minimum acceptable : 2.0 L/30 sec (= 240 L/h) → largement suffisant
  Si < 1.5 L/30 sec → Pompe fatiguée, à remplacer avant conversion
```

**Si remplacement nécessaire :** Pompe N54 ou pompe aftermarket (Walbro, Bosch haute pression)

### C. Filtre à Essence

L'E85 est un excellent solvant : il dissout tous les dépôts accumulés dans le réservoir depuis des années.

**Protocole filtre :**
1. Changez le filtre AVANT la conversion
2. Changez-le à nouveau après 200 km d'E85 (dépôts dissous)
3. Contrôlez à 500 km
4. Retour au rythme normal (10 000 km) ensuite

### D. Compatibilité Matériaux Système Carburant N52

| Composant | Compatibilité E85 | Action |
|---|---|---|
| Joints Viton/FKM | Excellente | Aucune |
| Joints NBR (nitrile standard) | Mauvaise | Remplacement nécessaire |
| Tuyaux caoutchouc E85-compatible | Bonne | Vérifier l'état |
| Pompe à essence N52 | Bonne (prévu alcool) | Aucune si état correct |
| Rail d'injection acier/alu | Excellente | Aucune |

### E. Bougies d'Allumage

**Pour N52 + E85 :**
```
Référence stock : NGK ILZKBR7A8DG ou DENSO FK20HR11 (iridium)
Gap stock : 0.75–0.80 mm
Gap E85 recommandé : 0.65–0.70 mm

Raison : L'E85 forme un mélange plus dense ; un gap plus serré
         améliore la fiabilité d'allumage à froid

Intervalle de remplacement : 20 000 km (vs 30 000 km essence)
```

### F. Démarrage Hivernal (< 0°C)

```
À −5°C : Démarrage difficile, assurez-vous que vos enrichissements cranking
         sont à ×1.80 minimum pour cette zone de température

À −10°C : Très difficile, considérez l'ajout de 10–15% d'essence 95
          dans le réservoir pour faciliter le démarrage

À −15°C et moins : E85 pur pratiquement impossible
                   → Repassez à l'essence ou utilisez un mélange E50
```

**Astuce hiver :** Si votre configuration E85 est définitive, gardez toujours un bidon de 5L d'essence 95 pour l'hiver.

---

