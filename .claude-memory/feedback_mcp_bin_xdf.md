---
name: Toujours utiliser le MCP pour lire bin/XDF
description: Ne jamais utiliser PowerShell ou Bash pour lire des fichiers bin ou XDF — utiliser uniquement le MCP msv70-xdf-analyzer. Si le MCP ne couvre pas un besoin, l'étendre.
type: feedback
originSessionId: 280859e2-a26f-4202-8be2-c633978c168d
---
Ne jamais utiliser PowerShell ou Bash pour lire des fichiers `.bin` ou `.xdf`.

Toujours passer par le MCP `mcp__msv70-xdf-analyzer__*` pour toute lecture de données ECU.

**Why:** PowerShell introduit des bugs silencieux (ex: overflow uint8 sur -shl, endianness, adressage) qui donnent des valeurs fausses sans erreur visible. Le MCP gère correctement ces cas.

**How to apply:** Si le MCP ne supporte pas un besoin (ex: lecture des axes, comparaison de bins hors des clés connues), modifier/étendre le serveur MCP plutôt que de contourner avec PowerShell/Bash.

Ne jamais créer de scripts Python standalone (.py) pour des opérations sur les bins. Tout doit passer par le MCP — étendre le MCP si besoin. Règle confirmée explicitement par l'utilisateur le 05/05/2026.
