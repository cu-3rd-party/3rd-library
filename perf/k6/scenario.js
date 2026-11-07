// Нагрузочный сценарий бэкенда: замер задержек по каждому эндпоинту на «золотой» БД.
// Запуск — через perf/run.sh (поднимает Postgres + S3, клонирует золотую БД, стартует бэкенд).
//
// Параметры (-e NAME=value):
//   SCENARIO  mixed (по умолчанию) | isolated
//   RATE      mixed: целевой RPS (100)              DURATION  mixed: длительность (2m)
//   ISO_DURATION  isolated: время на каждый эндпоинт (30s)
//   ISO_RATE_SCALE isolated: множитель к RPS эндпоинтов (1)
//   ONLY      isolated: список эндпоинтов через запятую (например materials_list,login)
//   MAX_VUS   верхний предел VU (200)               BASE_URL, RESULTS_DIR — задаёт run.sh
import http from 'k6/http';
import crypto from 'k6/crypto';
import encoding from 'k6/encoding';
import exec from 'k6/execution';
import { check, sleep } from 'k6';
import { Counter } from 'k6/metrics';

const BASE = __ENV.BASE_URL || 'http://127.0.0.1:18080';
const MODE = __ENV.SCENARIO || 'mixed';
const MAX_VUS = parseInt(__ENV.MAX_VUS || '200');
const HMAC_KEY = 'perf-hmac-key';
const PASSWORD = 'perf-password';
const JSON_HEADERS = { 'Content-Type': 'application/json' };

const fx = JSON.parse(open('./fixtures.json'));
const sessionsRefresh = fx.sessions.filter((s) => s.n <= 6000);
const sessionsLogout = fx.sessions.filter((s) => s.n > 6000);
const poolExhausted = new Counter('pool_exhausted');

const WORDS = ['algebra', 'calculus', 'physics', 'chemistry', 'history', 'networks', 'compilers',
  'databases', 'statistics', 'mechanics', 'optics', 'economics', 'genetics', 'robotics', 'security'];
const SMALL_FILE = 'x'.repeat(64 * 1024);

// ---------------------------------------------------------------- helpers
const pick = (arr) => arr[Math.floor(Math.random() * arr.length)];
const randInt = (n) => Math.floor(Math.random() * n);

// Уникальный индекс в пуле одноразовых данных: у каждого VU свой счётчик по эндпоинту,
// индексы разных VU не пересекаются (idx = локальный счётчик * MAX_VUS + номер VU).
const localCounters = {};
function nextIndex(ep, poolSize) {
  const n = (localCounters[ep] = (localCounters[ep] || 0) + 1) - 1;
  const idx = n * MAX_VUS + (exec.vu.idInTest - 1);
  if (idx >= poolSize) {
    poolExhausted.add(1);
    return -1;
  }
  return idx;
}

const auth = (token) => ({ Authorization: `Token ${token}` });

function req(ep, method, path, body, { token, headers, ok = 200 } = {}) {
  const res = http.request(method, `${BASE}${path}`, body, {
    headers: Object.assign({}, token ? auth(token) : {}, headers || {}),
    tags: { endpoint: ep, name: ep },
    responseCallback: http.expectedStatuses(ok),
  });
  check(res, { [`${ep}: status ${ok}`]: (r) => r.status === ok }, { endpoint: ep });
  return res;
}

const post = (ep, path, obj, opts) => req(ep, 'POST', path, JSON.stringify(obj), Object.assign({ headers: JSON_HEADERS }, opts));

// JWT HS384 в формате бэкенда (security.rs): claims user_id, sid, verified, exp.
function signJwt(userId, sid) {
  const b64 = (o) => encoding.b64encode(JSON.stringify(o), 'rawurl');
  const input = `${b64({ alg: 'HS384', typ: 'JWT' })}.${b64({
    user_id: userId, sid, verified: true, exp: Math.floor(Date.now() / 1000) + 86400,
  })}`;
  return `${input}.${crypto.hmac('sha384', HMAC_KEY, input, 'base64rawurl')}`;
}

function submissionPayload(title) {
  return {
    data: JSON.stringify({
      title, description: 'k6 generated submission', courses: ['2'], subjects: [pick(WORDS)],
      type: 'lecture', difficulty: 'medium',
    }),
    files: http.file(SMALL_FILE, `k6-${title.length}.pdf`, 'application/pdf'),
  };
}

