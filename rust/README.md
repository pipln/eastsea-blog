# Eastsea Rust/WASM (`rust/`)

802개 계산기 툴의 로직을 Rust 코어 하나로 모으고, wasm으로 빌드해 브라우저에서 실행한다.
레거시 `tools/*/calculator.js`는 **폴백 경로로 유지**한다(삭제 금지).

## 구조
```
rust/
├── Cargo.toml            # workspace
├── build.sh              # wasm-pack 빌드 → assets/wasm/ 복사 + gzip 크기 게이트(250KB)
├── parity/run.mjs        # 레거시 JS vs wasm 차등검증 하네스
└── toolcore/             # 계산 코어 (cdylib + rlib)
    └── src/{lib.rs,jsnum.rs,tools/{stripe,bond,upwork}.rs}
```

## ABI
```rust
#[wasm_bindgen]
pub fn invoke(tool_id: &str, input_json: &str) -> String
```
- 입력 `{...}` — 빠진 키는 레거시 `DEFAULTS`로 채워진다. `{"lang":"ko"}` 지원.
- 출력 `{"ok":true,"tool":"<id>","result":{...},"error":""}` / `{"ok":false,"error":"<사유>"}`

## 현재 이관된 툴
| tool_id | 원본 |
|---|---|
| `stripe-fee-profit-calculator` | `tools/stripe-fee-profit-calculator/calculator.js` |
| `bond-yield-calculator` | `tools/bond-yield-calculator/calculator.js` |
| `upwork-fee-calculator` (forward) | `tools/upwork-fee-calculator/calculator.js` |
| `upwork-fee-calculator-reverse` | 동일 |

## 게이트 (머지 전 필수)
```bash
cd rust && cargo test                 # 수식 단위 테스트
bash rust/build.sh                    # wasm 빌드 + gzip 예산
bash scripts/rust-tool-gate.sh        # cargo test + 패리티(diff=0)
```
패리티 하네스는 기존 `calculator.test.js` 케이스 + 경계값 + 고정시드 무작위 200개를 돌려
레거시(Node CJS) 결과와 wasm(nodejs 타깃) 결과를 비교한다. float 허용오차 = 상대 1e-9.

## 툴 추가 절차
1. `rust/toolcore/src/tools/<name>.rs` — 원본 `calculator.js` 수식을 1:1 이식(개선 금지).
2. `tools/mod.rs`와 `lib.rs`의 `invoke_impl` match에 id 추가.
3. `rust/parity/run.mjs`에 엔트리 추가 → `bash scripts/rust-tool-gate.sh`로 diff 0 확인.
4. `bash rust/build.sh` → `assets/wasm/toolcore_bg.wasm` 갱신.
5. 해당 툴 `index.html`에 `<script src="/assets/toolcore.js" defer>` 추가(선택적 사용).

## 배포
`assets/`는 `scripts/cf-pages-auto-deploy.sh` 화이트리스트에 포함되어 있어
`assets/wasm/*` 가 그대로 Cloudflare Pages에 올라간다(파일당 25MB 제한, 현재 147KB).

## 알려진 편차
- `result.summary` 문자열의 개행/공백이 레거시와 미세하게 다를 수 있다(숫자·판정값은 동일).
- 로더는 실패 시 조용히 폴백한다 — wasm 미로드 상태에서도 페이지는 정상 동작.
