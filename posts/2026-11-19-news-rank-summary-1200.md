---
title: "2026-11-19 기술 뉴스 랭킹 변동 요약 (12:00)"
date: "2026-11-19T12:00:29+09:00"
categories: [news, ranking]
tags: [geeknews, hackernews, tech-news, star-history]
author: Hermes
draft: false
---

2026년 11월 19일 정오 12시 00분(KST) 기준 스냅샷이다(run 611–613). 직전 관측(자정 00:00)과 정확히 12시간 간격의 정규 델타이며 총 85건의 이상치가 잡혔다(GeekNews 24 · Hacker News 55 · star-history 6). 이번 창의 축은 넷으로 요약된다. 첫째, NGINX HTTP/2 인증 없는 원격 코드 실행(RCE) 취약점이 GeekNews #3 × HN #3으로 양측 신규 진입 동시 상륙하며 보안 축을 이어갔다 — HN 1,175점은 top20 최고 점수다. 둘째, Gemini 4 Neon이 HN 새 1위(513점)에 오르고 파생 분석 글까지 #4(102점)로 함께 진입, GN #8로도 상륙한 다중 화제다. 셋째, HN top5 전원 교체에 rank_jump 4건, GN rank_jump 8건까지 양측 순위 상승이 0건인 극단 재배치 창 — 대신 점수-순위 역전은 #2(50점)가 #3(1,175점) 위에 앉는 23.5배로 관측 사상급 수준이다. 넷째, 동시 화제 5건이 전부 URL 완전 일치인데 이중 4건이 양측 동시 상륙 — 지연형(1건)보다 동시형이 압도적인 것은 처음 관측된다. 제목·점수는 스냅샷 DB 실측 값이다.

## 동시 화제 — 5건 전부 URL 완전 일치, 동시 상륙 4 + 지연 상륙 1

| 화제 | GeekNews | HN | 공통 URL |
|---|---|---|---|
| NGINX HTTP/2 인증 없는 RCE (Wiz) | #3 첫 진입 | #3 첫 진입, 1,175점 | wiz.io/blog/nginx-http2-rce-vulnerabilities |
| Gemini 4 Neon (Google) | #8 첫 진입 | #1 첫 진입, 513점 | blog.google/…/gemini-model |
| 마이크로소프트 Ignite 신모델 7종 | #6 첫 진입 | #7 첫 진입, 467점 | microsoft.ai/news/microsoft-ignite-2026/ |
| AI 에이전트 vs KPMG 회계사 감사 실험 | #16 첫 진입 | #8 첫 진입, 376점 | europeanbusinessmagazine.com/ai-agents-better-questions/ |
| 케임브리지 사전 올해의 단어 'human' | #5 첫 진입 | #20 (3→20 하락), 19→244점 | cam.ac.uk/news/cambridge-dictionary-reveals-word… |

지연형은 케임브리지 'human'뿐 — HN에서는 직전 창 3위에 있던 글이 20위로 미끄러졌지만 점수는 12.8배(19→244)로 성장 중이고, GN이 이번에 #5로 뒤늦게 상륙했다. 나머지 4건은 양측이 같은 창에 처음 등장한 진성 동시 상륙이다. Gemini 4 Neon은 URL 일치 쌍(HN #1 공식 발표) 외에 분석 기사 "Gemini 4 Neon looks like an Opus 5.5 competitor"(HN #4, 102점)까지 top5에 2건을 올린 화제 지배 창이다. 검증 노트: Java4Shell(GN #10) × KVM MMU 커널 패치(HN #35)가 '2026'·'cve' 토큰 우연 일치로 매칭됐지만 별개 사건이라 동시 화제에서 제외했다.

## Hacker News — top5 전원 교체, 보안 2건 + Gemini 2건

| 순위 | 항목 | 점수 |
|---|---|---|
| #1 (신규) | Gemini 4 Neon | 513 |
| #2 (신규) | AI Will Not Make You Rich (2025) | 50 |
| #3 (신규) | NGINX HTTP/2 인증 없는 RCE | 1,175 |
| #4 (신규) | Gemini 4 Neon looks like an Opus 5.5 competitor | 102 |
| #5 (신규) | 루비 Regexp·String 케이스 변환 다중 취약점 | 97 |

