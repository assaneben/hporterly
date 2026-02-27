import json
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from datetime import datetime

ROOT = Path.cwd()
HANDLERS_DIR = ROOT / 'backend' / 'src' / 'handlers'
SERVICES_DIR = ROOT / 'backend' / 'src' / 'services'
PACK_DIR = ROOT / 'DOSSIER_REPRISE_DEV'
INV_DIR = PACK_DIR / 'inventaires'
INV_DIR.mkdir(parents=True, exist_ok=True)

PUBLIC_EXACT_PATHS = {
    '/api/auth/login',
    '/api/v1/auth/login',
    '/api/health',
    '/api/ready',
    '/api/version',
    '/api/v1',
    '/api/v1/health',
    '/api/v1/ready',
    '/api/v1/version',
}
CORE_ROLES = ['demandeur', 'brancardier', 'administrateur', 'regulateur', 'moderateur', 'admin']

FN_RE = re.compile(
    r'(?ms)(?P<attrs>(?:^[ \t]*#\[[\s\S]*?\][ \t]*\r?\n)*)^[ \t]*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\(',
)
IMPL_RE = re.compile(r'(?m)^\s*impl\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{')
DIRECT_ROUTE_RE = re.compile(r'#\[\s*(get|post|put|patch|delete)\s*\(\s*"([^"]+)"\s*\)\s*\]', re.I | re.S)
ROUTE_ATTR_RE = re.compile(r'#\[\s*route\s*\((.*?)\)\s*\]', re.S)
STRING_RE = re.compile(r'"([^"]+)"')
REQUIRE_ROLE_RE = re.compile(r'require_role\s*\([^,]+,\s*&\[(.*?)\]\s*\)', re.S)
REQUIRE_ADMIN_RE = re.compile(r'\brequire_admin\s*\(')
REQUIRE_PORTER_RE = re.compile(r'\brequire_porter\s*\(')
SERVICE_CALL_RE = re.compile(r'([A-Z][A-Za-z0-9_]+Service)::([A-Za-z0-9_]+)\s*\(')
ROLE_CMP_RE = re.compile(r'\b(?:actor|user)\.role(?:\.as_str\(\))?\s*(==|!=)\s*"([a-z_]+)"')
STRICT_GUARD_RE = re.compile(r'\bif\s+([^\{\n]+?)\{', re.S)


def line_no(text: str, idx: int) -> int:
    return text.count('\n', 0, idx) + 1


def find_matching_brace(text: str, open_idx: int) -> int:
    assert text[open_idx] == '{'
    i = open_idx
    depth = 0
    n = len(text)
    in_str = False
    in_char = False
    in_line_comment = False
    in_block_comment = 0
    raw_hashes = None
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ''

        if in_line_comment:
            if ch == '\n':
                in_line_comment = False
            i += 1
            continue

        if in_block_comment:
            if ch == '/' and nxt == '*':
                in_block_comment += 1
                i += 2
                continue
            if ch == '*' and nxt == '/':
                in_block_comment -= 1
                i += 2
                continue
            i += 1
            continue

        if raw_hashes is not None:
            if ch == '"':
                j = i + 1
                hashes = 0
                while j < n and text[j] == '#':
                    hashes += 1
                    j += 1
                if hashes == raw_hashes:
                    raw_hashes = None
                    i = j
                    continue
            i += 1
            continue

        if in_str:
            if ch == '\\':
                i += 2
                continue
            if ch == '"':
                in_str = False
            i += 1
            continue

        if in_char:
            if ch == '\\':
                i += 2
                continue
            if ch == "'":
                in_char = False
            i += 1
            continue

        # normal state
        if ch == '/' and nxt == '/':
            in_line_comment = True
            i += 2
            continue
        if ch == '/' and nxt == '*':
            in_block_comment = 1
            i += 2
            continue
        if ch == 'r' and nxt == '"':
            raw_hashes = 0
            i += 2
            continue
        if ch == 'r' and nxt == '#':
            j = i + 1
            hashes = 0
            while j < n and text[j] == '#':
                hashes += 1
                j += 1
            if j < n and text[j] == '"':
                raw_hashes = hashes
                i = j + 1
                continue
        if ch == '"':
            in_str = True
            i += 1
            continue
        if ch == "'":
            in_char = True
            i += 1
            continue

        if ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError('No matching brace found')


@dataclass
class FunctionDef:
    name: str
    attrs: str
    body: str
    start_line: int
    file: Path


