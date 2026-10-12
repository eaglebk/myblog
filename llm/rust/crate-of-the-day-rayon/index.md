# Крейт дня: Rayon - параллельные вычисления в одну строку

> Разбираем легендарный крейт Rayon: как замена всего одной строчки .iter() на .par_iter() распараллеливает вычисления по всем ядрам процессора с балансировкой Work-Stealing.

- **URL:** https://eagle2000.space/rust/crate-of-the-day-rayon/
- **Date:** 2026-08-15

- **Tags:** rust, crate-of-the-day, concurrency, performance

---

<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  Рубрика «Крейт дня» — краткий разбор популярных (и не очень) библиотек экосистемы Rust, которые экономят сотни часов разработки.
</div>

<p>Сегодня в рубрике — <strong><code>rayon</code></strong> (один из самых популярных и изящных крейтов в мире Rust).</p>

<h3 class="relative group">🛠 Проблема, которую он решает
    <div id="-проблема-которую-он-решает" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d0%bf%d1%80%d0%be%d0%b1%d0%bb%d0%b5%d0%bc%d0%b0-%d0%ba%d0%be%d1%82%d0%be%d1%80%d1%83%d1%8e-%d0%be%d0%bd-%d1%80%d0%b5%d1%88%d0%b0%d0%b5%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Допустим у вас есть огромный массив из 10 000 000 элементов и вам нужно применить к каждому элементу тяжелую операцию.</p>
<p>В обычном Rust вы пишете стандартный итератор:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">results</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="n">_</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">data</span><span class="p">.</span><span class="n">iter</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">x</span><span class="o">|</span><span class="w"> </span><span class="n">heavy_computation</span><span class="p">(</span><span class="n">x</span><span class="p">))</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">collect</span><span class="p">();</span></span></span></code></pre></div></div>
<p>Этот код выполняется строго в <strong>один поток</strong>. Если у вас 16-ядерный процессор, 15 ядер будут просто простаивать.</p>
<p>Писать параллельность вручную через <code>thread::spawn</code>, <code>Arc&lt;Mutex&lt;T&gt;&gt;</code> и каналы - дольше, а также можно допустить ошибки во владении памятью.</p>
<hr>

<h3 class="relative group">🚀 Решение через Rayon
    <div id="-решение-через-rayon" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d1%80%d0%b5%d1%88%d0%b5%d0%bd%d0%b8%d0%b5-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-rayon" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>С крейтом <code>rayon</code> всё распараллеливание сводится к замене <strong>всего одного метода</strong>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">rayon</span>::<span class="n">prelude</span>::<span class="o">*</span><span class="p">;</span><span class="w"> </span><span class="c1">// 1. Импортируем типажи Rayon
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// 2. Меняем .iter() на .par_iter()
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">results</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="n">_</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">data</span><span class="p">.</span><span class="n">par_iter</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">x</span><span class="o">|</span><span class="w"> </span><span class="n">heavy_computation</span><span class="p">(</span><span class="n">x</span><span class="p">))</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">collect</span><span class="p">();</span></span></span></code></pre></div></div>
<hr>

<h3 class="relative group">💡 За что Rayon любят разработчики?
    <div id="-за-что-rayon-любят-разработчики" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d0%b7%d0%b0-%d1%87%d1%82%d0%be-rayon-%d0%bb%d1%8e%d0%b1%d1%8f%d1%82-%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%87%d0%b8%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li><strong>Алгоритм Work-Stealing:</strong> Rayon автоматически создаст пулл потоков под количество ядер вашего процессора. Если один поток закончил свою порцию работы быстрее других, он не ждет, а «крадет» незавершенные задачи из очереди соседей.</li>
<li><strong>Гарантии безопасности типов:</strong> Благодарю типажам <code>Send</code> и <code>Sync</code>, если ваша функция или тип данных не являются поток-безопасными, компилятор Rust просто не даст вызвать <code>.par_iter()</code>.</li>
</ol>
<p>🔗 <strong>Репозиторий на GitHub:</strong> <a href="https://github.com/rayon-rs/rayon"  target="_blank" rel="noreferrer">rayon-rs/rayon</a><br>
📦 <strong>Crates.io:</strong> <a href="https://crates.io/crates/rayon"  target="_blank" rel="noreferrer">crates.io/crates/rayon</a></p>