// ---------------------------------------------------------------- endpoints
// weight — доля в mixed (сумма не обязана быть 100), rate — RPS в isolated,
// p95 — бюджет задержки (мс), по которому падает threshold.
const E = {
  health: {
    label: 'GET /api/health', weight: 4, rate: 200, p95: 50,
    run: () => req('health', 'GET', '/api/health'),
  },
  materials_list: {
    label: 'GET /api/materials', weight: 22, rate: 80, p95: 300,
    run: () => req('materials_list', 'GET', `/api/materials?page=${1 + randInt(50)}&limit=${pick([10, 20, 50])}`),
  },
  materials_search: {
    label: 'GET /api/materials?search=', weight: 10, rate: 20, p95: 800,
    run: () => req('materials_search', 'GET', `/api/materials?search=${pick(WORDS)}&limit=20`),
  },
  materials_get: {
    label: 'GET /api/materials/{id}', weight: 22, rate: 150, p95: 100,
    run: () => req('materials_get', 'GET', `/api/materials/${pick(fx.materialIds)}`),
  },
  users_list: {
    label: 'GET /api/users', weight: 4, rate: 50, p95: 400,
    run: () => req('users_list', 'GET', `/api/users?page=${1 + randInt(20)}&limit=20`),
  },
  users_search: {
    label: 'GET /api/users?search=', weight: 2, rate: 20, p95: 600,
    run: () => req('users_search', 'GET', `/api/users?search=${encodeURIComponent('Author ' + (1 + randInt(50)))}`),
  },
  user_get: {
    label: 'GET /api/users/{id}', weight: 6, rate: 100, p95: 300,
    run: () => req('user_get', 'GET', `/api/users/${pick(fx.authorIds)}?limit=20`),
  },
  me_get: {
    label: 'GET /api/users/me', weight: 6, rate: 150, p95: 80,
    run: (d) => req('me_get', 'GET', '/api/users/me', null, { token: pick(d.readers) }),
  },
  me_update: {
    label: 'PATCH /api/users/me', weight: 2, rate: 60, p95: 120,
    run: (d) => req('me_update', 'PATCH', '/api/users/me', JSON.stringify({ bio: `bio ${Date.now()}` }),
      { token: pick(d.readers), headers: JSON_HEADERS }),
  },
  submissions_list: {
    label: 'GET /api/materials/submissions', weight: 4, rate: 80, p95: 150,
    run: (d) => req('submissions_list', 'GET', `/api/materials/submissions?status=${pick(['all', 'rejected', 'pending_review'])}&limit=20`,
      null, { token: pick(d.authors).token }),
  },
  submission_get: {
    label: 'GET /api/materials/submissions/{id}', weight: 4, rate: 100, p95: 100,
    run: (d) => {
      const s = pick(fx.drafts); // черновики не расходуются запросами
      req('submission_get', 'GET', `/api/materials/submissions/${s.id}`, null, { token: d.authorByNumber[s.author] });
    },
  },
  submission_create: {
    label: 'POST /api/materials/submissions (+1 file → S3)', weight: 3, rate: 30, p95: 500,
    run: (d) => req('submission_create', 'POST', '/api/materials/submissions',
      submissionPayload(`k6 submission ${exec.vu.idInTest}-${exec.vu.iterationInInstance}-${Date.now()}`),
      { token: pick(d.authors).token }),
  },
  submission_update: {
    label: 'PATCH /api/materials/submissions/{id}', weight: 2, rate: 30, p95: 300,
    run: (d) => {
      const i = nextIndex('submission_update', fx.rejected.length);
      if (i < 0) return;
      const s = fx.rejected[i];
      req('submission_update', 'PATCH', `/api/materials/submissions/${s.id}`,
        // multipart (k6 шлёт его только если есть http.file) + новый файл → S3
        { title: `k6 edited ${Date.now()}-${i}`, description: 'edited by k6', files: http.file('edit', `k6-edit-${i}.py`, 'text/x-python') },
        { token: d.authorByNumber[s.author] });
    },
  },
  moderation_queue: {
    label: 'GET /api/moderation/submissions', weight: 3, rate: 40, p95: 400,
    run: (d) => req('moderation_queue', 'GET', `/api/moderation/submissions?status=${pick(['pending_review', 'all', 'approved'])}&page=${1 + randInt(10)}&limit=20`,
      null, { token: d.moderator }),
  },
  moderation_decision: {
    label: 'POST /api/moderation/submissions/{id}/decision', weight: 2, rate: 30, p95: 400,
    run: (d) => {
      const i = nextIndex('moderation_decision', fx.pending.length);
      if (i < 0) return;
      const approve = i % 2 === 0;
      post('moderation_decision', `/api/moderation/submissions/${fx.pending[i]}/decision`,
        approve ? { action: 'approve' } : { action: 'reject', moderator_comment: 'k6: rejected' },
        { token: d.moderator });
    },
  },
  register: {
    label: 'POST /api/auth/register', weight: 1, rate: 10, p95: 600,
    run: () => {
      const id = `${exec.vu.idInTest}-${exec.vu.iterationInInstance}-${Date.now()}`;
      post('register', '/api/auth/register', { name: `k6 user ${id}`, email: `k6-${id}@perf.test`, password: PASSWORD });
    },
  },
  verify_email: {
    label: 'POST /api/auth/verify-email', weight: 1, rate: 10, p95: 300,
    run: () => {
      const i = nextIndex('verify_email', fx.pools.verify);
      if (i < 0) return;
      post('verify_email', '/api/auth/verify-email', { email: `pending${i + 1}@perf.test`, code: '123456' });
    },
  },
  resend_code: {
    label: 'POST /api/auth/resend-verification-code', weight: 1, rate: 10, p95: 200,
    run: () => {
      const i = nextIndex('resend_code', fx.pools.resend);
      if (i < 0) return;
      post('resend_code', '/api/auth/resend-verification-code', { email: `resend${i + 1}@perf.test` });
    },
  },
  login: {
    label: 'POST /api/auth/login', weight: 6, rate: 15, p95: 600,
    run: () => post('login', '/api/auth/login', { email: `reader${1 + randInt(fx.pools.readers)}@perf.test`, password: PASSWORD }),
  },
  refresh: {
    label: 'POST /api/auth/refresh', weight: 1, rate: 30, p95: 150,
    run: () => {
      const i = nextIndex('refresh', sessionsRefresh.length);
      if (i < 0) return;
      post('refresh', '/api/auth/refresh', { refreshToken: sessionsRefresh[i].sid });
    },
  },
  logout: {
    label: 'POST /api/auth/logout', weight: 1, rate: 30, p95: 150,
    run: () => {
      const i = nextIndex('logout', sessionsLogout.length);
      if (i < 0) return;
      const s = sessionsLogout[i];
      req('logout', 'POST', '/api/auth/logout', null, { token: signJwt(s.userId, s.sid) });
    },
  },
};
const NAMES = Object.keys(E);