def parse_functions_in_text(text: str, file: Path, offset: int = 0, end_limit: Optional[int] = None) -> List[FunctionDef]:
    funcs: List[FunctionDef] = []
    for m in FN_RE.finditer(text):
        abs_start = offset + m.start()
        if end_limit is not None and abs_start >= end_limit:
            break
        attrs = m.group('attrs') or ''
        name = m.group('name')
        sig_search_start = m.end()
        brace_idx_local = text.find('{', sig_search_start)
        if brace_idx_local == -1:
            continue
        abs_brace = offset + brace_idx_local
        if end_limit is not None and abs_brace >= end_limit:
            continue
        try:
            abs_end = find_matching_brace((ROOT / file).read_text(encoding='utf-8') if False else (offset and None), 0)
        except Exception:
            pass
        # Match brace on full text for robust indexing
        # Determine original full text and local indexes externally is simpler: here `text` is exact parse slice
        try:
            brace_end_local = find_matching_brace(text, brace_idx_local)
        except Exception:
            continue
        body = text[brace_idx_local:brace_end_local + 1]
        funcs.append(FunctionDef(name=name, attrs=attrs, body=body, start_line=line_no(text, m.start()), file=file))
    return funcs


def parse_all_functions(file: Path) -> List[FunctionDef]:
    text = file.read_text(encoding='utf-8')
    funcs: List[FunctionDef] = []
    for m in FN_RE.finditer(text):
        name = m.group('name')
        attrs = m.group('attrs') or ''
        brace_idx = text.find('{', m.end())
        if brace_idx == -1:
            continue
        try:
            brace_end = find_matching_brace(text, brace_idx)
        except Exception:
            continue
        body = text[brace_idx:brace_end + 1]
        funcs.append(FunctionDef(name=name, attrs=attrs, body=body, start_line=line_no(text, m.start()), file=file))
    return funcs


def parse_routes_from_attrs(attrs: str) -> List[Tuple[str, str]]:
    routes: List[Tuple[str, str]] = []
    for m in DIRECT_ROUTE_RE.finditer(attrs):
        routes.append((m.group(1).upper(), m.group(2)))
    for m in ROUTE_ATTR_RE.finditer(attrs):
        body = m.group(1)
        path_match = STRING_RE.search(body)
        methods = re.findall(r'method\s*=\s*"([A-Z]+)"', body)
        if path_match and methods:
            path = path_match.group(1)
            for method in methods:
                routes.append((method.upper(), path))
    return routes


def extract_string_roles(segment: str) -> List[str]:
    return [s for s in re.findall(r'"([a-z_]+)"', segment) if re.fullmatch(r'[a-z_]+', s)]


def extract_explicit_allow_roles(body: str, helper_roles: Optional[Dict[str, List[str]]] = None) -> List[str]:
    roles = set()
    for m in REQUIRE_ROLE_RE.finditer(body):
        roles.update(extract_string_roles(m.group(1)))
    if REQUIRE_ADMIN_RE.search(body):
        roles.add('administrateur')
    if REQUIRE_PORTER_RE.search(body):
        roles.add('brancardier')
    if helper_roles:
        for helper_name, helper_allowed in helper_roles.items():
            if helper_allowed and re.search(rf'\b{re.escape(helper_name)}\s*\(', body):
                roles.update(helper_allowed)
    return sorted(roles)


def extract_service_guard_notes(body: str) -> Tuple[List[str], List[str], Optional[str]]:
    notes: List[str] = []
    mentions = sorted({role for _, role in ROLE_CMP_RE.findall(body)})
    explicit_roles = extract_explicit_allow_roles(body)

    if explicit_roles:
        notes.append('require_role/require_admin detecte')

    if 'ApiError::Forbidden' in body or 'ApiError::Unauthorized' in body or 'not authorized' in body.lower() or 'non autoris' in body.lower() or 'acces reserve' in body.lower():
        for expr_m in STRICT_GUARD_RE.finditer(body):
            expr = ' '.join(expr_m.group(1).split())
            if 'role' not in expr:
                continue
            if 'role != "administrateur" &&' in expr:
                notes.append('guard detecte: administrateur OU proprietaire (self)')
            elif 'role != "brancardier"' in expr:
                notes.append('guard detecte: brancardier uniquement')
            elif 'role != "administrateur"' in expr:
                notes.append('guard detecte: administrateur uniquement')
            elif 'role == "demandeur"' in expr:
                notes.append('guard detecte: demandeur interdit')

    inferred = None
    joined = ' | '.join(notes)
    if explicit_roles:
        inferred = 'allowlist:' + ','.join(explicit_roles)
    elif 'administrateur OU proprietaire (self)' in joined:
        inferred = 'self_or_admin'
    elif any('brancardier uniquement' in n for n in notes):
        inferred = 'allowlist:brancardier'
    elif any('administrateur uniquement' in n for n in notes):
        inferred = 'allowlist:administrateur'

    # dedupe notes preserving order
    seen = set()
    dedup_notes = []
    for n in notes:
        if n not in seen:
            dedup_notes.append(n)
            seen.add(n)
    return explicit_roles, dedup_notes, inferred


