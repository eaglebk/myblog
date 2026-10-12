# Arc в Rust: потокобезопасное разделённое владение

> Узнайте, как использовать Arc в Rust для безопасного разделения данных между потоками. Примеры с Arc, Rc, Mutex и практические советы.

- **URL:** https://eagle2000.space/rust/rust-arc/
- **Date:** 2025-06-05

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Если вы уже знакомы с <code>Rc&lt;T&gt;</code> (если не знакомы, то можете <a href="../rust-rc/" >прочитать прошлую статью</a>), то, возможно задавались вопросом - а можно ли безопасно делить данные между потоками? Ответ — можно, но для этого нужен другой тип: <code>Arc&lt;T&gt;</code>.</p>

<h2 class="relative group">Что такое Arc?
    <div id="что-такое-arc" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d1%82%d0%b0%d0%ba%d0%be%d0%b5-arc" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><code>Arc</code> расшифровывается как <strong>Atomically Reference Counted</strong> (атомарный счетчик ссылок). Он работает как <code>Rc</code>, но с одной важной особенностью: он потокобезопасен. То есть, в отличие от <code>Rc</code>, его можно использовать в многопоточном коде.</p>

<h3 class="relative group">Синтаксис базового использования:
    <div id="синтаксис-базового-использования" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%b8%d0%bd%d1%82%d0%b0%d0%ba%d1%81%d0%b8%d1%81-%d0%b1%d0%b0%d0%b7%d0%be%d0%b2%d0%be%d0%b3%d0%be-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">sync</span>::<span class="n">Arc</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">data</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Arc</span>::<span class="n">new</span><span class="p">(</span><span class="fm">vec!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">]);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">data_cloned</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Arc</span>::<span class="n">clone</span><span class="p">(</span><span class="o">&amp;</span><span class="n">data</span><span class="p">);</span></span></span></code></pre></div></div>

<h3 class="relative group">Ключевые особенности Arc:
    <div id="ключевые-особенности-arc" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%b2%d1%8b%d0%b5-%d0%be%d1%81%d0%be%d0%b1%d0%b5%d0%bd%d0%bd%d0%be%d1%81%d1%82%d0%b8-arc" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>Подсчёт ссылок происходит атомарно (через атомарные инструкции процессора).</li>
<li>Может использоваться в нескольких потоках.</li>
<li>Не даёт изменять данные напрямую — для этого используется <code>Mutex</code>, <code>RwLock</code> или другие инструменты о которых постараюсь рассказать в следующих статьях.</li>
</ul>

<h3 class="relative group">Сравнение с Rc:
    <div id="сравнение-с-rc" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%80%d0%b0%d0%b2%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d1%81-rc" aria-label="Якорь">#</a>
    </span>
    
</h3>
<table>
	<thead>
			<tr>
					<th>Тип</th>
					<th>Поддерживает многопоточность</th>
					<th>Атомарный подсчет</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td>Rc</td>
					<td>❌ Нет</td>
					<td>❌ Нет</td>
			</tr>
			<tr>
					<td>Arc</td>
					<td>✅ Да</td>
					<td>✅ Да</td>
			</tr>
	</tbody>
</table>

<h3 class="relative group">Пример
    <div id="пример" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h3>
### 💻 Пример проблемы с Rc в многопоточном коде


#### Шаг 1


```rust
// ?compile_fail
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);

    let handle = thread::spawn(move || {
        println!("{:?}", data);
    });

    handle.join().unwrap();
}
```


**1. Этот код не скомпилируется.**
- Ошибка: `Rc<Vec<i32>> cannot be sent between threads safely`


#### Шаг 2


```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let data_cloned = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("{:?}", data_cloned);
    });

    handle.join().unwrap();
}
```


**2. Теперь всё работает корректно.**



<h3 class="relative group">Пример для случая когда нужно изменить данные
    <div id="пример-для-случая-когда-нужно-изменить-данные" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b4%d0%bb%d1%8f-%d1%81%d0%bb%d1%83%d1%87%d0%b0%d1%8f-%d0%ba%d0%be%d0%b3%d0%b4%d0%b0-%d0%bd%d1%83%d0%b6%d0%bd%d0%be-%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d0%b8%d1%82%d1%8c-%d0%b4%d0%b0%d0%bd%d0%bd%d1%8b%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Arc сам по себе не даёт изменять содержимое.</p>
<p>Если нужно менять данные из разных потоков, используйте <code>Mutex</code> (<em>мютекс это как замок, который позволяет работать с данными только одному потоку за раз</em>):</p>
### 💻 Пример с изменением данных



```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_cloned = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_cloned.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```


**Разберем подробнее что же тут происходит.**
_Это пример демонстрации безопасного изменения общего значения из нескольких потоков:_
1. `counter` — это общая переменная, защищённая `Mutex` и обёрнутая в `Arc` (как мы уже знаем он позволяет безопасно делиться данными между потоками).
2. В цикле создаётся 10 потоков с `thread::spawn` каждый из которых:
- получает свою копию `Arc` для `counter`
- блокирует `Mutex` для получения доступа к данным
- увеличивает значение на 1
3. `handle.join()` ждёт завершения всех потоков.


<blockquote><p>Мы изучим <code>Mutex</code> и <code>thread::spawn</code> более глубоко в следующих статьях - это важные концепции языка Rust для работы с потоками.</p>
</blockquote>
<h2 class="relative group">Практические советы по Arc
    <div id="практические-советы-по-arc" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b5-%d1%81%d0%be%d0%b2%d0%b5%d1%82%d1%8b-%d0%bf%d0%be-arc" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li>Используйте <code>Arc</code> если данные нужны в нескольких потоках.</li>
<li>Для изменяемых данных — комбинируйте с <code>Mutex</code>, <code>RwLock</code> и т.д.</li>
<li>Не злоупотребляйте <code>Arc</code>: каждый клон — это атомарная операция, она дороже, чем обычный счётчик <code>Rc</code>.</li>
<li><div class="flex mt-2">
  <span
    class="rounded-full bg-primary-500 dark:bg-primary-400 text-neutral-50 dark:text-neutral-800 px-1.5 py-[1px] text-xs font-normal">
    <span class="flex flex-row items-center"><span>🦀 <strong>hard skill не для начинающих!</strong></span>
    </span>
  </span>
</div> По умолчанию <code>Arc&lt;T&gt;</code> — это <code>Send + Sync</code> для случая когда <code>T: Send + Sync</code>.</li>
</ul>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Arc полезен когда вы работаете с многопоточностью. Он гарантирует безопасность при доступе к общим данным. Но требует осторожности в плане производительности. Если вам не нужен доступ из разных потоков - используйте <code>Rc</code>. Но когда дело доходит до многопоточного доступа — <code>Arc</code> становится незаменимым.</p>
<p>На сегодня это все!</p>
<div id="typeit-967128b12bb7b6736f8d05685ce0661f"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["","Продолжение следует...",""];
      
      new TypeIt("#typeit-967128b12bb7b6736f8d05685ce0661f", {
        strings: strings,
        speed:  50 ,
        lifeLike:  false ,
        startDelay:  0 ,
        breakLines:  true ,
        waitUntilVisible:  true ,
        loop:  false 
      }).go();
    });
</script>


