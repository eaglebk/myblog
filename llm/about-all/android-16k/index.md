# Android 15 и 16 KB страница памяти: что изменится, как подготовить Android и Flutter-приложения

> Android 15 ввел поддержку 16KB memory pages. Разбираем, что это значит для нативных и Flutter-приложений, как избежать ошибок Play Console и что обновить.

- **URL:** https://eagle2000.space/about-all/android-16k/
- **Date:** 2025-09-25

- **Tags:** android, flutter, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>С выходом <strong>Android 15</strong> Google изменил фундаментальный принцип управления памятью: вместо фиксированных <strong>4 KB страниц</strong> операционная система теперь поддерживает <strong>16 KB memory pages</strong>.</p>
<p>Этот переход улучшает производительность и экономию энергии, но в то же время требует пересборки приложений, особенно тех, которые используют нативный код (NDK, C/C++, Flutter engine или плагины).</p>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M506.3 417l-213.3-364c-16.33-28-57.54-28-73.98 0l-213.2 364C-10.59 444.9 9.849 480 42.74 480h426.6C502.1 480 522.6 445 506.3 417zM232 168c0-13.25 10.75-24 24-24S280 154.8 280 168v128c0 13.25-10.75 24-23.1 24S232 309.3 232 296V168zM256 416c-17.36 0-31.44-14.08-31.44-31.44c0-17.36 14.07-31.44 31.44-31.44s31.44 14.08 31.44 31.44C287.4 401.9 273.4 416 256 416z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><strong>С 1 ноября 2025 года Google Play Console будет отклонять приложения без поддержки 16 KB страниц.</strong></span>
</div>

<p>Если хотите сразу перейти к описанию как обновить flutter проект - переходите к части: <a href="/about-all/android-16k/#3-flutter-%d1%87%d0%b0%d1%81%d1%82%d1%8c-%d1%87%d1%82%d0%be-%d0%bd%d1%83%d0%b6%d0%bd%d0%be-%d1%81%d0%b4%d0%b5%d0%bb%d0%b0%d1%82%d1%8c-flutter-%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%87%d0%b8%d0%ba%d0%b0%d0%bc" >Flutter-часть: что нужно сделать Flutter-разработчикам</a></p>

<h2 class="relative group">1. Ключевые термины: MMU, Memory Page, Page Fault, Page Table, TLB
    <div id="1-ключевые-термины-mmu-memory-page-page-fault-page-table-tlb" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%b2%d1%8b%d0%b5-%d1%82%d0%b5%d1%80%d0%bc%d0%b8%d0%bd%d1%8b-mmu-memory-page-page-fault-page-table-tlb" aria-label="Якорь">#</a>
    </span>
    
</h2>
<blockquote><p>Постараюсь привести объяснение терминов которые встретятся далее в статьею Чтобы легче было запоминать - приведу аналогию с городской библиотекой. Если вам не интересно читать описание терминов - можете переходить к следующей части: <a href="/about-all/android-16k/#2-%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-google-%d0%b2%d0%b2%d0%be%d0%b4%d0%b8%d1%82-16-kb-memory-pages" >Почему Google вводит 16 KB memory pages.</a></p>
</blockquote>
<h3 class="relative group">MMU (Memory Management Unit)
    <div id="mmu-memory-management-unit" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#mmu-memory-management-unit" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Аппаратный блок процессора, который сопоставляет виртуальные адреса с физической памятью и управляет правами доступа.<br>
📖 <strong>Аналогия:</strong> библиотекарь, который по запросу читателя ищет книгу в каталоге и определяет её местонахождение на полке.</p>
<hr>

<h3 class="relative group">Страница памяти (Memory Page)
    <div id="страница-памяти-memory-page" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%82%d1%80%d0%b0%d0%bd%d0%b8%d1%86%d0%b0-%d0%bf%d0%b0%d0%bc%d1%8f%d1%82%d0%b8-memory-page" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Минимальная единица управления памятью.</p>
<ul>
<li>До Android 15 стандарт → <strong>4 KB</strong>.</li>
<li>Теперь возможны → <strong>16 KB</strong>.</li>
</ul>
<p>📖 <strong>Аналогия:</strong> единообразие размеров книг библиотеке - одинаковый размер для упрощённого хранения книг.</p>
<hr>

<h3 class="relative group">Page Fault
    <div id="page-fault" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#page-fault" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Ошибка доступа к странице, когда её ещё нет в памяти. Ядро решает: подгрузить, выделить новую или запретить доступ.</p>
<p>📖 <strong>Аналогия:</strong> читатель просит книгу, библиотекарь идёт в архив потому, что на полке её пока нет.</p>
<hr>

