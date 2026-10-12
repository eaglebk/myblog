# ESP32 + Rust: Быстрый старт с нуля (помигаем светодиодом)

> Практическое руководство по первому запуску Rust на ESP32: настройка тулчейна (нативно и через Docker), генерация проекта и прошивка через espflash.

- **URL:** https://eagle2000.space/rust/esp32-rust-blink/
- **Date:** 2026-08-10

- **Tags:** rust, esp32, embedded, docker, espflash, gpio, hardware

---

<p>Программирование микроконтроллеров на Rust становится простым, как только вы один раз настроите окружение. В этом гиде мы с нуля подготовим инструменты, разберем код прошивки и заставим светиться встроенный светодиод на плашке <strong>ESP32</strong>.</p>
<hr>

<h2 class="relative group">1. Проверяем подключение платы
    <div id="1-проверяем-подключение-платы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8f%d0%b5%d0%bc-%d0%bf%d0%be%d0%b4%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5-%d0%bf%d0%bb%d0%b0%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Вставьте USB-кабель в плату ESP32 и убедитесь, что компьютер увидел серийный порт:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl"><span class="c1"># На macOS:</span>
</span></span><span class="line"><span class="cl">ls /dev/cu.usbserial*
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1"># На Linux:</span>
</span></span><span class="line"><span class="cl">ls /dev/ttyUSB*</span></span></code></pre></div></div>
<p><figure><img
    class="my-0 rounded-md"
    loading="lazy"
    decoding="async"
    fetchpriority="auto"
    alt="Макроснимок платы ESP32-WROOM-32 DevKit v1"
    width="768"
    height="1024"
    src="/rust/esp32-rust-blink/esp32_board_macro.png"
    srcset="/rust/esp32-rust-blink/esp32_board_macro.png 800w, /rust/esp32-rust-blink/esp32_board_macro.png 1280w"
    sizes="(min-width: 768px) 50vw, 65vw"
    data-zoom-src="/rust/esp32-rust-blink/esp32_board_macro.png"></figure>
</p>
<hr>

<h2 class="relative group">2. Настраиваем окружение (2 способа)
    <div id="2-настраиваем-окружение-2-способа" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bd%d0%b0%d1%81%d1%82%d1%80%d0%b0%d0%b8%d0%b2%d0%b0%d0%b5%d0%bc-%d0%be%d0%ba%d1%80%d1%83%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5-2-%d1%81%d0%bf%d0%be%d1%81%d0%be%d0%b1%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Выберите один из двух удобных способов сборки проекта:</p>

<h3 class="relative group">Вариант А: Нативная установка через <code>espup</code> (Рекомендуется)
    <div id="вариант-а-нативная-установка-через-espup-рекомендуется" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b0%d1%80%d0%b8%d0%b0%d0%bd%d1%82-%d0%b0-%d0%bd%d0%b0%d1%82%d0%b8%d0%b2%d0%bd%d0%b0%d1%8f-%d1%83%d1%81%d1%82%d0%b0%d0%bd%d0%be%d0%b2%d0%ba%d0%b0-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-espup-%d1%80%d0%b5%d0%ba%d0%be%d0%bc%d0%b5%d0%bd%d0%b4%d1%83%d0%b5%d1%82%d1%81%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Самый быстрый способ для Mac, Linux и Windows. Устанавливаем официальный тулчейн <code>espup</code> и прошивальщик <code>espflash</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">cargo install espup
</span></span><span class="line"><span class="cl">cargo install espflash
</span></span><span class="line"><span class="cl">espup install</span></span></code></pre></div></div>

<h3 class="relative group">Вариант Б: Изолированная сборка в Docker
    <div id="вариант-б-изолированная-сборка-в-docker" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b0%d1%80%d0%b8%d0%b0%d0%bd%d1%82-%d0%b1-%d0%b8%d0%b7%d0%be%d0%bb%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d0%b0%d1%8f-%d1%81%d0%b1%d0%be%d1%80%d0%ba%d0%b0-%d0%b2-docker" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если у вас установлен Docker и вы не хотите устанавливать тулчейны в систему, используйте официальный легкий образ Espressif под ESP32:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl"><span class="c1"># Скачиваем образ с преднастроенным тулчейном Rust для ESP32</span>
</span></span><span class="line"><span class="cl">docker pull espressif/idf-rust:esp32_1.93.0.0
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1"># Компилируем проект внутри Docker-контейнера</span>
</span></span><span class="line"><span class="cl">docker run --rm -v <span class="nv">$PWD</span>:/project -w /project espressif/idf-rust:esp32_1.93.0.0 cargo build</span></span></code></pre></div></div>
<hr>

