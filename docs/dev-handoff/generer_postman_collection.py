import json
import uuid
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT_DIR = ROOT / "DOSSIER_REPRISE_DEV" / "api-tests"
OUT_DIR.mkdir(parents=True, exist_ok=True)


def tests_status(codes, save_lines=None):
    lines = [
        "pm.test('Status code accepted', function () {",
        f"  pm.expect({codes}).to.include(pm.response.code);",
        "});",
    ]
    if save_lines:
        lines.extend(save_lines)
    return [{"listen": "test", "script": {"type": "text/javascript", "exec": lines}}]


def req(name, method, url, token=None, body=None, codes=None, save=None):
    headers = []
    if token:
        headers.append({"key": "Authorization", "value": f"Bearer {{{{{token}}}}}", "type": "text"})
    if body is not None:
        headers.append({"key": "Content-Type", "value": "application/json", "type": "text"})
    request = {"method": method, "header": headers, "url": "{{baseUrl}}" + url}
    if body is not None:
        request["body"] = {
            "mode": "raw",
            "raw": json.dumps(body, ensure_ascii=False, indent=2),
            "options": {"raw": {"language": "json"}},
        }
    item = {"name": name, "request": request}
    if codes:
        item["event"] = tests_status(codes, save)
    return item


def folder(name, items):
    return {"name": name, "item": items}


def save_login(var_name, user_id_var=None, porter_id_var=None):
    lines = [
        "let d = {}; try { d = pm.response.json(); } catch (e) {}",
        f"if (d.token) {{ pm.environment.set('{var_name}', d.token); pm.environment.set('token', d.token); }}",
    ]
    if user_id_var:
        lines.append(f"if (d.user && d.user.id) pm.environment.set('{user_id_var}', d.user.id);")
    if porter_id_var:
        lines.append(f"if (d.user && d.user.porter_id) pm.environment.set('{porter_id_var}', d.user.porter_id);")
    return lines


def save_id(var_name, expr):
    return [
        "let d = null; try { d = pm.response.json(); } catch (e) {}",
        f"try {{ const v = {expr}; if (v) pm.environment.set('{var_name}', v); }} catch (e) {{}}",
    ]


