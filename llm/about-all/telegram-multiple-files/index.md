# Как отправить несколько файлов одним сообщением через Telegram Bot API

> Пошаговый гайд по отправке медиагруппы из нескольких файлов, фотографий и документов одним альбомом или сообщением через Telegram Bot API с примерами кода.

- **URL:** https://eagle2000.space/about-all/telegram-multiple-files/
- **Date:** 2025-02-08
- **Categories:** about all


---

<p>В разработке приложений взаимодействующих с telegram bot api часто возникает необходимость отправки нескольких файлов одновременно.
(Я использую этот механизм для отладки иногда)</p>
<p>Telegram API предоставляет для этого специальный метод <code>sendMediaGroup</code>, который позволяет отправлять до 10 файлов в одном сообщении. В этой статье разберемся, как использовать этот метод с помощью различных инструментов.</p>

<h2 class="relative group">Преимущества использования sendMediaGroup
    <div id="преимущества-использования-sendmediagroup" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b5%d0%b8%d0%bc%d1%83%d1%89%d0%b5%d1%81%d1%82%d0%b2%d0%b0-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f-sendmediagroup" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li>Все файлы группируются в одно сообщение</li>
<li>Уменьшается количество запросов к API</li>
<li>Возможность добавления подписей к каждому файлу</li>
<li>Поддержка различных типов медиа (фото, документы, видео)</li>
</ul>

<h2 class="relative group">Подготовка тестовых файлов
    <div id="подготовка-тестовых-файлов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%b4%d0%b3%d0%be%d1%82%d0%be%d0%b2%d0%ba%d0%b0-%d1%82%d0%b5%d1%81%d1%82%d0%be%d0%b2%d1%8b%d1%85-%d1%84%d0%b0%d0%b9%d0%bb%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Для демонстрации работы метода создадим два тестовых файла:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl"><span class="nb">echo</span> <span class="s2">&#34;Test content 1&#34;</span> &gt; test1.txt
</span></span><span class="line"><span class="cl"><span class="nb">echo</span> <span class="s2">&#34;Test content 2&#34;</span> &gt; test2.txt</span></span></code></pre></div></div>

<h2 class="relative group">Отправка через cURL
    <div id="отправка-через-curl" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%82%d0%bf%d1%80%d0%b0%d0%b2%d0%ba%d0%b0-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-curl" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Самый простой способ протестировать API – использовать cURL. Вот полный пример команды:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">curl -X POST https://api.telegram.org/bot&lt;ТВОЙ_TOKEN&gt;/sendMediaGroup <span class="se">\
</span></span></span><span class="line"><span class="cl">-F <span class="nv">chat_id</span><span class="o">=</span><span class="s2">&#34;ТВОЙ_CHAT_ID&#34;</span> <span class="se">\
</span></span></span><span class="line"><span class="cl">-F <span class="s2">&#34;media=[{\&#34;type\&#34;:\&#34;document\&#34;,\&#34;media\&#34;:\&#34;attach://doc1\&#34;,\&#34;caption\&#34;:\&#34;Test files 1\&#34;,\&#34;parse_mode\&#34;:\&#34;HTML\&#34;},{\&#34;type\&#34;:\&#34;document\&#34;,\&#34;media\&#34;:\&#34;attach://doc2\&#34;,\&#34;caption\&#34;:\&#34;Test files 2\&#34;}]&#34;</span> <span class="se">\
</span></span></span><span class="line"><span class="cl">-F <span class="nv">doc1</span><span class="o">=</span>@<span class="s2">&#34;test1.txt&#34;</span> <span class="se">\
</span></span></span><span class="line"><span class="cl">-F <span class="nv">doc2</span><span class="o">=</span>@<span class="s2">&#34;test2.txt&#34;</span></span></span></code></pre></div></div>
<p>Разберем параметры запроса:</p>
<ul>
<li><code>-X POST</code> - указывает метод запроса</li>
<li><code>-F</code> - добавляет form-data поле</li>
<li><code>media</code> - JSON-массив с описанием медиафайлов</li>
<li><code>attach://doc1</code> - указывает на привязку к form-data полю doc1</li>
</ul>

