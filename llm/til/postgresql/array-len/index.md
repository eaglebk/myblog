# Как посчитать количество элементов в массиве в PostgreSQL: cardinality и array_length

> Разбираем способы подсчета элементов в массиве PostgreSQL. Сравнение функций cardinality() и array_length(), работа с многомерными массивами и обработка NULL.

- **URL:** https://eagle2000.space/til/postgresql/array-len/
- **Date:** 2025-02-14

- **Tags:** PostgreSQL

---


<h3 class="relative group">Вступление
    <div id="вступление" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%81%d1%82%d1%83%d0%bf%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Работа с массивами в PostgreSQL — это мощный инструмент, который позволяет хранить и обрабатывать наборы данных прямо в базе. Но как правильно посчитать количество элементов в массиве? Разберём два основных способа, которые помогут вам справиться с этой задачей.</p>
<hr>

<h4 class="relative group">1️⃣ <strong>Функция <code>array_length</code></strong>
    <div id="1-функция" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%84%d1%83%d0%bd%d0%ba%d1%86%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Функция <code>array_length</code> позволяет узнать длину массива по определённой размерности. Это особенно полезно, если вы работаете с многомерными массивами.</p>
<p><strong>Пример использования:</strong></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sql" data-lang="sql"><span class="line"><span class="cl"><span class="k">select</span><span class="w"> </span><span class="n">array_length</span><span class="p">(</span><span class="nb">array</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">],</span><span class="w"> </span><span class="mi">1</span><span class="p">);</span></span></span></code></pre></div></div>
<p><strong>Результат:</strong><br>
<code>3</code></p>
<p>Здесь мы передаём массив <code>[1, 2, 3]</code> и указываем размерность <code>1</code> (поскольку массив одномерный). Функция возвращает количество элементов в массиве.</p>
<p><strong>Для многомерных массивов:</strong></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sql" data-lang="sql"><span class="line"><span class="cl"><span class="k">select</span><span class="w"> </span><span class="n">array_length</span><span class="p">(</span><span class="nb">array</span><span class="p">[[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">],</span><span class="w"> </span><span class="p">[</span><span class="mi">3</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">]],</span><span class="w"> </span><span class="mi">2</span><span class="p">);</span></span></span></code></pre></div></div>
<p><strong>Результат:</strong><br>
<code>2</code></p>
<p>В этом примере мы работаем с двумерным массивом. Указываем размерность <code>2</code>, чтобы узнать количество элементов во втором измерении (количество столбцов).</p>
<hr>

<h4 class="relative group">2️⃣ <strong>Функция <code>cardinality</code></strong>
    <div id="2-функция" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%84%d1%83%d0%bd%d0%ba%d1%86%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Если вам нужно посчитать общее количество элементов в массиве, независимо от его размерности, используйте функцию <code>cardinality</code>. Она подходит как для одномерных, так и для многомерных массивов.</p>
<p><strong>Пример использования:</strong></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sql" data-lang="sql"><span class="line"><span class="cl"><span class="k">select</span><span class="w"> </span><span class="k">cardinality</span><span class="p">(</span><span class="nb">array</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">]);</span></span></span></code></pre></div></div>
<p><strong>Результат:</strong><br>
<code>3</code></p>
<p>Здесь функция возвращает общее количество элементов в массиве.</p>
<p><strong>Для многомерных массивов:</strong></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sql" data-lang="sql"><span class="line"><span class="cl"><span class="k">select</span><span class="w"> </span><span class="k">cardinality</span><span class="p">(</span><span class="nb">array</span><span class="p">[[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">],</span><span class="w"> </span><span class="p">[</span><span class="mi">3</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">]]);</span></span></span></code></pre></div></div>
<p><strong>Результат:</strong><br>
<code>4</code></p>
<p>В этом случае <code>cardinality</code> считает все элементы массива, независимо от их вложенности.</p>
<hr>

<h3 class="relative group">Какой метод выбрать? 🤔
    <div id="какой-метод-выбрать-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba%d0%be%d0%b9-%d0%bc%d0%b5%d1%82%d0%be%d0%b4-%d0%b2%d1%8b%d0%b1%d1%80%d0%b0%d1%82%d1%8c-" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><strong><code>array_length</code></strong> — используйте, если вам нужно узнать длину массива по определённой размерности. Это особенно полезно для многомерных массивов.</li>
<li><strong><code>cardinality</code></strong> — подходит, если вам нужно общее количество элементов в массиве, независимо от его структуры.</li>
</ul>
<hr>

<h3 class="relative group">Полезные ссылки 🔗
    <div id="полезные-ссылки-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%bb%d0%b5%d0%b7%d0%bd%d1%8b%d0%b5-%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b8-" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>📚 Официальная документация PostgreSQL по работе с массивами: <a href="https://www.postgresql.org/docs/current/functions-array.html"  target="_blank" rel="noreferrer">https://www.postgresql.org/docs/current/functions-array.html</a></li>
<li>🧩 Практика без установки PostgreSQL: <a href="https://pgplayground.com/"  target="_blank" rel="noreferrer">https://pgplayground.com/</a></li>
</ul>

