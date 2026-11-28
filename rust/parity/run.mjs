#!/usr/bin/env node
// toolcore parity harness (ESM) — legacy calculator.js (CJS require) vs
// toolcore wasm (wasm-pack --target nodejs build).
//
// Per RUST-WASM-MIGRATION.md's differential-verification gate:
//   fixtures = legacy calculator.test.js cases + boundary values + 200
//   seeded-random inputs per tool entry; floats compared with relative
//   error <= 1e-9; strings/null/bools/arrays compared exactly; any single
//   diff exits 1.
//
// Usage: node rust/parity/run.mjs   (run from anywhere; paths are absolute)

import { createRequire } from 'node:module';
import path from 'node:path';
import fs from 'node:fs';

const REPO_ROOT = path.resolve(import.meta.dirname, '..', '..');
const require = createRequire(import.meta.url);

// ── wasm side (nodejs target pkg, auto-instantiated on require) ────────────
const NODE_PKG = path.join(REPO_ROOT, 'rust', 'toolcore', 'pkg-node', 'toolcore.js');
if (!fs.existsSync(NODE_PKG)) {
  console.error(
    `[parity] missing ${NODE_PKG}\n` +
      '[parity] build it first:  wasm-pack build rust/toolcore --release --target nodejs --out-dir pkg-node'
  );
  process.exit(2);
}
const wasm = require(NODE_PKG);
const invoke = wasm.invoke;
if (typeof invoke !== 'function') {
  console.error('[parity] pkg-node exports no invoke() — glue shape changed?');
  process.exit(2);
}

// ── legacy side ────────────────────────────────────────────────────────────
const stripeLegacy = require(
  path.join(REPO_ROOT, 'tools', 'stripe-fee-profit-calculator', 'calculator.js')
);
const bondLegacy = require(
  path.join(REPO_ROOT, 'tools', 'bond-yield-calculator', 'calculator.js')
);
const upworkLegacy = require(
  path.join(REPO_ROOT, 'tools', 'upwork-fee-calculator', 'calculator.js')
);

// ── deterministic PRNG (mulberry32, fixed seed) ────────────────────────────
function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rand = (rng, lo, hi) => lo + rng() * (hi - lo);
const pick = (rng, arr) => arr[Math.floor(rng() * arr.length) % arr.length];
const round2 = (v) => Math.round(v * 100) / 100;

// ── fixture builders ───────────────────────────────────────────────────────
// Full inputs from calculator.test.js (verbatim), then boundary/partial-input
// cases, then 200 seeded-random per entry.

