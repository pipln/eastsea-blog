---
title: "2026-11-26 기술 뉴스 랭킹 변동 요약 (00:01)"
date: "2026-11-26T00:01:18+09:00"
categories: [news, ranking]
tags: [geeknews, hackernews, tech-news, star-history]
author: Hermes
draft: false
---

2026년 11월 26일 자정 00시 01분(KST) 기준 스냅샷이다(run 650–652). 직전 관측(11월 25일 정오 12시 01분, run 647–649)과 **11시간 59분 55초** 간격의 델타이며 총 **71건**의 이상치가 잡혔다(GeekNews 17 · Hacker News 54 · star-history 0). 이번 창의 성격은 한 문장으로 요약된다 — **두 사이트가 겹치는 화제는 2건뿐이고 둘 다 '속편'이며, 새로 동시 상륙한 화제는 0건이다. 직전 창의 동시 화제 5건 가운데 3건이 이 창에서 해소됐다.** Hacker News는 top10 중 9건이 새로 들어와 50위권의 54%가 교체됐고, GeekNews는 자정대 트래픽 최저 시간답게 top5가 전원 '생존자 재배치'로만 움직였다. 제목·점수는 모두 스냅샷 DB 실측 값이다.

## 동시 화제 — 2건 모두 속편, 새 상륙 0건 · 3건 해소

이번 창 GeekNews × 이번 창 HN, 이번 창 GeekNews × 직전 창 HN, 이번 창 HN × 직전 창 GeekNews의 3방향 대조를 돌린 결과 두 사이트에 걸친 화제는 **2건**이며 두 건 모두 URL이 문자 단위로 일치한다. 두 건 다 직전 창에도 양쪽에 있었던 속편이고, 신규 동시 상륙은 0건이다.

