import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
API_TESTS_DIR = ROOT / "DOSSIER_REPRISE_DEV" / "api-tests"
COLLECTION_PATH = API_TESTS_DIR / "HPorterly_API_Local.postman_collection.json"
ENV_PATH = API_TESTS_DIR / "HPorterly_Local.postman_environment.json"


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def flatten_items(items, folder_prefix=None):
    folder_prefix = folder_prefix or []
    flat = []
    for item in items:
        if "item" in item and "request" not in item:
            flat.extend(flatten_items(item["item"], folder_prefix + [item.get("name", "Folder")]))
        elif "request" in item:
            flat.append((folder_prefix, item))
    return flat


def request_url_raw(req):
    url = req.get("url")
    if isinstance(url, str):
        return url
    if isinstance(url, dict):
        if "raw" in url:
            return url["raw"]
        host = "".join(url.get("host", []))
        path = "/".join(url.get("path", []))
        return f"https://{host}/{path}"
    return ""


def get_env_values(env_doc):
    values = {}
    for item in env_doc.get("values", []):
        key = item.get("key")
        if key:
            values[key] = item.get("value", "")
    return values


VAR_PATTERN = re.compile(r"\{\{([A-Za-z0-9_]+)\}\}")


def to_bash_template(s: str) -> str:
    return VAR_PATTERN.sub(lambda m: "${" + m.group(1) + "}", s)


def to_ps_template_literal(s: str) -> str:
    # Keep Postman-style placeholders and resolve them at runtime in PowerShell.
    return s


def bash_quote_header(value: str) -> str:
    return "'" + value.replace("'", "'\"'\"'") + "'"


