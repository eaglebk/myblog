# Crate of the Day: Strum - iterating and converting Rust Enum

> Exploring the popular Strum crate: how to iterate over enum variants, convert enums to strings and back, using a real-world example from esp-generate.

- **URL:** https://eagle2000.space/en/rust/crate-of-the-day-strum/
- **Date:** 2026-08-16

- **Tags:** rust, crate-of-the-day, enums, macros, esp32

---

<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  &ldquo;Crate of the Day&rdquo; — concise breakdowns of popular and practical Rust ecosystem libraries that eliminate boilerplate and clean up your architecture.
</div>

<p>Today&rsquo;s featured crate — <strong><code>strum</code></strong> (a suite of procedural macros for working with <code>enum</code>s in Rust).</p>
<hr>

<h2 class="relative group">The Problem: Rust Enum lack built-in iterators
    <div id="the-problem-rust-enum-lack-built-in-iterators" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#the-problem-rust-enum-lack-built-in-iterators" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Enums are widely used in Rust to represent finite sets of states or variants. However, out of the box, <strong>Rust enums provide no built-in mechanism to iterate over all their variants</strong>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// We want to loop over all enum variants:
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">color</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">Color</span>::<span class="n">all_variants</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="c1">// ❌ Error! No such method exists
</span></span></span></code></pre></div></div>
<p>If you are developing a CLI tool (like <code>esp-generate</code>), a web service, or an API, you frequently need to:</p>
<ol>
<li>Print a list of all supported options in <code>--help</code> or an interactive console selector.</li>
<li>Validate and parse an incoming user input string or config file value into an <code>enum</code>.</li>
<li>Convert an enum variant into a formatted string (e.g., for <code>Color</code>: <code>&quot;black&quot;</code> or <code>&quot;white&quot;</code>).</li>
</ol>
<p>Without third-party crates, developers often resort to manually maintaining arrays like <code>const ALL_COLORS: &amp;[Color] = &amp;[...];</code>.</p>
<blockquote><p><strong>The primary risk of this approach:</strong> as soon as a new variant (like <code>GREY</code>) is added to the enum, someone will inevitably forget to update the manual array.</p>
</blockquote><hr>

<h2 class="relative group">Community solution: real-world example from <code>esp-generate</code>
    <div id="community-solution-real-world-example-from-esp-generate" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#community-solution-real-world-example-from-esp-generate" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>The official project generator for Espressif microcontrollers — <strong><code>esp-generate</code></strong> (developed by the <code>esp-rs</code> team) solves this exact problem using <code>strum</code>.</p>