def parse_services() -> Dict[str, Dict[str, dict]]:
    service_map: Dict[str, Dict[str, dict]] = {}
    for file in SERVICES_DIR.rglob('*.rs'):
        text = file.read_text(encoding='utf-8')
        for impl_m in IMPL_RE.finditer(text):
            service_name = impl_m.group(1)
            impl_brace = text.find('{', impl_m.end() - 1)
            if impl_brace == -1:
                continue
            try:
                impl_end = find_matching_brace(text, impl_brace)
            except Exception:
                continue
            impl_body = text[impl_brace + 1:impl_end]
            methods = {}
            for fn in parse_functions_in_text(impl_body, file, offset=impl_brace + 1):
                explicit_roles, guard_notes, inferred = extract_service_guard_notes(fn.body)
                methods[fn.name] = {
                    'file': str(fn.file.relative_to(ROOT)).replace('\\', '/'),
                    'line': fn.start_line + line_no(text[:impl_brace + 1], len(text[:impl_brace + 1])) - 1,
                    'explicit_allow_roles': explicit_roles,
                    'role_mentions': sorted({r for _, r in ROLE_CMP_RE.findall(fn.body)}),
                    'guard_notes': guard_notes,
                    'inferred_policy': inferred,
                }
            if methods:
                service_map.setdefault(service_name, {}).update(methods)
    return service_map


def classify_public(path: str) -> str:
    if not path.startswith('/api'):
        return 'public'
    if path in PUBLIC_EXACT_PATHS:
        return 'public'
    if path.startswith('/api/v1/{') or path.startswith('/api/v1/'):
        return 'partial'  # compat redirect route; auth skip depends on concrete v1 path
    return 'auth'


def confidence_label(source: str) -> str:
    return {'handler': 'elevee', 'handler_helper': 'elevee', 'service': 'moyenne', 'none': 'faible'}.get(source, 'faible')


def build_endpoint_records(service_map: Dict[str, Dict[str, dict]]) -> List[dict]:
    endpoints = []
    for file in HANDLERS_DIR.rglob('*.rs'):
        funcs = parse_all_functions(file)
        helper_roles = {}
        for fn in funcs:
            if parse_routes_from_attrs(fn.attrs):
                continue
            roles = extract_explicit_allow_roles(fn.body)
            if roles:
                helper_roles[fn.name] = roles
        for fn in funcs:
            routes = parse_routes_from_attrs(fn.attrs)
            if not routes:
                continue
            handler_roles_direct = extract_explicit_allow_roles(fn.body)
            helper_used_roles = []
            if not handler_roles_direct:
                helper_used_roles = extract_explicit_allow_roles(fn.body, helper_roles=helper_roles)
            service_calls = []
            for svc, method in SERVICE_CALL_RE.findall(fn.body):
                service_calls.append((svc, method))
            # keep order unique
            seen_calls = set()
            uniq_calls = []
            for c in service_calls:
                if c not in seen_calls:
                    seen_calls.add(c)
                    uniq_calls.append(c)
            primary_service = None
            for c in uniq_calls:
                if c[0] != 'AuditService':
                    primary_service = c
                    break
            if primary_service is None and uniq_calls:
                primary_service = uniq_calls[0]

            service_clue = None
            if primary_service:
                svc, meth = primary_service
                service_clue = service_map.get(svc, {}).get(meth)

            for method, path in routes:
                pub_class = classify_public(path)
                inferred_roles = []
                source = 'none'
                special_policy = None
                if handler_roles_direct:
                    inferred_roles = sorted(set(handler_roles_direct))
                    source = 'handler'
                elif helper_used_roles:
                    inferred_roles = sorted(set(helper_used_roles))
                    source = 'handler_helper'
                elif service_clue:
                    if service_clue.get('explicit_allow_roles'):
                        inferred_roles = sorted(set(service_clue['explicit_allow_roles']))
                        source = 'service'
                    elif (service_clue.get('inferred_policy') or '').startswith('allowlist:'):
                        inferred_roles = sorted(
                            set((service_clue.get('inferred_policy') or '').split(':', 1)[1].split(','))
                        )
                        source = 'service'
                    elif (service_clue.get('inferred_policy') or '') == 'self_or_admin':
                        special_policy = 'self_or_admin'
                        source = 'service'

                endpoints.append({
                    'method': method,
                    'path': path,
                    'handler_fn': fn.name,
                    'handler_file': str(file.relative_to(ROOT)).replace('\\', '/'),
                    'handler_line': fn.start_line,
                    'auth_class': pub_class,
                    'handler_explicit_roles': handler_roles_direct,
                    'handler_helper_roles': helper_used_roles if helper_used_roles and not handler_roles_direct else [],
                    'service_calls': [{'service': s, 'method': m} for s, m in uniq_calls],
                    'primary_service': {'service': primary_service[0], 'method': primary_service[1]} if primary_service else None,
                    'service_role_clue': service_clue,
                    'inferred_allow_roles': inferred_roles,
                    'special_policy': special_policy,
                    'inference_source': source,
                    'confidence': confidence_label(source),
                })
    endpoints.sort(key=lambda e: (e['path'], e['method']))
    return endpoints


