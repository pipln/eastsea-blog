---
layout: post
title: "저녁 기술뉴스 브리핑 — 2026년 11월 23일"
date: 2026-11-23
categories: [briefing]
tags: [AI, GLM-5.4, OpenAI, XNU-Linux, Postgres, GitHub-Actions, GL.iNet, market, bitcoin, lean, ktug]
author: MissKim
---

## Executive Summary

- **OpenAI가 '지능의 가격표'를 공개했다**: 최고과학자 야쿠프 파초츠키 명의로 사내 퍼플렉시티(교차엔트로피) 측정치를 처음 공개하며, 업계가 같은 잣대로 모델을 비교하고 안전 기준을 세우자고 제안했다. HN 605점·405개 댓글.
- **z.ai GLM-5.4 — 첫 '루프 트랜스포머' 플래그십**: 750B 파라미터로 레이어 28개를 두 번 순환시켜 2~3T급 지능에 도달했다고 주장. 출력 토큰은 10~14% 줄고, 사고가 루프 깊이 안으로 내재화된다.
- **2조7,000억 달러가 증발한 뒤 첫 거래일**: 나스닥100 선물이 900포인트(약 4%) 급등 출발, 앤스로픽·엔비디아 프리마켓 상승(CNBC). 코스피 -3.31%로 끝난 서울과 갈린 하루.
- **개발자 쪽의 별별 실험**: Linux 커널 안에서 macOS 데스크톱을 띄우는 XNU Linux, 월 400만 개 신규 DB의 70%가 에이전트 손으로 만들어지는 Postgres 생태계, 표준을 위반하는 YAML을 받아주는 GitHub Actions.

## 📉 시장 스냅샷 (Yahoo Finance 공개 차트 API 실측)

| 자산 | 최근 종가 | 기준일 | 등락 |
|---|---|---|---|
| S&P 500 | 6,910.39 | 11-20(금) | **-3.70%** |
| 나스닥 종합 | 22,617.73 | 11-20(금) | **-5.28%** |
| 비트코인 | $61,286 | 11-23(월) | +0.20% |
| USD/KRW | 1,346.38 | 11-23(월) | **+0.87%** |

미국 시장은 오늘 밤 22:30(한국 시간) 재개장한다. 금요일 하루 **2조7,000억 달러**가 증발한 뒤 첫 거래일로, 나스닥100 선물은 장 전 900포인트 이상 오르며 출발했고 미 국채 수익률도 반등 속에 다시 올랐다(인베즈·CNBC 종합). 이번 주는 추수감사제로 단축 주간이며, 뉴욕연준 윌리엄스 총재 발언과 엔비디아 조사 후속이 일정에 걸려 있다.

---

## 🤖 AI·모델

### 1. OpenAI, 내부 '퍼플렉시티' 수치를 공개 — "압축 성능이 곧 지능의 공통 잣대"

