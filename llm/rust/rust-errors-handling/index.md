# Обработка ошибок в Rust: паника, Option, Result и как с ними дружить

> Простой и понятный разбор обработки ошибок в Rust. Сравниваем подходы в разных языках, разбираемся с Option и Result, оператором `?` и учимся писать свои ошибки.

- **URL:** https://eagle2000.space/rust/rust-errors-handling/
- **Date:** 2026-08-02

- **Tags:** rust, dev, errors

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Ошибки в коде случаются у всех. Вопрос лишь в том, как язык программирования помогает нам с ними справляться. За годы развития IT-индустрии разработчики перепробовали много разных подходов: от простых возвращаемых циферок-кодов до сложных и тяжелых исключений.</p>
<p>Rust пошел своим путем. Вместо скрытых падений и неявного поведения он предлагает нам два четких инструмента: <strong>невосстановимые ошибки</strong> (когда продолжать работу бессмысленно) и <strong>восстановимые ошибки</strong> через специальные типы-контейнеры <code>Option&lt;T&gt;</code> и <code>Result&lt;T, E&gt;</code>.</p>
<p>Давайте разберем, как работают ошибки в разных языках, научимся пользоваться методами стандартной библиотеки Rust и разберемся, зачем нужны сторонние библиотеки вроде <code>thiserror</code> и <code>anyhow</code>.</p>
<hr>