def md_escape(s: str) -> str:
    return s.replace('|', '\\|')


def endpoint_summary_md(endpoints: List[dict]) -> str:
    total = len(endpoints)
    public_count = sum(1 for e in endpoints if e['auth_class'] == 'public')
    partial_count = sum(1 for e in endpoints if e['auth_class'] == 'partial')
    auth_count = sum(1 for e in endpoints if e['auth_class'] == 'auth')
    explicit_count = sum(1 for e in endpoints if e['inferred_allow_roles'])
    lines = []
    lines.append('## Auto-extraction (routes + roles)')
    lines.append('')
    lines.append(f'_Genere automatiquement le {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}_')
    lines.append('')
    lines.append('### Resume')
    lines.append(f'- Endpoints detectes: **{total}**')
    lines.append(f'- Publics (exact middleware bypass): **{public_count}**')
    lines.append(f'- Compat v1 (publics partiels selon chemin): **{partial_count}**')
    lines.append(f'- Authentifies (`AuthMiddleware`): **{auth_count}**')
    lines.append(f'- Endpoints avec roles explicitement inferes: **{explicit_count}**')
    lines.append('')
    lines.append('### Tableau endpoints (pre-remplissage automatique)')
    lines.append('')
    lines.append('| Methode | Path | Auth | Roles infers | Source inference | Handler | Service principal | Notes |')
    lines.append('|---|---|---|---|---|---|---|---|')
    for e in endpoints:
        roles = ', '.join(e['inferred_allow_roles']) if e['inferred_allow_roles'] else ('self+admin' if e['special_policy'] == 'self_or_admin' else '')
        auth_label = {'public': 'Public', 'partial': 'Partiel (v1)', 'auth': 'Auth'}.get(e['auth_class'], e['auth_class'])
        handler_ref = f"{e['handler_fn']} (`{e['handler_file']}:{e['handler_line']}`)"
        svc = e['primary_service']
        svc_label = f"{svc['service']}::{svc['method']}" if svc else ''
        notes = []
        if e['handler_explicit_roles']:
            notes.append('check role dans handler')
        elif e['handler_helper_roles']:
            notes.append('check role via helper handler')
        if e['service_role_clue']:
            if e['service_role_clue'].get('guard_notes'):
                notes.extend(e['service_role_clue']['guard_notes'])
        if e['auth_class'] == 'auth' and not roles:
            notes.append('restriction fine a confirmer (service/logique metier)')
        if e['auth_class'] == 'partial':
            notes.append('redirect compat /api/v1 -> /api')
        lines.append('| ' + ' | '.join([
            e['method'],
            md_escape(e['path']),
            auth_label,
            md_escape(roles or '-'),
            md_escape(f"{e['inference_source']} ({e['confidence']})"),
            md_escape(handler_ref),
            md_escape(svc_label or '-'),
            md_escape('; '.join(dict.fromkeys(notes)) if notes else '-'),
        ]) + ' |')
    lines.append('')
    lines.append('### Notes')
    lines.append('- `Public` = chemin explicitement exempté par `backend/src/middleware/auth.rs`.')
    lines.append('- `Partiel (v1)` = route de compatibilite `/api/v1/...`; le middleware bypass seulement certains chemins v1 (login/health/ready/version/root).')
    lines.append('- `Roles infers` = pre-remplissage automatique, a valider manuellement pour les cas metier complexes.')
    return '\n'.join(lines)