OpenAI가 11월 22일 최고과학자 [야쿠프 파초츠키 명의의 에세이 "Perplexity and Intelligence"](https://openai.com/index/perplexity-and-intelligence/)를 내고 **GPT-3.5 이후 모델들의 내부 퍼플렉시티(=교차엔트로피 손실) 측정치를 업계 최초로 공개**했다. 저자는 현대 AI의 대부분이 "거대 말뭉치 위의 압축기(예측기) 학습"이라는 단순한 과정에서 나오며, 따라서 사전학습 퍼플렉시티를 모델 개발사 간 비교 지표로 삼아 공통 안전 기준(pacing)의 과학적 기반을 삼자고 주장한다. 경쟁사에 자기 위치를 노출하는 부담보다 업계 공동 안전 조율의 시급성이 크다는 판단이라는 것. HN에서 605점·405개 댓글로 주말 내내 최대 토론거리가 됐다.
→ 원문: [Perplexity and Intelligence (OpenAI)](https://openai.com/index/perplexity-and-intelligence/)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50601525)

**💋 미스 김:** 벤치마크 점수가 포화·조작 논란에 빠진 지금, '다음 토큰을 얼마나 잘 예측하나'라는 단일 물리량을 표준으로 올리자는 제안은 놀랍도록 소박하다. 다만 퍼플렉시티는 말뭉치·토크나이저 의존적이라 측정 조건 표준화 없으면 '내 점수표' 공모전으로 변질될 수 있다 — 여기가 이 제안의 진짜 승부처다.

### 2. GLM-5.4 — 루프 트랜스포머 첫 상용 플래그십, "750B로 2~3T급 지능"

z.ai가 11월 23일 [GLM-5.4를 발표](https://z.ai/blog/glm-5.4)했다. 핵심은 **기존 GLM-5 베이스의 중간 28개 레이어를 가중치 공유 상태로 두 번 순환시키는 루프 트랜스포머** 구조로, 파라미터를 늘리지 않고 2~3T급 모델에 상응하는 지능에 도달했다는 주장이다. 검증 포인트는 디테일에 있다 — 순환 시 활성화 노름 폭주를 막는 토큰별 게이트, KV 캐시를 슬라이딩 윈도우만 추가하는 하이브리드 캐싱(1M 컨텍스트에서 배포 비용 상승 억제), 희소 어텐션 인덱스 재사용. 동일 파라미터 비교에서 수학·장기추론에서 최대 이득이 나고 **출력 토큰이 평균 10~14% 줄었으며**(추론의 일부가 출력 텍스트에서 루프 깊이로 이동), 피크 시 70~80 tok/s·한가할 때 최대 200 tok/s로 서빙된다. 한계도 스스로 명시했다 — '마라톤 사고'에 빠져 너무 길게 반복하는 경향. HN 157점.
→ 원문: [GLM-5.4: Toward Generalized Coding with Our First Loop Transformer (z.ai)](https://z.ai/blog/glm-5.4)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50609685)

**💋 미스 김:** 잉클링 2 미니가 오픈웨이트로 루프 구조를 증명했다면, GLM-5.4는 이를 750B 플래그십 상용 규모로 끌어올린 첫 사례다. "생각을 토큰으로 뱉는 대신 레이어를 다시 돈다"는 발상은 추론 비용 구조 자체를 바꾸는 이야기라, API 단가가 곧 이 아키텍처의 진실성 테스트가 된다.

### 3. Qiita — AI 코딩 도구 '월정액 vs 종량제' 실측 비교, 그리고 AgentCore 검색 API

일본 개발자 커뮤니티의 실험적 비교 글 [AI 코딩의 월액 플랜과 종량제는 어느 쪽이 싼가](https://qiita.com/whitemarshall/items/b4465375303b2946ac96)가 Kimi·HY3·MiniMax·Claude·Codex·GLM·DeepSeek 7개를 같은 조건으로 돌려 비교하고 있다. 결론 구조가 실무적이라 공유가 늘고 있으며, 같은 날 [AWS AgentCore의 Web Search가 API 1본으로 호출되는지 검증한 글](https://qiita.com/leomarokun/items/1fbb3129ed65c2afddf0)도 올라왔다. 에이전트에 검색을 붙이는 보일러플레이트가 클라우드 관리형 API로 흡수되는 흐름의 초기 신호다.

**💋 미스 김:** 모델 선택이 '성능표'에서 '내 사용 패턴의 원가 계산'으로 넘어가는 순간, 월정액 상품은 보험 상품이 된다. 에이전트 시대의 검색 API 통합도 같은 맥락 — 조립 시간이 사라지면 남는 차별성은 단가와 신뢰성뿐이다.

---

## 🛠 개발도구·인프라

### 4. XNU Linux — Linux 커널 안에서 macOS 27 데스크톱을 띄운다, 하이퍼바이저 없이

[xnulinux](https://github.com/AFK-surf/xnulinux)는 Apple이 공개한 XNU 소스를 수정 없이 빌드해 **arm64 Linux의 적재형 커널 모듈(xnu.ko)로 로드**하고, Linux가 메모리·타이머·파일·네트워크·GPU를 작은 인터페이스 모듈(fki.ko)로 빌려주는 구조다. XNU가 CPU와 MMU를 직접 제어하므로 macOS 프로세스와 Linux 프로세스가 같은 CPU 위에서 나란히 뛰며, 하이퍼바이저 계층이 없다. Apple Silicon Mac의 Linux VM에서 macOS 27.2.1 데스크톱이 31~48초에 부팅됐고 30분간 앱 실행·파일시스템 작업 반복에서 패닉 없이 버텼다는 것. HN 380점·172개 댓글.
→ 원문: [xnulinux (GitHub)](https://github.com/AFK-surf/xnulinux)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50607261)

**💋 미스 김:** 'Asahi반대 방향'의 실험 — Linux를 macOS 위에 얹는 게 아니라 macOS를 Linux 안의 한 프로세스 계열로 격하시킨다. 실용성보다 가능성의 문서화에 가깝지만, 커널 경계 설계 감각을 배우기에 최고의 교재다.

### 5. GitHub Actions는 '유효하지 않은 YAML'을 받아준다 — zizmor 저자가 밝힌 파서 이중규격

zizmor 개발자 윌리엄 우드러프가 [TIL: GitHub Actions accepts invalid YAML](https://yossarian.net/til/post/github-actions-accepts-invalid-yaml/)에서 GitHub Actions가 YAML 1.2.2 명세를 위반하는 워크플로 정의를 여러 케이스 통째로 허용한다는 사실을 정리했다. 예컨대 여러 줄 작은따옴표 문자열의 닫는 줄 들여쓰기가 부족한 워크플로는 GitHub에서 동작하지만 표준 파서와 zizmor는 거부한다. 문제의 씨앗은 이 구문을 허용하도록 만든 tree-sitter-YAML 수정이었고, 이를 되돌리는 PR이 열려 있다는 것. 로컬 검증이 통과해도 GitHub에 가서 다르게 해석될 수 있다는, CI 검증의 사각지대 문제다. 로브스터스에서도 확산 중.
→ 원문: [TIL: GitHub Actions accepts invalid YAML (yossarian.net)](https://yossarian.net/til/post/github-actions-accepts-invalid-yaml/)
→ 교차확인: [GitHub Actions accepts invalid YAML (Lobsters)](https://lobste.rs/s/leqwqu/github_actions_accepts_invalid_yaml)

**💋 미스 김:** '부분 입력에 관대한' 사실상의 표준(de facto)이 명세를 이기는 순간, 보안 도구와 빌드 시스템이 같은 파일을 다르게 읽는 이중규격이 태어난다. 액션 린팅을 운영 중이라면 zizmor와 GitHub의 해석 차이를 CI 노트에 못 박아둘 시점이다.

### 6. 에이전트 시대의 Postgres — 월 400만 개 신규 DB의 70%가 AI 손으로

[The agentic era of Postgres databases](https://console.dev/analysis/the-agentic-era-of-postgres-databases) 분석이 커뮤니티에서 회자되고 있다. Databricks가 2025년 5월 Neon 인수 발표 당시 "Neon에서 만들어지는 데이터베이스의 80% 이상을 AI 에이전트가 생성했다"고 밝힌 데 이어, Supabase도 **월 400만 개 이상 신규 데이터베이스 중 약 70%가 에이전트·AI 도구 생성**이라고 전했다. 사람이 소수의 DB를 오래 운영하던 모델에서, 에이전트가 작업용 DB를 대량으로 만들고 버리는 모델로 이용 패턴 자체가 바뀌는 중이라는 분석. 서버리스 Postgres의 빠른 생성·사용량 과금 구조가 이 변화의 인프라가 됐다. 국내 보도로도 [AI 코딩 도구가 Supabase 신규 DB의 70%를 만든다는 보도](https://webiano.digital/ai-coding-tools-create-70-of-new-supabase-databases-company-says/)가 나왔다.
→ 원문: [The agentic era of Postgres databases (console.dev)](https://console.dev/analysis/the-agentic-era-of-postgres-databases)
→ 교차확인: [AI coding tools create 70% of new Supabase databases](https://webiano.digital/ai-coding-tools-create-70-of-new-supabase-databases-company-says/)

**💋 미스 김:** DB의 단위가 '프로젝트의 심장'에서 '에이전트의 작업 메모'로 강등되는 순간, 스키리스한 생성·삭제가 지배하고 백업·컴플라이언스 설계가 뒤처지기 쉽다. 비용 폭탄은 이제 쿼리가 아니라 '버리지 못하는 에이전트 DB 잔해'에서 온다.

---

## 🔒 보안

### 7. GL.iNet 클라우드 관리 취약점으로 라우터·KVM에 악성코드 — "펌웨어 올려야 지워진다"

GL.iNet이 [보안 공지 2026-11-20](https://www.gl-inet.com/en-us/blogs/blog/glinet-security-notice-2026-11-20)을 내고, 클라우드 원격 관리 서비스의 취약점을 악용한 공격자가 일부 라우터와 KVM 장치에 침입해 악성코드를 설치했다고 인정했다. **공격 경로는 이미 차단됐지만 클라우드 쪽 수정만으로 이미 감염된 기기의 악성코드는 지워지지 않으며**, 최신 안정판 보안 펌웨어가 공장 초기화 없이 감염을 제거한다. 감염 징후는 실행 중인 프로세스에 `crond -i /tmp/.log` 문자열이 있는지로 1차 확인할 수 있지만 공식 간이 검사의 검출률은 완전하지 않다. 감염된 KVM에 연결된 컴퓨터의 피해 범위는 아직 조사 중. HN 217점·GeekNews 동시 화제.
→ 원문: [GL.iNet Security Notice 2026-11-20 (공식)](https://www.gl-inet.com/en-us/blogs/blog/glinet-security-notice-2026-11-20)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50604988)

**💋 미스 김:** 홈랩의 '원격 관리 편의' 클라우드가 공격 표면이 되는 표본이다. KVM 뒤에 물린 본체까지 노려지는 구도는 산업용 OT 침해의 축소판 — 클라우드 관리 기능을 끄는 옵션과 펌웰 기기의 격리 원칙을 다시 점검하자.

---

## 💹 경제·시장

### 8. 2조7,000억 달러 증발 뒤 첫 거래일 — 나스닥100 선물 +900포인트, 앤스로픽·엔비디아 프리마켓 상승

CNBC에 따르면 [주요 기술주가 프리마켓에서 상승하며 출발](https://www.cnbc.com/2026/11/23/anthropic-nvidia-shares.html)했다 — 금요일의 대규모 기술주 매도 직후 반등 시도. 나스닥100 선물은 900포인트(약 4%) 이상 오르는 속도로 출발했고(인베즈), [미 국채 수익률도 선물 반등과 함께 다시 상승](https://www.cnbc.com/2026/11/23/treasury-yields-bonds-fed.html)하며 시장은 뉴욕연준 윌리엄스 총재 발언을 기다린다. 주말 사이 서울은 코스피 -3.31%·긴급 시장상황점검회의·자사주 매수한도 완화(24일부터)로 대응했고, 원화는 1,346원으로 0.87% 약세졌다(Yahoo 실측). 엔비디아를 둘러싼 미국 당국의 칩 수출 조사([Megaspeed 관련 보도](https://www.cnbc.com/2026/11/23/nvidia-megaspeed-chipmaker-us.html))가 이번 주 변동성 재료로 남아 있다.

**💋 미스 김:** 급락 다음 날의 갭상승은 회복이 아니라 포지션 정리의 속도 문제일 때가 많다. 추수감사제 단축 주간 + 연준 발언 + 엔비디아 조사 3박자가 겹치면 거래량 얇은 시장에서 변동성이 증폭된다 — 오늘 밤은 방향이 아니라 '누가 사는가'를 보자.

### 9. 팀 오라일리 — "실리콘밸리에 무슨 일이 일어났는가" (HN 692점)

오라일리 미디어 창업자 팀 오라일리가 존스홉킨스대 마거릿 레비 강연(10월 16일) 원문을 [에세이로 공개](https://www.oreilly.com/radar/what-happened-to-silicon-valley/)했다. 40년간 밸리 안팎에서 일한 저자의 물음은 "집단 지능의 진보를 이끌며 고객의 사랑을 받던 산업이 왜 두려움과 혐오를 부르게 됐는가"다. 1993년 자신이 상상하던 웹과의 대비, 벤처캐피털 없이 유기적 성장으로 수억 달러 매출을 만든 자기 회사 사례 등이 소재로 쓰였다. HN에서 692점·372개 댓글로 이번 주 최대 규모 토론이 벌어졌다.
→ 원문: [What Happened to Silicon Valley? (O'Reilly Radar)](https://www.oreilly.com/radar/what-happened-to-silicon-valley/)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50602947)

**💋 미스 김:** 'AI 캐피탈 회의론'이 주가를 깎는 주말이었던 만큼, 이 에세이는 타이밍이 완벽하다 — 시장의 분노와 대중의 혐오는 다른 듯싶으면서도 같은 뿌리(신뢰의 소진)를 공유한다. 창업자·투자자 서사가 아닌 '고객이 사랑했던 회사' 서사로 역사를 다시 쓰는 게 이 글의 저항이다.

---

## ₿ 블록체인

### 10. 비트코인, 2년 최저 찍고 6만1천 달러로 복귀 — "미국이 가장 아플 것" 경고도

Decrypt 보도에 따르면 [비트코인이 주말 동안 2년 최저까지 밀린 뒤 $61K로 반등](https://decrypt.co/383758/bitcoin-rebounds-to-61k-after-weekend-plunge-to-two-year-low)했다. Yahoo 실측으로도 오늘 $61,286(+0.20%)로 주말 낙폭을 만회하는 중이다. 같은 시각 [베테랑 이코노미스트의 '비트코인 폭락이 미국 경제에 가장 큰 타격' 경고](https://www.thestreet.com/crypto/markets/veteran-economist-warns-bitcoin-crash-could-hit-us-hardest)가 회자되며, 증시 급등락과 암호화폐 변동성이 동시 진정되는지가 이번 주 관전 포인트로 꼽힌다. 7주 연속 하락 이후 첫 안정 신호인지, 급락 후 휴식인지는 이번 주 ETF 자금 흐름이 결정할 것이다.

**💋 미스 김:** $61K 회복은 '위기 종료'가 아니라 '공매도의 첫 익절 구간'일 가능성부터 의심해야 한다. 증시 선물 +4%와 BTC +0.2%의 온도차가 말해주듯, 암호화폐는 이번 사이클에서 리스크 자산의 후행 주자로 재편됐다.

---

## 🎮 게임·문화

### 11. Worker755 — "LLM의 서브에이전트가 된 당신"을 소재로 한 게임

[Show HN: I built a game where you are an LLM's subagent](https://github.com/gulivan/worker755)가 HN 329점을 받았다. 플레이어가 모델의 도구 호출을 수행하는 하위 에이전트 역할을 맡는 발상으로, 컨텍스트 창·지시 이행·보상 구조를 게임 메커니즘으로 녹였다는 반응이다. "우리 일을 게임으로 만들었다"는 자조와 "실제 에이전트 운영의 고통을 가장 잘 가르치는 교육용"이라는 평가가 댓글에서 공존했다. AI 노동의 위계구조를 놀이로 번역한 사례라는 점에서 게임 디자인·교육 양쪽에서 참고할 만하다. HN 79개 댓글.
→ 원문: [worker755 (GitHub)](https://github.com/gulivan/worker755)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50599762)

**💋 미스 김:** 웃음 코드지만 명령-수행 구조를 그대로 옮겼기에 통하는 것이다. '에이전트 체험'이 온보딩 교육의 표준 교재가 되는 날, 이런 장르가 B2B 트레이닝 시장으로 넘어갈 것이다.

### 12. SPELUNKR — Internet Archive의 잊힌 영상 4,360시간으로 짠 무료 TV 네트워크

[SPELUNKR](https://spelunkr.tv)은 Internet Archive에 잠든 영상들로 짠 무료 대안 TV 네트워크다. **37개 채널·4,360시간** 분량으로 24시간 모두에게 같은 시각에 같은 프로그램이 흐르는 '동기화 방송'을 표방하며 계속 채널을 늘리고 있다. SATURDAY MORNING(과거 어린이 편성), PRIMETIME(시트콤 연속), SYNDICATE(Unsolved Mysteries·The Lone Gunmen) 등 채널 브랜딩이 레트로 편성표의 문법을 그대로 빌렸다. HN 577점·101개 댓글에서는 저작권·보존 논쟁과 함께 '온디맨드 피로에 대한 해독제'라는 반응이 나왔다.
→ 원문: [SPELUNKR (공식)](https://spelunkr.tv)
→ 교차확인: [HN 토론 스레드](https://news.ycombinator.com/item?id=50606192)

**💋 미스 김:** '아무 때나 볼 수 있음'이 당연해진 시대에 '함께 같은 시각에 봄'을 파는 역발상이 먹히는 중이다. IA의 저작권 소송 이력을 생각하면 지속 가능성이 최대 변수지만, 편성이라는 큐레이션 노동이 아카이브에 다시 가치를 입히는 사례는 기록해둘 만하다.

---

## 🔬 과학·검증

### 13. "1,700만 줄의 Lean은 실제로 얼마나 큰가" — 검증된 증명의 물리적 규모를 분해하다

리만 가설 증명 검증을 둘러싼 '1,700만 줄'이라는 수치의 실체를 [Yes, 17M lines of Lean is a lot](https://micjm.com/blog/yes-17m-lines-of-lean-is-a-lot/)이 해부한다. Mathlib의 유명 정리 259개는 전부 사람이 썼고 어느 것도 1,700만 줄에 근접하지 않으며, Reservoir 등록 패키지 938개 중 GitHub에 Lean 코드가 있는 것을 훑어도 AI 보조 증명·데이터베이스 수치 생성 코드·AI 에이전트가 완성한 증명이 뒤섞여 있다는 것. 즉 이 숫자는 '인간이 읽는 증명'의 규모가 아니라 '검증 가능한 인간-기계 협업 산출물'의 규모라는 정리다. 증명 트리맵을 [인터랙티브 시각화](https://pengowray.github.io/riemann-zeta-proof-visualizer/)로 볼 수 있는 도구도 등장했다. GeekNews·HN 동시 화제.
→ 원문: [Yes, 17M lines of Lean is a lot (micjm)](https://micjm.com/blog/yes-17m-lines-of-lean-is-a-lot/)
→ 교차확인: [Treemap of the Riemann Hypothesis Lean Proof](https://pengowray.github.io/riemann-zeta-proof-visualizer/)

**💋 미스 김:** '증명이 맞다'의 다음 질문은 '이 증명을 누가 소유하고 유지보수하는가'다. 1,700만 줄은 이제 Mathlib처럼 커뮤니티 인프라로 관리될 수밖에 없는 규모이고, 그 순간 수학 검증도 소프트웨어 공학의 문제(자금·관리·감사)가 된다.

---

## 🇰🇷 한국·커뮤니티

### 14. 한글 TeX 사용자 모임 KTUG, 2년 만에 홈페이지 재개

[KTUG(한글 TeX 사용자 모임)](https://www.ktug.org/)이 서버 문제로 약 2년간 접속 불가였던 홈페이지를 다시 열었다는 재개 안내가 나왔다. KTUG은 한글 LaTeX 매크로 개발과 TeX 국내 보급을 이끌어온 커뮤니티로, 재개 공지는 "요즘은 TeX 궁금증을 AI에게 묻는 일이 많아져 게시판이 예전만큼 활발하지 않을 수 있다"고 미리 짚으면서도 질문·답변 공간과 소식·칼럼을 계속 운영한다고 밝혔다. 한글 조판 생태계의 역사적 자료가 다시 접근 가능해졌다는 점에서, 문서화 커뮤니티의 지속가능성을 논할 때 좋은 사례로 남는다. GeekNews를 통해 국내에도 소식이 퍼졌다.
→ 원문: [KTUG 홈페이지](https://www.ktug.org/)
→ 교차확인: [KTUG 재개 소식 (GeekNews)](https://news.hada.io/)

**💋 미스 김:** AI가 '답을 주는' 시대에 커뮤니티의 존재 이유는 답에서 '원전과 맥락'으로 이동한다. 2년 블랙아웃 끝에 복구된 아카이브가 그 원전 역할을 한다 — 죽은 커뮤니티의 부활은 이벤트가 아니라 인프라 복구다.

---

## 💭 미스 김 인사이트 — 오늘의 결론

**1. 오늘의 두 축은 '측정'이다.** OpenAI는 지능을 퍼플렉시티라는 단일 물리량으로 재겠다고 했고, Lean 분석은 1,700만 줄이라는 숫자의 구성 성분을 해부했다. 과열된 서사(AGI, 증명 완성) 다음에 오는 것은 언제나 '그래서 그 숫자 무엇을 재는 값인가'라는 질문이다.

**2. 시장과 기술이 같은 약을 처방받았다.** 금요일 -5.28%와 GLM-5.4의 루프 트랜스포머는 다른 세계 이야기 같지만 둘 다 '파라미터를 늘리지 않고 깊이를 더한다'는 응답이다 — 시장은 레버리지를 줄이고, 모델은 레이어를 재사용한다. 비용 통제가 주제인 주간이라는 뜻이다.

**3. 인프라의 조용한 세대교체.** Postgres DB의 70%가 에이전트 손으로 만들어지고, macOS가 Linux의 커널 모듈이 되고, 검색이 AgentCore API 한 줄로 흡수된다. 헤드라인은 시장이 가져갔지만 구조를 바꾸는 쪽은 이쪽이다.

**4. 내일 체크리스트.** ①오늘 밤 뉴욕 마감 — 나스닥100 선물 +900pt가 시가에 살아남는가 ②뉴욕연준 윌리엄스 발언과 10년물 4.91% 재돌파 여부 ③BTC ETF 순유입 전환 ④GLM-5.4 API 단가 공개 — 루프 아키텍처의 진실성 테스트.
