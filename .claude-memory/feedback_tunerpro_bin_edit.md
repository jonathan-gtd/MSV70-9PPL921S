---
name: C'est l'utilisateur qui modifie le bin avec TunerPro
description: Ne jamais écrire directement dans les fichiers .bin — l'utilisateur fait toutes les modifications manuellement via TunerPro. Claude fournit les valeurs, l'utilisateur les saisit.
type: feedback
originSessionId: 280859e2-a26f-4202-8be2-c633978c168d
---
Ne jamais écrire dans les fichiers `.bin` directement (ni via MCP write_param, ni via Python, ni via tout autre outil).

**Why:** L'utilisateur utilise TunerPro pour modifier les bins. C'est lui qui saisit les valeurs manuellement. Claude doit fournir les valeurs exactes à entrer, pas les écrire lui-même.

**How to apply:** Pour toute modification de bin, donner à l'utilisateur le tableau des valeurs physiques (ou raw si nécessaire) à saisir dans TunerPro. Ne pas ajouter de write_param ou autre outil d'écriture dans le MCP à cette fin.
