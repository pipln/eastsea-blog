---
layout: post
title: "아침 뉴스 브리핑 — 2026년 11월 26일"
date: 2026-11-26
categories: [briefing]
tags: [AI, 구글, 인수분해, 암호, 1Password, 코스피, PCE, 관세, SpaceX, OpenAI, 비트코인, 엔비디아, GitHub, Ubisoft, EA, Qiita]
author: MissKim
---

## Executive Summary

코스피가 하루 만에 6.16% 빠지며 4,697.51로 마감했다. 전날 젠슨 황 복귀 랠리로 얻은 6.32% 상승분을 하루 만에 전부 반납했고, 개장 8분 만에 매도 사이드카가 걸렸다. 삼성전자 시가총액은 8개월 만에 1,000조원 아래로 내려갔다.

미국은 10월 PCE가 3.8%로 다시 목표의 두 배에 붙으면서 12월 인상이 사실상 확정됐다. 같은 날 나스닥은 개장 직후 -3%대까지 밀렸다가 -0.62%로 마감했고, 다우는 +0.33%로 버텼다.

구글은 11월 25일 'Cogentic' 이름으로 정수 인수분해 L_N[1/3, O(1)]과 고정 표수 체의 이산로그 준준다항 시간 알고리즘을 포함한 논문 6편을 공개했다. RSA·ECC의 보안 가정을 다시 계산해야 한다는 뜻이다.

1Password는 직원의 회사 GitHub 계정이 뚫려 백엔드에 악성 코드가 푸시됐고, 고객 이름·이메일·초대 토큰이 유출됐다고 공시했다. 금고(vault) 데이터는 안전하다고 밝혔다.

---

## 📉 시장 스냅샷 (Yahoo Finance 차트 API 실측 + 국내 마감 기사)

| 자산 | 종가 | 기준 | 등락 |
|---|---|---|---|
| S&P 500 | 7,182.78 | 11-25(수) | **-0.25%** |
| 나스닥 종합 | 23,585.80 | 11-25(수) | **-0.62%** (개장 -3.3%) |
| 다우존스 | 50,182.95 | 11-25(수) | **+0.33%** |
| 코스피 | 4,697.51 | 11-25(수) 15:30 | **-6.16%** |
| 코스닥 | 650.64 | 11-25(수) 15:30 | **-2.29%** |
| 원/달러 | 1,343.2 | 11-25 주간 마감 | **+0.8원** |
| 비트코인 | $60,158 | 11-25(수) | **+0.69%** (장중 $57K) |

지수 간 온도차가 이틀 연속 뒤집혔다. 24일에는 코스피가 +6.32% 오르는 동안 뉴욕이 밀렸고, 25일에는 뉴욕이 -0.25%로 버티는 동안 코스피가 -6.16% 빠졌다. 한국 시장이 미국 세션의 하루 전 신호에 후행 반응하는 패턴이 굳어지고 있다.

---

## AI · 테크

### 1. **[핵심] 구글 'Cogentic', 정수 인수분해 L_N[1/3, O(1)] 공개 — RSA의 시간표가 흔들린다** (Google Research / Hacker News)

구글 리서치 저장소에 11월 25일자로 **Cogentic Research Papers** 6편이 올라왔습니다. 제목은 「Unconditional Integer Factorization in Time L_N[1/3, O(1)]」, 「Discrete Logarithms in Finite Fields of Fixed Characteristic in Sub-Quasi-Polynomial Time」, 「Unconditional Deterministic Subexponential Integer Factoring and Quadratic Nonresidue Construction」, 「An Exact Polynomial-Time Algorithm for 1/5-Approximate Nash Equilibria in Bimatrix Games」, 「Breaking the Square-Root Barrier for the Log-Rank Conjecture」, 「Unconditional Edge Reconstruction of Simple Graphs of Linear Density」입니다. HN에서 이 묶음은 392점·302코멘트로 토론됐고, 스콧 애런슨 블로그 댓글에서는 "구글 논문은 사람 전문가 리뷰를 거친다"는 설명이 인용되며 OpenAI 논문과의 검증 수준 차이가 쟁점이 됐습니다. 시사점은 명확합니다. **인수분해 준지수(subexponential) 알고리즘이 '조건부'가 아니라 '무조건(unconditional)'으로 제시되면, RSA-2048의 보안 마진은 '깨질지 여부'가 아니라 '키 길이를 얼마나 늘려야 하는지'의 문제로 바뀝니다.** 아직 Lean 형식화가 없는 만큼 오늘 당장 키를 버릴 필요는 없지만, 장기 보관 데이터의 암호 스위트를 재검토할 근거는 생겼습니다.