def build_collection():
    items = []

    items.append(folder("00 Health", [
        req("GET /api/health", "GET", "/api/health", codes=[200]),
        req("GET /api/ready", "GET", "/api/ready", codes=[200, 503]),
        req("GET /api/version", "GET", "/api/version", codes=[200]),
    ]))

    items.append(folder("01 Auth", [
        req("POST /api/auth/login (demandeur)", "POST", "/api/auth/login", body={
            "username": "{{requester_username}}", "password": "{{requester_password}}"
        }, codes=[200, 401], save=save_login("requester_token", "requester_user_id")),
        req("POST /api/auth/login (brancardier)", "POST", "/api/auth/login", body={
            "username": "{{porter_username}}", "password": "{{porter_password}}"
        }, codes=[200, 401], save=save_login("porter_token", "porter_user_id", "assigned_porter_id")),
        req("POST /api/auth/login (admin)", "POST", "/api/auth/login", body={
            "username": "{{admin_username}}", "password": "{{admin_password}}"
        }, codes=[200, 401], save=save_login("admin_token", "admin_user_id")),
        req("GET /api/auth/me (admin)", "GET", "/api/auth/me", token="admin_token", codes=[200, 401]),
        req("POST /api/auth/logout", "POST", "/api/auth/logout", token="admin_token", codes=[200, 401]),
    ]))

    items.append(folder("02 Core Lookup", [
        req("GET /api/services", "GET", "/api/services", token="requester_token", codes=[200, 401]),
        req("GET /api/patients?search", "GET", "/api/patients?search={{patient_search}}&limit=10", token="requester_token", codes=[200, 401]),
        req("GET /api/porters", "GET", "/api/porters", token="admin_token", codes=[200, 401]),
        req("GET /api/porters/available", "GET", "/api/porters/available", token="admin_token", codes=[200, 401]),
    ]))

    items.append(folder("03 Tickets", [
        req("GET /api/tickets", "GET", "/api/tickets?limit=50", token="admin_token", codes=[200, 401]),
        req("POST /api/tickets", "POST", "/api/tickets", token="requester_token", body={
            "patient_id": "IPP-100001",
            "patient_name": "Alice Martin",
            "origin": "Unit A",
            "destination": "Imaging",
            "priority": 3,
            "mode": "brancard",
            "notes": "DEMO DATA - SYNTHETIC / NOT REAL",
            "transport_type": "PATIENT",
            "transport_subtype": "TP-BRANC",
        }, codes=[201, 400, 401], save=save_id("ticket_id", "d && d.id")),
        req("GET /api/tickets/{id}", "GET", "/api/tickets/{{ticket_id}}", token="requester_token", codes=[200, 401, 403, 404]),
        req("GET recommendations", "GET", "/api/tickets/{{ticket_id}}/recommendations", token="admin_token", codes=[200, 400, 401, 404]),
        req("POST assign", "POST", "/api/tickets/{{ticket_id}}/assign", token="admin_token", body={"porter_id": "{{assigned_porter_id}}"}, codes=[200, 400, 401, 403, 404, 409]),
        req("PATCH status -> in_progress", "PATCH", "/api/tickets/{{ticket_id}}/status", token="admin_token", body={"status": "in_progress", "comment": "Demarrage test"}, codes=[200, 400, 401, 403, 404, 409]),
        req("PATCH notes", "PATCH", "/api/tickets/{{ticket_id}}/notes", token="admin_token", body={"notes": "Note operationnelle fictive"}, codes=[200, 400, 401, 403, 404]),
        req("PATCH priority", "PATCH", "/api/tickets/{{ticket_id}}/priority", token="admin_token", body={"priority": 1, "reason": "Escalade operationnelle fictive"}, codes=[200, 400, 401, 403, 404]),
        req("PATCH equipment-status", "PATCH", "/api/tickets/{{ticket_id}}/equipment-status", token="admin_token", body={"delivered": True, "label_returned": False}, codes=[200, 400, 401, 403, 404]),
        req("GET available-porters-for-help", "GET", "/api/tickets/{{ticket_id}}/available-porters-for-help", token="admin_token", codes=[200, 401, 404]),
        req("POST request-help", "POST", "/api/tickets/{{ticket_id}}/request-help", token="porter_token", body={"requested_porter_id": "{{help_target_porter_id}}"}, codes=[200, 400, 401, 403, 404, 409], save=save_id("help_request_id", "d && d.help_request_id")),
        req("GET my help requests", "GET", "/api/porters/me/help-requests", token="porter_token", codes=[200, 401, 403]),
        req("POST respond help", "POST", "/api/help-requests/{{help_request_id}}/respond", token="porter_token", body={"accepted": True}, codes=[200, 400, 401, 403, 404, 409]),
        req("POST pause", "POST", "/api/tickets/{{ticket_id}}/pause", token="admin_token", codes=[200, 400, 401, 403, 404, 409]),
        req("POST cancel", "POST", "/api/tickets/{{ticket_id}}/cancel", token="admin_token", codes=[200, 400, 401, 403, 404, 409]),
        req("POST hard-delete", "POST", "/api/tickets/{{ticket_id}}/hard-delete", token="admin_token", body={"reason": "Suppression de donnee de demo"}, codes=[200, 400, 401, 403, 404, 409]),
    ]))

    items.append(folder("04 Admin Users & Porters", [
        req("GET /api/users", "GET", "/api/users", token="admin_token", codes=[200, 401, 403]),
        req("POST /api/users", "POST", "/api/users", token="admin_token", body={
            "username": "{{new_user_username}}",
            "password": "{{new_user_password}}",
            "role": "demandeur",
            "first_name": "Nora",
            "last_name": "Lefevre",
            "email": None,
            "service": "Unit A",
        }, codes=[201, 400, 401, 403], save=save_id("user_id", "d && d.id")),
        req("PUT /api/users/{id}", "PUT", "/api/users/{{user_id}}", token="admin_token", body={
            "first_name": "Nora", "last_name": "Martin", "service": "Unit B", "is_active": True
        }, codes=[200, 400, 401, 403, 404]),
        req("DELETE /api/users/{id}", "DELETE", "/api/users/{{user_id}}", token="admin_token", codes=[200, 401, 403, 404]),
        req("POST /api/porters", "POST", "/api/porters", token="admin_token", body={
            "username": "{{new_porter_username}}",
            "password": "{{new_porter_password}}",
            "first_name": "Alex",
            "last_name": "Rossi",
            "skills": ["O2", "LIT"],
        }, codes=[201, 400, 401, 403], save=save_id("porter_id", "d && d.id")),
        req("PATCH /api/porters/{id}/skills", "PATCH", "/api/porters/{{porter_id}}/skills", token="admin_token", body={"skills": ["O2", "LIT", "URG"]}, codes=[200, 400, 401, 403, 404]),
        req("PATCH /api/porters/{id}/status", "PATCH", "/api/porters/{{porter_id}}/status", token="admin_token", body={"status": "available", "location": "Unit A"}, codes=[200, 400, 401, 403, 404]),
    ]))

    items.append(folder("05 Notifications", [
        req("GET /api/notifications", "GET", "/api/notifications?limit=20", token="admin_token", codes=[200, 401], save=save_id("notification_id", "Array.isArray(d) && d.length ? d[0].id : null")),
        req("GET unread-count", "GET", "/api/notifications/unread-count", token="admin_token", codes=[200, 401]),
        req("POST mark-read", "POST", "/api/notifications/mark-read", token="admin_token", body={"notification_ids": ["{{notification_id}}"]}, codes=[200, 400, 401]),
        req("POST mark-all-read", "POST", "/api/notifications/mark-all-read", token="admin_token", codes=[200, 401]),
        req("GET preferences", "GET", "/api/notifications/preferences", token="admin_token", codes=[200, 401]),
        req("PATCH preferences", "PATCH", "/api/notifications/preferences", token="admin_token", body={
            "sound_enabled": True,
            "preferences": {
                "ticket_created": {"enabled": True, "sound": True},
                "ticket_assigned": {"enabled": True, "sound": True},
                "ticket_updated": {"enabled": True, "sound": False},
                "user_message": {"enabled": True, "sound": True},
            },
        }, codes=[200, 400, 401]),
        req("GET message-recipients", "GET", "/api/notifications/message-recipients", token="admin_token", codes=[200, 401]),
        req("POST send-message (admins)", "POST", "/api/notifications/send-message", token="admin_token", body={
            "message": "Message de test synthetique",
            "channel": "general",
            "target_type": "admins",
        }, codes=[200, 400, 401]),
        req("DELETE notification", "DELETE", "/api/notifications/{{notification_id}}", token="admin_token", codes=[200, 401, 404]),
    ]))

    ref_specs = [
        {
            "label": "Services",
            "path": "services",
            "id_var": "ref_service_id",
            "create_body": {"site_name": "Site Demo", "building_name": "BAT-A", "level_name": "Niveau 1", "zone_name": "Unit A", "subzone_name": "R-101"},
            "update_body": {"zone_name": "Unit A Updated", "subzone_name": "R-102", "is_active": True},
        },
        {
            "label": "Equipment",
            "path": "equipment",
            "id_var": "ref_equipment_id",
            "create_body": {
                "label": "Stretcher Standard",
                "sizes": ["S", "M", "L"],
                "required_fields": {"recipient": False, "priority": True, "origin": True, "destination": True, "note_free": False, "scheduled": False},
            },
            "update_body": {"label": "Stretcher Standard Updated", "sizes": ["M", "L"], "is_active": True},
        },
        {
            "label": "Transport Modes",
            "path": "transport-modes",
            "id_var": "ref_transport_mode_id",
            "create_body": {"label": "Mode Demo", "sort_order": 99},
            "update_body": {"label": "Mode Demo Updated", "sort_order": 100, "is_active": True},
        },
        {
            "label": "Specimens",
            "path": "specimens",
            "id_var": "ref_specimen_id",
            "create_body": {"label": "Specimen Demo"},
            "update_body": {"label": "Specimen Demo Updated", "is_active": True},
        },
    ]

    for idx, spec in enumerate(ref_specs, start=6):
        p = spec["path"]
        v = spec["id_var"]
        items.append(folder(f"{idx:02d} Referentials - {spec['label']}", [
            req(f"GET /api/referentials/{p}", "GET", f"/api/referentials/{p}", token="admin_token", codes=[200, 401, 403]),
            req(f"GET /api/referentials/{p}/active", "GET", f"/api/referentials/{p}/active", token="admin_token", codes=[200, 401]),
            req(f"POST /api/referentials/{p}", "POST", f"/api/referentials/{p}", token="admin_token", body=spec["create_body"], codes=[201, 400, 401, 403], save=save_id(v, "d && d.id")),
            req(f"PUT /api/referentials/{p}/{{id}}", "PUT", f"/api/referentials/{p}/{{{{{v}}}}}", token="admin_token", body=spec["update_body"], codes=[200, 400, 401, 403, 404]),
            req(f"DELETE /api/referentials/{p}/{{id}}", "DELETE", f"/api/referentials/{p}/{{{{{v}}}}}", token="admin_token", codes=[200, 401, 403, 404]),
            req(f"DELETE /api/referentials/{p}/{{id}}/hard", "DELETE", f"/api/referentials/{p}/{{{{{v}}}}}/hard", token="admin_token", codes=[200, 401, 403, 404]),
        ]))

    return {
        "info": {
            "_postman_id": str(uuid.uuid4()),
            "name": "HPorterly - API Tests (Local, Synthetic)",
            "description": "Collection Postman generee depuis les endpoints/payloads documentes (donnees synthetiques uniquement).",
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json",
        },
        "item": items,
        "variable": [{"key": "baseUrl", "value": "http://127.0.0.1:8080", "type": "string"}],
    }