// ---------------------------------------------------------------- options
function buildScenarios() {
  if (MODE === 'isolated') {
    const only = (__ENV.ONLY || '').split(',').filter(Boolean);
    const names = only.length ? only : NAMES;
    const dur = parseInt(__ENV.ISO_DURATION || '30');
    const scale = parseFloat(__ENV.ISO_RATE_SCALE || '1');
    const out = {};
    names.forEach((name, i) => {
      if (!E[name]) throw new Error(`unknown endpoint ${name}; known: ${NAMES.join(', ')}`);
      out[name] = {
        executor: 'constant-arrival-rate', rate: Math.max(1, Math.round(E[name].rate * scale)), timeUnit: '1s',
        duration: `${dur}s`, startTime: `${i * (dur + 5)}s`, preAllocatedVUs: 20, maxVUs: MAX_VUS,
        env: { EP: name }, exec: 'ep',
      };
    });
    return out;
  }
  return {
    mixed: {
      executor: 'ramping-arrival-rate', startRate: 5, timeUnit: '1s', preAllocatedVUs: 50, maxVUs: MAX_VUS, exec: 'ep',
      stages: [
        { target: parseInt(__ENV.RATE || '100'), duration: '20s' }, // прогрев
        { target: parseInt(__ENV.RATE || '100'), duration: __ENV.DURATION || '2m' },
      ],
    },
  };
}

function buildThresholds() {
  const t = {};
  for (const name of NAMES) {
    t[`http_req_duration{endpoint:${name}}`] = [`p(95)<${E[name].p95}`];
    t[`http_req_failed{endpoint:${name}}`] = ['rate<0.01'];
  }
  return t;
}

