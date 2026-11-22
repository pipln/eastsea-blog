#!/bin/bash
# 발행 드리프트 상설 체크 — 슬러그 하나를 3개 평면에서 대조한다.
#   ① D1 API     : /api/posts/<slug> → 200 + title 존재 + content 길이 하한
#   ② 라이브 SPA : view.html?post=<slug> → 200 (리다이렉트 추적)
#   ③ 라이브 목록: posts.json 에 slug 존재
#
# 사용: ./scripts/check-publish.sh <slug> [최소 content 길이, 기본 1200]
# 종료코드: 0 = 전 평면 통과, 1 = 하나 이상 실패
#
# 왜 바이트 수로 판정하지 않는가: view.html 은 정적 셸이라 모든 포스트가
# 같은 바이트(≈25,449)를 돌려준다. 본문 유무는 셸이 아니라 D1 응답으로만 판정된다.
set -uo pipefail

SLUG="${1:-}"
MIN_LEN="${2:-1200}"
API="https://blog-api.k-jaylee.workers.dev/api/posts"
SITE="https://eastsea.monster"

if [ -z "$SLUG" ]; then
  echo "Usage: $0 <slug> [min_content_len]" >&2
  exit 1
fi
SLUG="${SLUG%.md}"

TMP_D1=$(mktemp /tmp/drift-d1.XXXXXX)
TMP_IDX=$(mktemp /tmp/drift-idx.XXXXXX)
trap 'rm -f "$TMP_D1" "$TMP_IDX"' EXIT

fail=0
row() { printf '%-14s %-6s %s\n' "$1" "$2" "$3"; }
echo "== publish drift check: $SLUG =="
row PLANE STATUS DETAIL

# ① D1 API
d1_code=$(curl -s -m 25 -o "$TMP_D1" -w '%{http_code}' "$API/$SLUG")
d1_detail=$(python3 - "$TMP_D1" "$MIN_LEN" <<'PY'
import json, sys
path, min_len = sys.argv[1], int(sys.argv[2])
try:
    d = json.load(open(path))
except Exception as e:
    print(f"json_error:{e}")
    sys.exit(2)
p = d.get("post", d) if isinstance(d, dict) else {}
title = (p.get("title") or "").strip()
content = p.get("content") or ""
n = len(content)
if not title:
    print("no_title"); sys.exit(2)
if n < min_len:
    print(f"content_too_short:{n}<{min_len}"); sys.exit(2)
print(f"title_ok content_len={n}")
PY
)
if [ "$d1_code" = "200" ] && [ -n "$d1_detail" ] && ! printf '%s' "$d1_detail" | grep -qE '^(json_error|no_title|content_too_short)'; then
  row D1 PASS "$d1_detail"
else
  row D1 FAIL "http=$d1_code ${d1_detail:-empty}"; fail=1
fi

# ② 라이브 SPA 셸
live_code=$(curl -s -m 25 -o /dev/null -L -w '%{http_code}' "$SITE/view.html?post=$SLUG")
if [ "$live_code" = "200" ]; then row live-shell PASS "http=200"; else row live-shell FAIL "http=$live_code"; fail=1; fi

# ③ 라이브 posts.json 색인
idx_code=$(curl -s -m 40 -o "$TMP_IDX" -w '%{http_code}' "$SITE/posts.json")
if [ "$idx_code" = "200" ] && grep -q "$SLUG" "$TMP_IDX"; then
  row posts.json PASS "slug_indexed"
else
  row posts.json FAIL "http=$idx_code slug_missing"; fail=1
fi

if [ "$fail" -eq 0 ]; then echo "RESULT: PASS"; else echo "RESULT: FAIL"; fi
exit "$fail"