def build_environment():
    vals = [
        ("baseUrl", "http://127.0.0.1:8080"),
        ("token", ""),
        ("requester_token", ""),
        ("porter_token", ""),
        ("admin_token", ""),
        ("requester_username", "marie.durand"),
        ("requester_password", "password123"),
        ("porter_username", "jean.martin"),
        ("porter_password", "password123"),
        ("admin_username", "admin"),
        ("admin_password", "password123"),
        ("patient_search", "ali"),
        ("ticket_id", ""),
        ("help_request_id", ""),
        ("assigned_porter_id", "jean.martin"),
        ("help_target_porter_id", "jean.martin"),
        ("notification_id", ""),
        ("user_id", ""),
        ("porter_id", ""),
        ("ref_service_id", ""),
        ("ref_equipment_id", ""),
        ("ref_transport_mode_id", ""),
        ("ref_specimen_id", ""),
        ("new_user_username", "demo.requester.pm"),
        ("new_user_password", "password123"),
        ("new_porter_username", "porter.demo.pm"),
        ("new_porter_password", "password123"),
    ]
    return {
        "id": str(uuid.uuid4()),
        "name": "HPorterly Local (Synthetic Demo)",
        "values": [{"key": k, "value": v, "type": "default", "enabled": True} for k, v in vals],
        "_postman_variable_scope": "environment",
        "_postman_exported_using": "Codex GPT-5",
    }