<h2 class="relative group">3. Генерируем проект шаблона
    <div id="3-генерируем-проект-шаблона" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%b3%d0%b5%d0%bd%d0%b5%d1%80%d0%b8%d1%80%d1%83%d0%b5%d0%bc-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82-%d1%88%d0%b0%d0%b1%d0%bb%d0%be%d0%bd%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Для генерации готовой структуры проекта используем <code>cargo-generate</code> и официальный шаблон <code>esp-rs</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">cargo install cargo-generate
</span></span><span class="line"><span class="cl">cargo generate esp-rs/esp-idf-template cargo</span></span></code></pre></div></div>
<p>В появившемся интерактивном меню задаем параметры:</p>
<ul>
<li><strong>Project Name</strong>: <code>esp32-blink</code></li>
<li><strong>MCU</strong>: <code>esp32</code></li>
<li><strong>Advanced Options</strong>: <code>false</code></li>
</ul>
<p>Утилита автоматически создаст структуру проекта с файлом конфигурации <code>.cargo/config.toml</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-toml" data-lang="toml"><span class="line"><span class="cl"><span class="p">[</span><span class="nx">build</span><span class="p">]</span>
</span></span><span class="line"><span class="cl"><span class="nx">target</span> <span class="p">=</span> <span class="s2">&#34;xtensa-esp32-none-elf&#34;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="p">[</span><span class="nx">target</span><span class="p">.</span><span class="nx">xtensa-esp32-none-elf</span><span class="p">]</span>
</span></span><span class="line"><span class="cl"><span class="nx">runner</span> <span class="p">=</span> <span class="s2">&#34;espflash flash --monitor --chip esp32&#34;</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">4. Разбор кода (<code>src/main.rs</code>)
    <div id="4-разбор-кода-srcmainrs" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80-%d0%ba%d0%be%d0%b4%d0%b0-srcmainrs" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Откройте сгенерированный файл <code>src/main.rs</code>. Программа на микроконтроллере отличается от обычной программы на ПК: здесь отключена библиотека <code>std</code> (<code>#![no_std]</code>) и стандартная точка входа C-runtime (<code>#![no_main]</code>).</p>

<h3 class="relative group">Интерактивные слайды кода:
    <div id="интерактивные-слайды-кода" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%bd%d1%82%d0%b5%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d0%b2%d0%bd%d1%8b%d0%b5-%d1%81%d0%bb%d0%b0%d0%b9%d0%b4%d1%8b-%d0%ba%d0%be%d0%b4%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
### 💻 Слайд 1. Каркас no_std и Паника



```rust
// ?ignore
#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // В реальной прошивке здесь логгируют ошибку в UART
    loop {}
}

fn main() {
    // Точка входа прошивки без стандартной библиотеки std
}
```


**Разбор каркаса #![no_std]**
- **#![no_std]**: Отключает тяжелый C-runtime и стандартную библиотеку OS.
- **#![no_main]**: Отменяет поиск стандартной точки входа main.
- **#[panic_handler]**: Функция, вызываемая компилятором при любой ошибке паники в микроконтроллере.


### 💻 Слайд 2. Настройка GPIO на выход



```rust
struct GpioPin;
struct OutputConfig;
struct Output;

impl Output {
    fn new(_pin: GpioPin, _config: OutputConfig) -> Self {
        Output
    }
    fn set_high(&mut self) {}
    fn set_low(&mut self) {}
}
fn configure_gpio() {
    let pin_gpio2 = GpioPin;
    let mut led = Output::new(pin_gpio2, OutputConfig);

    // Управляем уровнем напряжения на пине
    led.set_high(); // 3.3V (Светодиод горит)
    led.set_low();  // 0.0V (Светодиод гаснет)
}

fn main() {
    configure_gpio();
    println!("GPIO 2 сконфигурирован успешно!");
}
```


**Конфигурирование GPIO в Rust**
- **Захват пина**: Безопасность типов Rust гарантирует, что пин GPIO2 не может быть одновременно использован в двух разных местах программы.
- **Level::High (3.3V)**: Подает высокий уровень напряжения на вывод.
- **Level::Low (0.0V)**: Подает нулевой уровень (заземление).


### 💻 Слайд 3. Бесконечный цикл мигания



```rust
struct Led {
    is_on: bool,
}

impl Led {
    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}
fn blink_simulation() {
    let mut led = Led { is_on: false };

    for step in 1..=4 {
        led.toggle();
        let status = if led.is_on { "ВКЛ (HIGH 💡)" } else { "ВЫКЛ (LOW 🌑)" };
        println!("[Шаг {}] Состояние светодиода: {}", step, status);
    }
}

fn main() {
    blink_simulation();
}
```


