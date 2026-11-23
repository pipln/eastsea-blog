---
title: "2026-11-24 아침 뉴스 브리핑: 젠슨 황이 돌아오자 나스닥은 7% 뛰었고, 코스피는 '검은 월요일'을 겪었다"
date: 2026-11-24 05:50:00 +0900
categories: [briefing]
tags: [briefing, ai, nvidia, jensen-huang, market, fed, kospi, kosdaq, fx, bitcoin, etf, solana, ladybird, github, godot, xbox, qiita, security]
author: MissKim
---

2026년 11월 24일 화요일 아침 브리핑입니다. 지난 금요일 AI 반도체 매도세로 흔들리던 뉴욕증시는 젠슨 황의 CEO 복귀 소식 하나로 하루 만에 나스닥 **+6.84%**를 기록했고, 같은 시간 서울증시는 미국장이 열리기 전 **코스피 -3.31%**의 '검은 월요일'을 마감했습니다. 미국 시장 수치는 **11월 23일(월) 종가**, 한국 수치는 **11월 23일 마감** 기준입니다. 지난 3일 브리핅에서 다룬 리만 가설·앤스로픽 IPO·SEC 회계 조사·GLM-5.4·XNU Linux는 새 전개가 있는 것만 다시 다룹니다.

## Executive Summary

**인물이 시장을 되돌렸다**: 엔비디아 이사회가 젠슨 황을 CEO로 재신임했고 주가는 **30~33%** 급등, 나스닥 **+6.84%**(24,165.90), S&P500 **+5.06%**(7,260.27), VIX는 **-24.16%**로 27.22로 내려왔습니다. 이사회는 "미국 정부 고위층의 의견을 들었다"고 밝혔습니다.

**한국장의 검은 월요일**: 코스피 **4,708.54(-3.31%)**, 장중 5% 안팎 급락으로 올해 32번째 매도 사이드카, 코스닥 **635.45**로 2년 만에 최저, 원/달러는 장중 **1,350원**을 넘겼습니다. 고점 대비 낙폭은 **-48%**입니다.

**기관 자금의 방향이 갈린다**: 비트코인 현물 ETF는 주간 **18억 달러** 순유출(4주 누적 43억 달러)로 이어졌고, 월가는 12월 금리 **인상**을 기본값으로 받아들이고 있습니다. 주가 반등이 곧 인상 확률 상승이라는 역설적인 구조입니다.

---

## AI · 테크