<h3 class="relative group">Таблица страниц (Page Table)
    <div id="таблица-страниц-page-table" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b0%d0%b1%d0%bb%d0%b8%d1%86%d0%b0-%d1%81%d1%82%d1%80%d0%b0%d0%bd%d0%b8%d1%86-page-table" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Каталог соответствий виртуальных и физических страниц.</p>
<p>📖 <strong>Аналогия:</strong> карточный каталог библиотеки, где указано расположение каждой книги.</p>
<hr>

<h3 class="relative group">TLB (Translation Lookaside Buffer)
    <div id="tlb-translation-lookaside-buffer" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#tlb-translation-lookaside-buffer" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Быстрый кэш в процессоре для преобразований адресов.</p>
<p>📖 <strong>Аналогия:</strong> закладки для популярных книг - библиотекарь сразу знает, где они, без поиска в каталоге.</p>
<hr>

<h2 class="relative group">2. Почему Google вводит 16 KB memory pages
    <div id="2-почему-google-вводит-16-kb-memory-pages" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-google-%d0%b2%d0%b2%d0%be%d0%b4%d0%b8%d1%82-16-kb-memory-pages" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">История
    <div id="история" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%81%d1%82%d0%be%d1%80%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Исторически Android поддерживал только <strong>4 KB страницы</strong>. Это было оптимально для устройств с небольшим объёмом RAM.</p>
<p>Но современные устройства имеют десятки гигабайт памяти и многоядерные процессоры. В этих условиях миллионы маленьких страниц стали создавать избыточные накладные расходы:</p>
<ul>
<li>больше записей в <a href="/about-all/android-16k/#%d1%82%d0%b0%d0%b1%d0%bb%d0%b8%d1%86%d0%b0-%d1%81%d1%82%d1%80%d0%b0%d0%bd%d0%b8%d1%86-page-table" >таблицах страниц</a>;</li>
<li>больше промахов в <a href="/about-all/android-16k/#tlb-translation-lookaside-buffer" >TLB</a>;</li>
<li>больше обходов <a href="/about-all/android-16k/#%d1%82%d0%b0%d0%b1%d0%bb%d0%b8%d1%86%d0%b0-%d1%81%d1%82%d1%80%d0%b0%d0%bd%d0%b8%d1%86-page-table" >таблиц страниц</a>.</li>
</ul>
<hr>

<h3 class="relative group">Что изменилось в Android 15
    <div id="что-изменилось-в-android-15" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d0%b8%d0%bb%d0%be%d1%81%d1%8c-%d0%b2-android-15" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Android 15 впервые сделал платформу <strong>страница-независимой</strong> и добавил поддержку <strong>16 KB страниц</strong>.</p>
<p>Зачем это было нужно:</p>
<ul>
<li>меньше записей в таблицах страниц;</li>
<li>меньше TLB-промахов;</li>
<li>меньше обходов таблиц;</li>
<li>лучше масштабируемость под устройства с большим объёмом RAM.</li>
</ul>
<p><strong>Пример:</strong> чтобы отобразить 64 MB памяти</p>
<ul>
<li>при 4 KB страницах → <strong>16 384 записи</strong></li>
<li>при 16 KB страницах → всего <strong>4 096 записей</strong>.</li>
</ul>
<hr>

<h3 class="relative group">Результаты тестов от Google
    <div id="результаты-тестов-от-google" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d0%b7%d1%83%d0%bb%d1%8c%d1%82%d0%b0%d1%82%d1%8b-%d1%82%d0%b5%d1%81%d1%82%d0%be%d0%b2-%d0%be%d1%82-google" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p>Запуск приложений быстрее на <strong>3,16%</strong> в среднем (до 30% для отдельных).</p>
</li>
<li>
<p>Энергопотребление при запуске ниже на <strong>4,56%</strong>.</p>
</li>
<li>
<p>Камера стартует быстрее: <strong>+4,48% (hot start)</strong> и <strong>+6,60% (cold start)</strong>.</p>
</li>
<li>
<p>Загрузка системы быстрее на <strong>8%</strong> (~950 мс).</p>
</li>
</ul>
<hr>

<h3 class="relative group">Какие минусы
    <div id="какие-минусы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba%d0%b8%d0%b5-%d0%bc%d0%b8%d0%bd%d1%83%d1%81%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p><strong>Внутренняя фрагментация:</strong> небольшой объект (например, 5 KB) занимает целую страницу 16 KB.</p>
</li>
<li>
<p>Но таблицы страниц уменьшаются в 4 раза, что компенсирует перерасход памяти.</p>
</li>
</ul>
<hr>
<ul>
<li>Подробнее от Google → <a href="https://developer.android.com/guide/practices/page-sizes"  target="_blank" rel="noreferrer">Google: Support 16 KB page sizes</a></li>
</ul>
<hr>

