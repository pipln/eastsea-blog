#!/bin/bash
# 발행 드리프트 상설 체크 — 슬러그 하나를 3개 평면에서 대조한다.
#   ① D1 API     : /api/posts/<slug> → 200 + title 존재 + content 길이 하한
#   ② 라이브 SPA : view.html?post=<slug> → 200 (리다이렉트 추적)
#   ③ 라이브 목록: posts.json 에 slug 존재
#                  (색인은 D1에서 주기적으로 재생성되므로 지연 lag는 WARN,
#                   색인이 D1과 동기 상태인데도 누락이면 FAIL)
#
# 사용: ./scripts/check-publish.sh <slug> [최소 content 길이, 기본 1200]
# 종료코드: 0 = 실패 없음(WARN 허용), 1 = FAIL 존재
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
TMP_META=$(mktemp /tmp/drift-meta.XXXXXX)
TMP_IDX=$(mktemp /tmp/drift-idx.XXXXXX)
trap 'rm -f "$TMP_D1" "$TMP_META" "$TMP_IDX"' EXIT

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
    print(f"json_error:{e}"); sys.exit(2)
p = d.get("post", d) if isinstance(d, dict) else {}
title = (p.get("title") or "").strip()
n = len(p.get("content") or "")
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

# ② 리더 페이지 — 정적 셸 200은 증거가 아니다(soft-404).
#    정본 판정: /view?post=<slug> → 200 + <title>Post Viewer ...</title>
#    깨진 상태에서는 index.html이 25,449B로 폴백되며 title이 사이트 기본 제목이 된다.
view_html=$(curl -s -m 25 -o /tmp/drift_view.$$.html -w '%{http_code}' "$SITE/view?post=$SLUG")
view_title=$(grep -o "<title>[^<]*</title>" /tmp/drift_view.$$.html 2>/dev/null | head -1)
rm -f /tmp/drift_view.$$.html
if [ "$view_html" = "200" ] && printf '%s' "$view_title" | grep -qi "Post Viewer"; then
  row live-view PASS "http=200 viewer_title_ok"
elif [ "$view_html" = "200" ]; then
  row live-view FAIL "http=200 index_fallback ${view_title:-no_title}"; fail=1
else
  row live-view FAIL "http=$view_html"; fail=1
fi

# ③ 라이브 posts.json 색인 (지연 허용)
curl -s -m 40 -o "$TMP_META" "$API?limit=1" >/dev/null 2>&1
idx_code=$(curl -s -m 45 -o "$TMP_IDX" -w '%{http_code}' "$SITE/posts.json")
idx_detail=$(python3 - "$TMP_IDX" "$TMP_META" "$SLUG" <<'PY'
import json, sys
idx_path, meta_path, slug = sys.argv[1], sys.argv[2], sys.argv[3]
try:
    idx = json.load(open(idx_path))
except Exception as e:
    print("FAIL json_error:%s" % e); sys.exit(0)
if not isinstance(idx, list):
    idx = idx.get("posts", []) if isinstance(idx, dict) else []
blob = json.dumps(idx)
if slug in blob:
    print("PASS slug_indexed")
    sys.exit(0)
try:
    total = json.load(open(meta_path)).get("total")
except Exception:
    total = None
if isinstance(total, int) and len(idx) < total:
    print("WARN projection_lag idx=%d d1_total=%d" % (len(idx), total))
else:
    print("FAIL slug_missing idx=%d d1_total=%s" % (len(idx), total))
PY
)
status=$(printf '%s' "$idx_detail" | awk '{print $1}')
detail=$(printf '%s' "$idx_detail" | cut -d' ' -f2-)
case "$status" in
  PASS) row posts.json PASS "$detail" ;;
  WARN) row posts.json WARN "$detail" ;;
  *)    row posts.json FAIL "http=$idx_code ${detail:-unknown}"; fail=1 ;;
esac

if [ "$fail" -eq 0 ]; then echo "RESULT: PASS"; else echo "RESULT: FAIL"; fi
exit "$fail"