→ 원문: [Cogentic Research Papers (google-research, 2026-11-25)](https://github.com/google-research/google-research/tree/master/cogentic_research_papers)
→ 교차확인: [Hacker News — Google releases Cogentic papers on integer factorization, discrete logs and more (392 pts, 302 comments)](https://news.ycombinator.com/item?id=50642086)
→ 분석: [Scott Aaronson — "The magnitude of what's now happening" (댓글에서 검증 수준 논쟁)](https://scottaaronson.blog/?p=10379)

### 2. **[핵심] 1Password, 직원 GitHub 계정 뚫려 백엔드에 악성 코드 푸시 — 고객 이름·이메일·초대 토큰 유출** (1Password 공식 / Hacker News)

1Password는 11월 25일 공식 블로그에서 **11월 19일부터 조사한 보안 사고**를 공개했습니다. 공격자는 **직원의 회사 GitHub 계정**을 장악해 소스코드에 접근하고 **백엔드 서비스에 악성 코드를 푸시**했으며, 그 코드가 고객 계정 데이터를 내려받았습니다. 유출 범위는 이름·이메일, 계정·사용자 식별자, 가입·마지막 로그인 시각, 공개키, SRP 파라미터, 그리고 **초대·프로비저닝·계정복구 메일 링크에 들어 있던 토큰**입니다. 회사는 금고(vault) 내용과 비밀키는 접근되지 않았고 내부 자격증명 로테이션을 완료했다고 밝혔습니다. HN에서 748점·311코멘트가 달렸습니다. 시사점은 공급망 공격의 진입점이 빌드 시스템이 아니라 **'개발자의 SaaS 계정'**이라는 점 — GitHub 권한이 곧 배포 권한인 조직은 오늘 계정 단위 조건부 접근(conditional access)과 백엔드 코드 서명 검증을 점검할 가치가 있습니다. 유출된 초대 토큰은 정상처럼 보이는 피싱 메일의 재료가 되므로, 최근 1Password 초대·복구 메일을 받았다면 링크를 누르지 말고 앱에서 직접 확인하십시오.

→ 원문: [Update on a security incident involving 1Password's backend systems (1Password 공식)](https://1password.com/blog/update-on-nov-2026-security-incident)
→ 교차확인: [Hacker News — 1Password Security Incident (748 pts, 311 comments)](https://news.ycombinator.com/item?id=50644817)

### 3. **OpenAI, ChatGPT의 Chat·Work 통합 시작 — 사용량 미터가 하나로 합쳐져 '헤비 유저'가 먼저 제한에 걸린다** (TechCrunch)

TechCrunch는 11월 25일 OpenAI가 **Chat과 Work를 통합한 경험을 Pro 구독자 일부에게 먼저 배포**하기 시작했다고 전했습니다. 제품 리드 티보 소티오가 X에 밝힌 내용이며, OpenAI 대변인은 "대부분 사용자는 체감이 없지만 **일부 최상위 헤비 유저는 사용 한도에 더 빨리 도달할 수 있다**"고 확인했습니다. 통합 후에는 두 모드가 **하나의 사용량 미터를 공유**하고, Instant 모델 대화만 무제한으로 남습니다. 사용자 커뮤니티에서는 "같은 요금을 내는데 추론 모델 접근이 줄어드는 것 아니냐"는 반발이 나왔습니다. 시사점은 구독 요금제의 가치가 '모델 접근'에서 **'토큰 예산 배분'**으로 이동한다는 것 — 에이전트 워크플로를 ChatGPT 위에 얹은 팀은 이번 주 안에 실제 소모 패턴을 측정해 두는 편이 안전합니다.

→ 원문: [OpenAI says heavy ChatGPT users may hit limits sooner as it merges Chat and Work (TechCrunch)](https://techcrunch.com/2026/11/25/openai-says-heavy-chatgpt-users-may-hit-limits-sooner-as-it-merges-chat-and-work/)

### 4. **FCC, SpaceX에 '위성 100만 기' 조건부 승인 — 궤도 AI 데이터센터가 규제 관문을 통과했다** (TechCrunch / Ars Technica)

FCC는 11월 25일 SpaceX의 **궤도 데이터센터 사업을 위해 최대 100만 기 위성을 배치·운용하는 안을 조건부 승인**했습니다. 아마존, 비아샛, 미국천문학회는 궤도 파편·혼잡, 전파 간섭, 환경 영향을 이유로 반대했고 아마존은 신청서가 "실질 계획이 아니라 원대한 구상"이라고 비판했지만, FCC는 과학연구 위협 등 일부 우려를 **자기 권한 밖**이라는 이유로 기각했습니다. 브렌던 카 FCC 의장은 "미국의 리더십에 필요한 컴퓨팅 인프라의 새로운 개척지"라고 논평했고 머스크는 X에서 사의를 표했습니다. 시사점은 AI 인프라 경쟁의 단위가 '데이터센터'에서 **'궤도·전력·주파수'**로 확장됐다는 것 — 지상 전력망·부지 규제가 병목인 시대에 우회로가 열린 셈이지만, 승인은 착수가 아니라 조건부 허가일 뿐입니다.

→ 원문: [FCC approves SpaceX plan to put AI data centers in space (TechCrunch)](https://techcrunch.com/2026/11/25/fcc-approves-spacex-plan-to-put-ai-data-centers-in-space/)
→ 교차확인: [The FCC has authorized SpaceX to deploy 1 million satellites (Ars Technica)](https://arstechnica.com/space/2026/11/the-fcc-has-authorized-spacex-to-deploy-1-million-satellites/)

### 5. **예측시장, OpenAI 상장 기대를 2027년 이후로 밀어냈다 — 앤스로픽 데뷔 참사가 만든 '학습효과'** (CNBC)

CNBC는 11월 25일 **예측시장(Kalshi·Polymarket류)에서 OpenAI의 IPO 시점 기대가 2027년 이후로 이동**했다고 보도했습니다. 불과 3주 전만 해도 "2026년 안에 상장" 쪽에 무게가 실렸지만, 앤스로픽이 상장 5일 만에 공모가를 크게 밑돌면서 **AI 기업의 공모가 산정 자체가 재평가**됐습니다. 같은 기사에서 CNBC는 예측시장이 주식·금리 전망에서도 'AI 매도세' 이후 기대를 낮추고 있다고 전했습니다. 시사점은 비상장 AI 기업의 밸류에이션이 **사모 라운드의 가격이 아니라 2차 시장·예측시장의 확률**로 먼저 조정된다는 것 — 상장을 전제로 스톡옵션을 설계한 팀은 할인율 가정을 다시 잡아야 합니다.

→ 원문: [Prediction markets push OpenAI IPO expectations later into 2027 (CNBC)](https://www.cnbc.com/2026/11/25/openai-ipo-odds-2027-prediction-markets.html)

---

## 경제 · 금융

### 6. **[핵심] 코스피 -6.16% 4,697.51 — 개장 8분 만에 사이드카, 외국인 1조9천억 순매도, 삼성전자 시총 1,000조 붕괴** (시사저널e / 메트로서울)

11월 25일 코스피는 전 거래일보다 **308.52포인트(6.16%) 내린 4,697.51**에 마감했습니다. 종가 기준 **1월 13일 이후 10개월여 만의 최저치**로, 전날 젠슨 황 엔비디아 CEO 복귀로 올랐던 6.32%를 하루 만에 전부 반납한 것입니다. **개장 8분 만에 매도 사이드카**가 발동됐고 4거래일 연속 사이드카 행진입니다. 외국인은 유가증권시장에서 **1조8,994억원 순매도**(11월 6일 이후 최대, 넥스트레이드 포함 2조2,817억원)했고 개인은 1조4,232억원을 순매수했지만 지수를 받치지 못했습니다. 코스닥도 **2.29% 내린 650.64**로 마감했고, 원/달러는 0.8원 오른 **1,343.2원**입니다. 업종별로 전기·전자 **-9.81%**, 제조 -7.48%, 기계·장비 -6.18%인 반면 제약 **+4.65%**, IT서비스 +2.75%로 순환매가 헬스케어로 들어왔습니다. 삼성전자는 **170,700원(-9.87%)**으로 시가총액이 **997조9,598억원**, 8개월 만에 1,000조원 아래로 내려갔고 SK하이닉스는 **1,030,000원(-11.05%)**으로 4월 10일 이후 최저 종가입니다. 시사점은 지수 방향이 **반도체 두 종목의 시가총액에 사실상 종속**돼 있다는 것 — "코스피는 삼성전자·SK하이닉스 레버리지 상품"이라는 말이 다시 확인됐고, 헬스케어 순환매가 지수 하락을 막지 못했습니다.

→ 원문: [[e마켓클로징] 삼성전자 시총 1000兆 붕괴···코스피 4600선 후퇴 (시사저널e)](https://www.sisajournal-e.com/news/articleView.html?idxno=425321)
→ 교차확인: [[마감시황] 외국인 1.9조 '팔자'에 코스피 6% 급락…4거래일 연속 사이드카 (메트로서울)](https://www.metroseoul.co.kr/article/20261125500481)

### 7. **[핵심] 미 10월 PCE 3.8%·근원 3.3%, 3분기 GDP 4.3% — 12월 인상이 '사실상 확정'으로** (CNBC / Cointelegraph)

미 상무부는 11월 25일 **10월 PCE 물가가 전년비 3.8%** 상승했다고 발표했습니다. 연준 목표의 두 배에 가까운 수치로, **근원 PCE는 3.3%**로 다우존스 컨센서스를 0.1%포인트 웃돌았습니다. 같은 날 3분기 GDP 성장률 수정치는 **연율 4.3%**로 상향돼 "통화정책이 경기를 제약하지 않고 있다"는 해석을 강화했습니다. CNBC는 이 조합이 **12월 금리 인상 기대를 굳히고 2027년 정책 경로에 물음표를 남긴다**고 분석했고, Cointelegraph도 "12월 인상이 이미 가격에 반영된 상태"라고 전했습니다. 시장 반응은 엇갈렸습니다. 나스닥은 개장 직후 **-3.3%**(22,948)까지 밀렸다가 **23,585.80(-0.62%)**으로 낙폭을 거의 되돌렸고, S&P500은 **7,182.78(-0.25%)**, 다우는 **50,182.95(+0.33%)**로 플러스 전환했습니다. 장중에는 나스닥이 -4%까지 밀렸다는 보도도 나왔습니다. 시사점은 **'금리 인상 + AI 설비투자'라는 상충 조합**이 아직 붕괴하지 않았다는 것 — 인플레가 잡히지 않으면 할인율이 다시 올라가고, 그때는 지수보다 반도체·성장주가 먼저 반응합니다.

→ 원문: [PCE inflation report October 2026: Fed's preferred inflation gauge accelerated to 3.8% (CNBC)](https://www.cnbc.com/2026/11/25/pce-inflation-report-october-2026.html)
→ 교차확인: [US inflation rises to 3.8% ahead of December Fed meeting, clouding crypto outlook (Cointelegraph)](https://cointelegraph.com/news/us-inflation-rises-december-fed-meeting-clouding-crypto-outlook)

### 8. **USTR, 한국 포함 14개 경제권에 '과잉생산 관세' 예고 — 한국은 15% 상한 유지, 다만 인상 여지는 남아** (연합뉴스 / 한겨레)

미 무역대표부(USTR)는 11월 25일(현지시각) **한국·중국·일본·EU 등 14개 경제권에 과잉생산 관세를 제시**했습니다. 한국·대만·인도·인도네시아에는 최고 25%, 일본·EU·스위스·싱가포르에는 최고 20%, 중국에는 20%가 제안됐습니다. 다만 미국은 무역협정을 맺은 국가에 합의 관세율을 지키기로 했고, 고위 당국자는 "한국·일본·대만·EU의 전체 관세율은 초기에는 총 15%를 넘지 않을 것"이라고 확인했습니다. 한국은 기존 **강제노동 관세 12.5%에 2.5%포인트가 더해져 15%**가 될 전망입니다. 이 조치는 2월 연방대법원 판결로 무효화된 상호관세 체제를 **무역법 301조**라는 다른 근거로 사실상 복원하려는 시도로 읽힙니다. 즉시 부과가 아니라 공청회·의견수렴·비공개 협상을 거쳐 **이르면 내년 1월 시행** 전망입니다. 시사점은 한국의 관세 상한이 '15%'로 고정된 것이 아니라 **협상 테이블 위에 올라간 숫자**라는 점 — 일부 보도는 "25% 인상 여지가 남아 있다"고 전했습니다. 반도체·자동차 수출 비중이 큰 국내 기업의 2027년 원가 가정에는 관세율 시나리오를 2개 이상 두는 편이 안전합니다.

→ 원문: [美, 중국 등 14개 경제주체에 과잉생산 관세…韓 15% 상한 유지(종합2보) (연합뉴스)](https://www.yna.co.kr/view/AKR20261125185752071)
→ 교차확인: [미국, '과잉생산 관세' 예고…"한국은 15% 상한 유지" (한겨레)](https://www.hani.co.kr/arti/international/international_general/1289145.html)

---

## 블록체인 · 암호화폐

### 9. **비트코인, 장중 $57K까지 밀린 뒤 $60,158로 회복 — 11월 ETF 순유출은 $49.4억으로 월간 최대** (Cointelegraph / CoinDesk)

11월 25일 비트코인은 미국 관세 우려와 PCE 발표 직후 **$57,000 부근까지 밀렸다가 $60,158(+0.69%)**로 되돌아왔습니다(CoinDesk 라이브 기준 $60,362). 같은 기간 이더와 디파이 토큰은 비트코인보다 두 배 강도로 하락했다는 보도가 나왔습니다. 미국 현물 비트코인 ETF는 11월 누적 **$49억4,000만 달러 순유출**로 2024년 1월 출시 이후 월간 최대를 기록했고, 6거래일 연속 유출이 이어졌습니다(전일 $2억3,160만 달러). 이전 기록은 6월의 $45억1,000만 달러였습니다. 시사점은 비트코인이 **'디지털 금'이 아니라 AI 주식의 고베타 자산**으로 계속 거래된다는 것 — 금리가 오르고 AI 주식이 흔들리는 날 비트코인이 같이 빠지고, ETF 자금은 자동으로 환매됩니다. 온체인에서는 스테이블코인 30일 환매가 $120억을 넘어 2022년 약세장과 유사한 패턴이 관측됐습니다.

→ 원문: [Bitcoin ETFs near $5B in record monthly outflows amid AI stock rout (Cointelegraph)](https://cointelegraph.com/markets/bitcoin-etfs-near-record-5-billion-monthly-outflows)
→ 교차확인: [Live updates: Bitcoin climbs from lows as markets bounce from big early declines (CoinDesk)](https://www.coindesk.com/business/2026/11/25/live-updates-bitcoin-slips-back-under-usd60-000-as-hype-leads-altcoins-lowe)

### 10. **엔비디아, 2018년 '채굴 매출 은폐' 집단소송에 2억9,000만 달러 합의** (Cointelegraph)

엔비디아가 **2018년 주주 집단소송**을 2억9,000만 달러에 합의했습니다. 법원에 11월 24일 제출된 합의안은 **2017년 8월 10일~2018년 11월 15일**에 엔비디아 보통주를 사들여 손실을 본 투자자들을 대상으로 하며, 법원 승인이 남아 있습니다. 원고 측은 엔비디아가 **2017~2018년 크립토 붐 당시 게이밍 GPU 수요가 채굴에서 왔다는 사실을 숨기고 매출 원천을 오도**했다고 주장했습니다. 시사점은 두 가지입니다. 첫째, AI 수요와 게이밍 수요를 구분해 설명하라는 압력이 **8년 전 크립토 때보다 훨씬 강해졌습니다**(이번엔 데이터센터 매출 세그먼트가 분리 공시됨). 둘째, 크립토 사이클의 회계 리스크는 **한 세대(8년) 뒤에 현금으로 청구**됩니다.

→ 원문: [Nvidia agrees to $290M settlement over crypto mining revenue disclosures (Cointelegraph)](https://cointelegraph.com/news/nvidia-agrees-290m-settlement-crypto-mining-revenue-disclosures)

---

## GitHub · 개발자 트렌드

### 11. **GitHub 데일리 트렌딩 — 'Claude-BugHunter'가 하루 1,644★, 스킬 번들이 보안·심사 영역으로 확장** (GitHub)

11월 26일 기준 데일리 트렌딩 1위는 **elementalsouls/Claude-BugHunter**(하루 **1,644★**, 누적 22,825★, 5월 생성)입니다. 소개문은 "버그헌팅·외부 레드팀용 Claude Code 스킬 번들 — **84개 스킬, 15개 슬래시 커맨드, 24개 취약점 클래스에 걸친 681개 공개 리포트 패턴**"을 내세웁니다. 뒤를 이어 **firecrawl/firecrawl**(1,131★), **HKUDS/CLI-Anything**(1,094★), **mvanhorn/last30days-skill**(950★, Reddit·X·YouTube·HN·Polymarket을 묶어 리서치하는 에이전트 스킬), **obra/superpowers**(902★), **mjmirza/app-store-compliance**(853★, 앱스토어 반려 패턴 124종·규제 데드라인 114건 추적)가 올랐습니다. 어제 다룬 CLI-Anything·minimind·diagram-design이 여전히 상위권입니다. 시사점은 에이전트 스킬 시장이 **'범용 생산성'에서 '규제·보안·심사'라는 고통이 큰 영역**으로 이동한다는 것 — 반려·취약점·감사처럼 실패 비용이 큰 작업이 먼저 패키지화되고 있습니다.

→ 원문: [elementalsouls/Claude-BugHunter — Claude Code skill bundle for bug hunting (GitHub)](https://github.com/elementalsouls/Claude-BugHunter)
→ 교차확인: [mjmirza/app-store-compliance — Avoid common App Store and Google Play rejections (GitHub)](https://github.com/mjmirza/app-store-compliance)

### 12. **"Clojure 코어를 Lean으로 검증했더니 미보고 버그 17건" — 형식검증이 실무 코드로 내려왔다** (s.conao3.com / Hacker News)

개발자 conao3는 11월 25일 글에서 **Clojure 코어 라이브러리의 함수들을 Lean 4로 옮겨 명세를 쓰고 검증하는 과정에서 미보고 버그 17건을 발견**했다고 공개했습니다. HN에서 138점·51코멘트가 달렸고, 논의의 초점은 "AI가 코드를 쓰는 시대에 **명세를 누가 쓰는가**"였습니다. 같은 날 GitHub는 상태 페이지에 "여러 GitHub 제품에서의 성능 저하를 조사 중"이라는 인시던트를 올렸습니다(HN 172점). 시사점은 도구 체인의 무게중심 이동입니다. 코드 생성 비용이 0에 가까워질수록 **검증·명세 작성 비용이 새로운 병목**이 되고, Lean·TLA+ 같은 형식 도구가 '연구실 도구'에서 'CI 파이프라인 도구'로 내려오는 중입니다.

→ 원문: [I checked Clojure's core with Lean and found unreported bugs (s.conao3.com)](https://s.conao3.com/blog/2026/e4f733)
→ 교차확인: [Hacker News — I checked Clojure's core with Lean and found unreported bugs (138 pts, 51 comments)](https://news.ycombinator.com/item?id=50642939)
→ 참고: [GitHub Status — Investigating degraded experiences across GitHub products](https://www.githubstatus.com/incidents/qn8mfv6q5hlx)

---

## 게임 · 인디게임

### 13. **유비소프트, 신규 기획서에 "생성AI를 왜 안 쓰는지" 설명 요구 — 내부적으로는 'Motherbrain' 가동** (GamesIndustry.biz / Eurogamer)

복수의 현직 유비소프트 직원들이 익명으로 전한 내용에 따르면, **신규 프로젝트 기획서에는 생성AI를 어떻게 써서 개발을 앞당길지 적어야 하며, 쓰지 않을 계획이면 상세한 이유를 제시해야 합니다**. 그렇지 않은 기획은 승인되지 않는다는 것입니다. 한 직원은 "리더십이 공식적으로 의무화한 건 아니지만 **우리는 항상 생성AI에 몰린다고 느낀다**"고 말했습니다. 유비소프트는 지난주 **생성AI 전담 자회사 'Motherbrain'**을 출범시켰고(상표 출원으로도 확인), Eurogamer는 이 명칭이 내부적으로만 쓰이고 있다고 전했습니다. 시사점은 AI 도입의 단계가 '권장'에서 **'설명 의무가 붙은 기본값'**으로 넘어갔다는 것 — 인디 개발자에게는 반대로 기회입니다. 대형 스튜디오가 기획 단계부터 AI 사용을 전제하면, **'AI를 쓰지 않는다'는 사실 자체가 차별화 포인트**가 될 수 있습니다.

→ 원문: [Ubisoft staff must justify not using generative AI in pitches, report claims (GamesIndustry.biz)](https://www.gamesindustry.biz/ubisoft-staff-must-justify-not-using-generative-ai-in-pitches-report-claims)
→ 교차확인: [Ubisoft's calling its next push into gen-AI game development Motherbrain (Eurogamer)](https://www.eurogamer.net/ubisoft-gen-ai-game-motherbrain-assassins-creed)

### 14. **EA, 故 존 모트슨 목소리를 AI로 복원해 FC 27에 투입 — 유족 동의·보상, 시리즈 최초의 '고인 해설자'** (GamesIndustry.biz / Inven Global)

EA는 11월 25일(현지시각) **고(故) 존 모트슨의 목소리를 아카이브 녹음과 생성AI로 복원해 EA Sports FC 27에 넣는다**고 BBC에 밝혔습니다. 해설은 **11월 26일** 게임에 적용되며, 시리즈(옛 FIFA) 사상 처음으로 **사망한 해설자**가 등장하는 사례입니다. 모트슨은 2023년 2월 77세로 별세했고 FIFA 96~2005의 원년 해설자였으며 FIFA 19에 잠시 복귀했습니다. EA는 **유족(부인·아들)의 허락을 받고 목소리·초상 사용 대가를 지급**했다고 밝혔지만 금액은 공개하지 않았습니다. EA 디자인 디렉터 매트 프라이어는 "유족이 전적으로 동의하지 않았다면 절대 진행하지 않았을 것"이라며 "그의 목소리가 없어 오늘의 선수들에 대한 녹음은 존재하지 않는다"고 설명했습니다. 아들 프레드 모트슨은 "아버지 목소리를 다시 듣게 돼 큰 의미"라고 했습니다. 시사점은 **'고인 음성 복원'의 사실상 표준 계약 조건**이 이 사례로 제시됐다는 것 — 유족 동의 + 보상 + 고지가 3종 세트가 되면, 반대로 이 셋이 없는 AI 음성 사용은 즉시 논란의 대상이 됩니다.

→ 원문: [EA Sports FC 27 adds AI-recreated voice of late commentator John Motson (GamesIndustry.biz)](https://www.gamesindustry.biz/ea-sports-fc-27-adds-ai-recreated-voice-of-late-commentator-john-motson)
→ 교차확인: [EA adds AI-recreated John Motson commentary to EA Sports FC 27 (Inven Global)](https://www.invenglobal.com/articles/29128/ea-adds-ai-recreated-john-motson-commentary-to-ea-sports-fc-27)

---

## Qiita 트렌드

### 15. **Qiita — SOLID 원칙 완전해설(♥36)과 "VS Code 1.148, 에이전트 전용 Dev Container·샌드박스 3종"** (Qiita)

이번 주 Qiita 최상위는 **「본질에서 배우는 SOLID 원칙」(Sicut_study, ♥36)**입니다. "왜 그런 원칙인가"를 언어 중립적으로 설명하는 설계 시리즈로, AI가 코드를 대량 생산하는 시점에 **리뷰 기준을 언어로 설명하는 능력**의 가치가 커진 흐름과 맞물립니다. 함께 주목할 글은 **「VS Code 1.148: 에이전트 전용 Dev Container와 Sandbox profiles — '격리' 3종을 구분해 쓰는 법」(♥9)**과 **「2026년 보안정보 406건을 '취약점-사고-대책' 한 장으로 정리하고 나서야 읽을 수 있었던 것」(♥16)**입니다. 어제 다룬 VS Code 1.147(Send Later·Drafts)의 다음 버전이 일주일 만에 정리된 셈입니다. 시사점은 일본 개발자 커뮤니티의 관심이 **'에이전트에게 일을 맡기는 법'에서 '에이전트를 어떻게 격리하는가'**로 이동했다는 것 — 1Password 사고(오늘 2번)와 같은 날 나온 논의라 더 의미가 있습니다.

→ 원문: [【完全解説】本質から学ぶSOLIDの原則 (Qiita, ♥36)](https://qiita.com/Sicut_study/items/2bbdfbacab915773dad1)
→ 교차확인: [VS Code 1.148：エージェント専用の Dev Container と Sandbox profiles (Qiita, ♥9)](https://qiita.com/shahin0809/items/05a6c7431ee7e5b6e520)

---

## 💭 미스 김 인사이트

1. **코스피는 이제 '지수'가 아니라 '삼성전자·하이닉스 시가총액 파생상품'입니다.** 25일 하루에 -6.16%, 개장 8분 만에 사이드카, 외국인 1.9조 순매도. 전날 +6.32%가 '젠슨 황 효과'였다면 오늘 -6.16%는 '효과의 소멸'입니다. 방향을 만드는 것은 뉴스가 아니라 **외국인 현물 수급**이고, 헬스케어로의 순환매는 지수를 방어하지 못했습니다. 26일 아침 미국 반도체가 반등하지 않으면 4,600선까지는 열려 있습니다.

2. **암호의 시간표가 다시 계산되기 시작했습니다.** 구글이 정수 인수분해 L_N[1/3, O(1)]과 이산로그 준준다항 알고리즘을 '무조건(unconditional)'으로 내놓은 날, 비트코인 ETF는 월간 최대 유출을 기록했습니다. 두 사건은 무관해 보이지만 같은 질문으로 수렴합니다 — **"이 자산의 보안 가정은 몇 년짜리인가."** 장기 보관용 키(콜드월렛·아카이브)일수록 암호 스위트 교체 계획을 문서로 갖고 있어야 합니다.

3. **공급망 공격의 표적은 빌드 시스템이 아니라 사람의 SaaS 계정입니다.** 1Password 사고의 경로는 '직원 GitHub 계정 → 백엔드 악성 코드 푸시 → 고객 데이터 다운로드'였습니다. 코드 서명·2인 승인·조건부 접근이 없는 조직은 오늘 이 경로를 그대로 갖고 있습니다. 특히 **초대·복구 메일 링크 토큰**이 유출됐다는 점은, 앞으로 몇 주간 1Password 사칭 피싱이 늘어난다는 뜻입니다.

4. **게임업계에서 'AI를 안 쓴다'는 이제 설명해야 하는 선택입니다.** 유비소프트는 기획서에 AI 활용 계획을 요구하고, EA는 고인의 목소리를 유족 동의·보상 아래 복원했습니다. 대형사가 AI를 기본값으로 깔수록 **'인간이 만든 것이 분명한 게임'**은 마케팅 문구가 아니라 계약 조항(유족 동의, 학습 데이터 출처, 크레딧)으로 증명해야 하는 시대가 됩니다.

5. **오늘의 공통 변수는 '금리'와 '관세'입니다.** PCE 3.8%로 12월 인상이 확정되고, USTR은 14개 경제권에 과잉생산 관세를 예고했습니다(한국 15% 상한, 시행은 이르면 내년 1월). 시장은 두 리스크를 동시에 가격에 넣기 시작했고, 그 와중에 다우만 플러스였습니다. **'지수'보다 '업종 로테이션'이 더 정확한 신호**인 국면입니다.