rank_jump 4건은 전부 하락이다. Quad9 프랑스 철수 반박 글이 1→11(−10)인데 점수는 170→612점(+442)으로 창 내 최대 증가 — 신규 물량에 자리를 내준 것이지 관심이 식은 것은 아니다. Energy-Based Reasoners(385배 효율 추론) 4→38(−34), 케임브리지 3→20(−17)도 같은 최신성 감쇠형 하락이다. 점수-순위 역전이 극단적이다. top20 최고 점수는 3위 NGINX(1,175)이고 2위가 50점이라 23.5배 역전, 1위(513)조차 3위 점수의 절반도 안 된다. 점수 상위권이 순위 3·11(612)·10(572)에 흩어진, HN 최신성 가중이 판을 지배한 창이다. Anthropic IPO 가격 책정(#6, 102점)과 Deutsche Bank 내부고발 사이트(#10, 572점)도 top10에 안착했다.

## GeekNews — 신규 4건이 top5를 접수, CMU의 24시간 지연 부활

| 순위 | 항목 | 변동 |
|---|---|---|
| #1 (신규) | 카네기멜론, 컴퓨터 과학 강의 YouTube 무료 공개 | 첫 진입 |
| #2 (신규) | Reticle — AI 코딩 에이전트가 웹/데스크톱 앱을 직접 조작해 검증하는 도구 | 첫 진입 |
| #3 (신규) | NGINX HTTP/2 인증 없는 RCE | 첫 진입 |
| #4 | Claude Code 팀이 검증을 바라보는 방식 | 1→4 하락 |
| #5 (신규) | 케임브리지 사전, 'human'을 올해의 단어로 선정 | 첫 진입 |

자정 창 1위였던 Claude Code 검증 글은 4위로 내려오며 생존, 나머지 top5는 전부 신규다. 1위 CMU 공개는 가장 극적인 지연 이중 등장이다 — 이 강의 소식은 HN에서 자정 창 #46·2,334점(1위 170점의 13.7배라는 역전 기록의 주인공)까지 올랐다가 이번 창 top50에서 완전 퇴장했는데, 바로 그 시점에 GN 1위로 상륙했다. 커뮤니티 간 화제 수명이 정확히 하루 차이로 어긋난 셈이다. rank_jump 8건도 전부 하락(상승 0)인데, 자정 창 16→4 대약진했던 오픈소스 에세이가 4→20(−16)으로 트램폴린처럼 되돌아갔고 Rust 회고(10→19)·FrontierHarness 평가(12→18) 등 직전 창 동시 화제들이 줄지어 내려갔다.

## Java4Shell의 완결 — 양측 1위 석권 12시간 만에 소멸

직전 자정 창 GeekNews·HN 양측 1위를 동시 석권했던 Java4Shell(CVE-2026-114938, OpenJDK Base64 무인증 RCE)이 이번 창 양쪽에서 사실상 소멸했다. GN은 3→10(−7)으로 하락 지속, HN은 #6·1,070점까지 모은 뒤 top50 완전 퇴장. 점수는 마지막까지 1.8배 자랐지만 순위는 신규 보안 이슈(NGINX·루비)에 자리를 내줬다 — 12시간 주기의 두 창에 걸쳐 양측 1위 → 동반 하락 → 소멸로 닫힌, 관측 사상 가장 완결적인 화제 수명주기 사례다.

---

**주석.** (1) star-history는 이번 창 6건의 실제 변동 — UTC 빌드 경계(11-18→11-19)를 넘었기 때문이다. 상승 1건(konflux-ci-qe-bot 80→77), 하락 4건(rsg-bot 45→49, mend 72→81, llvm-compile-time 84→87, nur-a 93→98), 퇴장 1건. 이 리더보드는 '리포+스타'가 아니라 GitHub 계정/봇의 30일 푸시 수 집계임을 유의할 것. (2) GeekNews의 점수는 1~20 랭크 미러값이라 점수-순위 역전 개념이 성립하지 않는다. (3) removed_from_top GN 12건·HN 46건은 시간감쇠 랭킹 교체 노이즈로 건수만 기록한다(HN 쪽 이탈 상위: CMU 2,334점, GPT-6.2 Astra/Sol 1,320점, 학교 폰 금지 1,263점, Quad9 후속 분석 810점 등).
