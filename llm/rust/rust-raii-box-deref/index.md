# Управление памятью в Rust: идиома RAII, указатель Box и механизмы Deref Coercion

> Разбираем устройство памяти в Rust: идиома RAII, авто-очистка через Drop, управляющий указатель Box<T>, 'static ссылки с помощью Box::leak, а также механизмы Deref и Deref Coercion.

- **URL:** https://eagle2000.space/rust/rust-raii-box-deref/
- **Date:** 2026-08-12

- **Tags:** rust, dev, raii, memory, box, deref

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>При работе с системными ресурсами - выделенной памятью в куче, открытыми файлами или сетевыми сокетами - одна из главных трудностей в программировании заключается в их своевременном и гарантированном освобождении. Забытый вызов <code>free()</code> ведет к постоянным утечкам памяти, а преждевременное удаление - к уязвимостям вроде <code>use-after-free</code> или <code>double free</code>.</p>
<p>В Rust за порядок с ресурсами отвечает <strong>идиома управления ресурсами RAII (Resource Acquisition Is Initialization)</strong>.</p>
<p>В языках с ручным управлением разработчик вынужден лично отслеживать точки выхода из функций, а в языках со сборщиком мусора (Java, Go, Python) специальный рантайм фоном ищет неиспользуемые объекты. В Rust же компилятор автоматически связывает время жизни ресурса со временем жизни его владельца.</p>
<p>Сегодня мы разберем, как работает идиома RAII, для чего нужен владеющий указатель <code>Box&lt;T&gt;</code>, когда полезно отменить автоматическое освобождение через <code>Box::leak</code> и как устроено автоматическое приведение ссылок <strong>Deref Coercion</strong>.</p>
<hr>

<h2 class="relative group">1. Идиома RAII и трейт Drop
    <div id="1-идиома-raii-и-трейт-drop" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%b8%d0%b4%d0%b8%d0%be%d0%bc%d0%b0-raii-%d0%b8-%d1%82%d1%80%d0%b5%d0%b9%d1%82-drop" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust владение ресурсом (память в куче, открытый файл, сетевой сокет, мютекс) обычно привязано к времени жизни значения.</p>
<ul>
<li><strong>Ресурс захватывается</strong> при создании переменной (в ее конструкторе или функции <code>new</code>).</li>
<li><strong>Ресурс освобождается</strong> в тот момент, когда переменная выходит за пределы своей области видимости (<code>scope</code>).</li>
</ul>

<h3 class="relative group">Как устроен трейт Drop?
    <div id="как-устроен-трейт-drop" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d1%83%d1%81%d1%82%d1%80%d0%be%d0%b5%d0%bd-%d1%82%d1%80%d0%b5%d0%b9%d1%82-drop" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>За очистку ресурсов отвечает трейт <code>std::ops::Drop</code>. Когда значение выходит из области видимости, Rust автоматически запускает его <code>Drop::drop</code>, если тип реализует <code>Drop</code>. После этого автоматически уничтожаются его поля.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">CustomResource</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">name</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="nb">Drop</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">CustomResource</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">drop</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Освобождаем ресурс: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="bp">self</span><span class="p">.</span><span class="n">name</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">res</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">CustomResource</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">name</span>: <span class="s">&#34;Файл конфигурации&#34;</span><span class="p">.</span><span class="n">into</span><span class="p">()</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Работаем с ресурсом...&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w"> </span><span class="c1">// &lt;- Переменная res выходит из блока: автоматически вызывается drop()!
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Программа идет дальше.&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Вы не можете вызвать метод <code>res.drop()</code> вручную напрямую - компилятор запретит это, чтобы избежать ошибки двойного освобождения (<code>double free</code>). Если вам нужно принудительно освободить ресурс раньше конца блока, используйте функцию <code>std::mem::drop(res)</code>.</p></div></div>
<h3 class="relative group">Всегда ли освобождение ресурса гарантировано?
    <div id="всегда-ли-освобождение-ресурса-гарантировано" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%81%d0%b5%d0%b3%d0%b4%d0%b0-%d0%bb%d0%b8-%d0%be%d1%81%d0%b2%d0%be%d0%b1%d0%be%d0%b6%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5-%d1%80%d0%b5%d1%81%d1%83%d1%80%d1%81%d0%b0-%d0%b3%d0%b0%d1%80%d0%b0%d0%bd%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%be" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Трейт <code>Drop</code> не является абсолютной гарантией освобождения ресурса при любых обстоятельствах. Деструкторы не вызовутся, если:</p>
