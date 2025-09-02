const RANKS = [
  { min: 0, title: "Новичок" },
  { min: 10, title: "Падаван" },
  { min: 25, title: "Исследователь" },
  { min: 50, title: "Мастер" },
  { min: 100, title: "Архитектор" },
];

function rankTitle(xp) {
  let t = RANKS[0].title;
  for (const r of RANKS) if (xp >= r.min) t = r.title;
  return t;
}
const mmss = s => {
  s = Math.max(0, Math.min(359999, Math.floor(s || 0)));
  const m = String(Math.floor(s/60)).padStart(1,"0");
  const r = String(s%60).padStart(2,"0");
  return `${m}:${r}`;
};

// Опциональная загрузка Firebase compat по CDN
async function ensureFirebase(firebaseConfig) {
  if (!firebaseConfig || !firebaseConfig.apiKey) return null;
  if (window.firebase?.apps?.length) return window.firebase;

  const base = "https://www.gstatic.com/firebasejs/10.12.5";
  await Promise.all([
    import(`${base}/firebase-app-compat.js`),
    import(`${base}/firebase-auth-compat.js`),
    import(`${base}/firebase-firestore-compat.js`),
  ]);
  firebase.initializeApp(firebaseConfig);
  return firebase;
}

class QuizWidgetEl extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.state = {
      view: "idle", // idle | running | finished | report
      quizIdx: 0,
      qIdx: 0,
      timeLeft: 0,
      answers: [], // {qid, pick, correct, correctIndex, elapsedSec}
      startedAt: null,
      quizzes: [],
      user: null, // uid/displayName/photoURL
      stats: { rankXP: 0, totalCompleted: 0, totalCorrect: 0 },
      name: this.dataset.name || "Quiz",
    };
    this._timer = null;
    this._db = null;
    this._auth = null;
  }

  connectedCallback() { this.renderShell(); }

  async init({ srcs, firebaseConfig }) {
    // Загружаем квизы
    const quizzes = [];
    for (const url of srcs || []) {
      const r = await fetch(url, { cache: "no-store" });
      if (!r.ok) throw new Error(`HTTP ${r.status} for ${url}`);
      quizzes.push(await r.json());
    }
    this.state.quizzes = quizzes;

    // Firebase (только статистика)
    const fb = await ensureFirebase(firebaseConfig);
    if (fb) {
      this._auth = fb.auth();
      this._db   = fb.firestore();
      await this._auth.signInAnonymously().catch(()=>{});
      this.state.user = this._auth.currentUser ? {
        uid: this._auth.currentUser.uid,
        displayName: this._auth.currentUser.displayName || null,
        photoURL: this._auth.currentUser.photoURL || null,
      } : null;

      // Загрузим базовые counters
      if (this.state.user) {
        const uref = this._db.collection("users").doc(this.state.user.uid);
        const snap = await uref.get();
        if (snap.exists) {
          const d = snap.data() || {};
          this.state.stats.rankXP = d.rankXP || 0;
          this.state.stats.totalCompleted = d.totalCompleted || 0;
          this.state.stats.totalCorrect = d.totalCorrect || 0;
        } else {
          await uref.set({
            displayName: this.state.user.displayName,
            photoURL: this.state.user.photoURL,
            rankXP: 0, totalCompleted: 0, totalCorrect: 0,
            createdAt: fb.firestore.FieldValue.serverTimestamp(),
          });
        }
      }
    } else {
      // Локальный аноним
      const RAW = localStorage.getItem("quiz_user_cache");
      if (RAW) {
        try { this.state.stats = JSON.parse(RAW); } catch {}
      } else {
        localStorage.setItem("quiz_user_cache", JSON.stringify(this.state.stats));
      }
    }

    this.render();
  }

  startQuiz() {
    const qz = this.currentQuiz();
    if (!qz) return;
    this.state.timeLeft = qz.timeLimitSec || (qz.questions.length * 30);
    this.state.startedAt = Date.now();
    this.state.answers = [];
    this.state.qIdx = 0;
    this.state.view = "running";
    this.tickStart();
    this.render();
  }

  tickStart() {
    if (this._timer) clearInterval(this._timer);
    this._timer = setInterval(() => {
      this.state.timeLeft -= 1;
      if (this.state.timeLeft <= 0) {
        this.state.timeLeft = 0;
        this.finishQuiz();
      }
      this.updateTimerOnly();
    }, 1000);
  }
  tickStop() { if (this._timer) clearInterval(this._timer); this._timer = null; }

  answer(pick) {
    const qz = this.currentQuiz();
    if (!qz) return;
    const q = qz.questions[this.state.qIdx];
    const correctIndex = q.correctIndex;
    const correct = pick === correctIndex;
    const elapsed = this.state.startedAt ? Math.floor((Date.now() - this.state.startedAt)/1000) : 0;

    // не позволяем менять ответ
    if (this.state.answers[this.state.qIdx]) return;

    this.state.answers.push({ qid: q.id, pick, correct, correctIndex, elapsedSec: elapsed });
    this.render(); // отрисует фидбек и кнопку "Далее"
  }

  next() {
    const qz = this.currentQuiz();
    if (!qz) return;
    const next = this.state.qIdx + 1;
    if (next >= qz.questions.length) {
      this.finishQuiz();
    } else {
      this.state.qIdx = next;
      this.render();
    }
  }

  async finishQuiz() {
    this.tickStop();
    const qz = this.currentQuiz();
    if (!qz) return;
    this.state.view = "finished";
    const correctCount = this.state.answers.filter(a => a.correct).length;
    const wrongCount   = this.state.answers.length - correctCount;
    const elapsed = this.state.startedAt ? Math.floor((Date.now() - this.state.startedAt)/1000) : 0;
    const gainedXP = correctCount * 2 + 3;

    // Persist только статистики
    try {
      if (this._db && this.state.user) {
        const fb = window.firebase;
        const uref = this._db.collection("users").doc(this.state.user.uid);
        await this._db.collection("users").doc(this.state.user.uid)
          .collection("sessions")
          .add({
            quizId: qz.id,
            correctCount, wrongCount,
            total: this.state.answers.length,
            elapsedSec: elapsed,
            createdAt: fb.firestore.FieldValue.serverTimestamp(),
            answers: this.state.answers,
          });

        const sref = this._db.collection("quizzes").doc(qz.id).collection("stats").doc("stats");
        await sref.set(
          { runs: fb.firestore.FieldValue.increment(1),
            correctTotal: fb.firestore.FieldValue.increment(correctCount),
            wrongTotal: fb.firestore.FieldValue.increment(wrongCount) },
          { merge: true }
        );
        await uref.set(
          { rankXP: fb.firestore.FieldValue.increment(gainedXP),
            totalCompleted: fb.firestore.FieldValue.increment(1),
            totalCorrect: fb.firestore.FieldValue.increment(correctCount) },
          { merge: true }
        );
        // локально обновим
        this.state.stats.rankXP += gainedXP;
        this.state.stats.totalCompleted += 1;
        this.state.stats.totalCorrect += correctCount;
      } else {
        // локальный storage
        this.state.stats.rankXP += gainedXP;
        this.state.stats.totalCompleted += 1;
        this.state.stats.totalCorrect += correctCount;
        localStorage.setItem("quiz_user_cache", JSON.stringify(this.state.stats));
      }
    } catch (e) { console.error(e); }

    this.render();
  }

  proceedNextQuiz() {
    const next = this.state.quizIdx + 1;
    if (next < this.state.quizzes.length) {
      this.state.quizIdx = next;
      this.state.view = "idle";
      this.state.answers = [];
      this.state.qIdx = 0;
      this.state.timeLeft = 0;
      this.state.startedAt = null;
    } else {
      this.state.view = "report";
    }
    this.render();
  }

  restart() {
    this.state.view = "idle";
    this.state.answers = [];
    this.state.qIdx = 0;
    this.state.timeLeft = 0;
    this.state.startedAt = null;
    this.render();
  }

  currentQuiz() { return this.state.quizzes[this.state.quizIdx] || null; }

  /* ---------- RENDER ---------- */
  renderShell() {
    const css = `
:host{--bg:#0b0b0e;--card:#131319;--muted:#8b8ea3;--border:#282a36;--ok:#16a34a;--bad:#ef4444;--acc:#6366f1}
*{box-sizing:border-box}
.container{border:1px solid var(--border);background:var(--card);color:#fff;border-radius:16px;padding:16px;max-width:820px}
.top{display:flex;align-items:center;justify-content:space-between;gap:12px}
.stat{font-size:12px;color:var(--muted)}
.stat strong{display:block;color:#fff;font-size:18px}
.avatar{width:32px;height:32px;border-radius:999px;border:1px solid var(--border)}
.hr{height:1px;background:var(--border);margin:12px 0}
.btn{border:1px solid var(--border);background:#1b1e27;color:#fff;padding:8px 12px;border-radius:12px;cursor:pointer}
.btn.primary{background:var(--acc);border-color:transparent}
.badge{display:inline-flex;align-items:center;gap:6px;padding:4px 8px;border:1px solid var(--border);border-radius:999px;background:#11121a;color:#fff;font-size:12px}
.timer{font-family:ui-monospace, SFMono-Regular, Menlo, monospace;border:1px solid var(--border);border-radius:8px;padding:2px 8px}
h2{margin:4px 0 8px 0;font-size:20px}
.code{white-space:pre-wrap;background:#0b0c12;border:1px solid var(--border);padding:8px;border-radius:12px;margin:8px 0;font-family:ui-monospace,monospace;font-size:13px;overflow:auto}
.opts{display:grid;gap:8px}
.opt{padding:10px 12px;border:1px solid var(--border);border-radius:12px;background:#10121a;text-align:left;cursor:pointer;transition:transform .12s ease}
.opt:hover{transform:scale(1.01)}
.opt.correct{border-color:var(--ok);background:#072812}
.opt.wrong{border-color:var(--bad);background:#2a0e10}
.feedback{font-size:14px;margin-top:8px}
.progress{height:10px;border:1px solid var(--border);border-radius:999px;overflow:hidden;background:#0e0f15}
.progress>div{height:100%;background:var(--ok);transition:width .2s}
.cards{display:grid;grid-template-columns:1fr 1fr;gap:8px;margin:12px 0}
.card{border:1px solid var(--border);border-radius:12px;background:#10121a;padding:12px;text-align:center}
.pie{width:160px;height:160px;border-radius:50%;background:
  conic-gradient(var(--ok) calc(var(--p)*1%), var(--bad) 0)}
.pie-wrap{display:flex;align-items:center;justify-content:center;height:180px}
.fade{animation:fade .2s ease}
@keyframes fade{from{opacity:.001;transform:scale(.98)}to{opacity:1;transform:scale(1)}}
    `;
    this.shadowRoot.innerHTML = `
      <style>${css}</style>
      <div class="container">
        <div class="top">
          <div><div class="stat">Текущий ранг</div><div class="rank"><strong>-</strong></div></div>
          <div><div class="stat">Выполнено</div><div class="done"><strong>0</strong> тест(ов)</div></div>
          <div class="auth"></div>
        </div>
        <div class="hr"></div>
        <div class="body"></div>
      </div>
    `;
  }

  updateTimerOnly() {
    const t = this.shadowRoot.querySelector(".timer");
    if (t) t.textContent = mmss(this.state.timeLeft);
  }

  render() {
    // Верхняя панель
    this.shadowRoot.querySelector(".rank strong").textContent =
      rankTitle(this.state.stats.rankXP);
    this.shadowRoot.querySelector(".done strong").textContent =
      String(this.state.stats.totalCompleted);

    const body = this.shadowRoot.querySelector(".body");
    body.innerHTML = "";

    const qz = this.currentQuiz();

    if (this.state.view === "idle" && qz) {
      body.innerHTML = `
        <div class="fade">
          <div class="stat" style="display:flex;justify-content:space-between">
            <span>Тема</span><span>Тест ${this.state.quizIdx+1}/${this.state.quizzes.length}</span>
          </div>
          <h2>${qz.topic}</h2>
          <div class="stat">Вопросов: ${qz.questions.length} · Лимит времени: ${mmss(qz.timeLimitSec || qz.questions.length*30)}</div>
          <div style="margin-top:8px">
            <button class="btn primary">Пройти тест</button>
          </div>
        </div>
      `;
      body.querySelector(".btn.primary").onclick = () => this.startQuiz();
      return;
    }

    if (this.state.view === "running" && qz) {
      const q = qz.questions[this.state.qIdx];
      const a = this.state.answers[this.state.qIdx];
      const answered = !!a;
      const correctIndex = q.correctIndex;

      const head = document.createElement("div");
      head.className = "fade";
      head.innerHTML = `
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px">
          <div class="badge"><span>?</span> <span>Вопрос ${this.state.qIdx+1}/${qz.questions.length}</span></div>
          <div class="timer">${mmss(this.state.timeLeft)}</div>
        </div>
        <div style="font-weight:600;font-size:18px;margin:6px 0">${q.title}</div>
      `;
      body.appendChild(head);

      if (q.code) {
        const pre = document.createElement("div");
        pre.className = "code";
        pre.textContent = q.code;
        body.appendChild(pre);
      }

      const opts = document.createElement("div");
      opts.className = "opts";
      q.options.forEach((text, idx) => {
        const btn = document.createElement("button");
        btn.className = "opt";
        btn.innerHTML = `<span style="opacity:.7;margin-right:6px">${String.fromCharCode(65+idx)})</span> ${text}`;
        if (answered) {
          const picked = a.pick === idx;
          if (idx === correctIndex) btn.classList.add("correct");
          if (picked && idx !== correctIndex) btn.classList.add("wrong");
          btn.disabled = true;
          btn.style.cursor = "default";
        } else {
          btn.onclick = () => this.answer(idx);
        }
        opts.appendChild(btn);
      });
      body.appendChild(opts);

      if (answered) {
        const fb = document.createElement("div");
        fb.className = "feedback";
        const ok = a.correct;
        fb.style.color = ok ? "var(--ok)" : "var(--bad)";
        fb.innerHTML = ok ? "Верно!" : `Неверно. <div class="stat" style="margin-top:4px">
          Правильный ответ: <strong>${String.fromCharCode(65+correctIndex)})</strong><br>
          ${q.explanation ? `<div style="margin-top:4px">${q.explanation}</div>` : ""}
        </div>`;
        body.appendChild(fb);

        const next = document.createElement("div");
        next.style.marginTop = "8px";
        next.innerHTML = `<button class="btn">Далее</button>`;
        next.querySelector(".btn").onclick = () => this.next();
        body.appendChild(next);
      }
      return;
    }

    if (this.state.view === "finished" && qz) {
      const correct = this.state.answers.filter(a=>a.correct).length;
      const wrong   = this.state.answers.length - correct;
      const total = this.state.answers.length || qz.questions.length;
      const percent = total ? Math.round(correct*100/total) : 0;

      body.innerHTML = `
        <div class="fade">
          <h2>Тест пройден</h2>
          <div class="pie-wrap">
            <div class="pie" style="--p:${percent}"></div>
          </div>
          <div class="cards">
            <div class="card"><div class="stat">Верный</div><div style="font-size:24px">${correct}</div></div>
            <div class="card"><div class="stat">Неверный</div><div style="font-size:24px">${wrong}</div></div>
          </div>
          <div style="display:flex;gap:8px;flex-wrap:wrap">
            <button class="btn primary" data-report>Перейти к отчёту</button>
            ${(this.state.quizIdx+1 < this.state.quizzes.length)
              ? `<button class="btn" data-next>Следующий тест</button>`
              : `<button class="btn" data-restart>Пройти снова</button>`}
          </div>
        </div>
      `;
      body.querySelector("[data-report]").onclick = () => { this.state.view = "report"; this.render(); };
      const n = body.querySelector("[data-next]");
      const r = body.querySelector("[data-restart]");
      if (n) n.onclick = () => this.proceedNextQuiz();
      if (r) r.onclick = () => this.restart();
      return;
    }

    if (this.state.view === "report" && qz) {
      const correct = this.state.answers.filter(a=>a.correct).length;
      const total = this.state.answers.length || qz.questions.length;
      const percent = total ? Math.round(correct*100/total) : 0;
      const bar = `
        <div class="progress"><div style="width:${percent}%"></div></div>
        <div class="stat" style="margin-top:6px">${correct} верный, ${total-correct} неверных</div>
      `;
      const top = document.createElement("div");
      top.className = "fade";
      top.innerHTML = `
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:6px">
          <h2 style="margin:0">Отчёт</h2>
          <button class="btn" data-close>Завершить</button>
        </div>
        <div class="stat">Успех: ${percent}%</div>
        ${bar}
        <div class="hr"></div>
      `;
      body.appendChild(top);
      top.querySelector("[data-close]").onclick = () => { this.state.view = "idle"; this.render(); };

      qz.questions.forEach((q, idx) => {
        const a = this.state.answers[idx] || null;
        const isOk = !!a?.correct;
        const card = document.createElement("div");
        card.className = "card";
        card.style.textAlign = "left";
        card.innerHTML = `
          <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:6px">
            <div class="stat">Вопрос ${idx+1}</div>
            <div style="color:${isOk?'var(--ok)':'var(--bad)'};font-weight:600">
              ${isOk?'верно':'ошибка'}
            </div>
          </div>
          <div style="font-weight:600;margin-bottom:6px">${q.title}</div>
          ${q.code ? `<div class="code">${q.code.replace(/[&<>]/g, c=>({ '&':'&amp;','<':'&lt;','>':'&gt;' }[c]))}</div>` : ""}
          <div class="stat">Ваш ответ: <strong>${
            a && a.pick!=null ? `${String.fromCharCode(65+a.pick)}) ${q.options[a.pick]}` : "—"
          }</strong></div>
          <div>Правильный: <strong>${String.fromCharCode(65+q.correctIndex)}) ${q.options[q.correctIndex]}</strong></div>
          ${q.explanation ? `<div class="stat" style="margin-top:6px">${q.explanation}</div>` : ""}
        `;
        body.appendChild(card);
      });

      const actions = document.createElement("div");
      actions.style.marginTop = "8px";
      actions.innerHTML = `<button class="btn" data-restart>Пройти снова</button>`;
      actions.querySelector("[data-restart]").onclick = () => this.restart();
      body.appendChild(actions);
      return;
    }

    // Фоллбек
    body.innerHTML = `<div class="stat">Квизы не найдены.</div>`;
  }
}

customElements.define("quiz-widget", QuizWidgetEl);

export async function mountQuizWidget(mountEl, { srcs = [], firebaseConfig = {}, name = "Quiz" } = {}) {
  // Вставляем элемент и инициализируем
  mountEl.innerHTML = ""; // очистка
  const el = document.createElement("quiz-widget");
  el.dataset.name = name;
  mountEl.appendChild(el);
  await el.init({ srcs, firebaseConfig });
}
