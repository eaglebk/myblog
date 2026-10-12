# Эффективная обработка данных из нескольких источников на Dart

> Как объединять асинхронные потоки данных (StreamGroup) в Dart и Flutter с сохранением контекста источников? Разбор практических паттернов и примеров кода.

- **URL:** https://eagle2000.space/dart/streams-groups/
- **Date:** 2024-05-11

- **Tags:** dart, dev

---


<h2 class="relative group">Введение:
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Сегодня я расскажу вам о том, как можно объединить несколько потоков данных Stream в Dart, сохраняя при этом их идентичность. Это полезно, если вы создаете систему реального времени, где данные поступают из разных источников и вам важно знать, откуда они пришли. Давайте начнем!</p>

<h2 class="relative group">Проблема
    <div id="проблема" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b1%d0%bb%d0%b5%d0%bc%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Представьте, что вы разрабатываете приложение для мониторинга различных сенсоров в умном доме. (Люблю эту тему 😁)</p>
<p>У вас есть три потока данных:</p>
<ul>
<li>температура (Temperature)</li>
<li>влажность (Humidity)</li>
<li>статус датчика движения (Motion)</li>
</ul>
<p>Эти потоки поступают от разных сенсоров:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">temperatureStream</span> <span class="o">=</span> <span class="n">getTemperatureStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">humidityStream</span> <span class="o">=</span> <span class="n">getHumidityStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">motionStream</span> <span class="o">=</span> <span class="n">getMotionStream</span><span class="p">();</span></span></span></code></pre></div></div>
<p>На первый взгляд, вы можете подумать, что объединить эти потоки довольно просто. Используем класс <a href="https://api.flutter.dev/flutter/async/StreamGroup-class.html"  target="_blank" rel="noreferrer">StreamGroup</a>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">combinedStream</span> <span class="o">=</span> <span class="n">StreamGroup</span><span class="p">.</span><span class="n">merge</span><span class="p">([</span><span class="n">temperatureStream</span><span class="p">,</span> <span class="n">humidityStream</span><span class="p">,</span> <span class="n">motionStream</span><span class="p">]);</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">combinedStream</span><span class="p">.</span><span class="n">listen</span><span class="p">((</span><span class="n">event</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="n">print</span><span class="p">(</span><span class="n">event</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">});</span></span></span></code></pre></div></div>
<p>И действительно, это работает. Но вот проблема: когда данные начинают поступать из этих объединенных потоков, вы видите что-то вроде этого:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="o">&gt;</span> <span class="m">25</span>
</span></span><span class="line"><span class="cl"><span class="o">&gt;</span> <span class="m">60</span>
</span></span><span class="line"><span class="cl"><span class="o">&gt;</span> <span class="m">1</span></span></span></code></pre></div></div>

<h2 class="relative group">Решение
    <div id="решение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d1%88%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Чтобы решить эту проблему, предлагаю метод, который позволит не только объединить потоки, но и сохранить информацию об их источнике. Мы создадим два вспомогательных класса и метод для объединения потоков. <a href="https://medium.com/@punnyarthabanerjee/handling-multiple-streams-better-than-streamgroup-class-in-flutter-bb11e7982de2"  target="_blank" rel="noreferrer">Увидел в статье Punnyarthabanerjee</a> подобный подход и решил адаптировать для себя</p>
<p>Шаг 1: Класс StreamIdentity</p>
<p>Этот класс будет хранить идентификатор потока и сам поток:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">StreamIdentity</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">String</span> <span class="n">identifier</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">Stream</span> <span class="n">stream</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">StreamIdentity</span><span class="p">(</span><span class="k">this</span><span class="p">.</span><span class="n">identifier</span><span class="p">,</span> <span class="k">this</span><span class="p">.</span><span class="n">stream</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Шаг 2: Класс MergedStreamResponse</p>
<p>Этот класс будет содержать данные <code>data</code> и информацию о том, из какого потока они пришли <code>StreamIdentity</code>, а также временную метку <code>timestamp</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">MergedStreamResponse</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">StreamIdentity</span> <span class="n">streamIdentity</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">dynamic</span> <span class="n">data</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">DateTime</span> <span class="n">timestamp</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">MergedStreamResponse</span><span class="p">(</span><span class="k">this</span><span class="p">.</span><span class="n">streamIdentity</span><span class="p">,</span> <span class="k">this</span><span class="p">.</span><span class="n">data</span><span class="p">)</span> <span class="o">:</span> <span class="n">timestamp</span> <span class="o">=</span> <span class="n">DateTime</span><span class="p">.</span><span class="n">now</span><span class="p">();</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Шаг 3: Метод mergeStream</p>
<p>Этот метод объединит все потоки и будет добавлять данные в контроллер вместе с их идентификатором:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="n">MergedStreamResponse</span><span class="o">&gt;</span> <span class="n">mergeStream</span><span class="p">(</span><span class="n">List</span><span class="o">&lt;</span><span class="n">StreamIdentity</span><span class="o">&gt;</span> <span class="n">streams</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">StreamController</span><span class="o">&lt;</span><span class="n">MergedStreamResponse</span><span class="o">&gt;</span> <span class="n">controller</span> <span class="o">=</span> <span class="n">StreamController</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="k">for</span> <span class="p">(</span><span class="kd">var</span> <span class="n">identity</span> <span class="k">in</span> <span class="n">streams</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">identity</span><span class="p">.</span><span class="n">stream</span><span class="p">.</span><span class="n">listen</span><span class="p">((</span><span class="n">event</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">      <span class="n">controller</span><span class="p">.</span><span class="n">add</span><span class="p">(</span><span class="n">MergedStreamResponse</span><span class="p">(</span><span class="n">identity</span><span class="p">,</span> <span class="n">event</span><span class="p">));</span>
</span></span><span class="line"><span class="cl">    <span class="p">});</span>
</span></span><span class="line"><span class="cl">  <span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="k">return</span> <span class="n">controller</span><span class="p">.</span><span class="n">stream</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h2 class="relative group">Применение
    <div id="применение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Теперь мы можем передать наши потоки в метод <code>mergeStream</code> и обрабатывать данные с учетом их источника:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="kt">void</span> <span class="n">main</span><span class="p">()</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">temperatureStream</span> <span class="o">=</span> <span class="n">getTemperatureStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">humidityStream</span> <span class="o">=</span> <span class="n">getHumidityStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">motionStream</span> <span class="o">=</span> <span class="n">getMotionStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">List</span><span class="o">&lt;</span><span class="n">StreamIdentity</span><span class="o">&gt;</span> <span class="n">streams</span> <span class="o">=</span> <span class="p">[</span>
</span></span><span class="line"><span class="cl">    <span class="n">StreamIdentity</span><span class="p">(</span><span class="s1">&#39;temperature&#39;</span><span class="p">,</span> <span class="n">temperatureStream</span><span class="p">),</span>
</span></span><span class="line"><span class="cl">    <span class="n">StreamIdentity</span><span class="p">(</span><span class="s1">&#39;humidity&#39;</span><span class="p">,</span> <span class="n">humidityStream</span><span class="p">),</span>
</span></span><span class="line"><span class="cl">    <span class="n">StreamIdentity</span><span class="p">(</span><span class="s1">&#39;motion&#39;</span><span class="p">,</span> <span class="n">motionStream</span><span class="p">),</span>
</span></span><span class="line"><span class="cl">  <span class="p">];</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="n">MergedStreamResponse</span><span class="o">&gt;</span> <span class="n">combinedStream</span> <span class="o">=</span> <span class="n">mergeStream</span><span class="p">(</span><span class="n">streams</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">combinedStream</span><span class="p">.</span><span class="n">listen</span><span class="p">((</span><span class="n">response</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">print</span><span class="p">(</span><span class="s1">&#39;Data from </span><span class="si">${</span><span class="n">response</span><span class="p">.</span><span class="n">streamIdentity</span><span class="p">.</span><span class="n">identifier</span><span class="si">}</span><span class="s1"> at </span><span class="si">${</span><span class="n">response</span><span class="p">.</span><span class="n">timestamp</span><span class="si">}</span><span class="s1">: </span><span class="si">${</span><span class="n">response</span><span class="p">.</span><span class="n">data</span><span class="si">}</span><span class="s1">&#39;</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">  <span class="p">});</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь данные выводятся с указанием их источника и времени получения:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">Data from temperature at 2024-05-10 12:00:00: <span class="m">25</span>
</span></span><span class="line"><span class="cl">Data from humidity at 2024-05-10 12:00:01: <span class="m">60</span>
</span></span><span class="line"><span class="cl">Data from motion at 2024-05-10 12:00:02: <span class="m">1</span></span></span></code></pre></div></div>

<h2 class="relative group">Пример реализации (полный код)
    <div id="пример-реализации-полный-код" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8-%d0%bf%d0%be%d0%bb%d0%bd%d1%8b%d0%b9-%d0%ba%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">StreamIdentity</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">String</span> <span class="n">identifier</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">Stream</span> <span class="n">stream</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">StreamIdentity</span><span class="p">(</span><span class="k">this</span><span class="p">.</span><span class="n">identifier</span><span class="p">,</span> <span class="k">this</span><span class="p">.</span><span class="n">stream</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">MergedStreamResponse</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">StreamIdentity</span> <span class="n">streamIdentity</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">dynamic</span> <span class="n">data</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">DateTime</span> <span class="n">timestamp</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">MergedStreamResponse</span><span class="p">(</span><span class="k">this</span><span class="p">.</span><span class="n">streamIdentity</span><span class="p">,</span> <span class="k">this</span><span class="p">.</span><span class="n">data</span><span class="p">)</span> <span class="o">:</span> <span class="n">timestamp</span> <span class="o">=</span> <span class="n">DateTime</span><span class="p">.</span><span class="n">now</span><span class="p">();</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="n">MergedStreamResponse</span><span class="o">&gt;</span> <span class="n">mergeStream</span><span class="p">(</span><span class="n">List</span><span class="o">&lt;</span><span class="n">StreamIdentity</span><span class="o">&gt;</span> <span class="n">streams</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">StreamController</span><span class="o">&lt;</span><span class="n">MergedStreamResponse</span><span class="o">&gt;</span> <span class="n">controller</span> <span class="o">=</span> <span class="n">StreamController</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="k">for</span> <span class="p">(</span><span class="kd">var</span> <span class="n">identity</span> <span class="k">in</span> <span class="n">streams</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">identity</span><span class="p">.</span><span class="n">stream</span><span class="p">.</span><span class="n">listen</span><span class="p">((</span><span class="n">event</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">      <span class="n">controller</span><span class="p">.</span><span class="n">add</span><span class="p">(</span><span class="n">MergedStreamResponse</span><span class="p">(</span><span class="n">identity</span><span class="p">,</span> <span class="n">event</span><span class="p">));</span>
</span></span><span class="line"><span class="cl">    <span class="p">});</span>
</span></span><span class="line"><span class="cl">  <span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="k">return</span> <span class="n">controller</span><span class="p">.</span><span class="n">stream</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="kt">void</span> <span class="n">main</span><span class="p">()</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="kt">double</span><span class="o">&gt;</span> <span class="n">temperatureSensor</span> <span class="o">=</span> <span class="n">getTemperatureStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="kt">double</span><span class="o">&gt;</span> <span class="n">humiditySensor</span> <span class="o">=</span> <span class="n">getHumidityStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">motionSensor</span> <span class="o">=</span> <span class="n">getMotionStream</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">List</span><span class="o">&lt;</span><span class="n">StreamIdentity</span><span class="o">&gt;</span> <span class="n">sensors</span> <span class="o">=</span> <span class="p">[</span>
</span></span><span class="line"><span class="cl">    <span class="n">StreamIdentity</span><span class="p">(</span><span class="s1">&#39;temperature&#39;</span><span class="p">,</span> <span class="n">temperatureSensor</span><span class="p">),</span>
</span></span><span class="line"><span class="cl">    <span class="n">StreamIdentity</span><span class="p">(</span><span class="s1">&#39;humidity&#39;</span><span class="p">,</span> <span class="n">humiditySensor</span><span class="p">),</span>
</span></span><span class="line"><span class="cl">    <span class="n">StreamIdentity</span><span class="p">(</span><span class="s1">&#39;motion&#39;</span><span class="p">,</span> <span class="n">motionSensor</span><span class="p">),</span>
</span></span><span class="line"><span class="cl">  <span class="p">];</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">Stream</span><span class="o">&lt;</span><span class="n">MergedStreamResponse</span><span class="o">&gt;</span> <span class="n">combinedStream</span> <span class="o">=</span> <span class="n">mergeStream</span><span class="p">(</span><span class="n">sensors</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">combinedStream</span><span class="p">.</span><span class="n">listen</span><span class="p">((</span><span class="n">response</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">print</span><span class="p">(</span><span class="s1">&#39;Data from </span><span class="si">${</span><span class="n">response</span><span class="p">.</span><span class="n">streamIdentity</span><span class="p">.</span><span class="n">identifier</span><span class="si">}</span><span class="s1"> at </span><span class="si">${</span><span class="n">response</span><span class="p">.</span><span class="n">timestamp</span><span class="si">}</span><span class="s1">: </span><span class="si">${</span><span class="n">response</span><span class="p">.</span><span class="n">data</span><span class="si">}</span><span class="s1">&#39;</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">  <span class="p">});</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">double</span><span class="o">&gt;</span> <span class="n">getTemperatureStream</span><span class="p">()</span> <span class="kd">async</span><span class="o">*</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">yield</span><span class="o">*</span> <span class="n">Stream</span><span class="p">.</span><span class="n">periodic</span><span class="p">(</span><span class="n">Duration</span><span class="p">(</span><span class="nl">seconds:</span> <span class="m">1</span><span class="p">),</span> <span class="p">(</span><span class="n">count</span><span class="p">)</span> <span class="o">=&gt;</span> <span class="m">20.0</span> <span class="o">+</span> <span class="n">count</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">double</span><span class="o">&gt;</span> <span class="n">getHumidityStream</span><span class="p">()</span> <span class="kd">async</span><span class="o">*</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">yield</span><span class="o">*</span> <span class="n">Stream</span><span class="p">.</span><span class="n">periodic</span><span class="p">(</span><span class="n">Duration</span><span class="p">(</span><span class="nl">seconds:</span> <span class="m">2</span><span class="p">),</span> <span class="p">(</span><span class="n">count</span><span class="p">)</span> <span class="o">=&gt;</span> <span class="m">50.0</span> <span class="o">+</span> <span class="n">count</span> <span class="o">*</span> <span class="m">0.5</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">Stream</span><span class="o">&lt;</span><span class="kt">int</span><span class="o">&gt;</span> <span class="n">getMotionStream</span><span class="p">()</span> <span class="kd">async</span><span class="o">*</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">yield</span><span class="o">*</span> <span class="n">Stream</span><span class="p">.</span><span class="n">periodic</span><span class="p">(</span><span class="n">Duration</span><span class="p">(</span><span class="nl">seconds:</span> <span class="m">3</span><span class="p">),</span> <span class="p">(</span><span class="n">count</span><span class="p">)</span> <span class="o">=&gt;</span> <span class="n">count</span> <span class="o">%</span> <span class="m">2</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>В этом примере данные от датчика температуры, влажностности и датчика движения объединяются и выводятся с указанием их источника и времени получения. Это позволяет легко отслеживать и анализировать данные от каждого сенсора, что значительно упрощает принятие решений.</p>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Надеюсь, эта статья была полезной и вы узнали что-то новое о работе с потоками в Dart. Если у вас есть вопросы или комментарии, обращайтесь. Стараюсь писать кратко и по делу. Спасибо за внимание и спасибо <a href="https://medium.com/@punnyarthabanerjee/handling-multiple-streams-better-than-streamgroup-class-in-flutter-bb11e7982de2"  target="_blank" rel="noreferrer">автору статьи на медиум</a> за оригинальную идею!</p>
<hr>
### 📝 Тест по асинхронным потокам и StreamGroup в Dart

**1. Какой класс из пакета async позволяет объединять несколько Stream в единый поток?**

- [ ] StreamController
- [x] StreamGroup
- [ ] FutureGroup
- [ ] StreamZip

> 💡 *StreamGroup позволяет добавлять несколько потоков (StreamGroup.add) и читать их как один общий поток.*


**2. Чем отличаются одиночные потоки (Single-subscription) от Broadcast Stream в Dart?**

- [x] Single-subscription допускает только одного слушателя, а Broadcast — нескольких
- [ ] Single-subscription работает только асинхронно
- [ ] Broadcast Stream нельзя отменить
- [ ] Ничем не отличаются

> 💡 *Single-subscription Stream допускает только одного слушателя, повторная подписка вызовет StateError.*