<ul>
<li><code>std::mem::forget(res)</code> намеренно предотвращает уничтожение значения и, соответственно, вызов <code>Drop</code>.</li>
<li>Циклическая структура из <code>Rc</code> или <code>Arc</code> может сохранить значения живыми навсегда: счетчик ссылок не достигает нуля, поэтому <code>Drop</code> для объектов в цикле не вызывается.</li>
<li>Происходит экстренное аварийное завершение процесса (<code>SIGKILL</code>, <code>std::process::abort</code>).</li>
</ul>
<hr>

<h2 class="relative group">2. Владеющий указатель Box<T>
    <div id="2-владеющий-указатель-box" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%b2%d0%bb%d0%b0%d0%b4%d0%b5%d1%8e%d1%89%d0%b8%d0%b9-%d1%83%d0%ba%d0%b0%d0%b7%d0%b0%d1%82%d0%b5%d0%bb%d1%8c-box" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Локальные значения в Rust обычно размещаются на стеке. Если нужно явно разместить значение в куче, используется <strong><code>Box&lt;T&gt;</code></strong>.</p>

<h3 class="relative group">Зачем нужен Box<T>?
    <div id="зачем-нужен-box" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d1%87%d0%b5%d0%bc-%d0%bd%d1%83%d0%b6%d0%b5%d0%bd-box" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Сам <code>Box&lt;T&gt;</code> представляет собой владеющий указатель на объект в куче. Для обычного <code>Sized</code>-типа размер <code>Box&lt;T&gt;</code> на стеке обычно соответствует размеру указателя - 8 байт на 64-битной платформе.</p>
<p><code>Box&lt;T&gt;</code> используется в нескольких ключевых сценариях:</p>
<ol>
<li><strong>Рекурсивные типы данных:</strong> Без <code>Box</code> компилятор не сможет определить конечный размер типа в памяти:
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">enum</span> <span class="nc">List</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Cons</span><span class="p">(</span><span class="kt">i32</span><span class="p">,</span><span class="w"> </span><span class="nb">Box</span><span class="o">&lt;</span><span class="n">List</span><span class="o">&gt;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Nil</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
Здесь <code>Box</code> необходим не ради оптимизации: без него <code>List</code> содержал бы сам себя напрямую, вызывая бесконечную рекурсию при расчете размера.</li>
<li><strong>Размещение крупных структур в куче:</strong> Когда большой объект удобнее хранить отдельно от локальных данных на стеке.</li>
<li><strong>Хранение трейт-объектов (<code>Box&lt;dyn Trait&gt;</code>):</strong> Позволяет работать с динамическим полиморфизмом.</li>
<li><strong>Стабильный адрес объекта:</strong> Сам <code>Box&lt;T&gt;</code> можно перемещать по стеку, при этом сам объект <code>T</code> в куче остается на том же месте в памяти.</li>
</ol>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">BigData</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">values</span>: <span class="p">[</span><span class="kt">u8</span><span class="p">;</span><span class="w"> </span><span class="mi">10000</span><span class="p">],</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Данные размещаются в куче
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">boxed_data</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">BigData</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">values</span>: <span class="p">[</span><span class="mi">0</span><span class="p">;</span><span class="w"> </span><span class="mi">10000</span><span class="p">]</span><span class="w"> </span><span class="p">});</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Модифицируем данные по указателю
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">boxed_data</span><span class="p">.</span><span class="n">values</span><span class="p">[</span><span class="mi">0</span><span class="p">]</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">42</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="c1">// При выходе boxed_data из scope освобождается память в куче!
</span></span></span></code></pre></div></div>

<h3 class="relative group">Отмена деаллокации: <code>Box::leak</code>
    <div id="отмена-деаллокации-boxleak" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%82%d0%bc%d0%b5%d0%bd%d0%b0-%d0%b4%d0%b5%d0%b0%d0%bb%d0%bb%d0%be%d0%ba%d0%b0%d1%86%d0%b8%d0%b8-boxleak" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Иногда требуется зарегистрировать динамически созданную конфигурацию или данные, которые должны жить в течение всего времени работы программы со ссылкой <code>'static</code>.</p>
