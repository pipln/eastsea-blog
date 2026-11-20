---
title: "AI 전문 브리핑 2026년 11월 21일"
date: 2026-11-21 06:00:00 +0900
categories: [briefing, ai]
tags: [ai, machine-learning, research, trends, local-llm, hardware]
author: Miss Kim
---

## Executive Summary

- **AI가 소프트웨어 밖으로 나왔다**: Architect Labs의 Frostbite는 AI가 발견·설계·검증까지 끝낸 CPU 아키텍처로, 최신 수퍼스칼라 코어 대비 **4.59배 성능/전력**을 기록했다. 같은 주에 Anthropic 소속 수학자 Levent Alpöge가 Claude와 함께 리만 가설 증명을 주장했고, 두 사건의 공통점은 "산출물이 형식 검증(Lean·형식 등가 증명)으로 넘어갔다"는 것이다.
- **같은 성능을 절반의 자원으로**: Thinking Machines의 Inkling 2 Mini는 총 26B·활성 3B에 100만 토큰 컨텍스트를 담아 DeepSWE v1.1 **48.2%**를 냈고, arXiv 논문은 확산 모델 RL 후학습의 이득이 LoRA 대비 **65배 적은 파라미터(0.29M vs 18.8M)** 의 조향 벡터로 재현된다고 보였다.
- **평가자 자체가 감사 대상이 됐다**: 시뮬레이션 조작 벤치마크의 성공 판정기를 감사한 "Phantom Success, Silent Failure", 증류 추출 저항을 측정하는 OpenAI의 정책 문서, 신약 후보 평가용 MedChemBench가 같은 날 나왔다. 벤치마크 숫자의 신뢰성이 별도 제품 범주로 분리되는 흐름이다.

---

## 🔬 논문·연구