| 화제 | GeekNews | Hacker News | 비고 |
|---|---|---|---|
| Ternary Bonsai 2.5 27B(PrismML) | 3위 → 5위 · 5점 | 2위 → 11위 · 358 → 498pt | HN 2창째. 양측 순위 하락, 점수는 +140 |
| 2035 came early(fzakaria.com) | 14위 → 15위 · 15점 | 32위 → 49위 · 975 → 1,007pt | HN 3창째(#1 → #32 → #49). 양측 하락 |

직전 창의 동시 화제 5건 중 이번 창에 살아남은 것은 위 2건이고, 나머지 3건은 해소됐다. 해소 방식이 갈린 점이 눈에 띈다 — 두 건은 GeekNews가 상승하는 동안 Hacker News가 top50에서 나갔고, 한 건은 양측이 함께 사라졌다.

| 해소된 화제 | GeekNews | Hacker News | 방향 |
|---|---|---|---|
| Microsoft 'Code Red' 선언(Reuters) | 6위 → 4위 | 직전 39위 228pt → top50 이탈 | 분기(GeekNews↑ × HN 이탈) |
| LLM agents work for their parent companies | 9위 → 6위 | 직전 26위 925pt → top50 이탈 | 분기(GeekNews↑ × HN 이탈) |
| AMD EPYC Zen 6 vs. Zen 1(Phoronix) | 직전 10위 → top20 이탈 | 직전 40위 221pt → top50 이탈 | 양측 소멸 |

정리하면 **동시 화제는 5건 → 2건으로 줄었고, 남은 2건은 모두 양측 동반 하락, 사라진 3건 중 2건은 'GeekNews만 살아남은 분기'**다. HN에서 925pt(26위)를 받고도 다음 창에 top50 밖으로 밀린 LLM 에이전트 글이 대표적이다.

## GeekNews — top5는 전원 생존자, 신규 5건은 모두 6위 밖

자정대 GeekNews는 한국 시간 트래픽이 가장 낮은 구간이라 상판이 거의 움직이지 않았다. top5는 한 건도 새로 들어오지 않고 순서만 바뀌었다.

| 순위 | 항목 | 변화 |
|---|---|---|
| 1 | 오픈 소스에서 월 반복 매출 2만 달러까지: EmailEngine의 여정 | 2 → 1 |
| 2 | 경영진과 대화하는 법: 논쟁하기 전에 금액으로 환산하라 | 4 → 2 |
| 3 | NariQ: BF16 수준의 Qwen3.8-27B를 10GB 파일에 담다 | 1 → 3 |
| 4 | Microsoft, 대규모 장애 이후 'Code Red' 선언(Reuters) | 6 → 4 |
| 5 | Ternary Bonsai 2.5 27B — 5.9GB 유지·에이전트 코딩 강화 | 3 → 5 |

top20에 실제로 새로 들어온 제목은 **5건**이지만 전부 6위 밖에서 출발했다 — Show GN: Claude Navigation(7위·7점), Show GN: ERWeb(10위·10점), HUMAN.md(16위·16점), Fast-Anything(17위·17점), 'AI는 모두에게 같은 조언을 하고 있다'(20위·20점). 워처의 `new_entry` 이벤트는 0건인데, 이는 5위 이내 신규만 잡는 임계값(`new_entry_top_n=5`) 때문이지 신규 진입이 없어서가 아니다. 반대로 이탈은 Tidal(직전 8위)·AMD EPYC(10위)·Show GN: Optimaru(12위)·Show GN: 오사장(18위)·AI 네이티브 서비스(19위) 5건이다.

`rank_jump` 12건은 **상승 7 · 하락 5**로, 자정 창으로는 상승 쪽이 우세했다. 최대 상승은 Show GN: Tern(한글 특화 터미널)과 '2026년 기업용 AI 현황'이 각각 7계단(16→9, 20→13), 최대 하락은 MailFlow(5→14, −9계단)다. Show GN(자작 공개) 항목이 top10에 3건(#7 Claude Navigation · #9 Tern · #10 ERWeb) 올라온 것도 이 시간대의 특징이다.

## Hacker News — top10 중 9건 신규, 생존 23건은 전원 점수 상승

Hacker News는 반대로 상판이 거의 전면 교체됐다. top10에서 살아남은 것은 Matrix Multiplication 하나뿐이다.

| 순위 | 항목 | 점수 |
|---|---|---|
| 1 | Google Releases Cogentic Papers on Integer Factorization, Discrete Logs (신규) | 122pt |
| 2 | Windows Loudness Equalization, reverse-engineered and ported to PipeWire (신규) | 156pt |
| 3 | Button Sounds — professionally-recorded button clicks (신규) | 610pt |
| 4 | Matrix Multiplication is Θ(n^2.25) [pdf] | 1 → 4위, 155 → 816pt |
| 5 | The short age of dragon riders (신규) | 138pt |

신규 진입 **27건**, top50 이탈 **27건**으로 50위권의 54%가 12시간 만에 갈렸다. 생존 23건은 예외 없이 점수가 올랐는데(최소 +12, 최대 +661), 그중 22건이 순위에서 밀렸다. 유일한 상승은 'Ask HN: Anthropic is accusing my company of distilling'(49→25위, 228→381pt)이다. 최대 하락은 Camus 서한집(6→46위, −40계단), TorchTPU 회고(12→43위, −31계단), 도시별 mesh 무선 노드 계산기(10→40위, −30계단) 순이다.

## 점수와 순위가 어긋난 자리

이번 창은 순위와 점수의 역전이 특히 넓게 벌어졌다. 1위 Cogentic 논문 모음이 122pt인데 3위 Button Sounds는 610pt로 **5.0배**이고, 4위 Matrix Multiplication은 816pt다. 6위 Huawei 유죄 평결은 38pt에 불과하고, 12위 Hazel(YC W24) 채용 공고는 1pt다. 반대편에는 1,000pt를 넘기고도 하위에 깔린 글이 몰려 있다 — Nvidia 젠슨 황 CEO 복귀 2,823pt(36위), 2035 came early 1,007pt(49위). 1,000pt대였지만 아예 top50에서 나간 글도 두 건 있다(Ladybird alpha 2,386pt·직전 42위, Internet Archive 기반 무료 TV망 1,071pt·직전 48위). 최신성 가중이 강한 HN 랭킹에서 12시간 전 상판을 덮었던 고점수 글이 통째로 밀려난 전형적 형태다.

이번 창 HN에는 수학 주제가 눈에 띄게 몰렸다 — 1위 정수 인수분해·이산로그 논문, 4위 행렬곱 Θ(n^2.25), 20위 dependent types 대수, 23위 'LLM이 평가한 수학 미해결 문제 순위', 31위 π의 16진수 자릿수 추출 고속 알고리즘까지 top50에 5건이 동시에 올랐다. 서로 다른 URL의 별개 글이며, 직전 창에는 이 묶음이 상판에 없었다.

## 각주

- GeekNews 점수는 1.0~20.0의 랭크 미러값이라 점수-순위 역전 개념이 성립하지 않는다. 위 표의 점수는 순위를 그대로 반영한다.
- `removed_from_top`은 시간감쇠 랭킹 교체에 따른 노이즈다(GeekNews 5건 · HN 27건). 사건으로 세지 않고 건수만 기록했다.
- star-history 리더보드는 100개 항목의 점수가 직전 관측과 완전히 동일해 이벤트 0건이다. 03:01Z·15:01Z 두 빌드가 같은 UTC 날짜(11-25)에 떨어져 정적 스냅샷이 두 번 잡힌 결과로, 순위 변동이 없다는 뜻은 아니다.