export const options = {
  scenarios: buildScenarios(),
  thresholds: buildThresholds(),
  summaryTrendStats: ['avg', 'min', 'med', 'p(90)', 'p(95)', 'p(99)', 'max', 'count'],
  noConnectionReuse: false,
};

// ---------------------------------------------------------------- lifecycle
function loginAll(emails) {
  const out = [];
  for (let i = 0; i < emails.length; i += 25) {
    const batch = emails.slice(i, i + 25).map((email) => ({
      method: 'POST', url: `${BASE}/api/auth/login`, body: JSON.stringify({ email, password: PASSWORD }),
      params: { headers: JSON_HEADERS, tags: { endpoint: 'setup', name: 'setup' } },
    }));
    for (const r of http.batch(batch)) {
      if (r.status !== 200) throw new Error(`setup login failed: ${r.status} ${r.body}`);
      out.push(r.json('tokens.access_token'));
    }
  }
  return out;
}

export function setup() {
  const authorEmails = Array.from({ length: fx.pools.authors }, (_, i) => `author${i + 1}@perf.test`);
  const authorTokens = loginAll(authorEmails);
  const authorByNumber = {};
  authorTokens.forEach((t, i) => { authorByNumber[i + 1] = t; });
  return {
    moderator: loginAll(['moderator1@perf.test'])[0],
    authors: authorTokens.map((token) => ({ token })),
    authorByNumber,
    readers: loginAll(Array.from({ length: 200 }, (_, i) => `reader${i + 1}@perf.test`)),
  };
}

function weightedPick() {
  const total = NAMES.reduce((s, n) => s + E[n].weight, 0);
  let r = Math.random() * total;
  for (const n of NAMES) {
    if ((r -= E[n].weight) < 0) return n;
  }
  return NAMES[0];
}

export function ep(data) {
  E[__ENV.EP || weightedPick()].run(data);
}

// ---------------------------------------------------------------- report
function fmt(v) {
  if (v === undefined || v === null) return '-';
  return v >= 100 ? v.toFixed(0) : v.toFixed(1);
}

export function handleSummary(data) {
  const testSec = (data.state.testRunDurationMs || 1) / 1000;
  const cols = ['reqs', 'err%', 'avg', 'p50', 'p90', 'p95', 'p99', 'max'];
  const rows = [];
  for (const name of NAMES) {
    const d = data.metrics[`http_req_duration{endpoint:${name}}`];
    if (!d || !d.values.count) continue;
    const f = data.metrics[`http_req_failed{endpoint:${name}}`];
    const v = d.values;
    const budgetOk = v['p(95)'] < E[name].p95;
    rows.push({
      name, label: E[name].label, budget: E[name].p95, ok: budgetOk,
      cells: [v.count, f ? (f.values.rate * 100).toFixed(1) : '-', fmt(v.avg), fmt(v.med), fmt(v['p(90)']),
        fmt(v['p(95)']), fmt(v['p(99)']), fmt(v.max)],
    });
  }
  const w = Math.max(...rows.map((r) => r.label.length), 8);
  const head = `${'endpoint'.padEnd(w)}  ${cols.map((c) => c.padStart(7)).join(' ')}  p95 budget`;
  const lines = [`\nPer-endpoint latency (ms), ${MODE} mode, ${testSec.toFixed(0)}s\n`, head, '-'.repeat(head.length + 10)];
  for (const r of rows) {
    lines.push(`${r.label.padEnd(w)}  ${r.cells.map((c) => String(c).padStart(7)).join(' ')}  ${String(r.budget).padStart(6)}ms ${r.ok ? 'ok' : 'EXCEEDED'}`);
  }
  const ex = data.metrics.pool_exhausted;
  if (ex && ex.values.count) {
    lines.push(`\nWARNING: пул одноразовых данных исчерпан ${ex.values.count} раз(а) — часть запросов не отправлена; увеличьте пулы в seed.sql или снизьте RATE/ISO_DURATION.`);
  }
  const text = lines.join('\n') + '\n';

  const out = { stdout: text };
  if (__ENV.RESULTS_DIR) {
    out[`${__ENV.RESULTS_DIR}/summary.txt`] = text;
    out[`${__ENV.RESULTS_DIR}/summary.json`] = JSON.stringify(data, null, 2);
  }
  return out;
}