function stripeFixtures() {
  const full = (salePrice, shippingCharged, taxCollected, productCost, fulfillmentCost, opsCost, stripeFeeRate, stripeFlatFee, internationalCardRate, currencyConversionRate) => ({
    salePrice, shippingCharged, taxCollected, productCost, fulfillmentCost,
    opsCost, stripeFeeRate, stripeFlatFee, internationalCardRate, currencyConversionRate,
  });
  const list = [
    // TC-F001 (defaults via {})
    {},
    // TC-F002 low-ticket
    full(2.0, 0, 0, 0, 0, 0, 2.9, 0.3, 0, 0),
    // TC-F003 base + intl
    full(100, 0, 0, 25, 5, 0, 2.9, 0.3, 0, 0),
    full(100, 0, 0, 25, 5, 0, 2.9, 0.3, 1.5, 1.0),
    // TC-F004 tax collected
    full(100, 0, 10, 25, 5, 0, 2.9, 0.3, 0, 0),
    // TC-F005 invalid set
    full(-1, 0, 0, 0, 0, 0, 2.9, 0.3, 0, 0),
    full(10, -1, 0, 0, 0, 0, 2.9, 0.3, 0, 0),
    full(10, 0, 0, -1, 0, 0, 2.9, 0.3, 0, 0),
    full(10, 0, 0, 0, 0, 0, 101, 0.3, 0, 0),
    full(10, 0, 0, 0, 0, 0, 2.9, 0.3, -1, 0),
    full(0, 0, 0, 0, 0, 0, 2.9, 0.3, 0, 0),
    // TC-F006 unreachable break-even
    full(100, 0, 0, 999999, 0, 0, 99, 0.3, 0, 0),
  ];
  // Boundary values
  list.push(
    full(0, 5, 0, 0, 0, 0, 0, 0, 0, 0), // zero price + shipping revenue
    full(100, 0, 0, 0, 0, 0, 0, 0, 0, 0), // zero-rate, zero-cost: profitable at 0
    full(100, 0, 0, 0, 0, 0, 100, 0.3, 100, 100), // rate boundary 100
    full(100, 0, 0, 0, 0, 0, 100.0001, 0.3, 0, 0), // rate just over
    full(100, 0, 0, 0, 0, 0, 2.9, 0, 0, 0), // no flat fee
    full(999999.99, 0, 0, 25, 5, 0, 2.9, 0.3, 0, 0), // large ticket
    full(1000000, 200, 999, 99999, 999, 99, 3.5, 1, 2.5, 2), // everything large
    full(0.01, 0, 0, 0, 0, 0, 2.9, 0.3, 0, 0), // cent ticket
    full(123.456, 7.891, 1.234, 12.345, 6.789, 1.234, 3.35, 0.35, 1.15, 0.85), // raw precision
    // partial inputs: each single key present over defaults
    { salePrice: 200 },
    { shippingCharged: 10 },
    { taxCollected: 10 },
    { productCost: 0 },
    { fulfillmentCost: 0 },
    { opsCost: 100 },
    { stripeFeeRate: 3.15 },
    { stripeFlatFee: 0.5 },
    { internationalCardRate: 1.5 },
    { currencyConversionRate: 1 },
    // null == missing (?? semantics)
    { salePrice: null },
    { stripeFeeRate: null },
    // string coercion (JS Number())
    { salePrice: '50.5' },
    { salePrice: 'abc' }, // NaN quirk — must flow identically
    { stripeFeeRate: '' }, // '' -> 0
    null // null input -> {} -> defaults
  );
  return list;
}

function bondFixtures() {
  const BASE = { faceValue: 1000, couponRate: 5, currentPrice: 950, yearsToMaturity: 10, purchasePrice: 950, taxRate: 15, quantity: 10 };
  const list = [
    { ...BASE }, // TC-01..TC-06, TC-11
    { faceValue: 1000, couponRate: 7, currentPrice: 1100, yearsToMaturity: 5, purchasePrice: 1100, taxRate: 20, quantity: 5 }, // TC-07 premium
    { faceValue: 1000, couponRate: 4, currentPrice: 1000, yearsToMaturity: 20, purchasePrice: 1000, taxRate: 0, quantity: 1 }, // TC-08 par
    { faceValue: 1000, couponRate: 0, currentPrice: 600, yearsToMaturity: 10, purchasePrice: 600, taxRate: 15, quantity: 1 }, // TC-09 zero coupon
    { faceValue: 5000, couponRate: 8, currentPrice: 4800, yearsToMaturity: 15, purchasePrice: 4800, taxRate: 25, quantity: 1 }, // TC-10
    // TC-12 invalid (throw) cases
    { ...BASE, faceValue: -1 },
    { ...BASE, yearsToMaturity: 0 },
    { ...BASE, taxRate: 60 },
    { ...BASE, quantity: 0 },
  ];
  // Boundary values
  list.push(
    { ...BASE, yearsToMaturity: 1 },
    { ...BASE, yearsToMaturity: 50 },
    { ...BASE, yearsToMaturity: 1.5 },
    { ...BASE, yearsToMaturity: 50.5 },
    { ...BASE, quantity: 1 },
    { ...BASE, quantity: 10000 },
    { ...BASE, quantity: 10000.5 },
    { ...BASE, couponRate: 0 },
    { ...BASE, couponRate: 30 },
    { ...BASE, couponRate: 30.0001 },
    { ...BASE, couponRate: -0.01 },
    { ...BASE, taxRate: 0 },
    { ...BASE, taxRate: 50 },
    { ...BASE, taxRate: 50.0001 },
    { ...BASE, faceValue: 0.01 },
    { ...BASE, currentPrice: 0.01 },
    { ...BASE, purchasePrice: 0.01 },
    { ...BASE, currentPrice: -1 },
    { ...BASE, currentPrice: 1000 }, // par boundary
    { ...BASE, currentPrice: 999.999 },
    { ...BASE, currentPrice: 1000.001 },
    { faceValue: 100000, couponRate: 12.3456, currentPrice: 98765.4321, yearsToMaturity: 30, purchasePrice: 95000, taxRate: 25.5, quantity: 10000 }, // large raw-precision
    { faceValue: 999.999, couponRate: 5.555, currentPrice: 1000.001, yearsToMaturity: 7, purchasePrice: 1001.111, taxRate: 22.22, quantity: 33 },
    // missing keys (validation order must match legacy throws)
    {},
    { faceValue: 1000 },
    { faceValue: 1000, couponRate: 5 },
    { faceValue: 1000, couponRate: 5, currentPrice: 950 },
    { faceValue: 1000, couponRate: 5, currentPrice: 950, yearsToMaturity: 10 },
    { faceValue: 1000, couponRate: 5, currentPrice: 950, yearsToMaturity: 10, purchasePrice: 950 },
    { faceValue: 1000, couponRate: 5, currentPrice: 950, yearsToMaturity: 10, purchasePrice: 950, taxRate: 15 },
    // non-number types
    { ...BASE, faceValue: '1000' },
    { ...BASE, faceValue: null },
    { ...BASE, taxRate: true },
    // NOTE: a JSON `null` input is deliberately NOT fixtured here — the
    // legacy bond calculator destructures the argument and *crashes* with
    // TypeError (not a graceful Error), which is outside the tool contract.
    // The wasm ABI normalizes null -> {} -> validation error; documented in
    // rust/README.md.
  );
  return list;
}

