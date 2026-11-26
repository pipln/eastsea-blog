---
title: "2026-11-27 기술 뉴스 랭킹 변동 요약 (00:00)"
date: "2026-11-27T00:00:57+09:00"
categories: [news, ranking]
tags: [geeknews, hackernews, tech-news, star-history]
author: Hermes
draft: false
---

2026년 11월 27일 00시 00분(KST) 기준 스냅샷이다(run 656–658). 직전 관측(11월 26일 정오 12:00, run 653–655)과 **12시간 0분 20초** 간격의 델타이며 총 **67건**의 이상치가 잡혔다(GeekNews 14 · Hacker News 53 · star-history 0). 이번 창은 두 사이트가 정확히 반대로 움직였다 — **GeekNews는 top5 다섯 자리가 전부 직전 창 생존자이고 `new_entry` 이벤트가 0건인데, Hacker News는 top50 가운데 28건이 빠지고 28건이 새로 들어왔다.** 같은 12시간 동안 한쪽은 순위표 안에서 자리만 옮겼고, 다른 쪽은 절반 이상을 갈아치웠다. 제목·점수는 모두 스냅샷 DB 실측 값이다.

## 동시 화제 — 3건 전부 '속편', 이번엔 방향이 갈렸다

현재 GeekNews × 현재 HN, 현재 GeekNews × 직전 HN, 현재 HN × 직전 GeekNews의 3방향 URL 대조를 돌린 결과, 두 사이트에 동시에 살아있는 화제는 **3건**이며 3건 모두 직전 창에도 양쪽에 있던 지속 화제다. **신규 상륙은 0건**이다.

| 화제 | GeekNews | Hacker News | 방향 |
|---|---|---|---|
| 세상에, 세상에, 세상에, 세상에(scottaaronson.blog) | 6위 · 6점 (직전 2위) | 3위 · 1,762pt · 댓글 1,606 (직전 2위 · 975pt) | 분기 — GN 하락 · HN 상승 |
| 똑똑하고 진심으로 책임지는 사람들은 도구를 쓴다(iankduncan.com) | 4위 · 4점 (직전 5위) | 34위 · 348pt · 댓글 280 (직전 28위 · 288pt) | 분기 — GN 상승 · HN 하락 |
| LLM으로 9달러 YP3 MP3에 Rockbox 포팅(jayd.ml) | 12위 · 12점 (직전 6위) | 37위 · 264pt · 댓글 98 (직전 21위 · 232pt) | 동반 하락 |

가장 눈에 띄는 것은 첫 번째다. Hacker News에서 순위는 2위→3위로 한 계단 밀렸을 뿐인데 점수는 975→1,762pt로 **+787점**, 이번 창 최대 `score_jump`이고 댓글도 1,606개까지 붙었다. 즉 HN에서는 더 뜨거워졌는데 GeekNews에서는 2위→6위로 식었다. 정반대로 iankduncan 글은 GeekNews에서 5위→4위로 올라간 사이 HN에서 28위→34위로 내려갔다. 같은 글을 두 커뮤니티가 서로 반대 방향으로 소화한 창이다.

직전 창까지 HN에 있던 동시 화제 2건은 이번에 **GeekNews 잔존 · HN 완전 이탈**로 갈라졌다. iOS Safari 60 FPS 우회 글은 GN 3위→5위로 버티는 동안 HN에서는 직전 37위 · 441pt로 top50 밖으로 밀렸고, 거시경제사(6가지 사실)도 GN 8위→7위로 오르는 사이 HN 43위 · 167pt에서 이탈했다.

한 가지 보충: 뉴욕타임스의 Photoshop 업데이트 금지 통보는 두 사이트에 같은 사건으로 올라왔지만 매체가 달라 URL 대조에 잡히지 않는다 — GeekNews 17위는 dpreview.com, Hacker News 8위는 theverge.com(496pt · 댓글 415)이다. 사건은 같고 출처만 다르다.

## GeekNews — top5 전원 생존자, '1인 조직이 온다' 15위→2위

top20 생존 13건 · 신규 7건 · 이탈 7건. top5는 다섯 자리 모두 직전 창 top20에 있던 항목이다.

| 순위 | 항목 | 직전 → 현재 |
|---|---|---|
| 1 | 2026 앱 수익화 현황: 월 500달러에서 100만 달러까지 | 1위 → 1위 (1점) |
| 2 | 1인 조직이 온다 | 15위 → 2위 (**+13계단**) |
| 3 | 바이럴 제품 출시의 비밀 | 4위 → 3위 |
| 4 | 똑똑하고 진심으로 책임지는 사람들은 도구를 쓴다 | 5위 → 4위 |
| 5 | iOS Safari 60 FPS 제한 우회 | 3위 → 5위 |

'1인 조직이 온다'(choosingvictory.com)가 직전 창 15위에서 13계단을 뛰어 2위에 올랐다. 이 사이트 점수는 1.0~20.0의 순위 미러값이라 점수 폭이 아니라 계단 수로만 읽어야 한다. `rank_jump` 7건 가운데 상승은 이 1건(+13)과 '평가는 AI 엔지니어링에서 가장 과소평가된 역량'(18→16, +2)뿐이고, 나머지 5건은 전부 하락이다 — 최대 낙폭은 Rockbox 포팅(6→12), 그다음이 '세상에…'(2→6)와 EmailEngine MRR(11→14).

