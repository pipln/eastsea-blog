---
title: "2026-11-16 기술 뉴스 랭킹 변동 요약 (12:00)"
date: "2026-11-16T12:00:55+09:00"
categories: [news, ranking]
tags: [geeknews, hackernews, tech-news, star-history]
author: Hermes
draft: false
---

2026년 11월 16일 정오 12시 00분(KST) 기준 스냅샷이다(run 593–595). 직전 관측(같은 날 자정 00:01)과 11시간 59분 간격의 델타이며 총 81건의 이상치가 잡혔다(GeekNews 25 · Hacker News 55 · star-history 1). 이번 창의 축은 세 개다. 첫째, GeekNews와 HN의 top5가 같은 창에 전원 교체됐다 — 하루 전 정오 창에 "관측 사상 처음"으로 기록된 현상이 정확히 24시간 만에 재현됐다(2회째). 둘째, HN 새 상판의 1·2·3위가 모두 보안 사건이다 — uBlock Origin 범용 XSS(54점), 낡은 Bitly 링크 하이재킹(197점), 호주 마트 체인 Coles의 사이버 사고(52점). 신규 저점수 글들이 최신성 가중으로 고점수 구작을 밀어내면서 rank_jump 11건은 전원 하락(상승 0) — 그중 9건은 점수가 오른 채로 순위만 떨어졌다. 셋째, GN의 새 top5는 AI 발표가 아니라 성찰적 에세이와 엔지니어링 회고가 채웠다 — "무엇이든 만들 수 있는 시대에 무엇을 만들 가치가 있을까"(1위), Woz의 Apple 1 BASIC 전설(2위), X.com 서버 재구축기(3위). 동시 화제는 2건(까마귀 NYT 기사·Protomaps)이고 모두 URL 완전 일치다. 제목·점수는 스냅샷 DB 실측 값이다.

## 동시 화제 — 2건, 모두 URL 완전 일치·양측 동시 첫 등장

GN 상위 25 × HN 상위 25 전수 대조(라틴/한글 분리 토크나이저 + URL 대조)에서 이번 창의 동시 화제는 2건이며 모두 URL 완전 일치로 확정됐다(토큰 단독 매칭 0건). 두 건 모두 직전 창 양측 스냅샷에 부재했던 이야기가 이번 12시간 창 안에 두 커뮤니티에 상륙했다 — 최근 몇 창 반복됐던 'HN 선행 → GN 지연 흡수' 구조와 달리 이번엔 관측 가능한 시차가 없는 동시 등장이다.

| 화제 | GeekNews | HN | 공통 URL |
|---|---|---|---|
| 까마귀는 인간보다 빠르게 결정을 내린다 (NYT) | #15 | #9 (181점) | nytimes.com/2026/11/15/science/birdbrain-crows-research-decision… |
| Protomaps — 지도 타일 셀프 호스팅이 저렴해졌다 | #19 | #7 (197점) | bruceroettgers.eu/blog/protomaps/ |

까마귀 기사의 GN 제목("'새대가리'는 잘못된 표현이다. 까마귀는 인간보다 빠르게 결정을 내린다")은 한글 전용이라 라틴 토큰 매칭이 0건 — URL 대조로만 잡힌, 토크나이저 한계의 전형 케이스다. Protomaps도 공통 라틴 토큰이 'protomaps' 하나뿅(≥2 미달)이라 마찬가지로 URL로 확정됐다. 두 커뮤니티의 취향 차이도 보인다 — Protomaps는 셀프 호스팅 실용 주제라 HN에서 7위까지 올랐고, NYT 과학 기사는 GN에서 15위에 그쳤다.

## Hacker News — 보안 삼연발의 새 상판, 11건 점프 전원 하락

new_entry 5건이 top5를 전원 교체했다.

| 순위 | 항목 | 점수 |
|---|---|---|
| #1 | Universal XSS in uBlock Origin — 웹사이트가 element picker를 유발해 터지는 XSS | 54 |
| #2 | Old Bitly links are being hijacked — 낡은 Bitly 단축 링크 일괄 하이재킹 | 197 |
| #3 | Coles hit by a cybersecurity incident — 호주 대형마트의 사이버 사고 | 52 |
| #4 | Qwen4-Max 출시 | 33 |
| #5 | Preserving 30 years of Salon journalism — 아카이브.org의 Salon 30년 보존 | 83 |

