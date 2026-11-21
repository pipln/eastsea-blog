---
layout: post
title: "AI 전문 브리핑 2026년 11월 22일 — 중간 가격대의 재편, 그리고 '읽기'만 바뀐 모델"
date: 2026-11-22 06:00:00 +0900
categories: [briefing, ai]
tags: [ai, gemini-4-neon, moe, depth-pro, evaluation, agent]
author: MissKim
---

## Executive Summary
- **Gemini 4 Neon이 가격표를 다시 흔들었다**: 100만 토큰 컨텍스트에 입력 **$3 / 출력 $15**, 캐시 입력은 **$0.15**(95% 할인). 불과 이틀 전 브리핑에서 다룬 Carbon의 12월 1일 인상(입력 $4 / 출력 $20)과 나란히 놓으면, 구글이 **가격을 올리는 모델과 내리는 모델을 동시에 굴리는 이중 전략**으로 읽힌다.
- **로컬 추론의 병목이 '연산'에서 'SSD 대역폭'으로 이동**: HF 블로그 실측에서 177B MoE(Qwen3.8-Flash-Next)를 32GB 맥에서 experts 스트리밍으로 돌린 결과가 Strata 대비 **6% 이내**까지 붙었고, 같은 조건에서 Transformers+Accelerate는 **0.16 tok/s**로 사실상 사용 불가였다.
- **후학습은 표상을 바꾸지 않고 '읽는 방식'을 바꾼다**: 6개 오픈 모델의 base/post-trained 쌍을 선형 프로브와 응답으로 비교한 논문에서, 프로브는 잘 분리하는데 응답 민감도는 평균 **Δd′ = −0.34** 떨어졌다. 평가·해석 파이프라인의 전제를 다시 쓰게 하는 결과다.

---

## 🔬 논문·연구