<h2 class="relative group">3. Flutter-часть: что нужно сделать Flutter-разработчикам
    <div id="3-flutter-часть-что-нужно-сделать-flutter-разработчикам" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-flutter-%d1%87%d0%b0%d1%81%d1%82%d1%8c-%d1%87%d1%82%d0%be-%d0%bd%d1%83%d0%b6%d0%bd%d0%be-%d1%81%d0%b4%d0%b5%d0%bb%d0%b0%d1%82%d1%8c-flutter-%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%87%d0%b8%d0%ba%d0%b0%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">❓ Почему это важно для разработки под Flutter
    <div id="-почему-это-важно-для-разработки-под-flutter" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-%d1%8d%d1%82%d0%be-%d0%b2%d0%b0%d0%b6%d0%bd%d0%be-%d0%b4%d0%bb%d1%8f-%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%ba%d0%b8-%d0%bf%d0%be%d0%b4-flutter" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Flutter-приложения почти всегда используют нативный код:</p>
<ul>
<li>сам <strong>Flutter engine</strong> написан на C/C++;</li>
<li>плагины (ads, analytics, camera, ML и др.) содержат <code>.so</code> библиотеки;</li>
<li>собственный нативный код через FFI или C/C++ тоже требует пересборки.</li>
</ul>
<p>Даже если у вас проект написан на чистом Dart, устаревший плагин может вызвать падение на устройствах с Android 15 (16 KB).</p>
<p>Поэтому вероятность несовместимости у Flutter-приложений <strong>очень высокая</strong>.</p>
<hr>

<h3 class="relative group">🛠 Как исправить проблему совместимости во Flutter
    <div id="-как-исправить-проблему-совместимости-во-flutter" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d0%ba%d0%b0%d0%ba-%d0%b8%d1%81%d0%bf%d1%80%d0%b0%d0%b2%d0%b8%d1%82%d1%8c-%d0%bf%d1%80%d0%be%d0%b1%d0%bb%d0%b5%d0%bc%d1%83-%d1%81%d0%be%d0%b2%d0%bc%d0%b5%d1%81%d1%82%d0%b8%d0%bc%d0%be%d1%81%d1%82%d0%b8-%d0%b2%d0%be-flutter" aria-label="Якорь">#</a>
    </span>
    
</h3>

<h4 class="relative group">1. Обновите инструменты
    <div id="1-обновите-инструменты" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%be%d0%b1%d0%bd%d0%be%d0%b2%d0%b8%d1%82%d0%b5-%d0%b8%d0%bd%d1%81%d1%82%d1%80%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>Flutter SDK → последняя стабильная (3.x)</li>
<li>NDK → r28+</li>
<li>Gradle → 8.14.3+</li>
<li>AGP → 8.6+ (лучше 8.9.1)</li>
<li>Target SDK → 36</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">flutter upgrade
</span></span><span class="line"><span class="cl">flutter pub upgrade</span></span></code></pre></div></div>
<hr>

<h4 class="relative group">2. Обновите зависимости
    <div id="2-обновите-зависимости" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%be%d0%b1%d0%bd%d0%be%d0%b2%d0%b8%d1%82%d0%b5-%d0%b7%d0%b0%d0%b2%d0%b8%d1%81%d0%b8%d0%bc%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>Проверьте все плагины в <code>pubspec.yaml</code>.</li>
<li>Обновите плагины с нативными <code>.so</code> (firebase, camera, ads, analytics, ML).</li>
</ul>
<hr>

<h4 class="relative group">3. Пересоберите проект
    <div id="3-пересоберите-проект" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%bf%d0%b5%d1%80%d0%b5%d1%81%d0%be%d0%b1%d0%b5%d1%80%d0%b8%d1%82%d0%b5-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Обновлённые Flutter SDK, AGP и NDK автоматически пересоберут нативные библиотеки с 16 KB alignment.</p>
<hr>

<h4 class="relative group">4. Тестирование
    <div id="4-тестирование" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%82%d0%b5%d1%81%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>
<p>Эмулятор Android 15 (Device Manager → <em>Kernel page size = 16 KB</em>).</p>
</li>
<li>
<p>Физическое устройство (Pixel) → включить “Use 16 KB memory pages” в Developer Options.</p>
</li>
<li>
<p>Проверка:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">adb shell getconf PAGE_SIZE</span></span></code></pre></div></div>
<p><code>16384</code> → устройство работает в 16 KB режиме.</p>
</li>
</ul>
<hr>