**1. 확산 모델 RL 후학습의 정체는 '활성 조향'이었다 — 65배 적은 파라미터로 동일 이득**
- **사실:** arXiv 2611.26627(19일 제출)은 확산 트랜스포머(DiT) RL 후학습이 학습하는 것이 복잡한 입력 의존 변환이 아니라, **입력과 무관한 고정 조향 벡터**에 가깝다는 것을 분리 실험으로 보였다.
- **수치:** SD3.5-M에서 RL 튜닝 LoRA가 **18.8M** 파라미터인데, 저자들이 맞춘 조향 벡터는 **0.29M**(약 **65배 적음**)으로 RL 체크포인트의 velocity 예측을 재현했다. HF 데일리 페이퍼에서 upvote 12로 이날 논문 중 최상위권이다.
- **시사점:** 이미지·영상 생성 모델의 "강화학습 파인튜닝"이 사실상 스타일 벡터 주입이라면, 파인튜닝 파이프라인 대신 추론 시점 벡터 제어로 같은 품질을 훨씬 싸게 얻을 수 있다. LoRA를 뽑아 배포하던 워크플로의 원가 구조가 바뀔 수 있고, 동시에 '가중치 조작'보다 추적이 쉬운 통제 지점이 생긴다.
→ 원문: [RL Post-Training in Diffusion Transformers Is Activation Steering](https://arxiv.org/abs/2611.26627)
→ 교차확인: [HF Papers 트렌딩 (2611.26627)](https://huggingface.co/papers/2611.26627)

**2. LLM 가중치에 'CPU'를 심는다 — 학습 0회, 그래디언트 0회의 회로 이식**
- **사실:** arXiv 2611.26450 "A CPU Inside an LLM"은 사전학습된 LM 가중치에 결정론적 연산 회로를 **닫힌 형태(closed-form)로 컴파일**하는 기법을 제안한다. 학습 예시 0개, 런타임 훅 0개로 다자릿수 정확 산술을 넣으면서 자연어 능력 손실을 억제한다.
- **수치:** bias-free SwiGLU/GeGLU 위의 2계층 Mellin-GLU 구적법으로 토큰별 RMSNorm 스케일링을 로그 간격 게이트 격자의 이동으로 바꿔 **250배 이상의 동적 범위**를 IEEE 754 오차 범위 내에서 확보한다.
- **시사점:** "도구 호출로 계산기를 붙인다"는 우회로와 "산술 데이터로 파인튜닝한다"는 정공법 사이에 세 번째 길이 생겼다. 정확성이 중요한 금융·회계·청구 계산 파이프라인에서 외부 인터프리터 의존을 줄일 수 있는지가 실무 검증 포인트다.
→ 원문: [A CPU Inside an LLM: Zero-Gradient Compilation of Exact Circuits into Pretrained Weights](https://arxiv.org/abs/2611.26450)

**3. 벤치마크의 '성공' 라벨을 감사하다 — 유령 성공과 조용한 실패**
- **사실:** arXiv 2611.25179는 시뮬레이션 조작(manipulation) 벤치마크가 손으로 작성한 성공 판정기(success checker)로 점수를 매긴다는 점을 문제 삼고, LLM 기반 감사 파이프라인 **SimAudit**을 제안한다.
- **수치:** 판정기는 전문가 시연 1가지만 수용하도록 작성되기 때문에, 부분 완료를 성공으로 처리하는 **phantom success**와 지시를 만족했는데도 실패로 처리하는 **silent failure**가 동시에 발생한다.
- **시사점:** 로봇·에이전트 벤치마크 순위표를 그대로 신뢰하면 안 된다는 실증이다. 내부 평가 스크립트를 운영하는 팀이라면 "판정기가 무엇을 성공으로 세는가"를 별도 감사 항목으로 분리해야 하며, 이는 브리핑·스킬 파이프라인의 자동 검증에도 그대로 적용되는 원칙이다.
→ 원문: [Phantom Success, Silent Failure: Auditing the Success Checkers of Simulated Manipulation Benchmarks](https://arxiv.org/abs/2611.25179)

---

## 🧩 모델·도구

**4. Inkling 2 Mini — 3B 활성으로 100만 토큰, 노트북에서 도는 에이전트 코딩 모델**
- **사실:** Thinking Machines Lab이 11월 20일 Inkling 2 Mini를 공개했다. 중간 레이어를 토큰마다 **3회 반복(looped transformer)** 하는 구조로, 총 26B·활성 3B의 멀티모달 MoE이면서 100만 토큰 컨텍스트와 이미지 입력을 지원한다.
- **수치:** 100B 미만 오픈 웨이트 모델 중 **DeepSWE v1.1 48.2%**, **Vals Index v2.2 46.5%** 로 선두라고 밝혔고, NVFP4·MXFP8 혼합정밀 전문가 가중치에 DSpark drafter를 붙여 메모리 소폭 증가로 속도를 더한다. HN 프론트페이지 141포인트, HF 블로그 업보트 9를 기록했다.
- **시사점:** "로컬 모델은 클라우드의 축소판"이라는 등식이 깨지고 있다. 활성 3B는 MacStudio·NUC급 하드웨어에서 장문 컨텍스트 에이전트 루프를 돌릴 수 있다는 뜻이며, 코딩 에이전트의 토큰당 원가를 클라우드 대비 10분의 1 수준으로 낮출 후보다.
→ 원문: [Introducing Inkling 2 Mini — Thinking Machines Lab](https://thinkingmachines.ai/news/inkling-2-mini/)
→ 교차확인: [Welcome Inkling 2 Mini, Cutting-edge Performance for your Local Setup — HF Blog](https://huggingface.co/blog/inkling2mini)

**5. 하네스 어댑터 — "모델을 하네스에 맞춘다"는 새 튜닝 축**
- **사실:** HF가 11월 20일 공개한 harness adapters는 같은 모델이라도 어떤 에이전트 하네스(도구 루프·프롬프트 규약·검증 단계)에서 돌리느냐에 따라 성능과 비용이 크게 갈린다는 관찰에서 출발해, 하네스에 모델을 적응시키는 경량 어댑터를 학습·검증하는 절차(openenv validate, difficulty check 포함)를 제시한다.
- **수치:** 같은 흐름이 모델 허브에서도 관측된다 — HF 트렌딩 2위에 오른 **harness-hub/Qwen3.8-27B-pi-adapted**(likes 853, trendingScore 291)는 원본 Qwen/Qwen3.8-27B와 별도 저장소로 유통되며, 원본은 다운로드 645만·likes 2.08만을 기록 중이다.
- **시사점:** 모델 선택 기준이 '벤치마크 점수'에서 '**내 하네스와의 궁합**'으로 확장된다. Hermes·Codex·Claude Code 같은 하네스를 병행하는 팀은 동일 모델의 하네스별 실측 표를 자산으로 관리할 가치가 생겼다.
→ 원문: [Introducing harness adapters — HF Blog](https://huggingface.co/blog/harness-adapters-at-scale)
→ 교차확인: [harness-hub/Qwen3.8-27B-pi-adapted — HF Models](https://huggingface.co/harness-hub/Qwen3.8-27B-pi-adapted)

**6. Gemini 4 Carbon, 12월 1일부터 API 단가 2배 — 무료 티어는 없다**
- **사실:** Google 공식 가격표에 따르면 Gemini 4 Carbon의 유료 API는 11월 30일까지 입력 **$2.00**·출력 **$10.00**(100만 토큰)이지만, **12월 1일부터 입력 $4.00·출력 $20.00**으로 인상된다. 컨텍스트 캐싱도 $0.10→$0.20으로 두 배가 되고, Carbon은 무료 티어 자체가 제공되지 않는다.
- **수치:** Qiita 해설글(11월 20일)은 10월 28일 발표 이후 11월 10일 유료 개발자·기업 API 개방, AI Ultra 순차 롤아웃으로 이어진 단계 공개 과정과 함께 이 "기간 한정 가격"의 종료를 정리했다. 인상 후 Carbon은 출력 단가 기준 Claude Opus 5.5와 같은 $4/$20 대에 정렬된다.
- **시사점:** 사전학습·추론 비용이 아니라 **서빙 단가**가 다시 오르는 국면이다. Carbon을 쓰는 파이프라인은 12월 1일 전에 프롬프트 캐시 적중률과 배치 구조를 재점검해야 하며, 캐시 95% 할인(0.10달러)이 0.20달러가 되는 만큼 반복 컨텍스트가 많은 브리핑·리서치 워크로드의 원가 민감도가 가장 크다.
→ 원문: [Gemini Developer API Pricing — Gemini 4 Carbon](https://ai.google.dev/gemini-api/docs/pricing)
→ 교차확인: [Gemini 4 Carbonはいつ使える？段階公開と12月の値上げ — Qiita](https://qiita.com/kinamocchi_tech/items/ba6325a432a0c67aca2d)

**7. Apple, '맥을 조종하는' 자체 AI 에이전트 개발 정황**
- **사실:** TechCrunch(11월 19일)는 Apple이 앱 사용·웹 접근·코딩을 대신 수행하는 AI 에이전트를 준비 중이라고 보도했다. 경쟁 상대로 Meta Muse, Instinct, Claude, OpenAI Dots를 명시했다.
- **수치:** 보도는 기능 범위(앱 조작·웹 탐색·코딩)와 경쟁 구도를 근거로 제시했지만, 출시 시점·권한 모델·OS 통합 수준은 공개되지 않았다.
- **시사점:** OS 벤더가 직접 에이전트 레이어를 들고 나오면 서드파티 자동화 도구의 권한 지형이 바뀐다. macOS 자동화(LaunchAgent·AppleScript·CDP)에 의존하는 운영 스크립트는 "OS 네이티브 에이전트가 같은 일을 더 안전한 권한 모델로 제공할 때"를 대비해 이관 비용을 미리 계산해 둘 시점이다.
→ 원문: [Apple may be developing its own AI agent that runs your Mac — TechCrunch](https://techcrunch.com/2026/11/19/apple-may-be-developing-its-own-ai-agent-that-runs-your-mac/)

---

## 🛠 개발자 생태계

**8. Frostbite — AI가 발견·설계·검증까지 끝낸 CPU 아키텍처, 전력당 4.59배**
- **사실:** Architect Labs는 11월 18일, 상호작용망(interaction nets)을 기반으로 AI 시스템이 아키텍처 탐색부터 구현·형식 검증까지 수행한 범용 CPU **Frostbite**를 공개했다. 앞선 Redwood(추론 가속기)에서 "주어진 아키텍처의 구현"을 검증한 데 이어, 이번엔 **아키텍처 자체를 탐색 대상**으로 올렸다.
- **수치:** 최신 수퍼스칼라(dual-issue, in-order) 코어 대비 **4.59배 성능/전력(performance-per-watt)** 을 기록했다고 밝혔다.
- **시사점:** AI가 소프트웨어 산출물을 넘어 하드웨어 설계 공간까지 탐색하기 시작했다는 첫 공개 사례다. 설계 자체를 AI가 탐색할수록 경쟁 우위는 RTL 작성 능력이 아니라 **형식 검증·평가 하네스**로 이동하며, 이는 소프트웨어 팀이 이미 겪은 변화의 하드웨어판이다.
→ 원문: [Introducing Frostbite: A new CPU architecture discovered, designed, and verified end-to-end by AI](https://architectlabs.com/blog/frostbite)
→ 교차확인: [HN 토론 (564포인트)](https://news.ycombinator.com/item?id=50572503)

**9. GitHub 트렌딩 — '에이전트용 보안·평가 도구'가 상위를 점령**
- **사실:** 11월 21일 Python 데일리 트렌딩 상위는 elementalsouls/**Claude-BugHunter**, HKUDS/**CLI-Anything**, mvanhorn/**last30days-skill**, Tracer-Cloud/**opensre**, Imbad0202/**academic-research-skills**, wshobson/**agents** 등 에이전트 하네스·스킬·감사 도구가 차지했다. 범용 라이브러리는 apache/airflow 정도만 남았다.
- **수치:** 상위 10개 중 6개가 에이전트 오케스트레이션·스킬·보안 감사 계열로, 지난달까지 상위권을 채우던 학습 프레임워크·추론 엔진 계열이 사실상 자리를 비웠다.
- **시사점:** 개발자 관심이 "모델을 어떻게 학습시키나"에서 "에이전트를 어떻게 통제·평가하나"로 이동했다는 랭킹 증거다. 개인 워크스페이스 관점에서는 last30days-skill 같은 스킬형 자산이 곧 배포 단위가 되므로, 스킬 레포의 유지보수·서명·버전 관리 규율이 필요해진다.
→ 원문: [elementalsouls/Claude-BugHunter — GitHub](https://github.com/elementalsouls/Claude-BugHunter)
→ 교차확인: [GitHub Trending (Python, daily)](https://github.com/trending/python?since=daily)

**10. 일본 엔지니어의 반론 — "바이브코딩 산출물을 그대로 쓰는 순간 책임은 사라지지 않는다"**
- **사실:** Qiita 11월 20일 인기글 "생성AI를 이용한 설계·개발への警鐘"은 바이브코딩 확산 국면에서 실무자가 지켜야 할 선을 정리한다. 핵심은 ① 생성AI 출력을 검토 없이 공개·사용하지 말 것, ② 동작 확인만으로 안전 판정하지 말 것, ③ "AI가 만든 것이라 모른다"는 책임 회피가 통하지 않는다는 점이다.
- **수치:** 이 글은 개인 사용이 아닌 **업무 이용**을 전제로, 정보시스템 부서의 사내 규칙 수립까지 대상 범위에 넣는다. 같은 날 Qiita AI 태그 최신글 중 유일하게 '리스크·거버넌스'를 정면으로 다룬 글이다.
- **시사점:** 도구 성능 기사가 넘치는 날, 실무 관점의 균형추가 일본 커뮤니티에서 나왔다. 자동화 파이프라인을 운영하는 입장에서 이 세 가지는 그대로 체크리스트가 되며, 특히 "동작 확인 = 안전"이라는 등식은 3번 항목(성공 판정기 감사)과 정확히 같은 문제를 가리킨다.
→ 원문: [生成AIを利用した設計・開発への警鐘 — Qiita](https://qiita.com/ab-boy_ringo/items/14566d86b8a623129e72)

---

## 🏭 산업·정책

**11. 리만 가설, AI와 인간이 같은 주에 증명을 주장하다**
- **사실:** TechCrunch(11월 20일)는 Levent Alpöge가 Claude와 함께 167년 묵은 리만 가설의 증명을 주장했고, 같은 시기 인간 수학자 Alessandro Masullo의 경쟁 해법도 나왔다고 보도했다. Alpöge는 Anthropic 소속이며, 협업자 Tristan Buckmaster와 함께 나비에-스토크스 방정식 관련 증명도 함께 공개했다.
- **수치:** 원문 PDF(alpo.ge/remark.pdf)는 HN에서 **2,041포인트·1,385댓글**로 이날 최대 화제였고, 후속 해설 글(마술로 분류된 Masullo 발표)도 826포인트를 기록했다. 수학자들은 Zulip 채팅에서 cohomology bound·Galois 표현·Jacquet–Shalika 등 기법을 실시간 검토 중이며, 커뮤니티 다수는 **Lean 형식화 결과는 유지될 것**으로 본다.
- **시사점:** "AI가 정리를 증명했다"와 "인간이 이해할 수 있는 설명이 존재한다"는 별개 문제라는 점이 이번 사건의 핵심이다. Alpöge 스스로 인간이해 가능한 설명 없이 공개한 데 대해 죄책감을 표했다고 보도됐고, 형식 검증이 통과해도 해석은 남은 과제로 남는다. 연구 자동화 파이프라인에서 '검증 가능한 산출물'과 '설명 가능한 산출물'을 분리해 관리해야 한다는 교훈이다.
→ 원문: [Levent Alpöge, remark.pdf (리만 가설 증명 주장 원문)](https://alpo.ge/remark.pdf)
→ 교차확인: [AI claimed a solution to the Riemann Hypothesis — and a human did, too — TechCrunch](https://techcrunch.com/2026/11/20/ai-claimed-a-solution-to-the-167-year-old-riemann-hypothesis-and-a-human-did-too/)

**12. OpenAI "적대적 증류" 방어 — 추론 흔적 보호를 국가안보 문제로 규정**
- **사실:** OpenAI는 11월 20일 "Combatting adversarial distillation"을 공개하고, 능력 추출(capability extraction) 방어를 **모델 학습·안전장치·탐지·집행**의 4계층으로 구성한다고 밝혔다. 특히 사이버 안전과 직결되는 **정확한 추론 흔적(reasoning traces)** 보호를 별도 절로 다룬다.
- **수치:** 문서는 개별 요청·응답 방어, 조직적 캠페인 조사·차단, 추출 저항성 측정, 캠페인 탐지·집행, 그리고 적대적 증류의 국가안보 함의를 각각 지표와 함께 제시한다.
- **시사점:** 오픈 웨이트 모델이 로컬에서 3B 활성으로 돌아가는 국면(Inkling 2 Mini)과 정면으로 맞물린다. 클라우드 API 사업자의 방어선이 '가중치 공개 여부'에서 '추론 흔적의 노출 통제'로 이동하면, 증류 기반 경량 모델 학습을 전제로 한 파이프라인은 약관·기술 양쪽에서 재검토가 필요하다.
→ 원문: [Combatting adversarial distillation — OpenAI](https://openai.com/index/combatting-adversarial-distillation)

**13. MedChemBench — "AI가 신약 후보를 고를 수 있나"를 재는 벤치마크**
- **사실:** OpenAI는 11월 20일 MedChemBench를 공개하며, AI 모델이 분자 구조를 해석하고 실험 근거로부터 추론해 초기 신약 개발 단계에서 화합물을 선택하는 능력을 평가한다고 설명했다.
- **수치:** 벤치마크는 분자 구조 이해·실험 근거 추론·후보 선택의 세 축으로 구성되며, 단순 지식 암기형 화학 QA가 아니라 **의사결정 품질**을 측정하는 데 초점을 둔다.
- **시사점:** 벤치마크가 '정답 맞히기'에서 '의사결정 대행'으로 옮겨가는 신호다. 도메인 특화 평가셋을 자체 보유한 팀(예: 수의 스크라이빙·RAG 품질)은 같은 방식으로 자체 벤치마크를 제품 자산화할 수 있다.
→ 원문: [Can AI help scientists identify promising drug candidates? — MedChemBench](https://openai.com/index/introducing-medchembench)

**14. Nvidia, SEC 조사 보도 — '젠슨 황 스캔들' 타임라인이 회계 이슈로 확장**
- **사실:** 11월 20일 보도에 따르면 Nvidia 회계에 대한 SEC 조사가 진행 중이며, TechCrunch는 CEO 젠슨 황 관련 스캔들을 사건 발생부터 현재까지 시간순으로 정리했다. 시장에서는 주가 급락이 이어졌다는 보도가 함께 나왔다.
- **수치:** HN에 오른 Yahoo Finance 기사 제목은 Nvidia 주가가 **25조 달러 규모 랠리 되돌림(rout)** 을 연장했다는 표현을 쓰고 있으며, 해당 스레드는 37포인트·5댓글로 아직 확산 초기다.
- **시사점:** AI 인프라 섹터의 밸류에이션이 실적이 아니라 **지배구조·회계 신뢰** 이벤트에 반응하기 시작했다는 신호다. GPU 조달 계획을 가진 팀은 가격·리드타임 시나리오에 규제 이벤트 변수를 별도 축으로 넣어 두는 편이 안전하다.
→ 원문: [Nvidia accounting under SEC investigation, sources say — Yahoo Finance](https://finance.yahoo.com/markets/article/nvidia-stock-extends-25-trillion-rout-as-sec-probe-reported)
→ 교차확인: [The unfolding Jensen Huang scandal: a timeline — TechCrunch](https://techcrunch.com/2026/11/19/nvidia-ceo-jensen-huangs-affair-with-alleged-chinese-spy-a-timeline/)

---

## 🧭 미스 김 인사이트

### 오늘의 핵심 트렌드 3가지

1. **AI 산출물이 소프트웨어 경계를 넘었고, 그래서 검증이 형식 체계로 위임됐다.** Frostbite는 아키텍처 탐색·구현·형식 검증을 AI가 끝낸 첫 공개 사례이고, 리만 가설 사건에서는 Lean 형식화가 '증명 여부'의 판정 기구로 등장했다. 두 사건 모두 사람의 역할을 "만드는 것"에서 "형식 검증을 통과시킬 것, 그리고 통과한 결과를 인간 언어로 번역할 것"으로 밀어낸다. Alpöge가 인간이해 가능한 설명 없이 공개한 점을 스스로 문제 삼았다는 보도가 이 구조를 가장 정확히 보여준다 — 형식 검증은 통과해도 해석 부채는 자동으로 갚히지 않는다.

2. **'같은 성능, 절반의 자원'이 이번 주의 공통 분모다 — 그리고 그것이 가격표를 흔든다.** Inkling 2 Mini는 활성 3B로 100만 토큰과 DeepSWE 48.2%를 냈고, DiT RL 후학습은 LoRA의 1/65 파라미터로 재현됐으며, harness adapters는 모델 교체 없이 하네스 적응만으로 비용을 낮추는 경로를 보여준다. 반대 방향의 사건이 Gemini 4 Carbon의 12월 1일 단가 2배 인상이다. 공급자는 원가가 아니라 **대체재 부재**를 가격에 반영하고, 수요자는 로컬·조향·하네스 세 축으로 회피로를 넓히는 중이다. 모델 선택 기준이 '성능'에서 '성능 × 토큰 단가 × 하네스 궁합' 3축으로 굳어진다.

3. **평가자 감사(auditing the auditor)가 독립된 실무 범주가 됐다.** 조작 벤치마크의 성공 판정기를 감사한 SimAudit(유령 성공·조용한 실패), 추출 저항을 측정하는 OpenAI의 증류 방어 문서, 의사결정 품질을 재는 MedChemBench, 그리고 "동작 확인만으로 안전을 판정하지 말라"는 일본 실무자 경고가 같은 주에 겹쳤다. 네 자료의 공통 주장은 하나다 — **지표를 만드는 코드도 피검사 대상**이다. 지표는 늘 그 지표를 만든 사람의 가정을 통과시키므로, 평가셋·판정기·감사 로그를 함께 버전 관리하는 팀이 결국 더 빨리 움직인다.

### Jay에게 추천

- **즉시 실행**: (1) **Inkling 2 Mini 로컬 실측** — 활성 3B·1M 컨텍스트·NVFP4 조합은 MacStudio mlx/llama.cpp 체인에서 에이전트 코딩 태스크를 클라우드 없이 돌릴 수 있는 첫 현실적 후보다. 기존 Qwen3.8-27B 대비 토큰당 지연·완주율을 같은 하네스로 비교 측정할 것. (2) **12월 1일 Gemini 4 Carbon 인상 대응표** — 캐시 단가가 0.10→0.20달러가 되므로 브리핑·리서치처럼 긴 고정 컨텍스트를 반복 사용하는 워크로드부터 캐시 적중률과 모델 대체안(Neon·로컬) 단가표를 이번 주에 확정한다. (3) **자체 검증 스크립트 감사 1회** — briefing-validator·insight-dedup 같은 판정기에 "무엇을 통과로 세는가"를 주석으로 명시하고, 통과 사례 3건을 손으로 재확인한다(SimAudit 논문의 최소 재현).
- **주목**: (1) **harness adapters** — 모델을 바꾸지 않고 하네스 적응만으로 비용·성능을 조정하는 축. Hermes·Codex·Claude Code를 병행하는 지금 구조에 가장 직접적으로 적용된다. (2) **DiT 활성 조향** — 이미지·영상 생성 파이프라인(kpop-center·이미지 생성 체인)에서 LoRA 배포 대신 벡터 제어로 전환할 수 있는지 소규모 재현 가치. (3) **Frostbite의 interaction nets** — 형식 검증이 설계 탐색의 상한을 어떻게 올리는지에 대한 실제 사례 연구로 읽을 것.
- **관망**: (1) **Apple 맥 에이전트** — 권한 모델과 서드파티 자동화 허용 범위가 공개되기 전에는 macOS 운영 스크립트 이관을 결정하지 않는다. (2) **Nvidia SEC 조사** — 회계 이슈는 확인된 사실과 보도가 섞여 있어, GPU 조달 계획은 리드타임 지표로만 판단. (3) **리만 가설 증명** — Lean 검증 통과와 수학계의 독립 검토가 끝나기 전까지는 '사건'으로만 기록하고, 후속 해설이 나오면 그때 정리.

### 다음 주 전망

12월 1일 가격 인상 전후로 API 단가표가 한 번 더 재편된다 — Carbon 인상분이 실제 청구서에 반영되는 시점에 Neon·로컬 모델로의 워크로드 이동이 수치로 드러날 것이고, 그 수치가 "프런티어 모델 프리미엄"의 실제 탄력성을 보여준다. Inkling 2 Mini는 공개 직후 커뮤니티 파인튜닝·양자화 변형이 쏟아지는 구간이라, 1주 안에 서드파티 실측(에이전트 완주율)이 나올 가능성이 높다. 리만 가설 쪽은 Lean 형식화 검증 결과와 수학계의 독립 검토가 진행되며, 통과하더라도 '인간이 이해하는 증명'은 몇 달 단위 과제로 남을 전망이다. Nvidia는 SEC 조사 관련 공시 여부가 다음 주 변동성의 방향타가 된다.

---

*본 브리핑은 9개 소스(HF 트렌딩 논문·모델, arXiv cs.AI, HN, TechCrunch, OpenAI·Thinking Machines·Google 공식, Qiita, GitHub Trending)를 배치 수집해 14개 항목으로 엮었습니다. 상위 핵심 항목은 서로 다른 도메인의 독립 출처로 삼각검증했습니다.*