<h2 class="relative group">Реализация на Dart
    <div id="реализация-на-dart" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d0%bd%d0%b0-dart" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Для тех, кто разрабатывает на Dart/Flutter, вот пример реализации той же функциональности:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:http/http.dart&#39;</span> <span class="k">as</span> <span class="n">http</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:http_parser/http_parser.dart&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">Future</span><span class="o">&lt;</span><span class="kt">void</span><span class="o">&gt;</span> <span class="n">sendMediaGroup</span><span class="p">()</span> <span class="kd">async</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">token</span> <span class="o">=</span> <span class="s1">&#39;ТВОЙ_TOKEN&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">chatId</span> <span class="o">=</span> <span class="s1">&#39;ТВОЙ_CHAT_ID&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">url</span> <span class="o">=</span> <span class="n">Uri</span><span class="p">.</span><span class="n">parse</span><span class="p">(</span><span class="s1">&#39;https://api.telegram.org/bot</span><span class="si">$</span><span class="n">token</span><span class="s1">/sendMediaGroup&#39;</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">request</span> <span class="o">=</span> <span class="n">http</span><span class="p">.</span><span class="n">MultipartRequest</span><span class="p">(</span><span class="s1">&#39;POST&#39;</span><span class="p">,</span> <span class="n">url</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">    <span class="p">..</span><span class="n">fields</span><span class="p">[</span><span class="s1">&#39;chat_id&#39;</span><span class="p">]</span> <span class="o">=</span> <span class="n">chatId</span>
</span></span><span class="line"><span class="cl">    <span class="p">..</span><span class="n">fields</span><span class="p">[</span><span class="s1">&#39;media&#39;</span><span class="p">]</span> <span class="o">=</span> <span class="s1">&#39;&#39;&#39;
</span></span></span><span class="line"><span class="cl"><span class="s1">    [
</span></span></span><span class="line"><span class="cl"><span class="s1">      {&#34;type&#34;: &#34;document&#34;, &#34;media&#34;: &#34;attach://doc1&#34;, &#34;caption&#34;: &#34;Test files 1&#34;},
</span></span></span><span class="line"><span class="cl"><span class="s1">      {&#34;type&#34;: &#34;document&#34;, &#34;media&#34;: &#34;attach://doc2&#34;, &#34;caption&#34;: &#34;Test files 2&#34;}
</span></span></span><span class="line"><span class="cl"><span class="s1">    ]
</span></span></span><span class="line"><span class="cl"><span class="s1">    &#39;&#39;&#39;</span>
</span></span><span class="line"><span class="cl">    <span class="p">..</span><span class="n">files</span><span class="p">.</span><span class="n">add</span><span class="p">(</span><span class="kd">await</span> <span class="n">http</span><span class="p">.</span><span class="n">MultipartFile</span><span class="p">.</span><span class="n">fromPath</span><span class="p">(</span><span class="s1">&#39;doc1&#39;</span><span class="p">,</span> <span class="s1">&#39;test1.txt&#39;</span><span class="p">))</span>
</span></span><span class="line"><span class="cl">    <span class="p">..</span><span class="n">files</span><span class="p">.</span><span class="n">add</span><span class="p">(</span><span class="kd">await</span> <span class="n">http</span><span class="p">.</span><span class="n">MultipartFile</span><span class="p">.</span><span class="n">fromPath</span><span class="p">(</span><span class="s1">&#39;doc2&#39;</span><span class="p">,</span> <span class="s1">&#39;test2.txt&#39;</span><span class="p">));</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="n">response</span> <span class="o">=</span> <span class="kd">await</span> <span class="n">request</span><span class="p">.</span><span class="n">send</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">  <span class="n">print</span><span class="p">(</span><span class="kd">await</span> <span class="n">response</span><span class="p">.</span><span class="n">stream</span><span class="p">.</span><span class="n">bytesToString</span><span class="p">());</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h2 class="relative group">Важные замечания
    <div id="важные-замечания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b0%d0%b6%d0%bd%d1%8b%d0%b5-%d0%b7%d0%b0%d0%bc%d0%b5%d1%87%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ol>
<li>Максимальное количество файлов в одной группе - 10</li>
<li>Все файлы должны быть одного типа (например, все документы или все фотографии)</li>
<li>Размер каждого файла не должен превышать лимиты Telegram</li>
<li>Поддерживаются следующие типы медиа: photo, video, audio, document</li>
</ol>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Метод <code>sendMediaGroup</code> - мощный инструмент для отправки нескольких файлов в одном сообщении. Он особенно полезен при разработке ботов, где требуется отправка связанных файлов или создание альбомов медиа-контента.</p>
<p>Спасибо что читаете! 👾</p>

<h3 class="relative group">Полезные ссылки
    <div id="полезные-ссылки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%bb%d0%b5%d0%b7%d0%bd%d1%8b%d0%b5-%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><a href="https://core.telegram.org/bots/api#sendmediagroup"  target="_blank" rel="noreferrer">Официальная документация Telegram Bot API</a></li>
<li><a href="https://core.telegram.org/bots/api#senddocument"  target="_blank" rel="noreferrer">Limits in Telegram Bot API</a></li>
<li><a href="https://core.telegram.org/bots/tutorial"  target="_blank" rel="noreferrer">Документация как создать и работать с чат-ботом Telegram</a></li>
</ul>