<h4 class="relative group">5. Проверьте AAB перед публикацией
    <div id="5-проверьте-aab-перед-публикацией" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c%d1%82%d0%b5-aab-%d0%bf%d0%b5%d1%80%d0%b5%d0%b4-%d0%bf%d1%83%d0%b1%d0%bb%d0%b8%d0%ba%d0%b0%d1%86%d0%b8%d0%b5%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h4>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">unzip -q build/app/outputs/bundle/release/app-release.aab -d extracted_aab
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">find extracted_aab/base/lib -name <span class="s2">&#34;*.so&#34;</span> -exec sh -c <span class="s1">&#39;
</span></span></span><span class="line"><span class="cl"><span class="s1">for f; do
</span></span></span><span class="line"><span class="cl"><span class="s1">if readelf -lW &#34;$f&#34; | grep -q &#34;0x4000&#34;; then
</span></span></span><span class="line"><span class="cl"><span class="s1">  echo &#34;✅ $f 16KB совместимо&#34;
</span></span></span><span class="line"><span class="cl"><span class="s1">else
</span></span></span><span class="line"><span class="cl"><span class="s1">  echo &#34;❌ $f только 4KB совместимо&#34;
</span></span></span><span class="line"><span class="cl"><span class="s1">fi
</span></span></span><span class="line"><span class="cl"><span class="s1">done
</span></span></span><span class="line"><span class="cl"><span class="s1">&#39;</span> sh <span class="o">{}</span> +</span></span></code></pre></div></div>
<ul>
<li>✅ Хорошо: <code>0x4000</code> (16 KB aligned)</li>
<li>❌ Плохо: <code>0x1000</code> (4 KB aligned) → обновите зависимость и пересоберите.</li>
</ul>
<hr>

<h3 class="relative group">Ссылка на Github Gist с настройками для поддержки проекта Android 16K
    <div id="ссылка-на-github-gist-с-настройками-для-поддержки-проекта-android-16k" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b0-%d0%bd%d0%b0-github-gist-%d1%81-%d0%bd%d0%b0%d1%81%d1%82%d1%80%d0%be%d0%b9%d0%ba%d0%b0%d0%bc%d0%b8-%d0%b4%d0%bb%d1%8f-%d0%bf%d0%be%d0%b4%d0%b4%d0%b5%d1%80%d0%b6%d0%ba%d0%b8-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b0-android-16k" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p><a href="https://gist.github.com/eaglebk/8868cc96cf11e4ce98921449ead78bfd"  target="_blank" rel="noreferrer">https://gist.github.com/eaglebk/8868cc96cf11e4ce98921449ead78bfd</a></p>
<hr>

<h3 class="relative group">✅ Чек-лист для Flutter-разработчиков
    <div id="-чек-лист-для-flutter-разработчиков" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d1%87%d0%b5%d0%ba-%d0%bb%d0%b8%d1%81%d1%82-%d0%b4%d0%bb%d1%8f-flutter-%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%87%d0%b8%d0%ba%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>Flutter обновлён (последний stable release)</li>
<li>NDK r28+</li>
<li>Gradle 8.14.3+</li>
<li>AGP 8.6+</li>
<li>Target SDK 36</li>
<li>Зависимости обновлены</li>
<li>Протестировано на 16 KB эмуляторе/устройстве</li>
<li>Проверено через скрипт с <code>readelf</code></li>
<li>AAB загружен в Play Console без предупреждений</li>
</ul>
<hr>

<h2 class="relative group">4. Итоги
    <div id="4-итоги" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d0%b8%d1%82%d0%be%d0%b3%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li>Android 15 меняет стандарт страниц памяти: с 4 KB → 16 KB.</li>
<li>Это ускоряет запуск приложений, камеры и систему, снижает энергопотребление.</li>
<li>Минус — фрагментация, но она компенсируется меньшими таблицами страниц.</li>
<li>Flutter-разработчики обязаны обновить SDK, NDK, AGP и все плагины.</li>
<li>Проверка AAB и тесты на 16 KB — ключ к успешной публикации в Play Console.</li>
</ul>

<h2 class="relative group">FAQ
    <div id="faq" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#faq" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><strong>Что будет, если не обновить приложение под 16 KB страницы в Android 15?</strong></p>
<ul>
<li>Приложение может упасть при запуске на новых устройствах, а с 1 ноября 2025 года Google Play Console отклонит публикацию.</li>
</ul>
<p><strong>Коснутся ли изменения приложений без нативного кода?</strong></p>
<ul>
<li>Если в проекте только Dart/Kotlin/Java и нет <code>.so</code> библиотек, приложение совместимо автоматически.</li>
</ul>
<p><strong>Как проверить совместимость Flutter-приложения?</strong></p>
<ul>
<li>Создайте эмулятор Android 15 с <em>Kernel page size = 16 KB</em>. Если приложение запускается, значит совместимо.</li>
</ul>
<p><strong>Что означает предупреждение Play Console “The App isn’t 16KB compatible”?</strong></p>
<ul>
<li>Это сигнал, что внутри AAB есть 64-битные <code>.so</code> библиотеки, собранные под 4 KB. Нужно пересобрать их с NDK r28+ и AGP 8.6+.</li>
</ul>