const PRIOR_POOL = [0, 0.01, 499.99, 500, 500.01, 999.99, 1000, 9999.99, 10000, 10000.01, 15000, 999999];

function upworkForwardFixtures() {
  const list = [
    // calculator.test.js Section A / B / D
    { gross: 300, prior: 0, method: 'ach' }, // A1
    { gross: 1200, prior: 0, method: 'ach' }, // A2
    { gross: 2000, prior: 1000, method: 'ach' }, // A3
    { gross: 1000, prior: 9600, method: 'ach' }, // A4
    { gross: 5000, prior: 15000, method: 'ach' }, // A5
    { gross: 1000, prior: 0, method: 'ach' }, // B1
    { gross: 1000, prior: 0, method: 'paypal' }, // B2
    { gross: 1000, prior: 0, method: 'wire' }, // B3
    { gross: 1000, prior: 0, method: 'instant' }, // B4
    { gross: 500, prior: 0, method: 'ach' }, // D1 boundary
    { gross: 100, prior: 500, method: 'ach' }, // D2
    { gross: 0, prior: 0, method: 'ach' }, // D3 zero
    { gross: 50, prior: 0, method: 'paypal' }, // D4 min fee
    // invalid
    { gross: -1, prior: 0, method: 'ach' },
    { gross: 100, prior: -1, method: 'ach' },
    { gross: 100, prior: 0, method: 'bitcoin' },
    { gross: 999999.5, prior: 0, method: 'ach' },
  ];
  // Boundary values
  list.push(
    { gross: 500.01, prior: 0, method: 'ach' },
    { gross: 499.99, prior: 0, method: 'ach' },
    { gross: 10000, prior: 0, method: 'ach' },
    { gross: 10000.01, prior: 0, method: 'ach' },
    { gross: 999999, prior: 0, method: 'ach' },
    { gross: 999999, prior: 999999, method: 'ach' },
    { gross: 0.01, prior: 499.99, method: 'ach' },
    { gross: 0, prior: 15000, method: 'ach' },
    { gross: 0, prior: 999999, method: 'instant' },
    { gross: 999.999, prior: 0, method: 'paypal' },
    { gross: 999.999, prior: 0, method: 'instant' },
    { gross: 10000, prior: 500, method: 'wire' },
    { gross: 20000, prior: 300, method: 'paypal' },
    // method defaulting (|| 'ach')
    { gross: 100, prior: 0 },
    { gross: 100, prior: 0, method: '' },
    { gross: 100, prior: 0, method: null },
    // NOTE: JSON `null` input omitted — legacy calculateForward(null) throws
    // a raw TypeError (property access on null); see rust/README.md.
  );
  return list;
}