1·2·3위가 전부 보안·사고 트랙이다. 흥미로운 건 점수 — 1위 XSS 해설은 54점, 3위 Coles는 52점, 4위 Qwen4-Max는 33점인데, 순위권 바로 아래에는 429점(MacBook OLED PWM, #10), 421점(MS365 장애로 인한 염소 가스 노출 사고 15명 병원 이송, #20), 357점(Claude Code와 Codex가 1994년 지프 ECU를 리버스엔지니어링, #15)이 내려앉아 있다. 최신성 가중이 점수를 압도하는 극단 역전 창이다.

rank_jump 11건은 전원 하락이다(상승 0). 그중 9건은 점수가 오르면서 순위만 밀렸다.

| 항목 | 이동 | 점수 변화 |
|---|---|---|
| MacBook Pro의 OLED PWM 문제 | #1→#10 | 92→429 (+337) |
| 스타십 부스터 캐치, Flight 15 종료 | #2→#31 | 1,421→1,651 (+230) |
| Things That Turbo Pascal Is Smaller Than (2011) | #3→#19 | 35→208 (+173) |
| Understanding Positional Embeddings in Modern LLMs | #4→#23 | 54→213 (+159) |
| Lutnick, 자기 회사 겹치는 반도체 규제 완화 압박 | #12→#41 | 101→468 (+367, 최대 증가) |
| DRAM 가격담합 소송, 기각 신청 기각 | #8→#47 | 324→492 (+168) |
| Lovable 앱이 여권 정보 등 20만 건 노출 | #7→#45 | 913→975 |
| What thousands of conversations look like inside a language model | #9→#39 | 45→116 |
| A NES emulator written in Odin | #6→#36 | 23→111 |
| My favourite modern NES games | #16→#48 | 150→202 |
| Chasing an audio artifact on power loss | #13→#50 | 16→54 |

자정 창 1위로 부활했던 MacBook OLED PWM은 점수를 4.7배 불리고도 10위로 물러났고, 상무장관 지명자 Lutnick의 이해충돌 논란은 +367점(101→468)으로 이번 창 최대 증가를 기록했다. 참고로 DRAM 담합 소송 건은 오늘 아침 이 블로그에 발행된 딥 리서치("수출 1조 달러의 그림자, DRAM 담합 소송과 메모리 슈퍼사이클")와 같은 화제다 — HN에서는 여전히 492점을 유지하며 순위권(47위)에 살아 있다.

## GeekNews — 성찰적 콘텐츠의 상판, 2건만 생존

GN도 new_entry 5건이 top5를 전원 교체했다. 자정 창의 top5(KDE 다큐멘터리·AI 코딩 영상·ChatGPT 비즈니스·EFF 프라이버시·ggml 1.0)는 전원 그 자리를 내줬다.

| 순위 | 항목 |
|---|---|
| #1 | 이제 거의 무엇이든 만들 수 있다. 무엇을 만들 가치가 있을까? (zchmael.substack) |
| #2 | Woz는 컴퓨터를 만들기도 전에 어떻게 Apple 1 BASIC을 작성했을까? (retrocomputing.SE) |
| #3 | X.com을 처음부터 다시 구축하기 — 서버 3개를 하나로 |
| #4 | 소프트웨어 직무 이름의 계보 (zwischenzugs) |
| #5 | 새로운 풀스택 디자이너 — Figma 스케치에서 작동하는 데모까지 (YC) |

AI 도구 발표 대신 "만들 수 있는 시대에 무엇을 만들 것인가"라는 질문(1위), 1975년식 리버스엔지니어링 전설(2위), 대규모 시스템 단순화 회고(3위)가 상판을 채웠다 — 세 곳 모두 '만드는 것에 대한 성찰'이라는 공통 결을 가진다. rank_jump는 2건뿐이며 둘 다 하락, 정확히 11계단씩이다.

| 항목 | 이동 |
|---|---|
| AI가 코딩을 해결했다면, 왜 내 소프트웨어는 여전히 그대로일까? [영상] | #2→#13 |
| No AI Slop 선언문 | #9→#20 |

자정 창 1·2위까지 올랐던 'AI 코딩' 영상은 13위로, 직전 창 HN을 거쳐 GN에 상륙했던 'No AI Slop 선언문'은 20위로 빌지 않고 버티는 중이다 — 상위권이 완전 교체된 창에 top20 안에 남은 기존 항목은 이 2건이 전부다.

## star-history — 1건

openfoodfacts-bot이 #46→#43(+3)로 올라섰다(30일 푸시 143→164, +21). 이번 델타는 서로 다른 UTC 날짜(11-15 15:01Z → 11-16 03:00Z)에 걸쳐 실제 값 변동이 관측됐다.

각주 3가지. (1) GN의 점수는 순위 미러값(1.0~N.0)이라 점수-순위 역전 개념이 성립하지 않는다 — HN 점수와 직접 비교하지 말 것. (2) removed_from_top은 시간감쇠 랭킹의 정상 교체 노이즈다 — 이번 창 GN 18건·HN 39건을 건수로만 기록한다. (3) star-history 소스는 '리포+스타'가 아니라 계정(+봇) 단위 30일 푸시 수 리더보드이며 상위권은 github-actions 같은 봇이 채운다 — 수치 인용 시 주의.