def main():
    collection = build_collection()
    environment = build_environment()
    col_path = OUT_DIR / "HPorterly_API_Local.postman_collection.json"
    env_path = OUT_DIR / "HPorterly_Local.postman_environment.json"
    readme_path = OUT_DIR / "README_API_TESTS.md"

    col_path.write_text(json.dumps(collection, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    env_path.write_text(json.dumps(environment, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    readme_path.write_text(
        "\n".join(
            [
                "# API Tests (Postman)",
                "",
                "Collection Postman locale generee a partir des endpoints/payloads documentes.",
                "",
                "Fichiers:",
                "- `HPorterly_API_Local.postman_collection.json`",
                "- `HPorterly_Local.postman_environment.json`",
                "- `run_api_tests_curl.sh`",
                "- `run_api_tests_curl.ps1`",
                "",
                "Utilisation:",
                "1. Importer la collection et l'environnement dans Postman.",
                "2. Selectionner l'environnement `HPorterly Local (Synthetic Demo)`.",
                "3. Lancer d'abord les requetes du dossier `01 Auth` (remplissage automatique des tokens).",
                "4. Executer ensuite les dossiers `Tickets`, `Notifications`, `Referentials` selon le besoin.",
                "",
                "Notes:",
                "- Donnees de test strictement synthetiques.",
                "- Certaines requetes dependent de preconditions (ticket cree, brancardier dispo, droits role).",
                "- Les tests Postman verifient les codes HTTP attendus (succes + erreurs metier possibles).",
                "- Les scripts `curl` sont generes depuis la meme collection et servent de base shell/PowerShell (sans capture automatique des tokens/IDs).",
                "",
            ]
        ),
        encoding="utf-8",
    )

    # Validation JSON
    json.loads(col_path.read_text(encoding="utf-8"))
    json.loads(env_path.read_text(encoding="utf-8"))

    print(f"COLLECTION={col_path}")
    print(f"ENV={env_path}")
    print(f"README={readme_path}")
    print(f"REQUEST_FOLDERS={len(collection['item'])}")


if __name__ == "__main__":
    main()