def bash_dquote(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def ps_single_quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def sanitize_func_name(folder_names, req_name, idx):
    raw = "_".join(folder_names + [req_name])
    raw = raw.lower()
    raw = re.sub(r"[^a-z0-9]+", "_", raw).strip("_")
    return f"r_{idx:03d}_{raw[:80]}"


def build_bash_script(collection_doc, env_doc):
    env_values = get_env_values(env_doc)
    flat = flatten_items(collection_doc.get("item", []))

    lines = []
    lines.append("#!/usr/bin/env bash")
    lines.append("set -u")
    lines.append("")
    lines.append("# API tests (curl) generated from Postman collection - synthetic demo only")
    lines.append("# Usage:")
    lines.append("#   1) Edit variables below if needed (tokens/IDs can also be exported in the shell).")
    lines.append("#   2) Run: bash ./run_api_tests_curl.sh")
    lines.append("# Notes:")
    lines.append("#   - This script does not auto-extract tokens/IDs from responses.")
    lines.append("#   - Prefer Postman collection for fully chained execution with variable capture.")
    lines.append("")

    for key, value in env_values.items():
        safe_value = str(value).replace("\\", "\\\\").replace('"', '\\"')
        lines.append(f'{key}="${{{key}:-{safe_value}}}"')

    lines.extend([
        "",
        'run_curl() {',
        '  local label="$1"; shift',
        '  echo',
        '  echo "================================================================"',
        '  echo "$label"',
        '  echo "----------------------------------------------------------------"',
        '  curl -sS -i "$@"',
        '  local rc=$?',
        '  echo',
        '  echo "[curl exit=$rc]"',
        '  return 0',
        '}',
        "",
        'tmp_json() {',
        '  local f',
        '  f="$(mktemp)"',
        '  cat > "$f"',
        '  echo "$f"',
        '}',
        "",
        'cleanup_file() {',
        '  local f="$1"',
        '  if [ -n "${f:-}" ] && [ -f "$f" ]; then rm -f "$f"; fi',
        '}',
        "",
    ])

    current_folder = None
    for idx, (folders, item) in enumerate(flat, start=1):
        req = item["request"]
        req_name = item.get("name", f"Request {idx}")
        method = req.get("method", "GET").upper()
        url = to_bash_template(request_url_raw(req))
        headers = req.get("header", []) or []
        body = None
        if isinstance(req.get("body"), dict) and req["body"].get("mode") == "raw":
            body = req["body"].get("raw", "")

        folder_label = " / ".join(folders) if folders else "Root"
        if folder_label != current_folder:
            current_folder = folder_label
            lines.append(f"# === {folder_label} ===")

        label = f"{folder_label} :: {req_name}"
        lines.append("")
        lines.append(f"# {idx:03d}. {req_name}")

        curl_args = [f'-X {method}', f'"{url}"']
        for h in headers:
            key = h.get("key", "")
            value = to_bash_template(h.get("value", ""))
            header_value = f"{key}: {value}"
            curl_args.append(f"-H {bash_dquote(header_value)}")

        if body is None:
            lines.append(f"run_curl {bash_quote_header(label)} {' '.join(curl_args)}")
        else:
            body_rendered = to_bash_template(body)
            lines.append("body_file=$(tmp_json <<JSON")
            lines.append(body_rendered)
            lines.append("JSON")
            lines.append(")")
            curl_args.append('--data-binary @"${body_file}"')
            lines.append(f"run_curl {bash_quote_header(label)} {' '.join(curl_args)}")
            lines.append("cleanup_file \"$body_file\"")

    return "\n".join(lines) + "\n"


def build_powershell_script(collection_doc, env_doc):
    env_values = get_env_values(env_doc)
    flat = flatten_items(collection_doc.get("item", []))

    lines = []
    lines.append("param(")
    lines.append("  [switch]$ShowOnly")
    lines.append(")")
    lines.append("")
    lines.append("$ErrorActionPreference = 'Stop'")
    lines.append("")
    lines.append("# API tests (curl.exe) generated from Postman collection - synthetic demo only")
    lines.append("# This script does not auto-extract tokens/IDs from responses.")
    lines.append("")

    lines.extend([
        "function Get-ValueOrDefault {",
        "  param([string]$Name, [string]$Default)",
        "  $v = [Environment]::GetEnvironmentVariable($Name)",
        "  if ([string]::IsNullOrEmpty($v)) { return $Default }",
        "  return $v",
        "}",
        "",
        "function Resolve-Template {",
        "  param([string]$Text)",
        "  if ($null -eq $Text) { return $Text }",
        "  return ([regex]::Replace($Text, '\\{\\{([A-Za-z0-9_]+)\\}\\}', {",
        "    param($m)",
        "    $name = $m.Groups[1].Value",
        "    if ($script:Vars.ContainsKey($name)) { return [string]$script:Vars[$name] }",
        "    return $m.Value",
        "  }))",
        "}",
        "",
        "function Invoke-HCurl {",
        "  param(",
        "    [string]$Label,",
        "    [string]$Method,",
        "    [string]$Url,",
        "    [string[]]$Headers = @(),",
        "    [string]$Body = $null",
        "  )",
        "  Write-Host ''",
        "  Write-Host ('=' * 72)",
        "  Write-Host $Label",
        "  Write-Host ('-' * 72)",
        "  if ($ShowOnly) {",
        "    Write-Host (\"$Method $Url\")",
        "    foreach ($h in $Headers) { Write-Host (\"H: $h\") }",
        "    if ($null -ne $Body) { Write-Host $Body }",
        "    return",
        "  }",
        "  $args = @('-sS', '-i', '-X', $Method, $Url)",
        "  foreach ($h in $Headers) { $args += @('-H', $h) }",
        "  if ($null -ne $Body) { $args += @('--data-raw', $Body) }",
        "  & curl.exe @args",
        "  Write-Host ''",
        "}",
        "",
    ])

    lines.append("$script:Vars = [ordered]@{}")
    for key, value in env_values.items():
        lines.append(f"$script:Vars[{ps_single_quote(key)}] = Get-ValueOrDefault -Name {ps_single_quote(key)} -Default {ps_single_quote(str(value))}")
    lines.append("")

    current_folder = None
    for idx, (folders, item) in enumerate(flat, start=1):
        req = item["request"]
        req_name = item.get("name", f"Request {idx}")
        method = req.get("method", "GET").upper()
        url = to_ps_template_literal(request_url_raw(req))
        headers = req.get("header", []) or []
        body = None
        if isinstance(req.get("body"), dict) and req["body"].get("mode") == "raw":
            body = req["body"].get("raw", "")

        folder_label = " / ".join(folders) if folders else "Root"
        if folder_label != current_folder:
            current_folder = folder_label
            lines.append(f"# === {folder_label} ===")

        label = f"{folder_label} :: {req_name}"
        lines.append(f"# {idx:03d}. {req_name}")
        lines.append(f"$url = Resolve-Template {ps_single_quote(url)}")
        lines.append("$headers = @()")
        for h in headers:
            key = h.get("key", "")
            value = h.get("value", "")
            lines.append(f"$headers += (Resolve-Template {ps_single_quote(f'{key}: {value}')})")

        if body is None:
            lines.append(f"Invoke-HCurl -Label {ps_single_quote(label)} -Method {ps_single_quote(method)} -Url $url -Headers $headers")
        else:
            lines.append("$body = @'")
            lines.append(body)
            lines.append("'@")
            lines.append("$body = Resolve-Template $body")
            lines.append(f"Invoke-HCurl -Label {ps_single_quote(label)} -Method {ps_single_quote(method)} -Url $url -Headers $headers -Body $body")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def main():
    collection_doc = load_json(COLLECTION_PATH)
    env_doc = load_json(ENV_PATH)

    bash_path = API_TESTS_DIR / "run_api_tests_curl.sh"
    ps_path = API_TESTS_DIR / "run_api_tests_curl.ps1"

    bash_path.write_text(build_bash_script(collection_doc, env_doc), encoding="utf-8", newline="\n")
    ps_path.write_text(build_powershell_script(collection_doc, env_doc), encoding="utf-8", newline="\n")

    print(f"BASH={bash_path}")
    print(f"POWERSHELL={ps_path}")
    print(f"REQUESTS={len(flatten_items(collection_doc.get('item', [])))}")


if __name__ == "__main__":
    main()