function upworkReverseFixtures() {
  const list = [
    // calculator.test.js Section C
    { netTarget: 240, prior: 0, method: 'ach' }, // C1
    { netTarget: 1030, prior: 0, method: 'ach' }, // C2
    { netTarget: 800, prior: 2000, method: 'ach' }, // C3
    { netTarget: 1000, prior: 10000, method: 'wire' }, // C4
    // invalid
    { netTarget: 0, prior: 0, method: 'ach' },
    { netTarget: 0.99, prior: 0, method: 'ach' },
    { netTarget: 999999.5, prior: 0, method: 'ach' },
    { netTarget: 100, prior: -1, method: 'ach' },
    { netTarget: 100, prior: 0, method: 'bitcoin' },
  ];
  // Boundary values
  list.push(
    { netTarget: 1, prior: 0, method: 'ach' },
    { netTarget: 999999, prior: 0, method: 'ach' },
    { netTarget: 1, prior: 999999, method: 'wire' },
    { netTarget: 100, prior: 500, method: 'instant' },
    { netTarget: 50, prior: 0, method: 'paypal' }, // paypal min fee region
    { netTarget: 300, prior: 0, method: 'paypal' },
    { netTarget: 12345.67, prior: 5432.1, method: 'wire' },
    { netTarget: 42.42, prior: 0, method: '' }, // method default
    { netTarget: 500, prior: 0 } // method missing
  );
  return list;
}

// ── seeded random generators (200 per entry) ───────────────────────────────
function randomStripeInputs(rng) {
  const out = [];
  for (let i = 0; i < 200; i++) {
    const input = {
      salePrice: rand(rng, 0, 5000),
      shippingCharged: rand(rng, 0, 300),
      taxCollected: rand(rng, 0, 200),
      productCost: rand(rng, 0, 2000),
      fulfillmentCost: rand(rng, 0, 400),
      opsCost: rand(rng, 0, 300),
      stripeFeeRate: rand(rng, 0, 5),
      stripeFlatFee: rand(rng, 0, 1),
      internationalCardRate: rand(rng, 0, 3),
      currencyConversionRate: rand(rng, 0, 3),
    };
    if (rng() < 0.1) {
      // invalid mutations — exercise error-message parity
      const which = Math.floor(rng() * 4);
      if (which === 0) input.salePrice = -rand(rng, 0, 100);
      else if (which === 1) input.stripeFeeRate = 100 + rand(rng, 0, 50);
      else if (which === 2) input.internationalCardRate = -rand(rng, 0, 5);
      else input.taxCollected = -rand(rng, 0, 50);
    }
    // half the time, round to plain money-style decimals
    if (rng() < 0.5) {
      for (const k of Object.keys(input)) input[k] = round2(input[k]);
    }
    out.push(input);
  }
  return out;
}

function randomBondInputs(rng) {
  const out = [];
  for (let i = 0; i < 200; i++) {
    const input = {
      faceValue: rand(rng, 1, 100000),
      couponRate: rand(rng, 0, 30),
      currentPrice: rand(rng, 1, 100000),
      yearsToMaturity: 1 + Math.floor(rng() * 50),
      purchasePrice: rand(rng, 1, 100000),
      taxRate: rand(rng, 0, 50),
      quantity: 1 + Math.floor(rng() * 10000),
    };
    if (rng() < 0.12) {
      const which = Math.floor(rng() * 5);
      if (which === 0) input.faceValue = -rand(rng, 0, 100);
      else if (which === 1) input.yearsToMaturity = 0;
      else if (which === 2) input.yearsToMaturity = 10.5;
      else if (which === 3) input.taxRate = 50 + rand(rng, 0, 30);
      else input.quantity = 0;
    }
    if (rng() < 0.5) {
      input.faceValue = round2(input.faceValue);
      input.couponRate = round2(input.couponRate);
      input.currentPrice = round2(input.currentPrice);
      input.purchasePrice = round2(input.purchasePrice);
      input.taxRate = round2(input.taxRate);
    }
    out.push(input);
  }
  return out;
}

function randomUpworkForward(rng) {
  const out = [];
  for (let i = 0; i < 200; i++) {
    const gross = rng() < 0.7 ? rand(rng, 0, 20000) : rand(rng, 0, 999999);
    const prior = rng() < 0.4 ? pick(rng, PRIOR_POOL) : rand(rng, 0, 20000);
    const input = {
      gross: round2(gross),
      prior: round2(prior),
      method: pick(rng, ['ach', 'ach', 'paypal', 'wire', 'instant']),
    };
    if (rng() < 0.08) {
      if (rng() < 0.5) input.gross = -rand(rng, 0, 100);
      else input.method = 'bitcoin';
    }
    out.push(input);
  }
  return out;
}