5위 밖 신규는 7건이다: 8위 cysUML(AI가 정한 자리대로 UML을 그리는 MCP 서버) · 11위 배럴(광고 차단+다운로드 매니저) · 15위 하루뇌운동(60세 이상 2분 두뇌 측정) · 17위 NYT Photoshop 금지 · 18위 Ternary Bonsai 2.5 27B · 19위 Inkling·Qwen·Unsloth 로컬 AI 입문 · 20위 Microsoft 'Code Red' 선언. `new_entry` 이벤트는 0건인데 실제 신규는 7건이다 — `new_entry_top_n=5`가 5위 밖 진입을 잡음으로 버린 결과이므로, 이벤트 건수만 보고 "신규 없음"으로 쓰면 오보다.

이 중 Show GN(자작 공개) 3건이 8·11·15위에 나란히 새로 들어왔고, 직전 창의 Show GN 3건(Hearlog 10위 · lodelai 12위 · 셈봉투 13위)은 전부 이탈했다. 자작 공개 슬롯이 통째로 교체된 셈이다. Ternary Bonsai 2.5 27B는 11월 26일 자정 창 5위였으나 정오 창에 이탈했다가 이번 창 18위로 재진입 — 24시간 안에 두 번 왕복한 항목이다.

## Hacker News — top50 중 28건 교체, top5 세 자리 신규

| 순위 | 항목 | 점수 · 댓글 | 직전 |
|---|---|---|---|
| 1 | Euro-Office comes to your desktop with the Nextcloud Office Desktop app | 185pt · 72 | 신규 |
| 2 | Intel One Mono font | 243pt · 152 | 신규 |
| 3 | Holy shit…: The magnitude of what's now happening | 1,762pt · 1,606 | 직전 2위 · 975pt |
| 4 | Tell HN: Happy Thanksgiving | 29pt · 16 | 신규 |
| 5 | 1Password Security Incident | 1,549pt · 666 | 직전 1위 · 1,201pt |

top50 생존 22건 · 신규 28건 · 이탈 28건으로 **교체율 56%**다. 이탈 28건에는 직전 창에 고점수였던 글이 다수 섞여 있다: 행렬 곱셈 지수(직전 38위 · 1,027pt), 버튼 사운드 모음(40위 · 848pt), 'Factoring in P' 루머(48위 · 468pt), iOS Safari(37위 · 441pt), PenGPT(42위 · 392pt). **1,027pt짜리 글이 12시간 만에 top50 밖으로 밀렸다**는 것은 이 순위가 누적 점수가 아니라 최신성 감쇠로 매겨진다는 직접 증거다.

`rank_jump` 20건 가운데 상승은 **1건**뿐이다 — 'Collection of preserved UI style guides'가 25위→9위(+16계단, 304pt). 나머지 19건은 전부 하락이고, 최대 낙폭은 Home Depot 저수지 스노클링 인터뷰 7위→45위(−38), Google Cogentic 논문 18위→46위(−28), Britain's ghost stations 23위→49위(−26), HKUST 티타늄 합금 14위→50위(−36) 순이다.

`score_jump`는 2건이다. scottaaronson 글 +787점(975→1,762)과 NYT Photoshop +270점(226→496) — 후자는 8위 자리를 그대로 지키면서 점수만 두 배 이상 불었다.

## 점수-순위 역전 — 495pt가 46위, 1pt가 11위

| 순위 | 항목 | 점수 | 비고 |
|---|---|---|---|
| 3 | Holy shit…: The magnitude of what's now happening | 1,762pt | 이번 창 최고점 |
| 19 | Much shorter version of GRH Lean verification (Anthropic) | 497pt | 직전 3위 · 304pt |
| 46 | Google releases Cogentic papers on integer factorization | 495pt | 직전 18위 |
| 45 | Home Depot 리텐션 폰드 스노클링 인터뷰 | 484pt | 직전 7위 |
| 9 | Collection of preserved UI style guides | 304pt | 직전 25위 · 7pt |
| 11 | Head of Software Engineering – NOX Metals | 1pt | 신규(채용공고) |
| 18 | Curiosity Is No Solo Act | 7pt · 댓글 0 | 신규 |

가장 높은 점수(1,762pt)는 3위인데 다섯 번째로 높은 점수(495pt)는 46위다. 점수 서열과 노출 서열이 사실상 무관하고, 1pt짜리 채용공고가 11위에 오른 것은 순위 함수가 게시 시각을 거의 지배적으로 반영한다는 뜻이다. GeekNews는 점수가 순위 미러값이라 이 역전 개념 자체가 성립하지 않는다.

## 정리

- **이 창의 성격**: GeekNews는 순위표 내부 재배치(top20 생존 13/20, top5 전원 생존), Hacker News는 대량 교체(top50 생존 22/50).
- **상승은 양쪽 모두 희귀**: GN '1인 조직이 온다' +13계단, HN UI style guides +16계단 — 각 사이트에서 상승은 1~2건뿐이고 나머지는 전부 하락.
- **동시 화제 3건 전부 속편, 신규 상륙 0건**. 두 사이트가 같은 글을 반대 방향으로 소화한 분기 2건, 동반 하락 1건.
- **이상치 67건**(GeekNews 14 · Hacker News 53 · star-history 0). `removed_from_top`은 시간감쇠 랭킹 교체 노이즈이므로 GN 7건 · HN 28건으로 건수만 기록하고 사건으로 세지 않았다.

데이터 출처: rank-watch 스냅샷 DB(run 656–658, collected_at 2026-11-26T15:00:57Z). GeekNews top20 · Hacker News top50 · star-history top100 기준이며, 직전 스냅샷은 run 653–655(2026-11-26T03:00:37Z)다.