**Бесконечный цикл и переключение состояния**
- **led.toggle()**: Атомарно меняет состояние выхода с High на Low или наоборот.
- **Delay**: Использование аппаратного таймера отсчитывает ровно 1000 мс паузы.
- **loop**: Бесконечный цикл гарантирует, что прошивка продолжает работу всё время, пока подается питание.



<h3 class="relative group">Итоговый файл <code>src/main.rs</code>:
    <div id="итоговый-файл-srcmainrs" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%82%d0%be%d0%b3%d0%be%d0%b2%d1%8b%d0%b9-%d1%84%d0%b0%d0%b9%d0%bb-srcmainrs" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#![no_std]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#![no_main]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">esp_hal</span>::<span class="n">clock</span>::<span class="n">CpuClock</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">esp_hal</span>::<span class="n">delay</span>::<span class="n">Delay</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">esp_hal</span>::<span class="n">gpio</span>::<span class="p">{</span><span class="n">Level</span><span class="p">,</span><span class="w"> </span><span class="n">Output</span><span class="p">,</span><span class="w"> </span><span class="n">OutputConfig</span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">esp_hal</span>::<span class="n">main</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">esp_println</span>::<span class="n">println</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">esp_bootloader_esp_idf</span>::<span class="fm">esp_app_desc!</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[panic_handler]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">panic</span><span class="p">(</span><span class="n">info</span>: <span class="kp">&amp;</span><span class="nc">core</span>::<span class="n">panic</span>::<span class="n">PanicInfo</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="o">!</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;PANIC: </span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">info</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">loop</span><span class="w"> </span><span class="p">{}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[main]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="o">!</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 1. Получаем контроль над аппаратурой чипа
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">config</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">esp_hal</span>::<span class="n">Config</span>::<span class="n">default</span><span class="p">().</span><span class="n">with_cpu_clock</span><span class="p">(</span><span class="n">CpuClock</span>::<span class="n">max</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">peripherals</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">esp_hal</span>::<span class="n">init</span><span class="p">(</span><span class="n">config</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">delay</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Delay</span>::<span class="n">new</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;==========================================&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;💡 ESP32 + Rust (Blink LED)&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;==========================================&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 2. Настраиваем синий встроенный светодиод (GPIO 2)
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">led</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Output</span>::<span class="n">new</span><span class="p">(</span><span class="n">peripherals</span><span class="p">.</span><span class="no">GPIO2</span><span class="p">,</span><span class="w"> </span><span class="n">Level</span>::<span class="n">Low</span><span class="p">,</span><span class="w"> </span><span class="n">OutputConfig</span>::<span class="n">default</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">count</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">0</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 3. Бесконечный цикл работы платы
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">loop</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">count</span><span class="w"> </span><span class="o">+=</span><span class="w"> </span><span class="mi">1</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">led</span><span class="p">.</span><span class="n">toggle</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">state</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">led</span><span class="p">.</span><span class="n">is_set_high</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="s">&#34;ВКЛ (HIGH 💡)&#34;</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="s">&#34;ВЫКЛ (LOW 🌑)&#34;</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;[#</span><span class="si">{:03}</span><span class="s">] Светодиод: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">count</span><span class="p">,</span><span class="w"> </span><span class="n">state</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">delay</span><span class="p">.</span><span class="n">delay_millis</span><span class="p">(</span><span class="mi">1000</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">5. Прошиваем плату и смотрим результат
    <div id="5-прошиваем-плату-и-смотрим-результат" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d0%bf%d1%80%d0%be%d1%88%d0%b8%d0%b2%d0%b0%d0%b5%d0%bc-%d0%bf%d0%bb%d0%b0%d1%82%d1%83-%d0%b8-%d1%81%d0%bc%d0%be%d1%82%d1%80%d0%b8%d0%bc-%d1%80%d0%b5%d0%b7%d1%83%d0%bb%d1%8c%d1%82%d0%b0%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Запустите сборку и прошивку одной командой:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">cargo run --release</span></span></code></pre></div></div>
<p>Или вызовите <code>espflash</code> напрямую:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">espflash flash target/xtensa-esp32-none-elf/release/esp32-blink --monitor</span></span></code></pre></div></div>
<p>Утилита сама определит подключенный порт (например, <code>/dev/cu.usbserial-0001</code>), скомпилирует проект, загрузит прошивку в флеш-память ESP32 и откроет серийный монитор логов во встроенном терминале VS Code:</p>
<p><figure><img
    class="my-0 rounded-md"
    loading="lazy"
    decoding="async"
    fetchpriority="auto"
    alt="Вывод логов утилиты espflash во встроенном терминале VS Code"
    width="2356"
    height="1690"
    src="/rust/esp32-rust-blink/vscode_terminal_logs_hu_3afa3da708349eb5.png"
    srcset="/rust/esp32-rust-blink/vscode_terminal_logs_hu_3afa3da708349eb5.png 800w, /rust/esp32-rust-blink/vscode_terminal_logs_hu_1fbeee0c96e2824d.png 1280w"
    sizes="(min-width: 768px) 50vw, 65vw"
    data-zoom-src="/rust/esp32-rust-blink/vscode_terminal_logs.png"></figure>
</p>
<p><figure><img
    class="my-0 rounded-md"
    loading="lazy"
    decoding="async"
    fetchpriority="auto"
    alt="Анимированная GIF-демонстрация ритмичного мигания встроенного светодиода ESP32"
    width="400"
    height="711"
    src="/rust/esp32-rust-blink/esp32_blink.gif"
    srcset="/rust/esp32-rust-blink/esp32_blink.gif 800w, /rust/esp32-rust-blink/esp32_blink.gif 1280w"
    sizes="(min-width: 768px) 50vw, 65vw"
    data-zoom-src="/rust/esp32-rust-blink/esp32_blink.gif"></figure>
</p>
<p>А на самой плате ритмично замигает синий светодиод! 🎉</p>
<hr>

<h2 class="relative group">Проверьте свои знания
    <div id="проверьте-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c%d1%82%d0%b5-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий квиз по материалу статьи:</p>
### 📝 Тест: Мигаем светодиодом на ESP32 и Rust

**1. Зачем в начале прошивки под микроконтроллер указывается атрибут #![no_std]?**

- [ ] Чтобы ускорить сборку проекта в несколько раз
- [x] Чтобы отключить стандартную библиотеку std, требующую операционную систему
- [ ] Чтобы включить автоматическую оптимизацию памяти
- [ ] Чтобы разрешить использование глобальных переменных

> 💡 *Атрибут #![no_std] сообщает компилятору Rust, что целевая платформа не имеет операционной системы, поэтому нужно использовать только ядро языка core.*


**2. Что выполняет макрос #[panic_handler] в Rust?**

- [ ] Перезапускает операционную систему компьютеры
- [x] Определяет функцию, вызываемую компилятором при возникновении паники на микроконтроллере
- [ ] Автоматически исправляет синтаксические ошибки
- [ ] Логирует ошибки в веб-сервер

> 💡 *В режиме #![no_std] стандартный вывод паники отсутствует. Разработчик обязать задать функцию с атрибутом #[panic_handler], определяющую поведение при панике.*


**3. Как Rust гарантирует безопасность при работе с ножкой GPIO (например, GPIO 2)?**

- [ ] Через динамическую проверку типов во время работы
- [x] За счет модели владения и системы типов (Zero-Cost Abstractions), запрещающей доступ к пину из двух мест одновременно
- [ ] Используя виртуальную машину Java
- [ ] За счет встроенной операционной системы FreeRTOS

> 💡 *В esp-hal каждая ножка процессора является уникальным типом данных. Передача структуры GPIO2 в Output::new делает невозможным ее повторный случайный вызов в других частях программы на этапе компиляции.*


**4. Что делает метод led.toggle()?**

- [ ] Удаляет ножку GPIO из памяти
- [x] Инвертирует текущий уровень напряжения на выводе (с High на Low или с Low на High)
- [ ] Сбрасывает микроконтроллер в заводские настройки
- [ ] Измеряет силу тока в цепи

> 💡 *Метод toggle() считывает текущий уровень выхода и инвертирует его на противоположное значение.*


**5. Почему в прошивках под микроконтроллеры функция main не должна завершаться?**

- [ ] Потому что после выхода из main компилятор выдаст ошибку
- [x] Потому что микроконтроллер работает непрерывно, пока на него подается питание, и завершение программы приведет к неопределенному поведению
- [ ] Потому что это требование стандарта C99
- [ ] Потому что иначе отключится модуль Wi-Fi

> 💡 *На микроконтроллере нет OS, которая может принять код завершения exit code. Поэтому основной код работает в бесконечном цикле loop.*



<hr>

<h2 class="relative group">Что дальше?
    <div id="что-дальше" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%b4%d0%b0%d0%bb%d1%8c%d1%88%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В следующих статьях серии мы продолжим использовать эту настройку для работы с физическими кнопками, прерываниями, датчиками по I2C/SPI и Wi-Fi! 🦀</p>