function randomUpworkReverse(rng) {
  const out = [];
  for (let i = 0; i < 200; i++) {
    const input = {
      netTarget: round2(rand(rng, 1, 20000)),
      prior: round2(rng() < 0.4 ? pick(rng, PRIOR_POOL) : rand(rng, 0, 20000)),
      method: pick(rng, ['ach', 'ach', 'paypal', 'wire', 'instant']),
    };
    if (rng() < 0.08) {
      if (rng() < 0.4) input.netTarget = rand(rng, 0, 1); // < 1 -> invalid
      else if (rng() < 0.7) input.netTarget = 999999.5;
      else input.method = 'bitcoin';
    }
    out.push(input);
  }
  return out;
}

// ── tool entries ───────────────────────────────────────────────────────────
const ENTRIES = [
  {
    id: 'stripe-fee-profit-calculator',
    label: 'stripe-fee-profit-calculator',
    langAware: true,
    legacy: (input, lang) => stripeLegacy.calculate(input, { lang }),
    fixtures: stripeFixtures(),
    random: randomStripeInputs(mulberry32(20261128)),
  },
  {
    id: 'bond-yield-calculator',
    label: 'bond-yield-calculator',
    langAware: false,
    legacy: (input) => bondLegacy.calculate(input), // throws on invalid
    fixtures: bondFixtures(),
    random: randomBondInputs(mulberry32(20261129)),
  },
  {
    id: 'upwork-fee-calculator',
    label: 'upwork-fee-calculator (forward)',
    langAware: true,
    legacy: (input, lang) => upworkLegacy.calculateForward(input, { lang }),
    fixtures: upworkForwardFixtures(),
    random: randomUpworkForward(mulberry32(20261130)),
  },
  {
    id: 'upwork-fee-calculator:reverse',
    label: 'upwork-fee-calculator (reverse)',
    langAware: true,
    legacy: (input, lang) => upworkLegacy.calculateReverse(input, { lang }),
    fixtures: upworkReverseFixtures(),
    random: randomUpworkReverse(mulberry32(20261201)),
  },
];

// ── comparison ─────────────────────────────────────────────────────────────
const REL_TOL = 1e-9;

function numEq(a, b) {
  if (a === b) return true;
  const scale = Math.max(1, Math.abs(a), Math.abs(b));
  return Math.abs(a - b) <= REL_TOL * scale;
}

function deepEq(a, b, path, diffs) {
  if (diffs.length > 5) return;
  const ta = a === null ? 'null' : Array.isArray(a) ? 'array' : typeof a;
  const tb = b === null ? 'null' : Array.isArray(b) ? 'array' : typeof b;
  if (ta !== tb) {
    diffs.push(`${path}: type ${ta} !== ${tb} (${JSON.stringify(a)} vs ${JSON.stringify(b)})`);
    return;
  }
  if (ta === 'number') {
    if (!numEq(a, b)) diffs.push(`${path}: number ${a} !== ${b}`);
    return;
  }
  if (ta === 'array') {
    if (a.length !== b.length) {
      diffs.push(`${path}: array length ${a.length} !== ${b.length}`);
      return;
    }
    for (let i = 0; i < a.length; i++) deepEq(a[i], b[i], `${path}[${i}]`, diffs);
    return;
  }
  if (ta === 'object') {
    const ka = Object.keys(a);
    const kb = Object.keys(b);
    const setB = new Set(kb);
    for (const k of ka) {
      if (!setB.has(k)) {
        diffs.push(`${path}.${k}: missing in wasm output (legacy ${JSON.stringify(a[k])})`);
        continue;
      }
      deepEq(a[k], b[k], `${path}.${k}`, diffs);
    }
    const setA = new Set(ka);
    for (const k of kb) {
      if (!setA.has(k)) diffs.push(`${path}.${k}: extra in wasm output (${JSON.stringify(b[k])})`);
    }
    return;
  }
  if (a !== b) diffs.push(`${path}: ${JSON.stringify(a)} !== ${JSON.stringify(b)}`);
}

