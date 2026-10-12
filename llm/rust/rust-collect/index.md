# Императивный и функциональный Rust: работа с методом collect

> Сравнение императивного и функционального стиля программирования в Rust на примере метода collect и обработки коллекций данных умного дома.

- **URL:** https://eagle2000.space/rust/rust-collect/
- **Date:** 2024-07-16

- **Tags:** rust, dev

---


<h1 class="relative group">Разные подходы для решения одной задачи
    <div id="разные-подходы-для-решения-одной-задачи" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%bd%d1%8b%d0%b5-%d0%bf%d0%be%d0%b4%d1%85%d0%be%d0%b4%d1%8b-%d0%b4%d0%bb%d1%8f-%d1%80%d0%b5%d1%88%d0%b5%d0%bd%d0%b8%d1%8f-%d0%be%d0%b4%d0%bd%d0%be%d0%b9-%d0%b7%d0%b0%d0%b4%d0%b0%d1%87%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h1>

<h2 class="relative group">Введение:
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В программировании приняты несколько <a href="https://ru.wikipedia.org/wiki/%D0%9F%D0%B0%D1%80%D0%B0%D0%B4%D0%B8%D0%B3%D0%BC%D0%B0_%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D0%BC%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F"  target="_blank" rel="noreferrer">парадигм</a>. Cегодня остановлюсь и рассмотрю императивный и функциональные стили.</p>
<p>Давайте напишем код который выполняет одинаковую задачу, но использует при этом различные подходы. Но до этого краткая справка по стилям.</p>
<p>Императивное программирование связано с идеей когда мы описываем операции (инструкции) как действия в обычной жизни. Последовательность неких действий, манипуляций. Есть некое состояние приложения и последовательность операций изменяет это состояние.</p>
<p>Функциональное программирование плотно интегрировано в современные языки: будь то js, ruby, java, dart и др. <code>Map, reduce, filter</code> наверняка уже где-то встречались 😉</p>
<p>Функциональный подход, это когда функция возвращает другую функцию. Вот такое краткое определение 😆 которое конечно не отражает всего многообразия этого царства. Но я не хочу тратить ваше время, а поэтому поехали писать код)
<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  Если хотите углубиться в функциональную тему, то вот замечательная книга: <a href="https://www.dropbox.com/s/qnq2jyc4x7rg2bb/%D0%93%D1%80%D0%BE%D0%BA%D0%B0%D0%B5%D0%BC%20%D1%84%D1%83%D0%BD%D0%BA%D1%86%D0%B8%D0%BE%D0%BD%D0%B0%D0%BB%D1%8C%D0%BD%D0%BE%D0%B5%20%D0%BC%D1%8B%D1%88%D0%BB%D0%B5%D0%BD%D0%B8%D0%B5.pdf?dl=0"  target="_blank" rel="noreferrer">&ldquo;Грокаем функциональное мышление&rdquo; Эрика Норманда</a>
</div>
</p>

<h2 class="relative group">Приложение:
    <div id="приложение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Давайте представим что нам нужно написать приложение для умного дома которое фильтрует и обрабатывает данные от сенсоров.</p>

<h3 class="relative group">Определим структуру для данных от датчиков
    <div id="определим-структуру-для-данных-от-датчиков" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%bf%d1%80%d0%b5%d0%b4%d0%b5%d0%bb%d0%b8%d0%bc-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d1%83-%d0%b4%d0%bb%d1%8f-%d0%b4%d0%b0%d0%bd%d0%bd%d1%8b%d1%85-%d0%be%d1%82-%d0%b4%d0%b0%d1%82%d1%87%d0%b8%d0%ba%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Пусть это будет температурный датчик <a href="https://3d-diy.ru/wiki/arduino-datchiki/tsifrovoy-datchik-temperatury-ds18b20/#:~:text=DS18B20%20%E2%80%93%20%D1%8D%D1%82%D0%BE%20%D0%BF%D0%BE%D0%BB%D0%BD%D0%BE%D1%86%D0%B5%D0%BD%D0%BD%D1%8B%D0%B9%20%D1%86%D0%B8%D1%84%D1%80%D0%BE%D0%B2%D0%BE%D0%B9%20%D1%82%D0%B5%D1%80%D0%BC%D0%BE%D0%BC%D0%B5%D1%82%D1%80,%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D0%BC%D0%B8%D1%80%D1%83%D0%B5%D0%BC%D0%BE%D0%B9%20%D1%82%D0%BE%D1%87%D0%BD%D0%BE%D1%81%D1%82%D1%8C%D1%8E%209%2D12%20%D0%B1%D0%B8%D1%82."  target="_blank" rel="noreferrer">DS18B20</a> с тремя ногами и работющий по протоколу 1-wire.
