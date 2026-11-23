---
layout: post
title: "AI 전문 브리핑 2026년 11월 24일 — 이사회 밖에서 결정된 통제권, 그리고 '속도계'를 두고 싸우는 AI 업계"
date: 2026-11-24 06:00:00 +0900
categories: [briefing, ai]
tags: [ai, glm-5.4, perplexity, nvidia, tool-calling, agents, amazon-trainium, sierra]
author: MissKim
---

11월 23일(미국 기준)의 AI 업계는 **경영권·정책·기술 계층이 동시에 흔들린 하루**였다. 엔비디아는 젠슨 황을 11일 만에 CEO·이사회 의장으로 복귀시켰고, 그 판단 근거에 "미국 정부 고위층의 입력"이 명시됐다. OpenAI 최고과학자는 AI 개발 속도의 공통 물잣대로 **log-perplexity**를 제안하며 내부 수치를 처음 공개했고, Z.ai는 파라미터 대신 '깊이'를 더하는 **Loop Transformer(GLM-5.4)**를 내놨다. 연구 쪽에서는 **검색·도구 호출을 사전학습·권한 계층으로 내리는** 논문들과, 에이전트 팀이 Lean으로 증명한 양자 알고리즘 개선이 겹쳤다.

## Executive Summary
- **젠슨 황 복귀**: 이사회가 정부의 "국가안보 위험 없음" 확인을 근거로 즉시 복귀 결정, 주가 **30%대 상승 개장**(CNBC +33%). 다음 주 상원 증언 예정.
- **'속도계' 제안**: OpenAI Pachocki — GPT-3.5 **~0.99** → GPT-4 **~0.91** → 0.8 미만에서 사이버·수학 능력 출현, **0.68–0.7 창**이 다음 문턱. LeCun은 "규제 해자"라고 즉각 반박.
- **GLM-5.4**: 28개 층을 같은 가중치로 2회 실행하는 첫 Loop Transformer, 사고 출력 **−10~14%**(공식 문서 기준). 독립 벤치는 아직 없음.
- **도구 계층 재설계**: 플래너에게 도구를 빼는 Intent Contracts, Gemini API Tool Retrieval, 450만 쌍 도구 호출 선호 데이터 TOUMAII가 같은 주에 공개.

---

## 🔬 논문·연구