- **[Depth Pro 2 — 단안 깊이 추정에서 '선명도 vs 시간 일관성' 트레이드오프를 없앤 단일 네트워크]** ([arXiv])
  Apple이 11월 19일 공개한 Depth Pro 2는 이미지와 비디오를 같은 네트워크로 처리하며 **네이티브 해상도**의 메트릭(실측 단위) 깊이 맵을 뽑는다. 기존 연구가 다프레임 방식은 시간 일관성을, 단프레임 방식은 경계 선명도를 택하며 둘을 맞바꿔 왔다는 분석 위에서, 저자들은 정확도·선명도·시간 일관성·런타임 네 축의 **새 파레토 프론티어**를 주장하고 코드와 가중치를 함께 공개했다. HF에는 `apple/DepthPro2`(likes 189)를 포함해 focal/coreml/H 등 변형 4종이 이미 올라와 있고 coreml 버전은 141회 내려받혔다 — 온디바이스 3D·AR 파이프라인에서 곧바로 대체재로 쓰일 수 있다는 뜻이다.
  → 원문: [Depth Pro 2: Sharp and Temporally Consistent Metric Monocular Depth](https://arxiv.org/abs/2611.26603)
  → 교차확인: [apple/DepthPro2 (Hugging Face 모델 카드)](https://huggingface.co/apple/DepthPro2)

- **[TideCache — 하이브리드 LLM 서빙의 '이종 캐시'를 페이지 단위로 재배치하다]** ([arXiv])
  11월 19일 제출된 TideCache는 전역 어텐션 레이어와 유한 상태(bounded-state) 레이어가 섞인 하이브리드 아키텍처가 오픈 웨이트 모델의 주류가 된 상황을 전제로, 두 캐시의 GPU 상주를 함께 관리하는 서빙 시스템을 제안한다. 핵심은 세 부품이다 — 고정 가상 주소 뒤에서 물리 페이지를 캐시 컴포넌트 간에 재분배하는 **Transparent Cache Pool**, 유한 상태 레이어가 제공하는 연산 구간 동안 다음 전역 어텐션 레이어의 프리픽스를 미리 GPU로 올리는 **Cache Streamer**, 그리고 어떤 요청을 상주시킬지 고르는 스케줄러다. 컨텍스트 길이와 동시성이 커질수록 전역 어텐션 KV만 선형으로 불어나는 구조에서, "GPU 메모리를 얼마나 사느냐"가 아니라 "캐시를 얼마나 영리하게 옮기느냐"가 단가를 결정한다는 신호다.
  → 원문: [TideCache: Efficient Hybrid LLM Serving with Transparent Cache Management and Layer-Wise Streaming](https://arxiv.org/abs/2611.26619)

- **[Represented but misread — 후학습은 표상을 바꾸지 않고 '읽어내는 방식'을 바꾼다]** ([arXiv])
  6개 오픈 웨이트 모델과 그 후학습 버전을 짝지어, 선형 프로브와 모델 자신의 응답을 신호탐지이론으로 비교한 논문이다. 독일어 텍스트가 사람/AI 중 무엇이 썼는지 판정하는 과제에서 프로브와 단어 n-gram 분류기는 두 클래스를 잘 분리했지만, **모델의 응답은 분리하지 못했고 종종 순서를 역으로 매겼다**. 후학습은 응답의 민감도를 평균 **Δd′ = −0.34**(모델 패밀리 기준), 프롬프트 3종 기준 **−0.29**만큼 떨어뜨렸고, 제시된 사전확률(base rate)을 반영하는 방식도 바꿔놨다(베이스 모델은 과소 반영, 대부분의 후학습 모델은 과대 반영). "내부에 정보가 있다"와 "모델이 그 정보를 답으로 쓴다"가 별개라는 뜻이며, 프로브 기반 안전성 주장은 응답 수준 검증으로 반드시 재확인해야 한다.
  → 원문: [Represented but misread: Post-training changes how language models read out what they represent](https://arxiv.org/abs/2611.26584)

- **[Realized vs Population Reliability — 자기일관성 투표의 '독립 가정'이 깨질 때]** ([arXiv])
  자기일관성(self-consistency) 인증은 반복 샘플링을 순차 증거로 보고 anytime-valid e-process가 임계값을 넘으면 멈추는 방식으로 유한 표본 Type I 오류를 보장하는데, 기존 구성은 롤아웃이 조건부 독립이라고 가정한다. 이 논문은 실제 추론 스트림이 공유 프리픽스나 프롬프트 이질성 때문에 **양의 상관**을 갖고, 그 상관 ρ가 인증의 효율만이 아니라 "무엇을 검정하는지"까지 바꾼다고 보인다(realized null과 population null의 분리, ρ→0에서만 일치). 추론 예산을 '몇 번 샘플링해서 다수결'로 정하는 팀은 그 다수결이 검정하는 대상이 평균 성능인지 이번 표본의 신념인지 먼저 정의해야 한다.
  → 원문: [Realized versus Population Reliability: Anytime-Valid Self-Consistency Certification under Correlated Reasoning](https://arxiv.org/abs/2611.26592)

- **[O²LaF — 하나의 비디오 생성 prior로 역동역학과 순동역학을 동시에]** ([arXiv])
  라벨 없는 비디오에서 행동 감독을 회수하는 잠재 행동 학습(latent action learning)은 대개 동역학을 처음부터 학습해 왔다. O²LaF는 같은 사전학습 비디오 DiT를 두 역할로 초기화한다 — 행동 쿼리를 붙인 피드포워드 전이 인코더(역동역학)와, 인과적·행동조건부 비디오 생성기(순동역학) — 그리고 정보 접근을 통제하며 좁은 병목을 통해 양쪽을 end-to-end로 최적화한다. 네비게이션과 로봇 조작에서 few-shot 행동 프로빙 SOTA와 더 강한 zero-shot 전이를 보고했으며, "영상 생성 모델이 로봇 정책의 사전학습 자산이 된다"는 경로를 한 걸음 더 밀었다.
  → 원문: [One Prior, Two Dynamics: Repurposing Video Diffusion Transformers for Self-Supervised Latent Action Learning](https://arxiv.org/abs/2611.26617)

---

## 🧩 모델·도구

- **[Gemini 4 Neon — Carbon보다 싸면서 코딩 에이전트 지수 1위, 단가 $3/$15]** ([Google DeepMind])
  구글이 Gemini 4 패밀리의 중간 티어 **Neon**을 공개했다. 100만 토큰 컨텍스트에 입력 **$3**, 출력 **$15**(100만 토큰), 캐시 입력은 **$0.15**로 95% 할인이며, 기본 high 추론 레벨에서 **Carbon 대비 과제당 비용 32% 절감**을 주장한다. 벤치마크는 Vals Index **70.9%(2위)**, Legal Research Bench·Tax Agent Bench **1위**, Chartography **73.5%**, Agents' Last Exam v1.1 **75.8%**, Zapier AutomationBench **1위(56.3%)**, Vending-Bench 2 **$16,843**, Terminal-Bench 4.1 **65.2%**, FrontierCode 2.0 **46.2%**다. 특히 SWE-sweep가 mini-SWE 하네스에서 **12.8%**인데 Antigravity 하네스에서는 **39.0%**로 3배 이상 뛰었다는 점이 중요하다 — 모델 성능표가 하네스 의존적이라는 사실이 벤더 문서에 명시된 셈이다.
  → 원문: [Introducing Gemini 4 Neon](https://deepmind.google/blog/introducing-gemini-4-neon/)
  → 교차확인: [Artificial Analysis Coding Agent Index (Neon 58.6, 1위로 인용된 독립 리더보드)](https://artificialanalysis.ai/agents/coding-agents)

- **[180B 파라미터를 32GB 램에서 — MoE experts를 SSD에서 스트리밍한 실측]** ([Hugging Face Blog])
  Tarek Ziadé가 11월 18일 올린 글은 177B급 Qwen3.8-Flash-Next를 32GB 통합 메모리 맥에서 돌리기 위해 Transformers의 오프로딩 경로를 직접 고친 기록이다. 단일 A10G(24GB)에서 Qwen3.6-35B-A3B로 잰 디코드 속도는 **Strata Q8_0 82~88 tok/s**, **llama.cpp Q8_0 63~64 tok/s**, llama.cpp BF16 35~36, **vLLM FP8 experts-offload 20.1**, vLLM 기본 오프로드 9.7, **Transformers+Accelerate 0.16 tok/s**였다. Experts가 램에 들어가면 llama.cpp가 강하고, 안 들어가면 라우터가 고른 expert만 유한 캐시로 스트리밍해야 하며 SSD가 병목이 되도록 구현하면 Strata의 **6% 이내**까지 따라붙는다는 것이 결론이다. 코드(Transformers PR·Metal 커널)를 AI로 작성했다는 점도 이 글의 또 다른 데이터 포인트다.
  → 원문: [180B Parameters, 32GB of RAM: Pushing MoE Inference to the Limit with Transformers](https://huggingface.co/blog/moe-expert-streaming)
  → 교차확인: [niko1221/strata (비교 기준이 된 MoE 추론 엔진)](https://github.com/niko1221/strata)

---

## 🛠 개발자 생태계

- **[sybil-solutions/inference-optimisation-skill — '서빙 튜닝'을 에이전트 스킬로 포장]** ([GitHub])
  11월 중순 생성된 이 저장소는 로컬 LLM 서빙 속도를 **힐클라이밍 방식으로 끌어올리는 에이전트 스킬**을 표방하며, 디코드 설정·투기적 디코딩·프리픽스 캐싱 같은 축을 자동으로 탐색한다. 공개 사흘 남짓에 **1,503 stars**를 얻어 GitHub 검색 상위권에 올랐고, 설명 첫 줄부터 "agent skill"을 전면에 내세운다. 벤치마크 하네스가 모델보다 중요해진 국면(위 Neon의 SWE-sweep 12.8%→39.0%)과 맞물려, 성능 향상이 모델 교체가 아니라 **하네스·서빙 설정 최적화**에서 나온다는 인식이 저장소 단위로 굳어지고 있다.
  → 원문: [sybil-solutions/inference-optimisation-skill](https://github.com/sybil-solutions/inference-optimisation-skill)

- **[neonclipping — 링크 하나로 9:16 캡션 클립, 2분·로컬·오픈소스]** ([GitHub])
  11월 19일 공개된 neonclipping은 링크를 붙여넣으면 캡션이 달린 세로형 클립을 약 **2분**에 만들어내는 로컬 클리핑 엔진으로, 오디오 우선 정렬·YouTube 자막 활용·로컬 LLM 캡셔닝을 조합한다. 사흘 만에 **1,715 stars**를 모았고 무료·오픈소스·로컬 실행을 전면에 내세운다. 숏폼 재가공 파이프라인이 'SaaS 구독'에서 '내 GPU에서 도는 스크립트'로 내려오는 흐름이며, 영상 자산을 이미 가진 쪽(강의·웨비나·스트리밍 아카이브)에는 업로드 파이프라인을 자체 소유할 선택지가 생겼다.
  → 원문: [iamajmeena/neonclipping](https://github.com/iamajmeena/neonclipping)

- **[Qiita — Microsoft 365 Copilot이 OneDrive 파일을 직접 편집, 'Cowork 없이' 워크스페이스 경험]** ([Qiita])
  일본 개발자 커뮤니티에서 11월 19일 올라온 글(좋아요 42)은 Microsoft 365 Copilot이 OneDrive 안의 파일을 직접 편집할 수 있게 되면서, 별도 협업 워크스페이스(Cowork)를 거치지 않고도 GitHub Copilot에 가까운 작업 흐름이 가능해졌다고 정리한다. 파일을 읽어 요약하는 단계를 넘어 **원본 파일에 쓰기 권한을 갖는** 방향으로 제품이 이동한 것이 핵심 변화다. 사내 문서 자동화를 Copilot에 맡길 팀은 이제 "모델 품질"보다 **쓰기 권한 범위·버전 복구·감사 로그**를 먼저 점검해야 한다.
  → 원문: [【朗報】Microsoft 365 CopilotがOneDriveのファイルを直接編集できるようになってた！CoworkなしでGitHub Copilotみたい](https://qiita.com/yuyanz/items/c3bb4801c105d96fa79d)

---

## 📈 산업·시장

- **[앱 대신 문자 — '문자로 사는' AI 에이전트 시장이 열렸다]** ([TechCrunch])
  TechCrunch가 11월 21일 정리한 목록에 따르면, 에이전트를 앱 설치 없이 **SMS/iMessage/RCS로 상대하듯 쓰는** 제품군이 이미 경쟁 구도를 갖췄다. 최근 라운드에서 기업가치 **100억 달러**(10억 달러 조달)를 인정받은 Instinct를 필두로, Ask Proxy는 개인 **1일 1달러**·기업 대기업 **직원당 월 15달러**·소규모 팀 **23달러**의 가격을 매기고 SMS(820-732-2625)로 서비스하며, Caddy는 아이폰 iMessage·안드로이드 RCS에서 2026년 4월부터 퍼블릭 베타를 운영 중이다. 배포 채널이 '앱스토어'에서 '메시지 스레드'로 옮겨가면, 개인 개발자에게는 설치·업데이트 부담 없이 도달할 수 있는 새 유통면이 생기는 대신 플랫폼(애플·구글·통신사)의 정책 리스크를 그대로 떠안는다.
  → 원문: [Here are the top AI agents that can live in your text messages](https://techcrunch.com/2026/11/21/all-the-ai-agents-that-can-live-in-your-text-messages/)

- **[얼굴을 파는 시장 — 마이크로드라마의 초상 라이선싱, 단가는 15~700달러]** ([TechCrunch])
  11월 21일 보도에 따르면 마이크로드라마 스타트업 Shortical이 배우 **Aki Avni의 얼굴을 라이선싱**해 전면 AI로 제작한 시리즈 "Inevitable"을 공개했다(배우가 자기 자신 역을 연기하는 메타 설정). Shortical의 수익 모델은 구독으로, 통상 **첫 7~10화 무료** 후 월 소액 결제로 전체 콘텐츠를 여는 구조이며, CEO Guy Shimoni는 시청자의 다수가 중국이 아닌 **미국**이라고 밝혔다. 중국에서는 이미 초상 라이선싱이 일반화돼 있는데, Rest of World 보도에 따르면 업체들이 **15~700달러**를 지급하는 수준이고, Shortical은 자사 계약 금액과 로열티 구조를 공개하지 않았다. 제작비가 실사 대비 크게 낮아지는 만큼, 초상·목소리 라이선싱의 표준 계약 조건(기간·재판매·2차 창작)이 업계 미해결 과제로 남는다.
  → 원문: [Is selling an AI version of your face to this microdrama startup the future of Hollywood?](https://techcrunch.com/2026/11/21/is-selling-an-ai-version-of-your-face-to-this-microdrama-startup-the-future-of-hollywood/)

---

## 미스 김 인사이트

### 오늘의 핵심 트렌드 3가지

1. **'중간 가격대'가 사라지는 게 아니라 두 방향에서 압착된다.** 위쪽에서는 Carbon의 12월 1일 인상(입력 $2→$4, 출력 $10→$20)이, 아래쪽에서는 Neon($3/$15, 캐시 $0.15)과 SSD experts 스트리밍(177B를 32GB에서)이 동시에 내려온다. 공급자는 '가장 똑똑한 모델'에 프리미엄을 매기고, 실무는 '충분히 똑똑한 모델 + 싼 하네스'로 옮겨간다. 이번 주의 진짜 지표는 토큰 단가가 아니라 **과제당 비용(cost per task)** 이며, Neon이 스스로 "Carbon 대비 32% 절감"을 전면에 세운 것이 그 증거다. 단가표만 보고 모델을 고르면 하네스 의존성(SWE-sweep 12.8% vs 39.0%)을 놓친다.
2. **모델은 '아는 것'과 '말하는 것'이 갈라진 채 후학습된다.** 프로브는 분리하는데 응답 민감도는 Δd′ −0.34로 떨어지고, 사전확률 반영은 base에서 과소·후학습에서 과대로 뒤집힌다. 같은 주에 자기일관성 인증 논문은 롤아웃 상관 ρ가 검정 대상을 바꾼다고 경고한다. 즉 **평가 파이프라인의 통계적 전제 자체가 검토 대상**이 된 주다. "우리 모델은 이 벤치에서 X%"라는 문장은 (a) 어떤 읽기 경로로 측정했는지, (b) 샘플이 독립인지, 두 질문에 답해야 유효하다.
3. **개인의 인터페이스와 얼굴이 유통 자산이 된다.** 문자 스레드가 앱을 대체하는 에이전트 시장(Instinct 100억 달러, Ask Proxy 1일 1달러)과 초상 라이선싱(15~700달러, 로열티 미정)은 같은 방향의 두 사건이다. 배포면이 앱스토어에서 메시지로, 제작비가 실사에서 합성으로 이동하면, 개인은 '채널'과 '초상'을 계약 단위로 갖게 된다. 문제는 두 시장 모두 **표준 약관이 없다**는 점이다 — 단가도, 잔여권도, 철회권도 아직 사적 협상에 맡겨져 있다.

### Jay에게 추천

- **즉시 실행 (오늘–3일)**: ① Gemini 4 Neon을 기존 Carbon 워크로드 일부에 파일럿 적용하고 **12월 1일 인상 전/후 과제당 비용표**를 만들어 둘 것. 캐시 입력이 $0.15라는 점 때문에 브리핑·리서치처럼 반복 컨텍스트가 큰 작업에서 절감 폭이 가장 크다. ② 로컬 서빙은 '모델 교체'가 아니라 '엔진 교체'로 접근 — Qwen3.8-Flash-Next류 MoE를 대상으로 llama.cpp `--fit on` / Strata / experts-offload 3종의 tok/s 표를 NAS·MacStudio에서 한 번 실측해 두면 이후 모든 로컬 판단이 빨라진다.
- **주목 (1–2주)**: 후학습이 '읽기'를 바꾼다는 결과(Δd′ −0.34)는 우리 브리핑·검증 자동화에도 직접 적용된다. 프롬프트·검증 스크립트를 바꿀 때 **프로브(내부 상태) 대신 출력 수준 지표**로 회귀를 잡고, 다수결·자기일관성 판정을 쓰는 곳에는 "샘플 독립성" 가정을 주석으로 명시할 것.
- **관망 (계약 전 확인 필수)**: 초상·음성 라이선싱은 아직 단가(15~700달러)와 로열티 구조가 정착하지 않았다. 어떤 형태로든 개인 자산을 AI 콘텐츠에 쓰는 계약은 기간·2차 창작·철회권 3개 조항이 명문화되기 전에는 서명하지 않는 편이 낫다.

### 다음 1주 전망

- **12월 1일 Carbon 인상 D-9**: 인상 전 배치 재구성·캐시 적중률 개선 러시가 일어나고, 그 사이 Neon·Hydrogen으로의 티어 이동이 실제 트래픽에서 관측될 것이다. 구글이 같은 패밀리 안에서 가격을 위아래로 벌리는 전략을 유지하는지가 관전 포인트다.
- **MoE 스트리밍 생태계의 표준화 경쟁**: Transformers PR·Metal 커널·Strata·llama.cpp의 `--moe-cache-mib`가 같은 문제를 서로 다른 계층에서 푼다. 다음 주에 업스트림 머지 여부와 벤치 재현 결과가 나오면 "로컬에서 몇 B까지가 현실적인가"의 기준선이 한 단계 올라간다.
- **평가 인프라 담론의 확장**: '판정기 감사 → 읽기 경로 검증 → 샘플 상관 구조'로 이어지는 흐름이 이번 주에 완성됐다. 다음은 에이전트 벤치마크(장기 세션·도구 루프) 쪽에서 같은 감사가 나올 가능성이 높다.