<p>Для этого существует метод <code>Box::leak</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">GlobalConfig</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">port</span>: <span class="kt">u16</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">config</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">GlobalConfig</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">port</span>: <span class="mi">8080</span><span class="w"> </span><span class="p">});</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Забирает владение Box&lt;T&gt; и возвращает ссылку на объект
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">static_ref</span>: <span class="kp">&amp;</span><span class="nb">&#39;static</span> <span class="nc">mut</span><span class="w"> </span><span class="n">GlobalConfig</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">leak</span><span class="p">(</span><span class="n">config</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">static_ref</span><span class="p">.</span><span class="n">port</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">9090</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="tip">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span></div>
        <div class="grow">
          Совет
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p><code>Box::leak</code> забирает владение <code>Box&lt;T&gt;</code> и возвращает ссылку на объект в куче. Автоматически освободить этот объект после этого уже нельзя. Полученную ссылку можно использовать с временем жизни <code>'static</code> или более коротким.</p>
<p>Обратите внимание: <code>'static</code> здесь относится именно к времени жизни возвращаемой ссылки, а не означает, что объект обязан физически существовать ровно до завершения процесса.</p>
<p>Наличие <code>&amp;mut</code> при этом <strong>не отменяет стандартных правил заимствования Rust</strong> (алиасинг по-прежнему запрещен). На практике для многопоточных синглтонов чаще используют типы <code>OnceLock</code> или <code>LazyLock</code>, а <code>Box::leak</code> применяется как низкоуровневый инструмент передачи данных в API с требованиями <code>'static</code>.</p></div></div><hr>

<h2 class="relative group">3. Трейты Deref и DerefMut
    <div id="3-трейты-deref-и-derefmut" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d1%82%d1%80%d0%b5%d0%b9%d1%82%d1%8b-deref-%d0%b8-derefmut" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Как сделать так, чтобы умный указатель ощущался в коде так же удобно, как обычная ссылка? Для этого в модуле <code>std::ops</code> предназначены типажи <code>Deref</code> и <code>DerefMut</code>.</p>
<p>При разыменовании <code>*my_box</code> Rust использует реализацию <code>Deref</code>, концептуально получая ссылку через <code>my_box.deref()</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">ops</span>::<span class="p">{</span><span class="n">Deref</span><span class="p">,</span><span class="w"> </span><span class="n">DerefMut</span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">MyBox</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="p">(</span><span class="n">T</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="n">Deref</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">MyBox</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">type</span> <span class="nc">Target</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">T</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">deref</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="nc">Self</span>::<span class="n">Target</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="o">&amp;</span><span class="bp">self</span><span class="p">.</span><span class="mi">0</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_value</span><span class="p">(</span><span class="n">value</span>: <span class="kp">&amp;</span><span class="kt">i32</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Значение: </span><span class="si">{value}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">boxed_val</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">MyBox</span><span class="p">(</span><span class="mi">42</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Rust автоматически использует Deref для передачи ссылки
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">print_value</span><span class="p">(</span><span class="o">&amp;</span><span class="n">boxed_val</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">4. Механизм Deref Coercion (Автоматическое приведение ссылок)
    <div id="4-механизм-deref-coercion-автоматическое-приведение-ссылок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d0%bc%d0%b5%d1%85%d0%b0%d0%bd%d0%b8%d0%b7%d0%bc-deref-coercion-%d0%b0%d0%b2%d1%82%d0%be%d0%bc%d0%b0%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%be%d0%b5-%d0%bf%d1%80%d0%b8%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5-%d1%81%d1%81%d1%8b%d0%bb%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Мы реализовали <code>Deref</code> - и передача <code>&amp;MyBox&lt;i32&gt;</code> в функцию, ожидающую <code>&amp;i32</code>, сработала автоматически. Но как именно компилятор понимает, что одну ссылку можно передать вместо другой?</p>
<p>Здесь на сцену выходит механизм <strong>Deref Coercion</strong>. Если тип <code>T</code> реализует <code>Deref&lt;Target = U&gt;</code>, компилятор умеет автоматически приводить ссылки вида <code>&amp;T</code> к ссылкам <code>&amp;U</code>.</p>
<p>Важно подчеркнуть: <strong>Deref Coercion работает строго со ссылками</strong>, а не преобразует сам <code>Box&lt;T&gt;</code> по значению.</p>

<h3 class="relative group">Три правила приведения ссылок:
    <div id="три-правила-приведения-ссылок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d1%80%d0%b8-%d0%bf%d1%80%d0%b0%d0%b2%d0%b8%d0%bb%d0%b0-%d0%bf%d1%80%d0%b8%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d1%8f-%d1%81%d1%81%d1%8b%d0%bb%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li>Из <code>&amp;T</code> в <code>&amp;U</code>, если <code>T: Deref&lt;Target = U&gt;</code> (неизменяемая ссылка в неизменяемую).</li>
<li>Из <code>&amp;mut T</code> в <code>&amp;mut U</code>, если <code>T: DerefMut&lt;Target = U&gt;</code> (изменяемая ссылка в изменяемую).</li>
<li>Из <code>&amp;mut T</code> в <code>&amp;U</code>, если <code>T: Deref&lt;Target = U&gt;</code> (из изменяемой в неизменяемую).</li>
</ol>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="caution">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
<path fill="currentColor"  d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-2.8-5.6-5.6-11.2-9.8-16.8l-50.6 58.8s-81.4-103.6-87.1-110.6C133.1 243.8 112 273.2 112 306.8C112 375.4 162.6 416 225.7 416z"/></svg></span></div>
        <div class="grow">
          Предостережение
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Приведение из <code>&amp;T</code> в <code>&amp;mut U</code> <strong>запрещено фундаментально</strong> - оно нарушило бы ключевые гарантии алиасинга и безопасности памяти в Rust.</p></div></div>
<h3 class="relative group">Цепочки приведений (Chain Coercion)
    <div id="цепочки-приведений-chain-coercion" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%86%d0%b5%d0%bf%d0%be%d1%87%d0%ba%d0%b8-%d0%bf%d1%80%d0%b8%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b9-chain-coercion" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Компилятор умеет применять <code>Deref</code> цепочкой несколько раз подряд. Рассмотрим тип <code>Box&lt;String&gt;</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">&amp;Box&lt;String&gt;
</span></span><span class="line"><span class="cl">    ↓ Deref
</span></span><span class="line"><span class="cl">&amp;String
</span></span><span class="line"><span class="cl">    ↓ Deref
</span></span><span class="line"><span class="cl">&amp;str</span></span></code></pre></div></div>
<p>Благодаря этой цепочке при передаче <code>&amp;Box&lt;String&gt;</code> в функцию, принимающую <code>&amp;str</code>, Rust прозрачно приведёт ссылки без явных вызовов:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_greeting</span><span class="p">(</span><span class="n">name</span>: <span class="kp">&amp;</span><span class="kt">str</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Привет, </span><span class="si">{name}</span><span class="s">!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">boxed_name</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;Алексей&#34;</span><span class="p">));</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Автоматическая цепочка Deref Coercion: &amp;Box&lt;String&gt; -&gt; &amp;String -&gt; &amp;str
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">print_greeting</span><span class="p">(</span><span class="o">&amp;</span><span class="n">boxed_name</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Благодаря Deref Rust умеет искать подходящие методы и у целевого типа:
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Длина имени: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">boxed_name</span><span class="p">.</span><span class="n">len</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Благодаря <code>Deref</code> Rust умеет искать подходящие методы и у целевого типа. Поэтому <code>boxed_name.len()</code> работает так же естественно, как вызов <code>len()</code> у <code>String</code> или <code>str</code>.</p>
<p>Давайте изучим эти механизмы на интерактивных слайдах:</p>
### 💻 Управление памятью, Box и Deref Coercion


#### Шаг 1


```rust
struct SensorData {
    name: String,
}

impl Drop for SensorData {
    fn drop(&mut self) {
        println!("Очистка ресурса датчика: {}", self.name);
    }
}

fn main() {
    {
        let sensor = SensorData {
            name: "Температура Кухни".to_string(),
        };
        println!("Работаем с датчиком: {}", sensor.name);
    } // Здесь область видимости заканчивается - автоматически вызывается Drop!
    println!("Секция завершена.");
}
```


**Идиома RAII и трейт Drop**
- Концепция RAII гарантирует: ресурс выделяется при создании и автоматически деаллоцируется при выходе из области видимости.
- Трейт `Drop` позволяет переопределить логику очистки (закрытие файлов, освобождение памяти, логирование).
- Вызов метода `drop` происходит автоматически в порядке, обратном объявлению переменных.


#### Шаг 2


```rust
struct Config {
    port: u16,
}

fn main() {
    // 1. Создаем объект в куче через Box
    let mut heap_config = Box::new(Config { port: 8080 });
    heap_config.port = 9090; // Мутабельный доступ к данным в куче
    println!("Порт конфигурации в куче: {}", heap_config.port);

    // 2. Намеренно «утекаем» память для получения 'static ссылки
    let static_config: &'static mut Config = Box::leak(heap_config);
    static_config.port = 3000;
    println!("Статический порт после Box::leak: {}", static_config.port);
}
```


**Умный указатель Box<T> и Box::leak**
- `Box<T>` перемещает объект из стека в кучу (heap), сохраняя владение над ним.
- `Box::leak` отменяет автоматический вызов `Drop` и возвращает мутабельную ссылку со статическим временем жизни `'static`.
- Это полезно для создания глобальных конфигураций или долгоживущих синглтонов в рантайме.


#### Шаг 3


```rust
use std::ops::{Deref, DerefMut};

struct SmartWrapper<T> {
    value: T,
}

impl<T> SmartWrapper<T> {
    fn new(value: T) -> Self {
        SmartWrapper { value }
    }
}

impl<T> Deref for SmartWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SmartWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut wrapped_num = SmartWrapper::new(42);
    
    // Перегрузка оператора разыменования *
    assert_eq!(*wrapped_num, 42);
    
    *wrapped_num += 10;
    println!("Обновленное значение: {}", *wrapped_num);
}
```


**Перегрузка разыменования: Deref и DerefMut**
- Реализация трейта `Deref` позволяет использовать оператор разыменования `*` для кастомных умных указателей.
- `DerefMut` расширяет разыменование для изменяемых ссылок (`*x = new_val`).
- Ассоциированный тип `type Target = T` указывает, к какому типу приводит разыменование.


#### Шаг 4


```rust
// Функция ожидает обычный строковый срез &str
fn print_message(msg: &str) {
    println!("Сообщение: {msg}");
}

fn main() {
    let boxed_string: Box<String> = Box::new(String::from("Привет от Deref!"));
    
    // Цепочка приведений Deref Coercion: &Box<String> -> &String -> &str
    print_message(&boxed_string);
    
    // Проверка методов целевого типа прямо через умный указатель:
    println!("Длина строки: {}", boxed_string.len());
}
```


**Неявное приведение ссылок (Deref Coercion)**
- Компилятор автоматически преобразует ссылки на умный указатель в ссылку на внутренний тип: `&Box<String>` -> `&String` -> `&str`.
- Это работает для цепочек приведений типов, реализующих `Deref`.
- Позволяет вызывать методы внутреннего типа напрямую через указатель (например, `boxed_string.len()`).


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий тест, чтобы закрепить понимание идиомы RAII, работы <code>Box&lt;T&gt;</code></p>
### 📝 Управление памятью и Deref

**1. В чем суть идиомы RAII (Resource Acquisition Is Initialization) в Rust?**

- [ ] Выделение ресурсов производится только во время компиляции, а освобождение - при перезапуске системы
- [x] Захват ресурса происходит при инициализации объекта, а деаллокация гарантированно выполняется деструктором Drop при выходе из области видимости
- [ ] Все объекты принудительно копируются в кучу для предотвращения переполнения стека

> 💡 *RAII связывает время жизни ресурса с временем жизни объекта. При выходе объекта из текущей области видимости автоматический вызов `drop()` очищает память или закрывает ресурсы.*


**2. Для чего используется метод `Box::leak`?**

- [ ] Для экстренного переноса данных из кучи на стек
- [x] Для намеренной отмены деаллокации и получения ссылки со статическим временем жизни `'static`
- [ ] Для автоматической оптимизации размера объекта в памяти

> 💡 *`Box::leak` отменяет автоматический вызов `Drop` для `Box<T>` и возвращает ссылку со статическим временем жизни `&'static mut T`. Это полезно при создании глобальных конфигураций в рантайме.*


**3. Каковая главная функция трейта `Deref`?**

- [ ] Разрешение клонирования любых типов без явного вызова `.clone()`
- [x] Перегрузка оператора разыменования `*` и поддержка неявного приведения типов (Deref Coercion)
- [ ] Автоматическая упаковка данных на стек

> 💡 *`Deref` позволяет обращаться к внутренним данным умного указателя через оператор `*` и неявно приводит ссылки типа `&SmartPointer<T>` к `&T`.*


**4. Какое неявное приведение типов (Deref Coercion) поддерживается компилятором Rust?**

- [ ] Из иммутабельной ссылки `&T` в мутабельную `&mut U`
- [x] Из изменяемой ссылки `&mut T` в изменяемую `&mut U` (если `T: DerefMut<Target = U>`)
- [ ] Из строкового среза `&str` в `&mut String`

> 💡 *Rust разрешает Deref Coercion из `&T` в `&U`, из `&mut T` в `&mut U` и из `&mut T` в `&U`. Приведение из иммутабельной ссылки в мутабельную запрещено гарантиями безопасности памяти.*


**5. Почему компилятор запрещает вызывать метод `res.drop()` вручную напрямую?**

- [ ] Метод drop не существует во время работы программы
- [x] Прямой вызов привёл бы к ошибке двойного освобождения (double free), так как деструктор все равно автоматически вызвался бы в конце блока
- [ ] Метод drop можно вызывать только внутри сгенерированного кода компилятора

> 💡 *Если бы компилятор разрешил явный вызов `res.drop()`, то в конце области видимости объект был бы уничтожен повторно. Для явного раннего освобождения памяти используется `std::mem::drop(res)`.*