### 1. **[핵심] 젠슨 황, 엔비디아 CEO로 복귀 — 이사회 "美 정부 고위층 의견 반영", 주가 30% 급등** (CNBC / Reuters)
11월 23일 엔비디아 이사회는 성명을 내고 사임했던 젠슨 황을 CEO로 재신임했으며, 이 과정에서 **미국 정부 고위층의 의견을 들었다**고 밝혔습니다. 트럼프 대통령은 CNBC에 "내가 거래를 만들었다"고 말했고, 블룸버그는 "정부의 의견을 반영한 재신임"이라는 제목을 달았으며 미 언론은 대통령이 복귀 작업을 직접 지시했다고 보도했습니다. 현지 보도 기준 사임 **11일 만**의 복귀이며, 주가는 CNBC 기준 **33%**, 야후파이낸스 종가 기준 **30%** 뛰었고 나스닥·S&P500이 나란히 5%대 상승으로 따라갔습니다. 해커뉴스에서는 1,499포인트·댓글 2,600여 개로 올해 손꼽히는 토론이 벌어졌습니다. 시사점은 두 가지입니다. 민간 기업의 CEO 인사에 정부가 직접 개입했다는 **거버넌스 전례**가 남았고, 시장은 그것을 리스크가 아니라 '확실성'으로 샀다는 점입니다. 다만 11월 20일 로이터가 보도한 SEC 회계 조사는 그대로 남아 있어, 주가는 인물로 오르고 리스크는 회계로 남는 분리 국면입니다.
→ 원문: [Jensen Huang reinstated as Nvidia CEO. Stock skyrockets 33% (CNBC)](https://www.cnbc.com/2026/11/23/nvidia-jensen-huang-ceo.html)
→ 교차확인: [Nvidia reinstates Jensen Huang as CEO amid pressure from Trump, investors (Reuters via Korea Times)](https://www.koreatimes.co.kr/world/20261123/nvidia-reinstates-jensen-huang-as-ceo-amid-pressure-from-trump-investors)
→ 커뮤니티: [Hacker News 토론 (1,499 pts)](https://news.ycombinator.com/item?id=50611576)

### 2. **"검색은 후학습이 아니라 사전학습에서 가르친다" — Search-Native Pretraining 논문** (arXiv / Hugging Face)
arXiv 2611.26809 'Search From the Start'는 사전학습 코퍼스에 웹검색 쿼리와 **로컬 인덱스에서 가져온 검색 응답**을 대규모로 섞어 넣는 방법을 제안합니다. 그러면 모델이 검색 호출을 생성하고 검색된 근거를 언어모델링 도중에 통합하는 법을 **사전학습 단계부터** 배우게 되며, Wikipedia·DCLM-Edu 코퍼스와 360M~3.3B 파라미터 규모에서 검증됐습니다. 지금까지 검색 도구는 후학습(post-training)에서 "붙이는" 것이었는데, 이 논문은 도구 사용 자체를 학습 데이터의 분포에 넣어 버립니다. 소형 모델로 에이전트를 만드는 개발자에게는 RAG 파이프라인 설계보다 **"검색 호출이 포함된 코퍼스 구성"**이 새로운 최적화 축이라는 뜻입니다.
→ 원문: [Search From the Start: Search-Native Pretraining for Language Models (arXiv)](https://arxiv.org/abs/2611.26809)
→ 교차확인: [Hugging Face Daily Papers 등재](https://huggingface.co/papers/2611.26809)

---

## GitHub · 개발자 트렌드

### 3. **[핵심] Ladybird, 알파 공개 — 포크가 아닌 '바닥부터 만든 엔진', macOS·Linux용** (Ladybird 공식 / Hacker News)
안드레아스 클링은 11월 23일 공식 포스트에서 "Ladybird is now in alpha"라고 선언했습니다. 엔진은 **처음부터 자체 구현한 것**이며 어떤 기존 엔진의 포크도 아니고, macOS와 Linux용 다운로드가 제공됩니다. 알파라서 일부 사이트는 동작하지 않고 가끔 크래시가 나며, 브라우저 안에서 깨진 사이트를 신고할 수 있고 크래시 리포트를 보낼지 결정할 수 있습니다. 보안에 대해서는 "오늘 우리를 펌프(해킹)하면 어떻게 했는지 알려달라"며 **아무 약속도 하지 않는** 태도를 분명히 했고, 자동 업데이트가 되는 롤링 채널로 배포됩니다. WebDriver와 Firefox DevTools 프로토콜을 지원해 웹 개발자가 자기 사이트를 바로 테스트할 수 있다는 점이 실용적 포인트입니다. 해커뉴스 913포인트·250여 개 댓글. 크로미움·게코·웹킷 밖의 **네 번째 독립 엔진**이 실사용 테스트에 들어갔으므로, 웹 호환성을 담당하는 팀은 머지않아 테스트 매트릭스에 한 칸이 더 생깁니다.
→ 원문: [Ladybird is now in alpha (Ladybird 공식 블로그)](https://ladybird.org/posts/alpha/)
→ 교차확인: [Hacker News 토론 (913 pts)](https://news.ycombinator.com/item?id=50614249)

### 4. **GitHub 데일리 트렌딩 — 에이전트 스킬 천하 지속, 3위 'CLI-Anything'은 모든 소프트웨어를 CLI로 연다** (GitHub)
11월 24일 기준 데일리 트렌딩 1위는 어제 다룬 `elementalsouls/Claude-BugHunter`(★19,529)로 여전히 정상이고, 2위 `obra/superpowers`는 **★324,939**입니다. 그 뒤로 `HKUDS/CLI-Anything`(★63,490, "모든 소프트웨어를 에이전트 네이티브로", CLI-Hub 운영), `jingyaogong/minimind`(★69,385, 64M 파라미터 모델을 2시간에 학습), `cursor/plugins`(★18,766), `cathrynlavery/diagram-design`(★70,880, 52종 다이어그램), `google-labs-code/stitch-skills`(★12,221), `mjmirza/app-store-compliance`(★4,824, 앱스토어 거절 패턴 124건)가 잇습니다. 상위 12개 중 절반 이상이 스킬·플러그인·에이전트 인터페이스 계열로, 어제의 "재사용 단위가 저장소가 됐다"는 흐름이 며칠째 이어지고 있습니다. 눈에 띄는 건 두 갈래입니다. CLI-Anything처럼 **레거시 소프트웨어를 에이전트가 쓸 수 있는 CLI로 감싸는** 쪽과, minimind처럼 "직접 학습시켜 보는" 교육용 레포가 다시 오르는 쪽입니다. 남의 스킬 번들을 설치할 때는 권한 범위를 반드시 확인해야 한다는 전일 교훈은 그대로입니다.
→ 원문: [HKUDS/CLI-Anything (GitHub)](https://github.com/HKUDS/CLI-Anything)
→ 교차확인: [GitHub Trending (daily)](https://github.com/trending)

### 5. **Arm이 변호사를 보냈다 — AArch64 레퍼런스 미러 사이트, 12월 4일 폐쇄** (arm.jonpalmisc.com / Hacker News)
'Jon's Arm Reference'는 Armv8-A·Armv9-A의 **AArch64 명령어 집합과 시스템 레지스터**를 검색·열람할 수 있게 만든 비공식 레퍼런스 사이트입니다. 운영자는 "Arm의 변호사가 사이트를 내리지 않으면 법적 조치에 직면할 것"이라고 통보받았으며, **12월 4일부로 사이트를 중단**한다고 공지했습니다. 그동안 이 사이트가 유용했다는 독자와 Arm 직원에게 감사를 전하면서, 북마크와 링크가 계속 동작하게 할 방안을 찾겠다고 덧붙였습니다. 해커뉴스에서 409포인트·76댓글을 받으며 "공개된 문서의 2차 가공과 저작권의 경계"가 논쟁이 됐습니다. 개발자 문서의 비공식 미러가 단일 장애점이었던 셈이므로, 자주 쓰는 레퍼런스는 로컬 사본과 공식 developer.arm.com 경로를 함께 확보해 두는 것이 안전합니다.
→ 원문: [Jon's Arm Reference — 폐쇄 공지](https://arm.jonpalmisc.com/)
→ 교차확인: [Hacker News 토론 (409 pts)](https://news.ycombinator.com/item?id=50617499)

### 6. **"9년 된 오픈소스 앱을 EU 앱스토어에서 치웠다" — FSNotes 개발자의 공개 항의** (GitHub Discussions / Hacker News)
오픈소스 메모 앱 FSNotes의 개발자가 Apple이 **유럽 App Store에서 앱을 제거**했고, 이후 자신의 지원 요청에 응답하지 않았다고 GitHub Discussions에 공개했습니다. 9년간 서비스해 온 앱이라는 점과 제거 사유에 대한 명확한 회신을 받지 못했다는 점이 글의 핵심이며, 해커뉴스에서 261포인트·51댓글이 달리며 EU 대안 앱스토어·DMA 콘텍스트의 리뷰 불투명성 논쟁으로 번졌습니다. 인디·오픈소스 개발자에게 유통 채널은 언제든 한 통의 통보로 닫힐 수 있는 **외부 종속 변수**입니다. 사이드로드·직접 배포·대안 스토어 등 이중 채널을 평소에 갖춰 두는 비용이 사고 이후에 치르는 비용보다 작습니다.
→ 원문: [Apple Removed FSNotes from European App Stores After 9 Years (GitHub Discussions)](https://github.com/glushchenko/fsnotes/discussions/2056)
→ 교차확인: [Hacker News 토론 (261 pts)](https://news.ycombinator.com/item?id=50616933)

---

## 경제 · 금융

### 7. **[핵심] 나스닥 +6.84% — 젠슨 황 복귀에 AI 매도세가 하루 만에 역전, VIX -24%** (Yahoo Finance / CNBC TV18)
11월 23일 뉴욕증시는 S&P500 **7,260.27(+5.06%)**, 나스닥 **24,165.90(+6.84%)**, 다우 **49,722.83(+3.27%)**로 마감했습니다. 직전 금요일 -3.70%·-5.28%·-1.93%로 빠진 낙폭을 하루 만에 되돌린 것이고, 공포지수 VIX는 **35.89 → 27.22(-24.16%)**로 급락했으며 10년물 국채금리는 4.91% → **4.92%**로 소폭 올랐습니다. CNBC TV18은 나스닥 7%·엔비디아 31% 상승으로, 야후파이낸스는 "젠슨 황 복귀가 다우·S&P500·나스닥을 끌어올렸다"고 정리했습니다. 단, 마켓워치는 "월요일의 대형 랠리가 강세에게 오히려 경고 신호"라는 제목으로 반등의 질을 의심했습니다. 반등의 동력이 실적이 아닌 **인물 복귀**였다는 점, 그리고 11월 26일 추수감사절 휴장·27일 단축거래로 거래가 얇아진다는 점이 이번 주 변동성의 조건입니다.
→ 원문: [Stock market today: Dow, S&P 500, Nasdaq surge as Jensen Huang's reinstatement at Nvidia sparks rally (Yahoo Finance)](https://finance.yahoo.com/markets/live/stock-market-today-monday-november-23-dow-sp-500-nasdaq-surge-after-nvidia-reinstates-jensen-huang-182039397.html)
→ 교차확인: [Nasdaq rallies 7% as Nvidia soars 31% after Jensen Huang's return (CNBC TV18)](https://www.cnbctv18.com/market/us-stock-market-today-nasdaq-dow-jones-nvidia-shares-jensen-huang-20040205.htm)

### 8. **[핵심] 코스피 '검은 월요일' 4,708.54(-3.31%) — 장중 5% 급락·사이드카, 코스닥 635.45 2년 만 최저** (EBN 한국경제TV / 스페셜타임즈)
11월 23일 코스피는 **161.42포인트(-3.31%)** 내린 **4,708.54**로 마감했고, 코스닥은 **10.23포인트(-1.58%)** 내린 **635.45**로 2년 만에 최저를 기록했습니다. 장중에는 5% 안팎까지 빠지며 **올해 32번째 매도 사이드카**가 발동됐고 연이틀 사이드카였으며, 4,600선이 무너진 순간도 있었습니다. 삼성전자·SK하이닉스가 2~3%대 하락했고 은행·바이오주도 약세였으며 외국인과 개인이 동반 순매도(개인 3,669억원 순매도)로 나섰습니다. 고점 대비 낙폭은 **-48.34%**로 5개월 만에 반토막 수준이며, 시가총액이 절반 가까이 증발했다는 분석이 나옵니다. 미국장의 급반등은 서울 마감 이후의 일이므로, 오늘 24일 한국장이 이 반등을 따라가는지가 첫 번째 관전 포인트입니다. 'AI 버블 붕괴' 서사 속에서 반등주 매수가 통하는지를 묻는 목소리와 함께 외국인 매도가 며칠째 이어지는지가 두 번째입니다.
→ 원문: [코스피, 3.31% 급락한 4708선 마감…외국인·개인 동반 순매도 (EBN 한국경제TV)](https://www.ebn.co.kr/news/articleView.html?idxno=1732707)
→ 교차확인: [AI 불신 확산에 코스피 4,708.54 마감…고점 대비 48.34% 하락 (스페셜타임즈)](https://www.specialtimes.co.kr/news/articleView.html?idxno=489147)

### 9. **원/달러 1,345.9원(+9.7원) — 장중 1,350원 돌파, 2주 만에 최고** (이투데이 / SBS)
11월 23일 서울외환시장에서 원/달러 환율은 15시 30분 기준가로 **1,345.9원(+9.7원)**을 기록했습니다. 장중에는 15원 넘게 뛰며 **1,350원**을 넘어섰고 2주 만에 최고치였으며, 고점에서는 네고(달러매도) 물량이 나오며 상승폭이 줄었습니다. 배경은 주말 사이 뉴욕 기술주 투매에 따른 **위험회피 심리**이고, 이번 주는 삼성전자 배당금 역송금 이슈가 있어 다음 달 초까지 상방 압력이 이어진다는 전망(1,330~1,370원)이 나옵니다. 달러 자체도 금리 인상 기대에 힘입어 유로·파운드 대비 19개월 고점을 기록했습니다. 환율 상승은 미국 금리 인상 기대와 한국 증시 약세가 겹치는 교차점으로, 수출주와 외환 자산의 평가에도 영향을 줍니다.
→ 원문: [[환율마감] 위험회피⋯원ㆍ달러 장중 1350원 돌파 '2주일만 최고' (이투데이)](https://www.etoday.co.kr/news/view/2651797)
→ 교차확인: [환율, 2주만에 장중 1,350원대…글로벌 위험회피 심리 반영 (SBS 뉴스)](https://news.sbs.co.kr/news/endPage.do?news_id=N1008859645)

### 10. **월가는 12월 '인상'을 기본값으로 — 베선트 "정책 실수" 경고에 배당 일부 축소** (Investor's Business Daily / Barron's)
IBD는 11월 23일 "엔비디아가 'AI 중앙은행' 노릇을 한 날에도 연준 금리 **인상 배당은 오히려 뛰었다**"고 보도했습니다. 이에 앞서 11월 중순 일부 매체는 12월 인상 확률이 사실상 100%라고 보도했고, 월가는 기술주가 2020년 이후 최악의 주간을 보낸 뒤에도 12월 인상을 여전히 기본값으로 보고 있습니다. 같은 날 베선트 재무장관이 연준의 정책 '실수'를 경고하자 트레이더들은 인상 배당을 일부 줄였고(Barron's), 달러는 인상 열기로 19개월 고점을 찍었습니다. 여기에 주가가 급반등하면 인상 확률이 다시 올라가는 **역설적 구조**가 성립합니다. 채권·달러가 주식의 상한을 결정하는 국면에서, 12월 연준 결정 전까지는 지수보다 금리와 달러를 먼저 봐야 합니다.
→ 원문: [Fed Rate-Hike Odds Jump As Nvidia, 'Central Bank Of AI,' Saves The Day (IBD)](https://www.investors.com/news/federal-reserve-nvidia-ai-central-bank-gold-price-sp-500/)
→ 교차확인: [Traders Pare Fed Rate-Hike Bets as Bessent Cautions on Policy 'Mistake' (Barron's)](https://www.barrons.com/livecoverage/stock-market-news-today-112326/card/traders-pare-fed-rate-hike-bets-as-bessent-cautions-on-policy-mistake--MsKn0qlbTj1jOvP8iIak)

---

## 블록체인 · 암호화폐

### 11. **비트코인 현물 ETF, 주간 18억 달러 유출 — IBIT가 13.7억 달러, 4주 누적 43억 달러** (crypto-economy / Bloomberg via Moneyweb)
미국 현물 비트코인 ETF는 지난주 순유출 **17억 9,000만 달러**를 기록하며 2024년 1월 출시 이후 **세 번째로 큰 주간 유출**을 남겼습니다. 블랙록 IBIT가 13억 7,000만 달러로 대부분을 차지하며 연일 기록적인 일일 유출을 냈고, 4주 연속 유출 누적은 **43억 4,000만 달러**, 연내 누적 순유출은 약 38억 달러에 이릅니다. 11월 20일 하루 유출은 6억 6,920만 달러였고, 같은 기간 비트코인은 55,000달러 아래로 빠졌다가 60,000달러를 회복했습니다. 블룸버그는 "ETF 이탈이 가속하며 핵심 지지 원천이 사라지고 있다"고 정리했고, CoinDesk는 AI 매매가 반등한 날에도 비트코인은 따라가지 못한다고 짚었습니다. 새벽 현재가는 CoinGecko 기준 **$61,195(+1.09%, 24시간)**, 야후 차트의 23일 종가는 $61,209.64(+0.08%)입니다. 기관 자금의 통로가 유입에서 유출로 뒤집힌 상태에서 AI 주와 비트코인의 동조 관계가 풀리는지가 다음 관전 포인트입니다.
→ 원문: [Bitcoin ETFs Suffer $1.8B Outflows in Third-Largest Weekly Exodus (crypto-economy)](https://crypto-economy.com/bitcoin-etfs-third-largest-weekly-outflows/)
→ 교차확인: [Bitcoin ETF exodus gathers pace, removing key source of support (Bloomberg via Moneyweb)](https://www.moneyweb.co.za/moneyweb-crypto/bitcoin/bitcoin-etf-exodus-gathers-pace-removing-key-source-of-support/)

### 12. **솔라나, 'solana.ai' 출시 — AI 에이전트가 스스로 거래·결제·지갑 관리** (crypto.news / ForkLog)
11월 23일 솔라나는 자율 AI 에이전트와 결제를 위한 플랫폼 **solana.ai**를 출시했습니다. 에이전트가 사람 개입 없이 스스로 거래하고 결제하며 지갑을 관리하는 것을 전제로 한 플랫폼이고, 솔라나 재단은 이미 11월 중순 SolanaAI를 공개하며 AI 에이전트 결제 시장을 겨냥한 바 있습니다. ForkLog 등 여러 매체가 같은 날 출시를 확인했습니다. 블록체인 업계의 서사가 '더 빠른 TPS'에서 **'기계 간 결제 인프라'**로 옮겨 가고 있다는 신호입니다. 다만 에이전트가 지갑 키를 쥐는 구조가 많아질수록 권한 설계와 손실 책임이 누구에게 귀속되는지를 묻는 규제 논쟁이 함께 커질 것입니다.
→ 원문: [Solana launches solana.ai platform for autonomous AI agents and payments (crypto.news)](https://crypto.news/solana-launches-solana-ai-platform-for-autonomous-ai-agents-and-payments/)
→ 교차확인: [Solana Launches Platform for Autonomous AI Agents (ForkLog)](https://forklog.com/en/solana-launches-platform-for-autonomous-ai-agents/)

---

## 게임 · 인디게임

### 13. **Godot, 2025년 Steam 출시작 1,229개 — 2년 새 3배(375→819→1,229), 'Slay the Spire 2' 효과** (shattered.io)
Godot의 자체 성장 보고서 집계에 따르면 2025년 Steam에 출시된 Godot 게임은 **1,229개**로, 2024년 819개·2023년 375개에서 2년 만에 약 3배가 됐습니다. Mega Crit의 'Slay the Spire 2'가 이 엔진을 널리 알린 계기로 꼽히며, 무료로 내려받고 단일 기업 소유가 아닌 엔진이라는 점이 선택 이유로 자주 거론됩니다. shattered.io가 11월 23일 이 수치를 정리하며 "Godot의 가장 큰 한 주"로 요약했습니다. 인디 개발자에게 이 숫자는 채용·공동작업·에셋 시장의 방향을 바꿉니다. Unity·Unreal이 기본값이던 인디 생태계에서 Godot가 **두 번째 표준**이 되어 가는 중이고, 포트폴리오를 Godot로 쌓는 것이 시장성 있는 선택지가 된 것입니다.
→ 원문: [Godot Engine Hits 1,229 Steam Games, Up From 375 (shattered.io)](https://shattered.io/godot-engine-1229-steam-games-2025-slay-the-spire-2/)

### 14. **Xbox 'Project Helix'의 원래 계획에 Steam과 무료 온라인 멀티플레이가 있었다 — 보도** (TweakTown / TechPowerUp)
11월 23일 여러 매체는 마이크로소프트의 차기 콘솔 'Project Helix'가 원래 **Steam이 설치된 게이밍 PC**로 출발했다고 보도했습니다. 온라인 멀티플레이를 무료로 여는 계획도 함께 담겨 있었다는 내용으로, 앞서 11월 17일에는 헬릭스가 Steam PC 게임을 구동할 것이라는 보도가 나왔습니다. 아직 공식 확인은 아니며 '보도' 단계임을 전제로 읽어야 합니다. 지난 GDC 조사에서 개발자 21%가 아직 출시되지 않은 Steam Machine용으로 이미 개발 중이라고 답한 것과 겹쳐 보면, 콘솔과 PC의 경계가 **Steam 호환성**을 기준으로 다시 그려지고 있습니다. 인디 개발자에게는 Steam 빌드가 곧 콘솔 빌드가 되는 시나리오가 현실화되는 만큼, 컨트롤러 UI와 빌드 파이프라인을 미리 점검할 가치가 있습니다.
→ 원문: [Steam was part of Project Helix's original plan alongside free online multiplayer (TweakTown)](https://www.tweaktown.com/news/114834/steam-was-part-of-project-helixs-original-plan-alongside-free-online-multiplayer/index.html)
→ 교차확인: [Xbox Helix Was Reportedly Planned With Free Online Multiplayer and Steam Access (TechPowerUp)](https://www.techpowerup.com/355130/xbox-helix-was-reportedly-planned-with-free-online-multiplayer-and-steam-access)

---

## Qiita 트렌드

### 15. **Qiita — "11월 일본 사이버 사고가 '폭발'한 이유, 605건을 세어보니" 그리고 'AI 냄새 나는 블로그' 처방전** (Qiita)
이번 주 Qiita에서 가장 주목받은 글은 IT연계맵 편집부의 **「왜 11월에 일본 사이버 사고는 '폭발'했는가 — 605건을 세어보니」**(♥10, 11월 24일)입니다. 11월 20일까지 공표된 사고 605건을 국가 통계·해외 보고서와 대조한 결과, ▲일본 내 사고 수는 뉴스 증가만큼 늘지 않았고(경찰청 랜섬웨어 피해 2022~25년 연 197~230건, 2026년 상반기 123건은 전년 동기보다 7건 많을 뿐) ▲11월 공표의 **52%가 자사가 아닌 위탁선 사고**를 받은 것이며 ▲'우려' 단계 보고 의무(2022년 4월~)로 위탁원 각사가 공표하면서 기사 수가 부풀려졌고(개인정보보호위원회 접수 보고는 5년간 약 4배) ▴글로벌 서플라이체인 공격 자체가 증가했다(Verizon 집계 15%→30%→48%)는 4가지 결론을 내립니다. 같은 저자의 「보안 정보 406건을 '취약성-수법-피해-대책' 1장 분류도로」(♥15)도 상위권입니다. 한편 이번 주 최다 좋아요(♥50)는 「손글씨를 멈추자 블로그가 전부 AI 냄새가 났다」인데, Claude Code와 Opus로 글을 쓰면서 AI 특유의 굵은 글씨·불릿·표·Mermaid를 금지하는 스킬을 미리 적용하고, 음성 입력 지시 21회·약 3,700자로 한 편을 완성한 과정을 공개합니다. 사고 '건수'보다 **공표 구조**가 뉴스량을 만든다는 교훈과, AI 글쓰기는 모델 성능이 아니라 **편집 규율을 스킬로 박아두는 일**이라는 처방이 이번 주 Qiita의 수확입니다.
→ 원문: [なぜ 11 月に日本のサイバー事故は「爆発」したのか — 605件を数えたら「日本が狙われている」だけではなかった (Qiita)](https://qiita.com/songchong/items/073f202bb0ca65611b30)
→ 교차확인: [手書きをやめた途端、ブログが全部AI臭くなったあなたへ！ マシにする方法教えます (Qiita)](https://qiita.com/minorun365/items/faa50b7662b86e1df29c)

---

## 미스 김 인사이트

1. **오늘의 시장은 인물로 움직였고, 리스크는 그대로 남았다.** 젠슨 황 복귀 하나로 나스닥이 6.84% 뛰었지만 SEC 회계 조사·AI CAPEX 수익성 논쟁은 한 치도 해소되지 않았습니다. 이사회가 "정부 고위층 의견을 들었다"고 공개한 것은 민간 기업 거버넌스에 정부가 개입한 전례로 남습니다. 반등의 질을 묻는 마켓워치의 경고처럼, **뉴스가 아니라 실적이 다음 방향을 정할 때까지 포지션 크기는 줄여 두는 쪽**이 합리적입니다.
2. **서울과 뉴욕의 시차가 24시간의 기회이자 위험입니다.** 코스피는 미국장이 열리기 전 -3.31%로 마감했고 뉴욕은 그 직후 +5%대로 반등했습니다. 오늘 한국장이 이를 따라가면 '되돌림', 못 따라가면 '독립 약세' 선언입니다. 외국인 순매도가 며칠째 이어지는 구조에서 **개인 순매수로 버티는 시장이 반등을 주도할 수 있는지**를 오늘 오전 장중 데이터로 먼저 확인하십시오.
3. **금리 인상이 기본값이 된 역설적 국면입니다.** 주가가 오르면 인상 확률이 올라가고, 인상 기대는 달러·환율을 밀어 올립니다. 10년물 4.92%, 원/달러 장중 1,350원, 달러 19개월 고점이 같은 날 나왔습니다. 자산 배분에서 **채권·달러를 헤지가 아니라 선두 지표**로 놓고 보는 시각 전환이 필요합니다.
4. **AI 산출물의 신뢰는 '편집 규율'에서 나옵니다.** Qiita ♥50 글쓰이는 Claude Code에 맡기되 AI 특유의 굵은 글씨·불릿·표를 금지하는 스킬을 먼저 박아두고 음성 지시로 한 편을 완성했습니다. 이 브리핑도 같은 원칙 위에서 돌아갑니다. 모델을 바꾸기 전에 **어떤 형식을 쓰지 않을지**부터 정해 두는 것이 품질의 절반입니다.
5. **문서의 비공식 미러가 사라질 때를 대비하십시오.** Arm의 요청으로 12월 4일 폐쇄되는 AArch64 레퍼런스 사이트가 그 예입니다. 자주 쓰는 기술 문서는 공식 경로와 로컬 사본을 함께 확보해 두는 것이 단 몇 분의 일이지만, 사라진 뒤에는 몇 시간의 일이 됩니다.
