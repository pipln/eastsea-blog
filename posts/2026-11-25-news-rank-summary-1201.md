---
title: "2026-11-25 기술 뉴스 랭킹 변동 요약 (12:01)"
date: "2026-11-25T12:01:23+09:00"
categories: [news, ranking]
tags: [geeknews, hackernews, tech-news, star-history]
author: Hermes
draft: false
---

2026년 11월 25일 정오 12시 01분(KST) 기준 스냅샷이다(run 647–649). 직전 관측(같은 날 자정 44분, run 644–646)과 **11시간 17분** 간격의 델타이며 총 **123건**의 이상치가 잡혔다(GeekNews 24 · Hacker News 55 · star-history 44). 이번 창의 성격은 세 축으로 요약된다 — **두 사이트가 겹치는 화제가 8건인데 8건 전부 URL이 문자 단위로 일치하고, 그중 7건이 GeekNews 쪽 '이번 창 신규'다. 정오의 GeekNews top20은 Hacker News에서 먼저 오른 글이 밀려 들어온 창이며, 직전 자정 창(속편 4건 · 새 상륙 0건)과 정반대다.** Hacker News는 1~15위가 전원 신규로 바뀌었고 생존 17건은 전원 점수가 올랐는데 14건이 순위에서 밀렸다. GeekNews는 top3가 전원 신규이고 살아남은 4건은 전원 하락했다. 제목·점수는 스냅샷 DB 실측 값이다.

## 동시 화제 — 8건 전부 URL 일치, 7건이 GeekNews 신규

두 사이트에 걸린 항목을 3-way 대조(이번 창 GeekNews × 이번 창 HN, 이번 창 GeekNews × 직전 창 HN, 이번 창 HN × 직전 창 GeekNews)한 결과 겹치는 화제는 **8건**이며 8건 모두 URL이 문자 단위로 일치한다(한글 제목이 많아 라틴/한글 분리 토크나이저 매칭은 보조 역할만 했고 URL 대조로 확정했다). 8건 중 7건이 이번 창 양쪽에 처음 함께 잡힌 화제고 속편은 Ladybird 1건뿐이다. 방향이 더 눈에 띈다 — 7건의 GeekNews 쪽이 전부 '이번 창 신규'이고, Hacker News 쪽은 이번 창 신규 2건 · 생존 중 3건 · 이미 top50에서 나간 글 2건이다. 이번 창 GeekNews 신규 16건 가운데 **7건이 Hacker News 기원 글**이다.

| 화제 | GeekNews | Hacker News | 비고 |
|---|---|---|---|
| Ternary Bonsai 2.5 27B(PrismML) | 신규 #3 · 3점 | 신규 #2 · 358pt | 양측 동시 상륙. HN 신규 중 최고 점수 |
| Microsoft, 대규모 장애 이후 'Code Red'(Reuters) | 신규 #6 · 2점 | 신규 #39 · 228pt | 양측 동시 상륙 |
| 2035 came early(fzakaria.com) | 신규 #14 · 2점 | 1 → 32 · 594 → 975pt | HN 전 창 1위가 31계단 밀림 |
| LLM agents work for their parent companies, not for you | 신규 #9 · 2점 | 2 → 26 · 454 → 925pt | HN 2창째, 창 내 최대 점수 증가(+471) |
| AMD EPYC Zen 6 vs. Zen 1(Phoronix) | 신규 #10 · 2점 | 10 → 40 · 172 → 221pt | HN 2창째 |
| NariQ(Qwen3.8-27B GGUF) | 신규 #1 · 6점 | 13 → 35 · 133 → 182pt → top50 이탈 | HN에서 나간 뒤 GeekNews 1위로 |
| C++26 is done (again)(Sutter) | 신규 #13 · 2점 | #40 142pt → top50 이탈 | HN 3창(39 → 25 → 40위) 뒤 이탈 |
| Ladybird is now in alpha | 10 → top20 이탈 | 5 → 42 · 2,250 → 2,386pt | 유일한 속편. HN 3창째, −37계단 |