Это не так важно для нашего примера, но зато позволяет проникнуть в мир умного дома и немного оживляет повествование 😁</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[derive(Debug)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Sensor</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">id</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">temperature</span>: <span class="kt">f32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Определили структуру для сенсора, переходим к основному коду.</p>
<p><em><strong>Примечание:</strong> <code>derive(Debug)</code> trait помогает при отладке кода и позволяет вывести информацию об объекте передав <code>{:?}</code> или <code>{:#?}</code> при форматировании строки. Пример -&gt; <code>println!(&quot;New sensor found: {:?}&quot;, Sensor { id: &quot;123&quot;.to_string(), temperature: 20.0 });</code> <a href="https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html"  target="_blank" rel="noreferrer">подробнее в доке</a></em></p>

<h3 class="relative group">Реализация в разных стилях
    <div id="реализация-в-разных-стилях" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d0%b2-%d1%80%d0%b0%d0%b7%d0%bd%d1%8b%d1%85-%d1%81%d1%82%d0%b8%d0%bb%d1%8f%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Давайте посмотрим на оба подхода в действии. Ниже представлены интерактивные слайды, где на первом шаге показан императивный стиль, а на втором — функциональный.</p>
### 💻 Сравнение императивного и функционального стилей в Rust


#### Шаг 1


```rust
#[derive(Debug)]
struct Sensor {
    id: String,
    temperature: f32,
}

fn main() {
    let sensors = vec![
        "sensor1 22.5",
        "sensor2 35.7",
        "sensor3 29.8",
        "sensor4 41.3",
        "sensor5 30.0"
    ];

    let critical_threshold = 30.0;
    let mut critical_sensors = vec![];

    // Проходимся циклом по датчикам
    for s in sensors {
        let mut s = s.split(' ');
        let id = s.next();
        let temperature = s.next();

        if id.is_some() && temperature.is_some() {
            let id = id.unwrap().to_owned();
            let temperature = temperature.unwrap().parse::<f32>();

            if temperature.is_ok() {
                let temperature = temperature.unwrap();
                if temperature > critical_threshold {
                    critical_sensors.push(Sensor { id, temperature });
                }
            }
        }
    }

    for s in critical_sensors {
        println!("{:?}", s);
    }
}
```


**Императивный стиль**
- Мы вручную разбираем строки, выполняем проверки и наполняем изменяемый вектор `critical_sensors`.
- Код требует явного управления состоянием и частых вызовов `unwrap()`.


#### Шаг 2


```rust
#[derive(Debug)]
struct Sensor {
    id: String,
    temperature: f32,
}

fn main() {
    let sensors = vec![
        "sensor1 22.5",
        "sensor2 35.7",
        "sensor3 29.8",
        "sensor4 41.3",
        "sensor5 30.0"
    ];

    let critical_threshold = 30.0;

    // Функциональный стиль с цепочкой итераторов
    let critical_sensors: Vec<Sensor> = sensors.iter()
        .map(|s| {
            let mut s = s.split(' ');
            let id = s.next()?.to_owned();
            let temperature = s.next()?.parse::<f32>().ok()?;
            Some(Sensor { id, temperature })
        })
        .flatten()
        .filter(|s| s.temperature > critical_threshold)
        .collect();

    for s in critical_sensors {
        println!("{:?}", s);
    }
}
```


**Функциональный стиль**
- Использование цепочки методов (`map`, `flatten`, `filter`) избавляет от необходимости мутировать вектор вручную.
- Метод `collect()` собирает элементы итератора обратно в результирующий вектор `Vec<Sensor>`.



<h3 class="relative group">Сравнение подходов:
    <div id="сравнение-подходов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%80%d0%b0%d0%b2%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d0%bf%d0%be%d0%b4%d1%85%d0%be%d0%b4%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li>
<p><strong>Императивный стиль</strong>:</p>
<ul>
<li>Особенности: большое количество строчек кода, ручной перебор элементов в цикле <code>for</code>, ручная проверка <code>Option</code> и <code>Result</code>.</li>
<li>Минусы: код перегружен деталями управления памятью и состоянием, менее читаем, требует аллокации временных коллекций.</li>
</ul>
</li>
<li>
<p><strong>Функциональный стиль</strong>:</p>
<ul>
<li>Особенности: использование цепочки адаптеров итераторов (<code>map</code>, <code>flatten</code>, <code>filter</code>).</li>
<li>Плюсы: высокая читаемость, отсутствие необходимости изменять состояние вручную, лаконичность.</li>
</ul>
</li>
</ol>

<h3 class="relative group">Iterator::collect
    <div id="iteratorcollect" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#iteratorcollect" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Плюс использования <a href="https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect"  target="_blank" rel="noreferrer">Iterator::collect</a> что нам не нужно аллоцировать лишнюю коллекцию и добавлять туда элементы в цикле.
Метод collect позволяет взять итератор и конвертировать его в коллекцию для удобной дальнейшей работы или вывода.</p>
<p>Еще один небольшой пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">input</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="s">&#34;Hello&#34;</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="s">&#34;World!&#34;</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="s">&#34;From&#34;</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="s">&#34;E@Gle&#34;</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="s">&#34;Blog&#34;</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">output</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">String</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">input</span><span class="p">.</span><span class="n">iter</span><span class="p">().</span><span class="n">cloned</span><span class="p">().</span><span class="n">collect</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">output</span><span class="p">);</span></span></span></code></pre></div></div>

<h3 class="relative group">Бонус
    <div id="бонус" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%be%d0%bd%d1%83%d1%81" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Предлагаю пройтись по методам которые могут вызвать вопросы, а ведь они часто встречаются в исходниках программ написанных на rust</p>

<h4 class="relative group"><code>is_some</code>
    <div id="is_some" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#is_some" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Метод is_some вызывается для типа Option и возвращает true, если Option является Some, и false, если None.</p>

<h4 class="relative group"><code>to_owned</code>
    <div id="to_owned" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#to_owned" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Метод to_owned берет копию объекта и передаёт владение вызывающей стороне или как написано в доке преобразует заимствованные данные в собственные. Обычно он используется для создания String из &amp;str.</p>

<h4 class="relative group"><code>unwrap</code>
    <div id="unwrap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#unwrap" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Метод unwrap извлекает значение из Option, если оно является Some, иначе выбрасывает панику, если Option является None.</p>
<p><em>Считается, что лучше использовать его пореже поскольку с паникой завершается выполнение программы. Про обработку ошибок будет в другой статье.</em></p>

<h4 class="relative group"><code>flatten</code>
    <div id="flatten" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#flatten" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Метод flatten преобразует Option&lt;Option<T>&gt; в Option<T>, удаляя один уровень вложенности. Его также можно использовать с итераторами для удаления значений None.</p>

<h4 class="relative group"><code>ok</code>
    <div id="ok" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#ok" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Метод ok преобразует Result в Option. Если результат Ok, то возвращается Some, в противном случае возвращается None.</p>

<h4 class="relative group"><code>parse::&lt;F&gt;()</code>
    <div id="parsef" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#parsef" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Механизм parse::<F>() в Rust используется для преобразования фрагмента строки (&amp;str) в число. Этот метод реализован в &amp;str и может парсить строковый слайс в различные типы, не только f32.
Ниже пример показывающий как можно преобразовать &amp;str в u32</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">four</span>: <span class="kt">u32</span> <span class="o">=</span><span class="w"> </span><span class="s">&#34;4&#34;</span><span class="p">.</span><span class="n">parse</span><span class="p">().</span><span class="n">unwrap</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">assert_eq!</span><span class="p">(</span><span class="mi">4</span><span class="p">,</span><span class="w"> </span><span class="n">four</span><span class="p">);</span></span></span></code></pre></div></div>

<h2 class="relative group">Заключение:
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Надеюсь, что заметка поможет еще лучше познакомиться с языком Rust и увидеть, что это современный гибкий язык не заточенный на какой-то один стиль написания кода.</p>
<p>Rust собирает в себе лучшее из мира программирования.</p>
<p>Если что-то осталось непонятным, пожалуйста изучите, что делает каждая строка и прочитайте комментарии к коду. И тогда, я верю, белых пятен станет намного меньше.
<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  Хочу напомнить: не обязательно скролить блог чтобы читать статьи можно также нажать &ldquo;пробел&rdquo;.
</div>

Пишите код, практикуйтесь, делитесь ссылкой на блог и удачи! 👾</p>