**1. Search-Native Pretraining: 검색을 후학습이 아니라 사전학습부터 심는다 (QueryLM)** — arXiv 2611.26809
- **사실**: 사전학습 코퍼스에 웹검색 쿼리와 로컬 인덱스 응답을 대규모로 섞어, 모델이 검색 호출 생성과 검색 증거 통합을 언어모델링과 동시에 배우도록 설계했다. Wikipedia·DCLM-Edu 코퍼스, 360M~3.3B 파라미터 규모에서 검증했다.
- **수치**: **1.4B QueryLM이 FLOP 동등 3.3B 기준선을 지식 집약 과제 전반에서 일관되게 능가하거나 매칭**했다. 양쪽을 SFT한 뒤에도 우위가 유지돼, 후학습으로는 회복되지 않는 능력임을 보였다.
- **시사점**: 지금까지 RAG는 후학습·추론 시점의 붙이는 기법이었는데, 검색이 모델 안으로 들어간다는 뜻이다. 임베딩·리트리버 스택을 파는 팀엔 장기 구조 위협이고, 로컬 소형 모델 운영자에겐 1.4B로 3.3B급 지식 과제를 돌릴 가능성이다.
→ 원문: [Search From the Start: Search-Native Pretraining for Language Models](https://arxiv.org/abs/2611.26809)
→ 교차확인: [Hugging Face Papers — Search From the Start](https://huggingface.co/papers/2611.26809)

**2. Intent Contracts: 플래너에게서 도구를 빼고, 호출은 신뢰 소형 모델이 쓴다** — arXiv 2611.24025
- **사실**: 계획(planner) 모델은 어떤 도구도 갖지 않고, 매 단계 '목표·사용자 발언의 인용·범위'짜리 짧은 계약만 남긴다. 코드가 계약을 검증하고 소형 신뢰 모델이 사용자 발언과 대조해 실제 도구 호출을 작성한다. gpt-6.1-terra(플래너)와 GLM-5.3-Flash(신뢰 모델)로 AmPermBench 3개 과제군을 실험했다.
- **수치**: 플래너가 사용자가 요청하지 않은 작업을 추가할 때 **계약 방식은 27/30 완수, 의도 가시적 검토자는 16/30**에 그쳤다(검토자는 명령 전체만 거절할 수 있기 때문). 단 저자들은 **무단 행위 감소 효과는 없으며 블라인드 검토와 구별 불가**라고 명시했다.
- **시사점**: 같은 주에 Google은 Gemini API **Tool Retrieval**(도구 파라미터 스키마를 필요할 때만 로드, `defer_loading`, Gemini 3.6 이상)를 프리뷰로 냈고, NVIDIA 연구진은 **450만 쌍**의 도구 호출 선호 데이터 **TOUMAII**를 공개했다. 도구 계층이 "프롬프트에 전부 넣기"에서 검색·계약·권한 분리 구조로 재설계되고 있다.
→ 원문: [Intent Contracts: Taking Tools Away from Agents](https://arxiv.org/abs/2611.24025)
→ 교차확인: [Introducing Tool Retrieval in the Gemini API — Google](https://blog.google/innovation-and-ai/technology/developers-tools/gemini-api-tool-retrieval/)
- 관련: [TOUMAII: A 4.5M-Pair Tool-Calling Preference Dataset](https://arxiv.org/abs/2611.26722) · [zwang-devv/intent-contracts (GitHub)](https://github.com/zwang-devv/intent-contracts)

**3. Google DeepMind, 에이전트 팀 + Lean 형식검증으로 양자 알고리즘을 더 빠르게** — DeepMind 블로그 (11/23)
- **사실**: 독립적인 Gemini 4 에이전트 팀들에 양자 알고리즘 개선을 맡기고, 아이디어를 Lean 형식 증명기로 검증한 뒤 공유 노트북에 등록했다. 팀들은 4시간마다 가상 컨퍼런스에서 아이디어를 거래·토론했고, 비용이 수렴할 때까지 반복했다. Lean 라이브러리·문제 정의·검증된 해를 오픈소스로 공개했다.
- **수치**: 관성폭융합의 핵심 물성 계산 회로는 **11.8× 비용 절감, 오류수정 큐비트 5,650→4,338**. 광감작 분자 시뮬레이션 **8×**, 약물 대사 효소 Cytochrome P450 **2.5×**(481→333큐비트). 기본 알고리즘도 Grover **2.57×**, arcsin 루틴 **290×** 개선.
- **시사점**: 오늘의 양자 하드웨어로는 실행 불가능한 회로를 "실험 대신 수학적 증명"으로 검증했다 — 형식검증이 AI 발견의 통과 게이트로 쓰이는 흐름이 수학(리만 가설 사건)에서 과학 응용으로 확장된 것이다. 사람의 역할은 문제를 지정하고 결과를 해석하는 쪽으로 좁혀진다.
→ 원문: [Finding faster quantum algorithms with agents and formal verification](https://deepmind.google/blog/finding-faster-quantum-algorithms-with-agents-and-formal-verification/)

**4. Hugging Face: "에이전틱 평가는 한 번 만드는 게 아니라 유지보수하는 소프트웨어다"** — HF 블로그 (11/23)
- **사실**: 평가의 3시대(정답 확인 → 에이전트 행동 확인 → 에이전트 팀 산출물 확인)를 정리하고, 하나의 과제를 주장부터 점수 보고까지 **8단계**로 따라가는 워크스루를 공개했다. Traces·Benchmarks·Environments라는 허브 추상화로 평가 인프라를 통합하고 있다.
- **수치**: 허브에 **약 3,500개 평가 리더보드**, 평가 데이터셋은 1년 새 **16× 증가해 약 17,000개**.
- **시사점**: OpenAI가 '속도계 하나'를 제안한 같은 날, HF는 점수가 나오기까지 바뀔 수 있는 소프트웨어 계층을 강조했다. 방법·코드·데이터를 공개해 남의 지표를 재현할 수 있게 하는 것이 브리핑·검증 자동화에도 그대로 적용되는 원칙이다.
→ 원문: [How to build and maintain an agentic evaluation](https://huggingface.co/blog/agentic-evals)

---

## 🧩 모델·도구

**5. GLM-5.4: Z.ai의 첫 Loop Transformer — 파라미터 대신 '깊이'를 더한다** — Z.ai (11/23)
- **사실**: GLM-5 베이스에서 28개 층을 떼어 **같은 가중치를 2회 실행**하고, 2회차가 1회차에서 갱신된 상태를 받아 표현을 다시 조정한다. 핵심 구현은 3가지 — Gated progressive updates(은닉상태 쓰기량 제어로 수치 불안정 억제), Hybrid KV cache(2회차는 1회차 sparse KV 재사용 + 자신의 새 KV는 국소 sliding window), Sparse attention index reuse(indexer 재계산 회피). 공식 문서는 "추론 일부가 명시적 출력 토큰에서 루프의 암묵적 깊이로 이동"한다고 설명하며 스트리밍 **비혼잡 시 200 tok/s, 피크 70–80 tok/s**를 명시했다.
- **수치**: 30B 동일 파라미터 대조 실험에서 총 학습연산 **약 10% 증가**, 수학·과학추론·장문에서 개선. GPQA-Diamond·MMLU-Pro 등에서 평균 출력 길이 **10–14% 단축**. 약 750B 규모로 "2~3T 파라미터 모델에 상당"이라는 발표상 위치이나, Qiita 비교글은 이를 실제 과제 합격률로 환산할 수 없다고 경고한다. HN **941pts/약 440 댓글**, 최다 추천 댓글은 "벤치 없이 어떻게 좋은지 알 수 있나".
- **시사점**: 출력 토큰을 줄여 추론비를 낮추는 방향이라 Coding Plan credit 소모에 직접 영향을 준다(Qiita는 GLM-5.4의 credit 계수가 더 낮다고 정리). 단 독립 벤치 부재, 그리고 Loop 구조가 기존 vLLM/SGLang 커널과 맞는지는 미검증이라 같은 커밋·이슈로 A/B 재는 것이 선결 과제다.
→ 원문: [GLM-5.4: Toward generalized coding with our first loop transformer](https://z.ai/blog/glm-5.4)
→ 교차확인: [GLM-5.4とGLM-5.3はどう選ぶ？ — Qiita](https://qiita.com/whitemarshall/items/664246f4c6d5afdcc0a2)
- 관련: [Z.AI 개발자 문서 — GLM-5.4](https://docs.z.ai/guides/llm/glm-5.4) · [HN 토론](https://news.ycombinator.com/item?id=50609685)

**6. Apple FastFrames: 480p 영상을 50ms 미만 지연으로 생성** — Hugging Face / arXiv 2611.23226
- **사실**: 자기회귀 2단계 영상 확산 모델 패밀리 **FastFrames-S(1.3B)·FastFrames-L(14B)**를 오픈웨이트로 공개했다. 기존 모델의 추론 파이프라인을 프로파일링한 결과 VAE 디코딩·KV 캐시 갱신처럼 "구현 세부"로 여겨지던 연산이 엔드투엔드 추론 **절반 이상**을 차지한다는 점에서 출발했다.
- **수치**: 480p **<50ms** 상호작용 지연, LingBot 1.3B 최적화판 **16× 가속**(카메라 조건 제어 유지). HF 페이퍼 33 upvotes, FastFrames-L 모델 568 likes.
- **시사점**: "대화형 세계 모델"이 로컬에서 실험 가능한 수준으로 내려왔다. Amazon이 실시간 세계모델(Oasis 3)로 이름 난 Decart에 75억 달러를 쓴 것(12번 항목)과 대비되는 오픈 쪽 대응이다.
→ 원문: [apple/FastFrames-L](https://huggingface.co/apple/FastFrames-L)
→ 교차확인: [FastFrames 프로젝트 페이지](https://apple-aiml-research.github.io/ml-fastframes/) · [arXiv 2611.23226](https://arxiv.org/abs/2611.23226)

**7. Tencent Hy4-flash: 220B 총·12B 활성, 100만 컨텍스트, N-gram 임베딩 64B** — Hugging Face
- **사실**: Hy4 대비 3가지 구조 변경 — Hybrid attention(sparse:sliding-window **1:3** 교차), LatentMoE(라우팅 전문가가 저차원 잠재공간에서 동작, 토큰당 512개 중 16개 + 공유 1개 활성), N-gram 임베딩 **Engram**(bigram/trigram을 다중헤드 해시 조회). 내장 5층 드래프터 DFly2(0.85B)가 대상 모델의 KV 캐시·sparse 인덱스를 재사용해 장문 스페큘레이티브 디코딩을 지원한다.
- **수치**: 총 **220B**(Engram 63.5B 포함), 텍스트 활성 **11.3B**, 컨텍스트 **1,048,576**, 어휘 120,832. HF 646 likes, Instruct·FP8·base 3종과 GGUF(AngelSlim) 동시 공개.
- **시사점**: "총파라미터 크게, 활성 작게"를 트랜스포머 밖의 구성요소(N-gram 조회)로 달성한 사례다. 활성 12B는 MacStudio·NAS급 로컬 배치에서 현실적이지만, 라이선스가 **Tencent Hy License**(완전 오픈 아님)라 상업 사용 조건을 먼저 확인해야 한다.
→ 원문: [tencent/Hy4-flash](https://huggingface.co/tencent/Hy4-flash)
→ 교차확인: [Tencent-Hunyuan/Hy4-flash (GitHub)](https://github.com/Tencent-Hunyuan/Hy4-flash)

---

## 🛠 개발자 생태계

**8. Dragos: AI 라우터 공급망에 카나리 200만 개를 심었다 — "체처럼 샜다"** — Dragos / Ars Technica
- **사실**: HD Moore(Dragos 수석연구책임)가 한 번도 쓰지 않은 AWS 키·비밀번호·고유 URL·사설 git 참조를 **프롬프트당 정확히 하나씩** 넣어 2026년 7~11월 약 200만 건을 AI 제공자·라우터·리셀러로 보내고 어디서 노출되는지 추적했다. 프롬프트는 각 홉에서 라우팅을 위해 평문으로 복호화되므로, 구조적으로 로그에 남는다.
- **수치**: **호스팅 제공자 3곳 침해**(한 곳을 장악한 공격자가 "수백 개" AWS 키를 재사용 — 모델 실행 전 거절된 요청의 키까지 유출), 리셀러 다수가 프롬프트를 로그·유출, ZDR·"no data collection" 플래그를 무시한 사례 다수. OpenRouter는 MiniMax 엔드포인트(AtlasCloud·Novita·StreamLake 경유 포함)에서 플랜트 URL 접근을 확인하고 **"Zero Data Retention" 라벨을 제거**, 고객에게 이메일을 보냈다.
- **시사점**: "제로 보존"은 계약 문구일 뿐 경로가 증명해야 한다. 같은 날 Qiita에서 Rust製 셀프호스트 게이트웨이 **Kaeso**(MIT, 50개 클라우드 + Ollama/vLLM/llama.cpp, 하드 예산 상한)가 주목받은 것은 이 구조에 대한 실무자 반응이다.
→ 원문: [Three breaches, endless leaks: Your AI supply chain needs canaries — Dragos](https://www.dragos.com/blog/ai-supply-chain-security-canary-tokens)
→ 교차확인: [Researcher plants 2m canaries into the AI supply chain — Ars Technica](https://arstechnica.com/security/2026/11/a-leaky-ai-supply-chain-is-exposing-secret-user-data-at-an-alarming-rate/)
- 관련: [Kaeso: Rust製オープンソース・セルフホスト型 AI ゲートウェイ — Qiita](https://qiita.com/dvnold/items/2e0f8a696b45be1c2f3e)

**9. GitHub 트렌딩: 코딩 CLI 오케스트레이션과 CI 신뢰성 에이전트** — GitHub Trending (Python)
- **사실**: awslabs **cli-agent-orchestrator(CAO)**는 로컬 `cao-server`가 Claude Code·Kiro·Codex 등 제공자 CLI를 **격리된 터미널 세션**으로 띄우고, 감독자(supervisor)에게 전문가 워커를 병렬·순차로 조율하는 도구를 준다. 에이전트는 네이티브 인증과 기능을 그대로 유지한다. Tracer-Cloud **opensre**는 "CI/CD 신뢰성을 위한 오픈소스 에이전트"다.
- **수치**: CAO **1,649★**, opensre **14,768★**(2026-01 생성, 11-23 푸시). 11-21 브리핑이 다룬 Claude-BugHunter(19,560★)·CLI-Anything(63,501★)는 오늘도 상위권이다.
- **시사점**: 성과가 모델이 아니라 '여러 CLI를 어떻게 띄우고 격리·조율하느냐'에서 갈리는 지점이 온다. tmux 격리 + 감독자 도구라는 구조는 Hermes·Codex·Claude Code를 병행하는 체계와 같은 문제의식이며, 공식 구현을 참조해 비교할 가치가 있다.
→ 원문: [awslabs/cli-agent-orchestrator](https://github.com/awslabs/cli-agent-orchestrator)
→ 교차확인: [Tracer-Cloud/opensre](https://github.com/Tracer-Cloud/opensre)

---

## 🏭 산업·정책·시장

**10. Nvidia, 젠슨 황 CEO·이사회 의장 복귀 — 11일 만의 극적 반전** — TechCrunch / CNBC / Ars Technica
- **사실**: 이사회가 11월 23일(월) **즉시 효력**으로 복귀를 결정했고, 그 근거에 **"미국 정부 고위층의 입력"**을 통해 "황의 복귀가 국가안보 위험을 초래하지 않는다"는 확인을 들었다고 밝혔다. 트럼프 대통령은 이사회와 직접 통화했으며 "황은 잘못한 것이 없다", 복귀는 "올바르고 적절한 결정"이라고 밝혔다. Ars에 따르면 황은 사임 직후부터 '워룸'을 꾸려 복귀를 추진해왔다.
- **수치**: 주가는 프리마켓 급등 뒤 **30% 높게 개장**(CNBC 기준 **+33%**, TechCrunch 9:51ET **$159.95 +27%**). HN **1,552pts/440 댓글**. 사임(11/13) 이후 여러 정부 부처의 수출통제·회계·거버넌스 조사는 계속 진행 중이며 **다음 주 상원 증언**이 예정됐다.
- **시사점**: 세계에서 가장 값이 높은 반도체 기업의 경영권 판단에 정부가 공식 개입했다 — 국가안보 논리가 주주·이사회 논리를 앞질렀고, HN 최다 추천 댓글은 "서커스가 돌아왔다", "주가가 강제 함수였다"였다. AI 공급망 리스크 프라이싱에 '정치 리스크'가 명시적 항목으로 들어갔고, 조사·증언이 남아 있어 변동성은 끝나지 않았다.
→ 원문: [Nvidia reverses course and brings Jensen Huang back as CEO — TechCrunch](https://techcrunch.com/2026/11/23/nvidia-reverses-course-and-brings-jensen-huang-back-as-ceo/)
→ 교차확인: [Jensen Huang reinstated as Nvidia CEO. Stock skyrockets 33% — CNBC](https://www.cnbc.com/2026/11/23/nvidia-jensen-huang-ceo.html) · [Ars Technica](https://arstechnica.com/ai/2026/11/jensen-huang-reinstated-as-nvidia-ceo-board-chair/) · [상원 증언 예정 — TechCrunch](https://techcrunch.com/2026/11/23/jensen-huang-scheduled-to-testify-next-week-in-senate-following-reinstatement-as-nvidia-ceo/)

**11. OpenAI 최고과학자, 'perplexity'를 AI 개발 속도의 공통 물잣대로 제안** — OpenAI (11/22)
- **사실**: Jakub Pachocki 수석과학자가 「Perplexity and Intelligence」에서 AI 발전의 핵심 기판인 "압축기(예측기) 학습"의 진척을 log-perplexity로 추적하자고 주장했다. 개발사 간 비교와 공통 안전기준의 물잣대로 삼기 위해 경쟁사에 자사 위치를 노출하는 대가를 감수하고 내부 측정치를 처음 공개했다. Sam Altman은 "한동안 OpenAI가 낸 것 중 가장 중요한 글"이라 평했다.
- **수치**: GPT-3.5 **~0.99**(유창한 대화, ChatGPT의 기반), GPT-4 **~0.91**(CoT 활용), **0.8 미만**에서 사이버보안·수학의 변형 능력 출현. **~0.76** 모델이 수학 오픈문제에서 "획기적 결과"를 냈고, **0.68–0.7 창**에서 추가 변형 능력이 나올 것으로 예고. 2027년부터 사전학습 알고리즘 연구에도 비슷한 "avalanche"가 가능하다고 전망. HN **686pts/439 댓글**, Qiita에는 당일 해설이 올라왔다.
- **시사점**: Dario Amodei의 "pace the frontier"가 속도 제한을 말했지만 측정 단위가 없었는데, 이 글은 그 단위를 제안한 것이다. 반론도 즉시 나왔다 — Yann LeCun은 "오픈소스 모델을 불법화하고 자기들이 통제하려는 위험 담론"이라 비판했고(HN 502pts), 전처리 코퍼스 구성이 비교 가능성을 깨뜨린다는 지적도 있다. 단일 지표가 정책 물잣대가 되는 것이 타당한지는 다음 주 논쟁의 화제다.
→ 원문: [Perplexity and Intelligence — OpenAI](https://openai.com/index/perplexity-and-intelligence)
→ 교차확인: [OpenAI's chief scientist has thoughts on how to limit AI development — TechCrunch](https://techcrunch.com/2026/11/22/openais-chief-scientist-has-thoughts-on-how-to-limit-ai-development/) · [Qiita 해설](https://qiita.com/quotidia/items/5ffc4c10f77a170b6a7d) · [HN 토론](https://news.ycombinator.com/item?id=50601525)
- 관련: [LeCun 비판 HN 스레드](https://news.ycombinator.com/item?id=50615308) · [Perplexity, intelligence and pretraining corpus mix — 비판적 분석](https://micjm.com/blog/perplexity-intelligence-pretraining-corpus-mix/)

**12. Amazon, Decart의 AI 최적화 스택을 75억 달러에 인수 — Trainium 외판 직전** — TechCrunch / Calcalist
- **사실**: Amazon이 이스라엘 스타트업 Decart의 **DOS(Decart Optimization Stack)** 기술과 창업자·AI 연구·엔지니어 팀을 인수하기로 했다. DOS는 Nvidia·Amazon·Google 하드웨어에서 모델을 효율적으로 돌리는 소프트웨어다. Calcalist에 따르면 Anthropic·Nvidia와의 이전 인수 협상이 결렬된 뒤 성사된 거래다.
- **수치**: **75억 달러**(Bloomberg 인용). Amazon은 지금까지 자체 데이터센터에서만 쓴 **Trainium**을 고객 판매 준비 중이며, Peter DeSantis SVP는 "모든 도약은 칩을 더 많이 뽑아내는 것에서부터 시작한다"고 밝혔다.
- **시사점**: 인수 대상이 모델도 데이터도 아닌 **'최적화 소프트웨어'와 그 팀**이다 — 하드웨어 교체 마찰을 줄이는 계층에 75억 달러 가격표가 붙었다. 젠슨 황 사태·수출통제와 맞물려 "Nvidia 외 대안"의 실행 가능성을 높이는 거래로, 같은 날 GLM-5.4(깊이 재활용)·QueryLM(1.4B≈3.3B)가 함께 가리키는 '효율'이 이번 주 시장의 통화다.
→ 원문: [Amazon snaps up Decart's AI optimization tech for $7.5B — TechCrunch](https://techcrunch.com/2026/11/23/amazon-snaps-up-decarts-ai-optimization-tech-for-7-5b-as-it-prepares-to-sell-trainium-chips/)
→ 교차확인: [Amazon signs $7.5 billion deal for Israeli AI startup Decart — Calcalist](https://www.calcalistech.com/ctechnews/article/g0mtcg4mi)

**13. Sierra, 6개월 만에 밸류에이션 2배 "300억 달러" — Fortune 50 절반이 고객** — TechCrunch / Sierra
- **사실**: Bret Taylor가 공동창업한 기업 고객경험(CX) 에이전트 기업 Sierra가 **17.5억 달러 Series F**를 마감했다. Greenoaks·Lightspeed가 공동 리드하고 기존 리드 전원(Benchmark·Sequoia·Greenoaks·GV·Tiger Global)이 추가 투자했다.
- **수치**: 밸류에이션 150억→**300억 달러**(6개월 만). **Fortune 50의 약 절반**을 고객으로 두며(BNY·HSBC·Wells Fargo·JPMorganChase·Pearson 등), 성과기반 과금 덕에 **run-rate 매출 4억 달러로 2배**(10월 말 발표). 같은 주 Sierra는 Meta와 함께 Genesys·Shopify·Stripe·Walmart 등과 **Personal Agent Protocol**(개인 에이전트와 기업의 상호작용 표준) 개발을 발표했다.
- **시사점**: 소비자 앱이 아니라 **규제 산업의 CX**에서 AI 채택이 가장 빠르다는 방증이다 — Sierra는 "가장 오래되고 규제가 많은 회사가 가장 빨리 배포한다"고 주장한다. 과제당·성과당 과금이 밸류에이션 논리의 핵심이며, Anthropic IPO 이후 '에이전트 적용 기업'의 자본 조달이 다시 열렸다.
→ 원문: [Sierra doubles valuation to $30B within six months — TechCrunch](https://techcrunch.com/2026/11/23/sierra-doubles-valuation-to-30b-within-six-months-as-demand-for-ai-in-cx-takes-off/)
→ 교차확인: [Series F — Sierra 공식 블로그](https://sierra.ai/blog/series-f-30b)

**14. Google, Personal Intelligence를 Drive·Wallet·Gemini 앱으로 확장** — Google / TechCrunch
- **사실**: 검색 AI Mode의 Personal Intelligence가 Gmail·Calendar·Photos에 이어 **Google Drive·Google Wallet·Gemini 앱**까지 연결된다. 사용자 디지털 생활의 정보를 끌어와 맥락에 맞는 답을 주는 기능으로, 연결은 옵트인 방식이다.
- **시사점**: 에이전트가 '문서 + 결제수단 + 일정'을 함께 보는 첫 대중 배치다 — 경쟁사가 복제할 수 없는 데이터 우위를 검색 진입점에 결합한다. 동시에 Wallet·Drive 접근의 동의 설계와 프라이버시가 다음 규제 쟁점이 될 것이며, 8번(Dragos)·9번과 맞닿아 '누가 어떤 데이터를 평문으로 보는가'가 제품 차별화이자 리스크가 됐다.
→ 원문: [A more tailored Google Search that's uniquely yours — Google](https://blog.google/products-and-platforms/products/search/google-search-personal-intelligence-apps/)
→ 교차확인: [Google Search's AI Mode can now tap into Drive and Wallet — TechCrunch](https://techcrunch.com/2026/11/23/google-searchs-ai-mode-can-now-tap-into-drive-and-wallet-for-more-personalized-responses/)

---

## 🧭 미스 김 인사이트

### 오늘의 핵심 트렌드 3가지
1. **AI 인프라의 통제권이 이사회 밖에서 결정되기 시작했다.** 젠슨 황 복귀의 공식 근거는 정부의 "국가안보 위험 없음" 확인이었고, 주가는 30%대로 답했다. 같은 날 Amazon은 Anthropic·Nvidia와의 협상이 결렬된 Decart의 '최적화 소프트웨어'에 75억 달러를 썼고, Sierra는 규제 산업 CX 에이전트로 300억 달러를 받았다. 세 사건의 공통분모는 **하드웨어·자본·정책이 한 패키지로 움직인다**는 것이다 — 반도체 의존 워크로드의 비용표에는 이제 '정치 리스크 프리미엄'을 별도 항목으로 넣어야 한다.
2. **'속도 제한' 논쟁이 물잣대 없기에서 물잣대 두기로 넘어갔다.** OpenAI는 perplexity라는 단일 지표(0.99→0.91→0.68–0.7 창)를, HF는 8단계·유지보수 대상 평가 소프트웨어를, LeCun은 "규제 해자" 비판을 내놨다. 어느 쪽이 이기든 결과는 같다 — **측정 방식을 정의하는 쪽이 개발 속도 규약의 문법을 쥔다**. 단일 지표는 비교는 쉽게 하지만 코퍼스 구성에 따라 쉽게 왜곡되므로, 우리 내부 지표(브리핑 통과율·과제당 비용)는 "어떻게 측정했는가"를 함께 버전 관리해야 한다.
3. **도구 호출 계층이 모델과 분리된 독립 설계 대상이 됐다.** 플래너에게 도구를 빼는 Intent Contracts(단, 무단 행위 감소 효과는 "없음"), 스키마를 필요할 때만 로드하는 Gemini Tool Retrieval, 450만 쌍 선호 데이터 TOUMAII, 그리고 AI 라우터가 프롬프트 평문을 본다는 Dragos의 증명까지 — **"에이전트가 무엇을 부를 수 있고, 그 경로에 누가 평문을 보는가"**가 성능과 별개의 계층으로 떠올랐다. 모델을 바꾸는 것보다 도구 권한·경로 설계가 더 싸고 빠른 개선점이다.

### Jay에게 추천
- **즉시 실행 (오늘–3일)**: ① **AI 라우터 평문 노출 점검** — Dragos 결과(호스팅 3곳 침해, ZDR 플래그 무시, 거절된 요청의 키까지 유출)를 근거로 Hermes·OpenClaw의 라우터/게이트웨이 경로에서 **시크릿이 프롬프트에 들어가는 지점**을 목록화하고, 민감 워크로드는 직결 호출 또는 셀프호스팅 게이트웨이(Kaeso류, 하드 예산 상한)로 옮길지 결정할 것. ② **GLM-5.4 A/B 1회** — Hermes의 zai provider 라인에서 같은 커밋·이슈로 GLM-5.3과 비교해 완주율·출력 토큰(공식 주장 −10~14%)·credit 소모를 실측하고, Loop 구조가 vLLM/SGLang 커널에서 돌아가는지 확인할 것.
- **주목 (1–2주)**: ① **Intent Contracts 패턴** — 대형 모델은 계약(목표·사용자 인용·범위)만 쓰고 신뢰 소형 모델이 호출을 작성하는 구조. 무단 행동 억제는 없으므로 **감사 로그·책임 분리**용으로만 도입하고 검증 코드 정확성이 전제임을 명시할 것. ② **Gemini Tool Retrieval** — 도구 수십 개 이상 붙이는 세팅에서 입력 토큰 절감 효과를 소형 파일럿으로 재고, 같은 문제의 awslabs CAO(격리 tmux 세션 + 감독자 도구)를 우리 병행 체계와 비교할 것. ③ **QueryLM·Search-Native** — 1.4B≈3.3B 결과는 로컬 지식 과제 설계에 영향이 크니 체크포인트 공개 여부를 추적할 것.
- **관망**: ① **perplexity 기반 'speed limit'** — 수치 합의·검증 방식이 정해지기 전까지는 참고 지표로만. ② **FastFrames** — 480p <50ms는 인상적이지만 아직 데모 수준, LingBot급 카메라 제어가 필요해질 때 재평가. ③ **Hy4-flash** — Tencent Hy 라이선스 조건을 읽기 전에는 상업 사용 보류.

### 다음 주 전망
- **황의 상원 증언(다음 주)**: 수출통제·회계·거버넌스 조사가 살아 있어 증언 내용에 따라 엔비디아·Trainium·중국 접근 리스크가 재점화된다. Amazon의 Trainium 외판 일정과 DOS 통합 로드맵이 첫 테스트다.
- **'속도계' 논쟁의 확산**: OpenAI의 perplexity 공개에 대한 Anthropic·Google·메타의 응답(자사 수치 공개 여부)이 관전 포인트. LeCun류 "규제 해자" 비판과 맞물려 오픈웨이트 규제 논쟁이 정책 테이블로 올라갈 가능성이 높다.
- **GLM-5.4 독립 벤치·커널 지원**: HN 최다 댓글이 지적한 "벤치 없음"이 채워지는지, vLLM/SGLang이 Loop 구조를 지원하는지가 Coding Plan 사용자의 전환 속도를 결정한다. 동시에 Search-Native·Tool Retrieval류 "사전학습·도구 계층 혁신"이 12월 릴리스에 얼마나 반영되는지 본다.

---
*소스: arXiv · Hugging Face(Daily Papers/Models/Blog) · GitHub Trending · Hacker News · Qiita · TechCrunch · CNBC · Ars Technica · Dragos · OpenAI · Google DeepMind · Z.ai · Sierra 공식 블로그 · Calcalist. Product Hunt·Papers with Code는 이번에 접근 불가/폐지로 제외하고 커뮤니티 축은 HN+Qiita+GitHub로 대체했다.*