<p>Here is how their <code>Chip</code> enum is declared in the project source code:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[derive(
</span></span></span><span class="line"><span class="cl"><span class="cp">    Debug,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Clone,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Copy,
</span></span></span><span class="line"><span class="cl"><span class="cp">    PartialEq,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Eq,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Hash,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Serialize,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Deserialize,
</span></span></span><span class="line"><span class="cl"><span class="cp">    clap::ValueEnum,
</span></span></span><span class="line"><span class="cl"><span class="cp">    strum::EnumIter,
</span></span></span><span class="line"><span class="cl"><span class="cp">    strum::Display,
</span></span></span><span class="line"><span class="cl"><span class="cp">    strum::EnumString,
</span></span></span><span class="line"><span class="cl"><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[serde(rename_all = </span><span class="s">&#34;kebab-case&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[strum(serialize_all = </span><span class="s">&#34;kebab-case&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nc">Chip</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c5</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c6</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c61</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32h2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Just three macros from <code>strum</code> cover all requirements for string conversions and variant iteration!</p>
<hr>

<h2 class="relative group">Core strum features
    <div id="core-strum-features" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#core-strum-features" aria-label="Anchor">#</a>
    </span>
    
</h2>

<h3 class="relative group">1. Iterating over Enum variants (<code>strum::EnumIter</code>)
    <div id="1-iterating-over-enum-variants-strumenumiter" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-iterating-over-enum-variants-strumenumiter" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The <code>EnumIter</code> derive macro, combined with the <code>strum::IntoEnumIterator</code> trait, endows the <code>enum</code> with an <code>.iter()</code> method:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">strum</span>::<span class="n">IntoEnumIterator</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">chip</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">iter</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Supported microcontroller: </span><span class="si">{chip}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Now, whenever a new chip variant (such as <code>Esp32p4</code>) is added, it <strong>automatically</strong> participates in all iteration loops across your project without modifying any downstream loop logic!</p>
<hr>

<h3 class="relative group">2. Converting Enums to Strings (<code>strum::Display</code>)
    <div id="2-converting-enums-to-strings-strumdisplay" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-converting-enums-to-strings-strumdisplay" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The <code>Display</code> macro eliminates the boilerplate of writing <code>impl std::fmt::Display for Chip</code> manually.</p>
<p>The <code>#[strum(serialize_all = &quot;kebab-case&quot;)]</code> attribute configures proper string formatting:</p>
<ul>
<li><code>Chip::Esp32c3</code> becomes <code>&quot;esp32c3&quot;</code>.</li>
<li><code>Chip::Esp32s3</code> becomes <code>&quot;esp32s3&quot;</code>.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">Esp32c3</span><span class="p">);</span><span class="w"> </span><span class="c1">// Outputs: esp32c3
</span></span></span></code></pre></div></div>
<hr>

<h3 class="relative group">3. Parsing Strings into Enums (<code>strum::EnumString</code>)
    <div id="3-parsing-strings-into-enums-strumenumstring" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-parsing-strings-into-enums-strumenumstring" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The <code>EnumString</code> macro implements <code>std::str::FromStr</code>. This makes parsing string inputs straightforward:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="kt">str</span>::<span class="n">FromStr</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">chip</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">from_str</span><span class="p">(</span><span class="s">&#34;esp32c3&#34;</span><span class="p">)</span><span class="o">?</span><span class="p">;</span><span class="w"> </span><span class="c1">// Ok(Chip::Esp32c3)
</span></span></span><span class="line"><span class="cl"><span class="c1">// Or via standard parse() method:
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">chip</span>: <span class="nc">Chip</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;esp32c3&#34;</span><span class="p">.</span><span class="n">parse</span><span class="p">()</span><span class="o">?</span><span class="p">;</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Additional helpful Strum macros
    <div id="additional-helpful-strum-macros" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#additional-helpful-strum-macros" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>The <code>strum</code> crate offers several other powerful derives:</p>
<ul>
<li><strong><code>EnumCount</code></strong>: Provides a compile-time constant (e.g., <code>Chip::COUNT</code>) representing the total number of variants in the <code>enum</code>.</li>
<li><strong><code>VariantNames</code></strong>: Exposes a slice containing all variant names (e.g., <code>Chip::VARIANTS</code> — <code>[&quot;esp32&quot;, &quot;esp32c2&quot;]</code>).</li>
<li><strong><code>EnumProperty</code> / <code>EnumMessage</code></strong>: Allows attaching key-value metadata or human-readable descriptions directly to variants via attributes like <code>#[strum(message = &quot;...&quot;)]</code>.</li>
</ul>
<hr>

<h2 class="relative group">Complete code example
    <div id="complete-code-example" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#complete-code-example" aria-label="Anchor">#</a>
    </span>
    
</h2>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">strum</span>::<span class="p">{</span><span class="n">EnumIter</span><span class="p">,</span><span class="w"> </span><span class="n">EnumString</span><span class="p">,</span><span class="w"> </span><span class="n">Display</span><span class="p">,</span><span class="w"> </span><span class="n">IntoEnumIterator</span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="kt">str</span>::<span class="n">FromStr</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[derive(Debug, PartialEq, Eq, EnumIter, Display, EnumString)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[strum(serialize_all = </span><span class="s">&#34;kebab-case&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nc">Chip</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c5</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c6</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c61</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32h2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 1. Iterate over all enum variants
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;=== ESP Chips ===&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">for</span><span class="w"> </span><span class="n">chip</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">iter</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;- </span><span class="si">{chip}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 2. Parse string back into enum
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">input</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;esp32c3&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">match</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">from_str</span><span class="p">(</span><span class="n">input</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="nb">Ok</span><span class="p">(</span><span class="n">chip</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="se">\n</span><span class="s">Successfully parsed &#39;</span><span class="si">{input}</span><span class="s">&#39; into variant: </span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">chip</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="nb">Err</span><span class="p">(</span><span class="n">_</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="se">\n</span><span class="s">Unknown chip!&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Program Output:
    <div id="program-output" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#program-output" aria-label="Anchor">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">=== ESP Chips ===
</span></span><span class="line"><span class="cl">- esp32
</span></span><span class="line"><span class="cl">- esp32c2
</span></span><span class="line"><span class="cl">- esp32c3
</span></span><span class="line"><span class="cl">- esp32c5
</span></span><span class="line"><span class="cl">- esp32c6
</span></span><span class="line"><span class="cl">- esp32c61
</span></span><span class="line"><span class="cl">- esp32h2
</span></span><span class="line"><span class="cl">- esp32s2
</span></span><span class="line"><span class="cl">- esp32s3
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">Successfully parsed &#39;esp32c3&#39; into variant: Esp32c3</span></span></code></pre></div></div>
<hr>

<h2 class="relative group">🎯 Test your knowledge!
    <div id="-test-your-knowledge" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-test-your-knowledge" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Take this short quiz to reinforce your understanding of working with <code>strum</code>:</p>
### 📝 Strum Crate in Rust

**1. Which strum macro adds the .iter() method for an enum?**

- [ ] strum::Display
- [x] strum::EnumIter
- [ ] strum::EnumString
- [ ] strum::EnumCount

> 💡 *The strum::EnumIter macro generates an implementation of IntoEnumIterator, which provides the .iter() method to obtain an iterator over enum variants.*


**2. How does #[strum(serialize_all = "kebab-case")] convert the Esp32c3 variant into a string?**

- [ ] Converts it to ESP32C3
- [x] Converts it to esp32c3
- [ ] Converts it to esp_32_c3
- [ ] Leaves it unchanged

> 💡 *The kebab-case rule converts the variant name to lowercase using hyphens (Esp32c3 becomes esp32c3).*


**3. What is the primary role of the derive macro strum::EnumString?**

- [ ] Prints the enum to console via std::fmt::Display
- [ ] Counts the number of elements in the enum
- [x] Implements std::str::FromStr to parse strings into enum variants
- [ ] Creates a string array of all enum variant names

> 💡 *EnumString generates a FromStr parser, allowing strings like "esp32c3" to be converted back into an enum variant via .parse::<Chip>().*


**4. What is the main advantage of strum::EnumIter over a manual array like variants: &[Chip]?**

- [ ] It only works at runtime
- [x] Adding a new variant to the enum automatically includes it in all iteration loops
- [ ] It speeds up program compilation 2x
- [ ] It stores variants in memory as a linked list

> 💡 *Because strum generates the iterator at compile time, adding a new enum variant updates iteration logic automatically without the risk of forgetting an element.*



<hr>

<h2 class="relative group">How to add to your project
    <div id="how-to-add-to-your-project" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#how-to-add-to-your-project" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Simply add the dependency to your <code>Cargo.toml</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-toml" data-lang="toml"><span class="line"><span class="cl"><span class="p">[</span><span class="nx">dependencies</span><span class="p">]</span>
</span></span><span class="line"><span class="cl"><span class="nx">strum</span> <span class="p">=</span> <span class="p">{</span> <span class="nx">version</span> <span class="p">=</span> <span class="s2">&#34;0.28&#34;</span><span class="p">,</span> <span class="nx">features</span> <span class="p">=</span> <span class="p">[</span><span class="s2">&#34;derive&#34;</span><span class="p">]</span> <span class="p">}</span></span></span></code></pre></div></div>
<p>🔗 <strong>GitHub Repository:</strong> <a href="https://github.com/Petrochev/strum"  target="_blank" rel="noreferrer">Petrochev/strum</a><br>
📦 <strong>Crates.io:</strong> <a href="https://crates.io/crates/strum"  target="_blank" rel="noreferrer">crates.io/crates/strum</a><br>
🦀 <strong>Real-world example from esp-generate:</strong> <a href="https://github.com/esp-rs/esp-generate"  target="_blank" rel="noreferrer">esp-rs/esp-generate</a></p>