<h2 class="relative group">1. Как с ошибками борются в других языках?
    <div id="1-как-с-ошибками-борются-в-других-языках" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%ba%d0%b0%d0%ba-%d1%81-%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b0%d0%bc%d0%b8-%d0%b1%d0%be%d1%80%d1%8e%d1%82%d1%81%d1%8f-%d0%b2-%d0%b4%d1%80%d1%83%d0%b3%d0%b8%d1%85-%d1%8f%d0%b7%d1%8b%d0%ba%d0%b0%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Язык C: Коды ошибок
    <div id="язык-c-коды-ошибок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%8f%d0%b7%d1%8b%d0%ba-c-%d0%ba%d0%be%d0%b4%d1%8b-%d0%be%d1%88%d0%b8%d0%b1%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В C функции обычно возвращают числовой код ошибки (например, <code>-1</code> при сбое и <code>0</code> при успехе), а полезные данные передаются через указатели в аргументах.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-c" data-lang="c"><span class="line"><span class="cl"><span class="kt">int</span> <span class="nf">get_data</span><span class="p">(</span><span class="n">input</span> <span class="n">in</span><span class="p">,</span> <span class="n">output</span> <span class="o">*</span><span class="n">out</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="k">if</span> <span class="p">(</span><span class="n">something_wrong</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="k">return</span> <span class="o">-</span><span class="mi">1</span><span class="p">;</span> <span class="c1">// Ошибка
</span></span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl">    <span class="k">return</span> <span class="mi">0</span><span class="p">;</span> <span class="c1">// Все ок
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><strong>В чем проблема:</strong>
<ol>
<li>Разработчик может легко проигнорировать возвращаемый код и работать дальше с пустыми данными.</li>
<li>Успешный результат и ошибка смешиваются в рамках одного и того же типа <code>int</code>.</li>
</ol>
</li>
</ul>

<h3 class="relative group">C++, Java, C#, Python, JS: Исключения (Exceptions)
    <div id="c-java-c-python-js-исключения-exceptions" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#c-java-c-python-js-%d0%b8%d1%81%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d1%8f-exceptions" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В этих языках для сообщения об ошибке используют конструкцию <code>throw</code> / <code>raise</code>, которая прерывает выполнение кода и летит вверх, пока кто-то не поймает её в блок <code>catch</code> (или <code>except</code>).</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-cpp" data-lang="cpp"><span class="line"><span class="cl"><span class="kt">void</span> <span class="nf">process</span><span class="p">()</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="k">try</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="n">do_work</span><span class="p">();</span>
</span></span><span class="line"><span class="cl">    <span class="p">}</span> <span class="k">catch</span> <span class="p">(</span><span class="k">const</span> <span class="n">std</span><span class="o">::</span><span class="n">exception</span><span class="o">&amp;</span> <span class="n">e</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="c1">// Ловим ошибку здесь
</span></span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><strong>В чем проблема:</strong>
<ol>
<li>Ошибку все еще легко случайно проигнорировать, написав пустой блок <code>catch</code>.</li>
<li>Исключения сильно замедляют программу в момент их выброса (оверхед на раскрутку стека).</li>
<li>Сигнатура функции не говорит о том, какие именно исключения она может выбросить. Код становится непредсказуемым.</li>
</ol>
</li>
</ul>

<h3 class="relative group">Язык Go: Возврат пары значений
    <div id="язык-go-возврат-пары-значений" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%8f%d0%b7%d1%8b%d0%ba-go-%d0%b2%d0%be%d0%b7%d0%b2%d1%80%d0%b0%d1%82-%d0%bf%d0%b0%d1%80%d1%8b-%d0%b7%d0%bd%d0%b0%d1%87%d0%b5%d0%bd%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В Go функции возвращают кортеж (результат, ошибка).</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-go" data-lang="go"><span class="line"><span class="cl"><span class="nx">res</span><span class="p">,</span><span class="w"> </span><span class="nx">err</span><span class="w"> </span><span class="o">:=</span><span class="w"> </span><span class="nf">doSomething</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="nx">err</span><span class="w"> </span><span class="o">!=</span><span class="w"> </span><span class="kc">nil</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">return</span><span class="w"> </span><span class="kc">nil</span><span class="p">,</span><span class="w"> </span><span class="nx">err</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><strong>В чем проблема:</strong>
<ol>
<li>Ошибку легко заглушить, присвоив ее символу <code>_</code>.</li>
<li>Приходится писать огромное количество шаблонного кода <code>if err != nil</code> на каждом шагу.</li>
</ol>
</li>
</ul>

<h3 class="relative group">Haskell, Scala, TypeScript
    <div id="haskell-scala-typescript" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#haskell-scala-typescript" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Используют так называемые алгебраические типы данных (ADT) и обертки для значений (Option/Either). Именно эта идея легла в основу Rust, позволив сделать обработку ошибок безопасной и красивой.</p>
<hr>

<h2 class="relative group">2. Невосстановимые ошибки: Паника (<code>panic!</code>)
    <div id="2-невосстановимые-ошибки-паника-panic" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bd%d0%b5%d0%b2%d0%be%d1%81%d1%81%d1%82%d0%b0%d0%bd%d0%be%d0%b2%d0%b8%d0%bc%d1%8b%d0%b5-%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8-%d0%bf%d0%b0%d0%bd%d0%b8%d0%ba%d0%b0-panic" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Иногда в программе происходит то, к чему мы вообще не готовы: например, выход за границы массива или деление на ноль. В Rust такие ситуации приводят к <strong>панике</strong>.</p>
<ul>
<li><strong>Что такое паника:</strong> Это мгновенная остановка выполнения текущего потока.</li>
<li><strong>Главное правило:</strong> Программа не должна пытаться обрабатывать панику в обычном коде. Лучшее решение при панике — упасть, чтобы не испортить данные в базе данных или файлы на диске.</li>
<li><strong>Как вызвать:</strong> С помощью макроса <code>panic!(&quot;Сообщение об ошибке&quot;)</code> или методов извлечения значения <code>.unwrap()</code> / <code>.expect()</code>, если они вызваны на пустом значении.</li>
</ul>
<hr>

<h2 class="relative group">3. Восстановимые ошибки: Обертки Option и Result
    <div id="3-восстановимые-ошибки-обертки-option-и-result" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%b2%d0%be%d1%81%d1%81%d1%82%d0%b0%d0%bd%d0%be%d0%b2%d0%b8%d0%bc%d1%8b%d0%b5-%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8-%d0%be%d0%b1%d0%b5%d1%80%d1%82%d0%ba%d0%b8-option-%d0%b8-result" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Если ошибка ожидаема (например, файл не найден или пользователь ввел буквы вместо числа), мы используем типы-контейнеры.</p>

<h3 class="relative group">Тип <code>Option&lt;T&gt;</code>
    <div id="тип-optiont" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b8%d0%bf-optiont" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Используется, когда значение может просто отсутствовать.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">None</span><span class="p">,</span><span class="w">    </span><span class="c1">// Значения нет
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Some</span><span class="p">(</span><span class="n">T</span><span class="p">),</span><span class="w"> </span><span class="c1">// Значение есть и внутри лежит тип T
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Благодаря этому, компилятор заставит вас обработать случай <code>None</code> перед тем, как вы сможете получить доступ к данным внутри <code>Some</code>.</p>

<h3 class="relative group">Тип <code>Result&lt;T, E&gt;</code>
    <div id="тип-resultt-e" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b8%d0%bf-resultt-e" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Используется, когда операция может завершиться ошибкой, и нам важно знать её причину.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nb">Result</span><span class="o">&lt;</span><span class="n">T</span><span class="p">,</span><span class="w"> </span><span class="n">E</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Ok</span><span class="p">(</span><span class="n">T</span><span class="p">),</span><span class="w">  </span><span class="c1">// Успех, внутри лежит результат типа T
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Err</span><span class="p">(</span><span class="n">E</span><span class="p">),</span><span class="w"> </span><span class="c1">// Сбой, внутри лежит ошибка типа E
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Тип <code>Result</code> помечен специальным атрибутом <code>#[must_use]</code>. Если функция возвращает <code>Result</code>, а вы проигнорируете его (не сохраните в переменную и не обработаете), компилятор выдаст предупреждение.</p>
<hr>

<h2 class="relative group">4. Шпаргалка по методам Option и Result
    <div id="4-шпаргалка-по-методам-option-и-result" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%88%d0%bf%d0%b0%d1%80%d0%b3%d0%b0%d0%bb%d0%ba%d0%b0-%d0%bf%d0%be-%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d0%b0%d0%bc-option-%d0%b8-result" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Для работы со значениями внутри <code>Option</code> и <code>Result</code> стандартная библиотека предоставляет множество удобных методов.</p>
<table>
	<thead>
			<tr>
					<th>Группа методов</th>
					<th>Для <code>Option&lt;T&gt;</code></th>
					<th>Для <code>Result&lt;T, E&gt;</code></th>
					<th>Что делает / Нюансы</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><strong>Жёсткое извлечение (Паника при сбое)</strong></td>
					<td><code>unwrap()</code> <br> <code>expect(&amp;str)</code></td>
					<td><code>unwrap()</code> <br> <code>expect(&amp;str)</code> <br> <code>unwrap_err()</code></td>
					<td>Достают значение. Если внутри <code>None</code>/<code>Err</code> — программа <strong>падает</strong>. В <code>expect</code> можно передать сообщение для лога. <code>unwrap_err</code> достает ошибку, паникуя при успехе.</td>
			</tr>
			<tr>
					<td><strong>Безопасное извлечение по умолчанию</strong></td>
					<td><code>unwrap_or(default)</code> <br> <code>unwrap_or_else(f)</code> <br> <code>unwrap_or_default()</code></td>
					<td><code>unwrap_or(default)</code> <br> <code>unwrap_or_else(f)</code> <br> <code>unwrap_or_default()</code></td>
					<td>Возвращают значение или дефолтный вариант. В <code>_or_else</code> дефолт вычисляется лениво через функцию <code>f</code>.</td>
			</tr>
			<tr>
					<td><strong>Простые проверки</strong></td>
					<td><code>is_some()</code> <br> <code>is_none()</code></td>
					<td><code>is_ok()</code> <br> <code>is_err()</code></td>
					<td>Возвращают <code>true</code> или <code>false</code>. Не забирают владение значением.</td>
			</tr>
			<tr>
					<td><strong>Трансформация содержимого</strong></td>
					<td><code>map(f)</code></td>
					<td><code>map(f)</code> <br> <code>map_err(f)</code></td>
					<td>Применяют функцию <code>f</code> к значению внутри, если оно есть. Противоположное состояние не трогают. <code>map_err</code> меняет только тип ошибки.</td>
			</tr>
			<tr>
					<td><strong>Логические цепочки (И)</strong></td>
					<td><code>and(opt)</code> <br> <code>and_then(f)</code></td>
					<td><code>and(res)</code> <br> <code>and_then(f)</code></td>
					<td><code>and_then</code> (часто называют flat_map) принимает функцию, которая возвращает новый <code>Option</code> или <code>Result</code>. Позволяет выстраивать цепочки без вложенности.</td>
			</tr>
			<tr>
					<td><strong>Логические цепочки (ИЛИ)</strong></td>
					<td><code>or(opt)</code> <br> <code>or_else(f)</code></td>
					<td><code>or(res)</code> <br> <code>or_else(f)</code></td>
					<td>Возвращают оригинал, если он успешный. Если там <code>None</code>/<code>Err</code> — возвращают альтернативу.</td>
			</tr>
			<tr>
					<td><strong>Преобразования типов</strong></td>
					<td><code>transpose()</code> <br> <code>ok_or(err)</code> <br> <code>ok_or_else(f)</code></td>
					<td><code>transpose()</code> <br> <code>ok()</code> <br> <code>err()</code></td>
					<td><code>transpose</code> меняет вложенность: <code>Option&lt;Result&gt;</code> превращает в <code>Result&lt;Option&gt;</code>. <code>ok_or</code> превращает <code>Option</code> в <code>Result</code>, заменяя <code>None</code> на указанную ошибку. <code>ok()</code> и <code>err()</code> делают из <code>Result</code> опциональные значения.</td>
			</tr>
	</tbody>
</table>
<hr>

<h2 class="relative group">5. Лаконичный проброс ошибок: Оператор <code>?</code>
    <div id="5-лаконичный-проброс-ошибок-оператор-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d0%bb%d0%b0%d0%ba%d0%be%d0%bd%d0%b8%d1%87%d0%bd%d1%8b%d0%b9-%d0%bf%d1%80%d0%be%d0%b1%d1%80%d0%be%d1%81-%d0%be%d1%88%d0%b8%d0%b1%d0%be%d0%ba-%d0%be%d0%bf%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80-" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Чтобы не писать постоянно конструкции <code>match</code> или <code>if let</code>, в Rust есть оператор <code>?</code>. Он ставится в конце выражения, возвращающего <code>Option</code> или <code>Result</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">parsed</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">val_str</span><span class="p">.</span><span class="n">parse</span>::<span class="o">&lt;</span><span class="kt">i32</span><span class="o">&gt;</span><span class="p">()</span><span class="o">?</span><span class="p">;</span></span></span></code></pre></div></div>
<p><strong>Как он работает:</strong></p>
<ul>
<li>Если внутри <code>Result</code> лежит <code>Ok</code>, оператор <code>?</code> просто достает чистое значение и код идет дальше.</li>
<li>Если внутри лежит <code>Err</code>, выполнение функции немедленно прекращается, и эта ошибка возвращается наверх в вызывающий код.</li>
<li><strong>Магия приведения типов:</strong> Под капотом компилятор автоматически вызывает метод <code>From::from(error)</code>. Это значит, что если ваша функция возвращает большую общую ошибку, а локальная функция вернула маленькую специфичную ошибку, Rust сам превратит её в нужный тип, если для них реализована конвертация.</li>
</ul>
<hr>

<h2 class="relative group">6. Создание своих типов ошибок
    <div id="6-создание-своих-типов-ошибок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#6-%d1%81%d0%be%d0%b7%d0%b4%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d0%b2%d0%be%d0%b8%d1%85-%d1%82%d0%b8%d0%bf%d0%be%d0%b2-%d0%be%d1%88%d0%b8%d0%b1%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В реальных проектах нам часто нужно детально описывать свои ошибки.</p>

<h3 class="relative group">Стандартный трейт <code>std::error::Error</code>
    <div id="стандартный-трейт-stderrorerror" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%82%d0%b0%d0%bd%d0%b4%d0%b0%d1%80%d1%82%d0%bd%d1%8b%d0%b9-%d1%82%d1%80%d0%b5%d0%b9%d1%82-stderrorerror" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Любой наш тип ошибки должен реализовывать трейты форматирования:</p>
<ol>
<li><code>Debug</code> — технический вывод для логов (выводится через <code>{:?}</code>, легко сгенерировать с помощью <code>#[derive(Debug)]</code>).</li>
<li><code>Display</code> — красивый текст для людей (выводится через <code>{}</code>).</li>
</ol>

<h3 class="relative group">Библиотеки-помощники
    <div id="библиотеки-помощники" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%b8%d0%b1%d0%bb%d0%b8%d0%be%d1%82%d0%b5%d0%ba%d0%b8-%d0%bf%d0%be%d0%bc%d0%be%d1%89%d0%bd%d0%b8%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Чтобы не писать тонны рутинного кода для вывода ошибок, разработчики используют готовые крейты:</p>
<ul>
<li><strong><code>anyhow</code> (для приложений):</strong>
Отлично подходит для конечных программ (application-level). Позволяет свести все возможные ошибки в проекте к одному типу <code>anyhow::Error</code> и не расписывать их вручную.</li>
<li><strong><code>thiserror</code> (для библиотек):</strong>
Идеально подходит для библиотек и переиспользуемых модулей. Автоматически создает красивый вывод ошибок (<code>Display</code>) и приведения типов на основе простых аннотаций прямо над перечислением.</li>
<li><strong><code>snafu</code>:</strong>
Еще один мощный инструмент для гибкой настройки ошибок с сохранением контекста.</li>
</ul>
<p>Давайте посмотрим, как работать с Option, Result, оператором <code>?</code> и библиотекой <code>thiserror</code> на интерактивных примерах кода:</p>
### 💻 Обработка ошибок в Rust


#### Шаг 1


```rust
fn main() {
    let raw_input: Option<&str> = Some("42");
    
    // Преобразуем строку в число и умножаем на 2
    let processed = raw_input
        .and_then(|s| s.parse::<i32>().ok())
        .map(|num| num * 2);
        
    println!("Обработанное значение: {:?}", processed);
}
```


**Комбинаторы Option и Result**
- Вместо матчинга можно лаконично преобразовывать данные с помощью `map` и `and_then`.
- Метод `map` применяет функцию к значению внутри `Some` или `Ok`.
- Метод `and_then` принимает замыкание, возвращающее новый `Option`/`Result` — удобно для цепочки зависимых операций.


#### Шаг 2


```rust
use std::num::ParseIntError;

fn parse_and_double(val_str: &str) -> Result<i32, ParseIntError> {
    // Если parse вернет Err, выполнение прервется и вернется Err
    let parsed = val_str.parse::<i32>()?;
    Ok(parsed * 2)
}
fn main() {
    match parse_and_double("21") {
        Ok(res) => println!("Успех: {}", res),
        Err(e) => println!("Ошибка разбора: {}", e),
    }
    
    if parse_and_double("invalid").is_err() {
        println!("Функция корректно вернула ошибку при невалидном вводе.");
    }
}
```


**Оператор проброса ошибок `?`**
- Оператор `?` разворачивает `Ok(T)` или немедленно возвращает `Err(E)` из текущей функции.
- Он также автоматически приводит типы ошибок с помощью типажа `From` (`From::from`).
- Это позволяет прозрачно трансформировать локальные ошибки в общую ошибку функции.


#### Шаг 3


```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("Ошибка ввода-вывода при обращении к файлу: {0}")]
    Disconnect(#[from] std::io::Error),
    
    #[error("Некорректный формат данных в строке: {0}")]
    InvalidFormat(String),
}

fn read_data() -> Result<String, DataError> {
    // Симулируем чтение с ошибкой формата
    Err(DataError::InvalidFormat("Не хватает заголовка".to_string()))
}
fn main() {
    match read_data() {
        Ok(data) => println!("Данные: {}", data),
        Err(e) => println!("Произошла ошибка: {}", e),
    }
}
```


**Кастомные ошибки с thiserror**
- Крейт `thiserror` генерирует реализации трейта `Error` и форматирования `Display`.
- Мы аннотируем варианты макросом `#[error(...)]` для пользовательского вывода.
- Атрибут `#[from]` автоматически реализует типаж `From` для конвертации ошибок.


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание обработки ошибок в Rust.</p>
### 📝 Обработка ошибок в Rust

**1. Какое поведение демонстрирует макрос `panic!` в Rust?**

- [ ] Возвращает специальный код ошибки для обработки в вызывающей функции
- [x] Приводит к фатальному завершению выполнения текущего потока (панике) без возможности стандартного восстановления
- [ ] Приостанавливает выполнение программы и ожидает ввода пользователя

> 💡 *Паника в Rust — это механизм обработки невосстановимых ошибок. Он завершает выполнение потока и по умолчанию раскручивает стек.*


**2. В чем главное отличие метода `.expect(&str)` от `.unwrap()`?**

- [ ] `.expect()` не паникует при ошибке
- [x] `.expect()` позволяет передать кастомное сообщение, которое будет выведено в консоль при возникновении паники
- [ ] `.expect()` работает только с `Result`, но не с `Option`

> 💡 *Оба метода паникуют при возникновении ошибки, но `.expect()` принимает строковый срез в качестве понятного разработчику сообщения, облегчая отладку.*


**3. Какую важную операцию под капотом выполняет оператор `?` при работе с `Result`?**

- [ ] Всегда прерывает программу и вызывает `panic!`
- [ ] Логирует ошибку в stderr перед возвратом
- [x] Автоматически преобразует локальный тип ошибки в тип ошибки возвращаемого значения функции с помощью трейта `From`

> 💡 *Оператор `?` не только разворачивает успешное значение или пробрасывает ошибку, но и вызывает `From::from(error)`, автоматически приводя ошибки к нужному типу.*


**4. Для чего предпочтительно использовать крейт `anyhow` по сравнению с `thiserror`?**

- [ ] В коде библиотек, где важна строгая дифференциация ошибок
- [x] В коде приложений (application-level), где нужно быстро привести все разнородные ошибки к общему типу `anyhow::Error`
- [ ] Только для написания юнит-тестов

> 💡 *`anyhow` отлично подходит для приложений, так как избавляет от написания boilerplate-кода и позволяет собирать любые ошибки в один тип. Для библиотек лучше подходит `thiserror`.*