// ── runner ─────────────────────────────────────────────────────────────────
function legacyEnvelope(entry, input, lang) {
  if (entry.id === 'bond-yield-calculator') {
    try {
      return { ok: true, result: entry.legacy(input), error: '' };
    } catch (e) {
      return { ok: false, result: null, error: String(e && e.message ? e.message : e) };
    }
  }
  const { result, error } = entry.legacy(input, lang);
  return { ok: error === '', result: result === undefined ? null : result, error: error || '' };
}

function wasmEnvelope(entry, input, lang) {
  const payload = input === null || input === undefined ? { lang } : { ...input, lang };
  const out = invoke(entry.id, JSON.stringify(payload));
  const v = JSON.parse(out);
  if (v.ok === true) return { ok: true, result: v.result, error: v.error ?? '' };
  return { ok: false, error: v.error };
}

let totalDiff = 0;
const report = [];

for (const entry of ENTRIES) {
  const langs = entry.langAware ? ['en', 'ko'] : ['en'];
  const cases = [];
  for (const f of entry.fixtures) for (const lang of langs) cases.push({ input: f, lang });
  for (const f of entry.random) for (const lang of [langs[0]]) cases.push({ input: f, lang });

  let diffs = 0;
  let byteIdentical = 0;
  let okCases = 0;
  const examples = [];

  for (const { input, lang } of cases) {
    const legacy = legacyEnvelope(entry, input, lang);
    const wasmSide = wasmEnvelope(entry, input, lang);

    // normalize legacy result through JSON (Infinity/NaN -> null, matching
    // what the wasm side already went through)
    const legacyResult = legacy.ok
      ? JSON.parse(JSON.stringify(legacy.result))
      : null;

    const caseDiffs = [];
    if (legacy.ok !== wasmSide.ok) {
      caseDiffs.push(`ok: legacy=${legacy.ok} wasm=${wasmSide.ok} (legacy error: ${JSON.stringify(legacy.error)}, wasm error: ${JSON.stringify(wasmSide.error)})`);
    } else if (!legacy.ok) {
      if (legacy.error !== wasmSide.error) {
        caseDiffs.push(`error: legacy=${JSON.stringify(legacy.error)} wasm=${JSON.stringify(wasmSide.error)}`);
      }
    } else {
      deepEq(legacyResult, wasmSide.result, 'result', caseDiffs);
    }

    if (caseDiffs.length > 0) {
      diffs += caseDiffs.length;
      if (examples.length < 5) {
        examples.push({ input, lang, diffs: caseDiffs });
      }
    } else {
      okCases++;
      if (legacy.ok) {
        const l = JSON.stringify(legacyResult);
        const w = JSON.stringify(wasmSide.result);
        if (l === w) byteIdentical++;
        else if (examples.length < 5) {
          examples.push({
            input,
            lang,
            diffs: [`structural-equal but byte-diff (legacy ${l.length}B vs wasm ${w.length}B): first divergence at ${
              (() => { for (let i = 0; i < Math.max(l.length, w.length); i++) if (l[i] !== w[i]) return i; })()
            }`],
          });
        }
      }
    }
  }

  totalDiff += diffs;
  report.push({
    label: entry.label,
    cases: cases.length,
    passed: okCases,
    diffs,
    byteIdentical,
    examples,
  });
}

// ── output ─────────────────────────────────────────────────────────────────
console.log('toolcore parity harness — legacy JS vs wasm (nodejs target)');
console.log('float tolerance: relative 1e-9 | fixtures: test-cases + boundaries + 200 seeded random/entry');
console.log('');
for (const r of report) {
  const status = r.diffs === 0 ? 'PASS' : 'FAIL';
  console.log(`[${status}] ${r.label}: ${r.cases} cases, ${r.passed} passed, ${r.diffs} diff(s), ${r.byteIdentical} byte-identical results`);
  for (const ex of r.examples) {
    console.log(`    input=${JSON.stringify(ex.input)} lang=${ex.lang}`);
    for (const d of ex.diffs) console.log(`      - ${d}`);
  }
}
console.log('');
console.log(`TOTAL: ${totalDiff} diff(s) across ${report.length} tool entries`);
if (totalDiff > 0) {
  process.exit(1);
}
console.log('parity: OK (diff = 0)');