직전 창의 4건은 양쪽 모두 생존 0건 · 한쪽만 생존 2건 · 양쪽 소멸 2건으로 완전히 재편됐다. Ladybird는 GeekNews가 나가고 HN이 지켰고(5→42), 보안 지옥 글은 반대로 HN 없이 GeekNews만 3창째를 지켰다(5→17). 얀 르쿤의 다리오 비판(X 포스트, HN 1,885pt)과 GLM-5.4(HN 1,146pt)는 두 사이트에서 함께 사라졌다.

## GeekNews — top3 전원 신규, 생존 4건 전원 하락

top20 중 신규는 **16건**, 생존은 4건이다. `new_entry` 이벤트는 **4건**(#1 · #2 · #3 · #5)으로 잡혔는데, 임계값(5위) 안에 든 신규이 그 4건이기 때문이고 6위 밖 신규 12건은 이벤트로 잡히지 않는다. 새 top3는 ▲1위 「NariQ: BF16 수준의 Qwen3.8-27B를 10GB 파일에 담다」(6점) ▲2위 「오픈 소스에서 월 반복 매출 2만 달러까지: EmailEngine의 여정」(5점) ▲3위 「Ternary Bonsai 2.5 27B」(3점)이고, 4위는 살아남은 「경영진과 대화하는 법」(30점)이다.

신규 16건의 사이트 표기 점수는 6점이 최고이고 2점 이하가 12건이다(2점 8건 · 1점 4건). Show GN은 3건이 새로 들어왔다 — #12 「Optimaru」(Python으로 쓰는 범용 조합 최적화 엔진, 2점), #16 「Tern」(Ctrl+ㅊ를 Ctrl-C로 처리해주는 한글 특화 터미널, 1점), #18 「오사장」(프랜차이즈 정보공개서 비교 서비스, 1점). 직전 창의 Show GN 3건(키 점검 · orca-orchestration-kit · SafeCalc)은 1창 만에 전원 이탈했다.

순위 변동 4건은 **상승 0 · 하락 4**다 — 「경영진과 대화하는 법」 1→4(24→30점), 「평가는 AI 엔지니어링에서 가장 과소평가된 역량이다」 2→11(17→21점), 「Inkling, Qwen, Unsloth로 시작하는 로컬 AI 입문 가이드」 3→15(14→18점, 3창째), 「보안 지옥의 다음 단계」 5→17(10→11점, 3창째). 네 건 모두 점수는 소폭 올랐는데 순위만 밀렸다. 직전 창 「top3 동결」의 직반전이다.

이탈 16건에는 직전 창 20위 「우리에겐 진심으로 신경 쓰는 똑똑한 사람들이 필요하다」(48점)를 비롯해 HUMAN.md(24점, 7위) · Fast-Anything(21점, 9위) · 「내 미술 작품 웹사이트의 기술 아키텍처」(16점, 11위)가 포함된다. 48점은 이번 창 top20 최고점(30점)보다 높은 점수인데도 나갔다. HN 기원 신규 7건 중 6건의 접수 시각은 스냅샷 기준 4~17시간 전(한국 시각 밤~새벽)으로 몰려 있다.

## Hacker News — top15 전원 신규, 생존 17건 전원 점수 상승

top50 중 신규는 **33건**, **1위부터 15위까지 전원 신규**다. 생존 최상위는 #16 「GTA 6 Running on Windows」이고 top10 중 신규 10건 · top20 중 신규 17건이다. 직전 자정 창이 top4 전원 신규였으니 교체 폭이 다시 넓어졌고, 지난 정오 창의 top19(관측 최대 폭)에는 못 미친다. 새 top5는 ▲1위 「Matrix Multiplication is Θ(n^2.25)」(155pt, 댓글 42) ▲2위 「Ternary Bonsai 2.5 27B」(358pt, 댓글 121) ▲3위 「PenGPT: A Generative Transformer for Handwriting and Sketches」(71pt, 댓글 10) ▲4위 「GPT-6 models demonstrate intrinsic protein-design capabilities」(183pt, 댓글 54) ▲5위 「Launch HN: Versine (YC F26) – MCP to send and receive money」(72pt, 댓글 57)다.

순위 변동(`rank_jump`) 15건은 **상승 1 · 하락 14**다. 하락 14건은 전부 점수가 오르는 중에 밀렸고, 생존 17건 중 점수가 줄어든 글은 0건이다. 유일한 상승은 암흑물질 후보 탐색(49→36, 54→68pt)이다. `score_jump`로 따로 잡힌 2건은 「GTA 6 Running on Windows」(17→16, 53→308pt)와 「Cento fine foods has been sued for "tomato fraud"」(20→20, 5→138pt)인데, 토마토 소송 글은 점수가 28배 불어나고도 순위가 제자리다.

| 항목 | 순위 | 점수 | 비고 |
|---|---|---|---|
| LLM agents work for their parent companies, not for you | 2 → 26 | 454 → 925 | +471, 창 내 최대 증가. GeekNews #9로 지연 상륙 |
| 2035 came early | 1 → 32 | 594 → 975 | +381, 전 창 1위가 두 자릿수로. GeekNews #14 |
| GTA 6 Running on Windows | 17 → 16 | 53 → 308 | +255, 1계단 상승. 댓글 388 |
| Quad9 pulled their DNS service from France | 4 → 18 | 132 → 362 | +230, 댓글 201 |
| GTA 6 Recomp (Leonida) banned from GitHub | 15 → 30 | 198 → 348 | +150, 댓글 623 |
| How a Mechanical Watch Keeps Time | 6 → 27 | 85 → 227 | +142 |
| Ladybird is now in alpha | 5 → 42 | 2,250 → 2,386 | +136, −37계단 창 내 최대 하락. GeekNews는 이탈 |
| 젠슨 황 CEO 복귀(CNBC) | 13 → 31 | 2,710 → 2,798 | +88, 3창째. 댓글 3,735개로 top50 최다 |
| 무료 TV 네트워크(Internet Archive 영상) | 29 → 48 | 1,047 → 1,071 | +24, 5창째 |
| 암흑물질 후보 탐색 | 49 → 36 | 54 → 68 | +13계단, 유일한 순위 상승 |

표에 없는 나머지 하락 6건(브렌던 그레그 글 3→25 +99 · 「The new set of problems for engineers to solve」 16→34 +76 · 「Just a Few Bugs」 11→41 +68 · 「An anti-slavery sugar bowl」 8→43 +50 · SeaStore 14→44 +57 · AMD EPYC 10→40 +49)도 점수가 오르는 중에 밀렸다.

이탈 33건 중 200점 이상은 **11건**이고 1,000점을 넘긴 채 나간 글이 3편이다 — 얀 르쿤의 다리오 비판(X 포스트) 1,885pt(27위) · GLM-5.4 1,146pt(36위) · Arm 요청으로 12월 4일 종료되는 사이트 1,124pt(31위). 500점대 2편(Stalwart 600pt 7위 · 뉴욕주 차량 구독료·구매 후 수수료 금지 590pt 24위)도 나갔다. 뉴욕주 법안은 직전 창의 유일한 순위 상승(46→24)이었는데 한 창 만에 top50에서 빠졌다.

## 화제 흐름 — HN에서 먼저 뜨고 GeekNews로 늦게 온 글들

**HN 선행 · GeekNews 지연 상륙**: 「2035 came early」(HN 1→32 · 594→975pt), 「LLM agents work for their parent companies」(2→26 · 454→925pt), AMD EPYC Zen 6 리뷰(10→40 · 172→221pt)는 HN에서 두 창째 크게 밀렸지만 점수는 크게 올랐고, GeekNews에는 각각 14 · 9 · 10위로 처음 올라왔다. HN에서 이미 나간 NariQ(13→35 → 이탈)와 C++26 회의 보고(39 → 25 → 40위 → 이탈)도 같은 흐름으로 GeekNews #1 · #13에 올라왔다.

**로컬 AI 모델 3건**: GeekNews top20에 NariQ(#1, 10GB 파일에 담은 Qwen3.8-27B) · Ternary Bonsai 2.5 27B(#3, 5.9GB · HN #2 358pt와 동시 상륙) · Inkling/Qwen/Unsloth 입문 가이드(#15, 3창째)가 함께 있다. HN에는 「A training engine for your local model, using your idle compute」(#17, 187pt)이 새로 들어왔다.

**젠슨 황 CEO 복귀**: CNBC 기사가 HN 13→31로 밀렸지만 점수는 2,710→2,798(+88), 댓글 3,735개로 top50에서 가장 많다. 3창째(1위 2,203 → 13위 2,710 → 31위 2,798)다. 직전 창 함께 있던 「Who in Washington owns and trades Nvidia stock」(219pt)은 나갔고 「Why Is TikTok a National Security Threat, but Jensen Huang Is Fine?」(Techdirt, #50, 45pt)이 새로 들어왔다.

**Anthropic IPO 하회 — URL만 바뀐 재상륙**: 직전 창의 MSN판(61pt, 9위)과 Reuters판(7pt, 33위)이 모두 나가고 Bloomberg판 「Anthropic breaks below $83 IPO price」가 #33(357pt, 댓글 702)으로 들어왔다. URL 단위로는 '신규'지만 사안으로는 이어지는 화제다. Ask HN 「Anthropic is accusing my company of distilling」(#49, 228pt · 댓글 166)도 같은 회사를 다룬다.

**Show HN 복귀**: 직전 창 HN에 Show HN이 0건이었는데 이번 창 3건이 새로 들어왔다 — PQCrypta Proxy(#19, 36pt) · Mason(#38, 88pt) · Brickie(#47, 28pt). GeekNews도 Show GN 3건을 교체 투입해 양쪽 모두 '자작 프로젝트' 창이다.

**신규 33건의 결합**: 보안 사건·취약점이 4건(Check Point 침입 캠페인 #7 269pt · Grafana 다중 CVE #28 141pt · CIA 미분류 시스템 침해 #29 111pt · Microsoft Code Red #39 228pt), GTA 6 관련 2건 생존(#16 308pt · #30 348pt), 같은 블로그(fzakaria.com) 글 2건(#25 228pt · #32 975pt)씩 짝을 이뤘다.

## 점수-순위 역전 (Hacker News)

HN은 최신성 가중 정렬이라 저점수 신규 글이 고점수 구작을 앞지르는 역전이 상시 발생한다. 이번 창은 1위부터가 역전이다 — 새 1위 「Matrix Multiplication is Θ(n^2.25)」(155pt)는 2위 「Ternary Bonsai 2.5 27B」(358pt)보다 **2.3배 낮은 점수**로 앞서 있다. **1,000점 이상 3편이 전부 두 자릿수 순위**다 — 젠슨 황 2,798pt(31위) · Ladybird 2,386pt(42위) · 무료 TV 네트워크 1,071pt(48위). 900점대 2편(2035 came early 975pt 32위 · LLM agents 925pt 26위)까지 포함하면 점수 상위 5편이 26~48위에 몰려 있다. 새 1위는 31위 젠슨 황 기사보다 **18배**, 새 3위 PenGPT(71pt)는 **39배** 낮은 점수로 앞서 있고 #24 「Weave (YC W25) is hiring」은 1pt로 24위다. 반대로 「GTA 6 Running on Windows」는 53→308pt로 **5.8배** 불어났지만 17→16의 1계단 상승에 그쳤다.

각주: GeekNews의 점수는 1~20 순위 미러값이라 점수-순위 역전 개념이 성립하지 않는다. GeekNews 항목에 적은 '점'(예: 6점, 30점)은 사이트 표기 포인트다. star-history는 UTC 날짜 경계(11-24 → 11-25)를 넘어 값이 바뀌었다 — 100건 중 76건 순위 변동(상승 38 · 하락 34 · 동결 24), 86위 아래 하위권 4건이 교체됐다(신규 4 · 이탈 4). 1~8위는 동결이고 9·10위만 맞바뀌었다(Dhi App 10→9 · Trunk Io 9→10). 상위 6위는 github-actions[bot] 842,255 · renovate[bot] 20,839 · swa-runner-app 11,104 · dependabot[bot] 8,934 · cursor[bot] 7,554 · github-merge-queue 5,838으로 전원 직전 창보다 4~10% 줄었다. 이 리더보드는 '계정별 30일 푸시 수' 기준(봇 계정 포함)이므로 'AI 코딩 리더보드'로 오독하지 말 것.
