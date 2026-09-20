"""Vérifie le catalogue de codage sans modifier de fichier ni lire les DATEN installées."""

import csv
import re
import sys
from collections import Counter
from pathlib import Path
from urllib.parse import unquote


def main():
    daten = Path(__file__).resolve().parent
    coding = daten.parent
    errors = []

    def check(condition, message):
        if not condition:
            errors.append(message)

    with (daten / 'fsw_par_module.csv').open(encoding='utf-8-sig', newline='') as stream:
        rows = list(csv.DictReader(stream))
    keys = [(r['module'], r['fsw']) for r in rows]
    check(len(keys) == len(set(keys)), 'Doublons module/FSW dans le CSV')
    check(len(rows) == 1738, 'Le périmètre attendu contient 1 738 fonctions')
    modules = sorted({r['module'] for r in rows})
    check(len(modules) == 19, 'Le périmètre attendu contient 19 modules')
    index = (daten / 'README.md').read_text(encoding='utf-8')
    stock = (coding / 'stock/README.MD').read_text(encoding='utf-8')

    for row in rows:
        label = f"{row['module']}/{row['fsw']}"
        check(row['source'] in ('', 'traduit', 'maison', 'deduction_libelle'), f'{label}: source inconnue')
        check(bool(row['source']) == bool(row['explication_fr']), f'{label}: source/explication incohérentes')
        check(not row['signification_en'] or bool(row['explication_fr']), f'{label}: traduction FR manquante')
        if row['source'] == 'deduction_libelle':
            check(row['explication_fr'].startswith('Déduction du libellé : '), f'{label}: déduction non signalée')
            check(not row['signification_en'], f'{label}: déduction présentée comme description anglaise')

    for module in modules:
        expected = {r['fsw']: r for r in rows if r['module'] == module}
        path = daten / f'{module}.md'
        check(path.exists(), f'{module}: fiche absente')
        if not path.exists():
            continue
        content = path.read_text(encoding='utf-8')
        variants = {r['variante'] for r in expected.values()}
        check(len(variants) == 1, f'{module}: variantes multiples')
        variant = next(iter(variants))
        check(f'| Variante | **`{variant}`** |' in content, f'{module}: variante de fiche incohérente')
        check(f'| `{module}.TRC` | `{variant}` |' in stock, f'{module}: variante de sauvegarde incohérente')
        check((coding / 'stock' / f'{module}.TRC').exists(), f'{module}: sauvegarde absente')
        found = []
        for line in content.splitlines():
            match = re.match(r'^\| `([^`]+)` \|', line)
            if not match:
                continue
            fsw = match[1]
            found.append(fsw)
            check(fsw in expected, f'{module}/{fsw}: FSW absent du CSV')
            if fsw not in expected:
                continue
            cells = line[2:-2].split(' | ')
            check(len(cells) == 5, f'{module}/{fsw}: nombre de colonnes incorrect')
            if len(cells) != 5:
                continue
            row = expected[fsw]
            for cell, field in [(cells[1], 'traduction'), (cells[3], 'signification_en'), (cells[4], 'explication_fr')]:
                check(cell == (row[field] or '—'), f'{module}/{fsw}: {field} diffère du CSV')
            values = row['valeurs'].split('|') if row['valeurs'] else []
            displayed_values = values[:8] if len(values) > 10 else values
            displayed = ' · '.join(f'`{v}`' for v in displayed_values) or '—'
            if len(values) > 10:
                displayed += f' … *(+{len(values) - 8})*'
            check(cells[2] == displayed, f'{module}/{fsw}: valeurs différentes du CSV')
        check(Counter(found) == Counter(expected.keys()), f'{module}: fonctions manquantes ou dupliquées')
        counts = {
            'Avec valeurs': sum(bool(r['valeurs']) for r in expected.values()),
            'Signification (EN)': sum(bool(r['signification_en']) for r in expected.values()),
            'Explication (FR)': sum(bool(r['explication_fr']) for r in expected.values()),
        }
        for label, count in counts.items():
            check(f'| {label} | {count} |' in content, f'{module}: compteur {label} incorrect')
        check(f'| Fonctions retenues | **{len(expected)}** |' in content, f'{module}: compteur de fonctions incorrect')
        index_row = re.search(rf'^\| \[{module}\]\({module}\.md\) \| (.+)$', index, re.M)
        check(index_row is not None, f'{module}: absent de l’index')
        if index_row:
            cells = index_row[1].split(' | ')
            check(cells[1:4] == [str(len(expected)), str(counts['Avec valeurs']), str(counts['Explication (FR)'])], f'{module}: compteurs de l’index incorrects')

    french = sum(bool(r['explication_fr']) for r in rows)
    english = sum(bool(r['signification_en']) for r in rows)
    deductions = sum(r['source'] == 'deduction_libelle' for r in rows)
    formatted = lambda value: f'{value:,}'.replace(',', ' ')
    check(f'**{formatted(english)} avec une signification en anglais**' in index, 'Compteur EN global incorrect')
    check(f'**{formatted(french)} avec une explication française**' in index, 'Compteur FR global incorrect')
    check(f'**{deductions} déductions du libellé**' in index, 'Compteur global de déductions incorrect')
    check(f'**{len(rows) - french} fonctions sans explication**' in index, 'Compteur global de lignes sans explication incorrect')
    glossary = (daten / 'LEXIQUE_FRAGMENTS.md').read_text(encoding='utf-8')
    for section, label in [('Fragments traduits', 'fragments traduits ou contextualisés'), ('Lectures probables', 'lectures probables')]:
        section_text = glossary.split(f'## {section}\n', 1)[1].split('\n## ', 1)[0]
        count = len(re.findall(r'^\| `', section_text, re.M))
        check(f'**{formatted(count)} {label}**' in glossary, f'Lexique : compteur {section} incorrect')

    for path in coding.rglob('*'):
        if path.suffix.lower() not in ('.md', '.csv', '.trc'):
            continue
        check('DWAY' not in path.name, f'Ancien nom de fichier : {path.name}')
        if path.suffix.lower() == '.trc':
            continue
        text = path.read_text(encoding='utf-8-sig')
        check('DWAY' not in text, f'{path.name}: ancienne référence DWAY')
        if path.suffix.lower() != '.md':
            continue
        for target in re.findall(r'\]\(([^)]+)\)', text):
            if '://' in target or target.startswith('#'):
                continue
            local = unquote(target.split('#', 1)[0])
            check((path.parent / local).exists(), f'{path.name}: lien local absent : {target}')

    if errors:
        print('\n'.join(errors))
        return 1
    print(f'OK : {len(modules)} modules, {len(rows)} fonctions, {french} explications FR dont {deductions} déductions ; CSV, fiches, compteurs et liens cohérents.')
    return 0


if __name__ == '__main__':
    sys.exit(main())