def rbac_matrix_md(endpoints: List[dict]) -> str:
    lines = []
    lines.append('## Auto-extraction initiale de la matrice RBAC')
    lines.append('')
    lines.append('Pre-remplissage base sur les routes Actix et les controles de roles detectes dans les handlers/services.')
    lines.append('')
    lines.append('| Methode | Path | Public | demandeur | brancardier | administrateur | regulateur | moderateur | admin | Confiance | Source | Note |')
    lines.append('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for e in endpoints:
        values = {r: '?' for r in CORE_ROLES}
        public_val = 'Non'
        note = ''
        if e['auth_class'] == 'public':
            public_val = 'Oui'
            values = {r: '-' for r in CORE_ROLES}
            note = 'Endpoint public (middleware bypass)'
        elif e['auth_class'] == 'partial':
            public_val = 'Partiel'
            values = {r: '-' for r in CORE_ROLES}
            note = 'Compat /api/v1: auth selon chemin concret'
        elif e['inferred_allow_roles']:
            allowed = set(e['inferred_allow_roles'])
            for r in CORE_ROLES:
                values[r] = 'Oui' if r in allowed else 'Non'
            note = 'Allowlist inferee automatiquement'
        elif e['special_policy'] == 'self_or_admin':
            values['administrateur'] = 'Oui'
            # keep others uncertain because rule depends on ownership
            note = 'Administrateur ou utilisateur proprietaire (self)'
        else:
            note = 'Auth requis; restriction fine non detectee automatiquement'

        src = e['inference_source']
        if src == 'none' and e['auth_class'] in ('public', 'partial'):
            src = 'middleware'

        row = [
            e['method'],
            md_escape(e['path']),
            public_val,
            values['demandeur'],
            values['brancardier'],
            values['administrateur'],
            values['regulateur'],
            values['moderateur'],
            values['admin'],
            e['confidence'] if e['inference_source'] != 'none' else ('moyenne' if e['auth_class'] in ('public', 'partial') else 'faible'),
            md_escape(src),
            md_escape(note),
        ]
        lines.append('| ' + ' | '.join(row) + ' |')

    lines.append('')
    lines.append('### Interpretation')
    lines.append('- `Oui/Non` = detection automatique d une allowlist explicite (handler/service).')
    lines.append('- `?` = endpoint authentifie mais restriction role non deduite automatiquement (verification manuelle requise).')
    lines.append('- `Partiel` = route de compatibilite `/api/v1/...` avec bypass middleware limite a certains chemins.')
    lines.append('- Cette matrice est une **base initiale** pour completer `07_MATRICE_RBAC.md`.')
    return '\n'.join(lines)


def upsert_auto_section(doc_path: Path, generated_md: str) -> None:
    start_marker = '<!-- AUTO-GENERATED:START -->'
    end_marker = '<!-- AUTO-GENERATED:END -->'
    existing = doc_path.read_text(encoding='utf-8') if doc_path.exists() else ''
    auto_block = f"{start_marker}\n{generated_md}\n{end_marker}\n"
    if start_marker in existing and end_marker in existing:
        new_text = re.sub(
            re.escape(start_marker) + r'[\s\S]*?' + re.escape(end_marker),
            auto_block.strip(),
            existing,
            count=1,
        )
        if not new_text.endswith('\n'):
            new_text += '\n'
    else:
        new_text = auto_block + '\n---\n\n' + existing
    doc_path.write_text(new_text, encoding='utf-8')


def main():
    service_map = parse_services()
    endpoints = build_endpoint_records(service_map)

    (INV_DIR / 'ENDPOINTS_RBAC_AUTO.json').write_text(
        json.dumps({'generated_at': datetime.now().isoformat(), 'endpoints': endpoints}, ensure_ascii=False, indent=2),
        encoding='utf-8'
    )
    (INV_DIR / 'ENDPOINTS_EXTRAITS.md').write_text(endpoint_summary_md(endpoints) + '\n', encoding='utf-8')
    (INV_DIR / 'RBAC_INITIALE_AUTO.md').write_text(rbac_matrix_md(endpoints) + '\n', encoding='utf-8')

    upsert_auto_section(PACK_DIR / '08_CONTRATS_API_WEBSOCKET.md', endpoint_summary_md(endpoints))
    upsert_auto_section(PACK_DIR / '07_MATRICE_RBAC.md', rbac_matrix_md(endpoints))

    print(f'ENDPOINTS={len(endpoints)}')
    print(f'FILES_WRITTEN=4+2docs')


if __name__ == '__main__':
    main()
