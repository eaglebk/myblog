# Memory Management in Rust: RAII Pattern, Box Pointers, and Deref Coercion

> Understand memory management in Rust: RAII pattern, automatic cleanup via Drop, Box<T> heap pointers, 'static references with Box::leak, and Deref Coercion mechanisms.

- **URL:** https://eagle2000.space/en/rust/rust-raii-box-deref/
- **Date:** 2026-08-12

- **Tags:** rust, dev, raii, memory, box, deref

---


<h2 class="relative group">Introduction
    <div id="introduction" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#introduction" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>When working with system resources—heap allocations, open file handles, or network sockets—one of the main challenges in programming is guaranteeing their timely and safe deallocation. Forgetting a <code>free()</code> call causes memory leaks, while premature deallocation leads to critical vulnerabilities like <code>use-after-free</code> or <code>double free</code>.</p>
<p>In Rust, system resources are safely managed by the <strong>RAII (Resource Acquisition Is Initialization)</strong> pattern.</p>
<p>In languages with manual memory management (like C), developers must manually track every return path in functions. In garbage-collected languages (Java, Go, Python), a runtime background thread periodically scans memory for unreferenced objects. In Rust, the compiler automatically ties the lifetime of a resource to the lifetime of its owner variable.</p>
<p>In this article, we will examine how RAII works under the hood, why the owning pointer <code>Box&lt;T&gt;</code> is used, when it is useful to opt out of automatic deallocation via <code>Box::leak</code>, and how automatic reference coercion via <strong>Deref Coercion</strong> operates.</p>
<hr>

<h2 class="relative group">1. The RAII Pattern and the Drop Trait
    <div id="1-the-raii-pattern-and-the-drop-trait" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-the-raii-pattern-and-the-drop-trait" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>In Rust, ownership of a resource (heap memory, open file, socket, mutex guard) is directly tied to the scope lifetime of a variable value.</p>
<ul>
<li><strong>Resource Acquisition</strong> occurs when the variable is created (in its constructor or <code>new</code> function).</li>
<li><strong>Resource Release</strong> occurs automatically at the exact moment the variable goes out of scope.</li>
</ul>

<h3 class="relative group">How Does the Drop Trait Work?
    <div id="how-does-the-drop-trait-work" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#how-does-the-drop-trait-work" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Resource cleanup is managed by the <code>std::ops::Drop</code> trait. When a value goes out of scope, Rust automatically invokes its <code>Drop::drop</code> method if the type implements <code>Drop</code>. Afterwards, it recursively drops its fields.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">CustomResource</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">name</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="nb">Drop</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">CustomResource</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">drop</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Releasing resource: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="bp">self</span><span class="p">.</span><span class="n">name</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">res</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">CustomResource</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">name</span>: <span class="s">&#34;Config File&#34;</span><span class="p">.</span><span class="n">into</span><span class="p">()</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Working with resource...&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w"> </span><span class="c1">// &lt;- Variable `res` goes out of scope: `drop()` is automatically called!
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Program execution continues.&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Note
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>You cannot invoke <code>res.drop()</code> manually directly—the compiler will reject it to prevent double-free errors. If you need to force early resource deallocation before scope end, use the standalone function <code>std::mem::drop(res)</code>.</p></div></div>
<h3 class="relative group">Is Resource Cleanup Always Guaranteed?
    <div id="is-resource-cleanup-always-guaranteed" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#is-resource-cleanup-always-guaranteed" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The <code>Drop</code> trait does not provide an absolute guarantee of execution under all imaginable conditions. Destructors will not run if:</p>
<ul>
<li><code>std::mem::forget(res)</code> is explicitly called to prevent value destruction and <code>Drop</code> invocation.</li>
<li>A reference cycle created with <code>Rc</code> or <code>Arc</code> keeps values alive indefinitely: reference count never hits zero, so <code>Drop</code> is never triggered.</li>
<li>An emergency process abort occurs (<code>SIGKILL</code>, <code>std::process::abort</code>).</li>
</ul>
<hr>

<h2 class="relative group">2. Owning Heap Pointer: Box<T>
    <div id="2-owning-heap-pointer-box" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-owning-heap-pointer-box" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Local variables in Rust are placed on the stack by default. When you explicitly need to allocate a value on the heap, <strong><code>Box&lt;T&gt;</code></strong> is used.</p>

<h3 class="relative group">Why Use Box<T>?
    <div id="why-use-box" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#why-use-box" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p><code>Box&lt;T&gt;</code> is an owning smart pointer to a heap-allocated object. For a standard <code>Sized</code> type, the stack footprint of <code>Box&lt;T&gt;</code> corresponds to a single raw pointer size—8 bytes on a 64-bit platform.</p>
<p><code>Box&lt;T&gt;</code> is essential in several core scenarios:</p>
<ol>
<li><strong>Recursive Data Types:</strong> Without <code>Box</code>, the compiler cannot determine the finite memory size of a recursive type:
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">enum</span> <span class="nc">List</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Cons</span><span class="p">(</span><span class="kt">i32</span><span class="p">,</span><span class="w"> </span><span class="nb">Box</span><span class="o">&lt;</span><span class="n">List</span><span class="o">&gt;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Nil</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
Here <code>Box</code> is required not just for optimization: without it, <code>List</code> would contain itself directly, causing infinite recursion during type size calculation.</li>
<li><strong>Allocating Large Structs on Heap:</strong> When a large payload is better stored separately from stack frames.</li>
<li><strong>Storing Trait Objects (<code>Box&lt;dyn Trait&gt;</code>):</strong> Enables dynamic polymorphism.</li>
<li><strong>Stable Memory Location:</strong> The <code>Box&lt;T&gt;</code> pointer itself can be moved around on the stack, while the underlying heap payload <code>T</code> stays at a fixed memory address.</li>
</ol>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">BigData</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">values</span>: <span class="p">[</span><span class="kt">u8</span><span class="p">;</span><span class="w"> </span><span class="mi">10000</span><span class="p">],</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Allocation takes place on the heap
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">boxed_data</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">BigData</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">values</span>: <span class="p">[</span><span class="mi">0</span><span class="p">;</span><span class="w"> </span><span class="mi">10000</span><span class="p">]</span><span class="w"> </span><span class="p">});</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Modify value through pointer dereference
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">boxed_data</span><span class="p">.</span><span class="n">values</span><span class="p">[</span><span class="mi">0</span><span class="p">]</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">42</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="c1">// When `boxed_data` goes out of scope, heap memory is freed!
</span></span></span></code></pre></div></div>

<h3 class="relative group">Opting Out of Deallocation: <code>Box::leak</code>
    <div id="opting-out-of-deallocation-boxleak" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#opting-out-of-deallocation-boxleak" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Sometimes you need to register dynamically created configuration or data that must persist for the entire runtime of the application with a <code>'static</code> reference lifetime.</p>
<p>For this purpose, Rust provides <code>Box::leak</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">GlobalConfig</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">port</span>: <span class="kt">u16</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">config</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">GlobalConfig</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">port</span>: <span class="mi">8080</span><span class="w"> </span><span class="p">});</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Consumes Box&lt;T&gt; ownership and returns a raw reference to heap payload
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">static_ref</span>: <span class="kp">&amp;</span><span class="nb">&#39;static</span> <span class="nc">mut</span><span class="w"> </span><span class="n">GlobalConfig</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">leak</span><span class="p">(</span><span class="n">config</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">static_ref</span><span class="p">.</span><span class="n">port</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">9090</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="tip">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span></div>
        <div class="grow">
          Tip
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p><code>Box::leak</code> consumes ownership of <code>Box&lt;T&gt;</code> and returns a raw reference to the heap object. The object will never be automatically freed. The returned reference can be used with a <code>'static</code> or shorter lifetime.</p>
<p>Note: <code>'static</code> here refers specifically to the lifetime bound of the returned reference, not a mandate that the object must physically exist until process exit.</p>
<p>Having a <code>&amp;mut</code> reference does <strong>not bypass Rust&rsquo;s borrowing rules</strong> (aliasing is still strictly forbidden). In practice, thread-safe singletons often use <code>OnceLock</code> or <code>LazyLock</code>, while <code>Box::leak</code> serves as a low-level tool for passing data into APIs requiring <code>'static</code> bounds.</p></div></div><hr>

<h2 class="relative group">3. Deref and DerefMut Traits
    <div id="3-deref-and-derefmut-traits" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-deref-and-derefmut-traits" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>How do smart pointers feel as ergonomic in code as standard references? This is achieved via the <code>Deref</code> and <code>DerefMut</code> traits in <code>std::ops</code>.</p>
<p>When dereferencing <code>*my_box</code>, Rust invokes the <code>Deref</code> implementation, conceptually fetching a reference via <code>my_box.deref()</code>.</p>
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
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Value: </span><span class="si">{value}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">boxed_val</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">MyBox</span><span class="p">(</span><span class="mi">42</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Rust automatically uses Deref to pass reference
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">print_value</span><span class="p">(</span><span class="o">&amp;</span><span class="n">boxed_val</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">4. Deref Coercion (Implicit Reference Conversion)
    <div id="4-deref-coercion-implicit-reference-conversion" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-deref-coercion-implicit-reference-conversion" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>We implemented <code>Deref</code>—and passing <code>&amp;MyBox&lt;i32&gt;</code> to a function expecting <code>&amp;i32</code> worked seamlessly. But how does the compiler know one reference type can be passed in place of another?</p>
<p>This is powered by <strong>Deref Coercion</strong>. If type <code>T</code> implements <code>Deref&lt;Target = U&gt;</code>, the compiler automatically coerces reference <code>&amp;T</code> into reference <code>&amp;U</code>.</p>
<p>Crucially: <strong>Deref Coercion strictly operates on references</strong>, and does not convert <code>Box&lt;T&gt;</code> by value.</p>

<h3 class="relative group">Three Rules of Deref Coercion:
    <div id="three-rules-of-deref-coercion" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#three-rules-of-deref-coercion" aria-label="Anchor">#</a>
    </span>
    
</h3>
<ol>
<li>From <code>&amp;T</code> to <code>&amp;U</code> when <code>T: Deref&lt;Target = U&gt;</code> (immutable to immutable reference).</li>
<li>From <code>&amp;mut T</code> to <code>&amp;mut U</code> when <code>T: DerefMut&lt;Target = U&gt;</code> (mutable to mutable reference).</li>
<li>From <code>&amp;mut T</code> to <code>&amp;U</code> when <code>T: Deref&lt;Target = U&gt;</code> (mutable to immutable reference).</li>
</ol>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="caution">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
<path fill="currentColor"  d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-2.8-5.6-5.6-11.2-9.8-16.8l-50.6 58.8s-81.4-103.6-87.1-110.6C133.1 243.8 112 273.2 112 306.8C112 375.4 162.6 416 225.7 416z"/></svg></span></div>
        <div class="grow">
          Caution
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Coercion from <code>&amp;T</code> to <code>&amp;mut U</code> is <strong>fundamentally forbidden</strong>—it would violate core aliasing and memory safety guarantees in Rust.</p></div></div>
<h3 class="relative group">Chain Coercion
    <div id="chain-coercion" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#chain-coercion" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The compiler can chain <code>Deref</code> conversions multiple times in sequence. Consider <code>Box&lt;String&gt;</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">&amp;Box&lt;String&gt;
</span></span><span class="line"><span class="cl">    ↓ Deref
</span></span><span class="line"><span class="cl">&amp;String
</span></span><span class="line"><span class="cl">    ↓ Deref
</span></span><span class="line"><span class="cl">&amp;str</span></span></code></pre></div></div>
<p>Thanks to this coercion chain, passing <code>&amp;Box&lt;String&gt;</code> to a function accepting <code>&amp;str</code> compiles cleanly without explicit method calls:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_greeting</span><span class="p">(</span><span class="n">name</span>: <span class="kp">&amp;</span><span class="kt">str</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello, </span><span class="si">{name}</span><span class="s">!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">boxed_name</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;Alex&#34;</span><span class="p">));</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Automatic Deref Coercion chain: &amp;Box&lt;String&gt; -&gt; &amp;String -&gt; &amp;str
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">print_greeting</span><span class="p">(</span><span class="o">&amp;</span><span class="n">boxed_name</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Thanks to Deref, method resolution automatically searches target type methods:
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Name length: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">boxed_name</span><span class="p">.</span><span class="n">len</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Because of <code>Deref</code>, Rust automatically searches methods on the target type as well. Thus <code>boxed_name.len()</code> works as naturally as calling <code>len()</code> on a <code>String</code> or <code>str</code>.</p>
<p>Let&rsquo;s explore these mechanisms in our interactive code slides:</p>
### 💻 Memory Management, Box, and Deref Coercion


#### Step 1


```rust
struct SensorData {
    name: String,
}

impl Drop for SensorData {
    fn drop(&mut self) {
        println!("Releasing sensor resource: {}", self.name);
    }
}

fn main() {
    {
        let sensor = SensorData {
            name: "Kitchen Temperature".to_string(),
        };
        println!("Working with sensor: {}", sensor.name);
    } // Scope ends here — Drop is automatically called!
    println!("Section completed.");
}
```


**The RAII Pattern and the Drop Trait**
- The RAII concept guarantees: resources are allocated upon creation and automatically deallocated when exiting scope.
- The `Drop` trait allows overriding cleanup logic (closing files, releasing memory, logging).
- The `drop` method is called automatically in reverse order of variable declaration.


#### Step 2


```rust
struct Config {
    port: u16,
}

fn main() {
    // 1. Allocate object on the heap using Box
    let mut heap_config = Box::new(Config { port: 8080 });
    heap_config.port = 9090; // Mutable access to heap data
    println!("Heap config port: {}", heap_config.port);

    // 2. Intentionally "leak" memory to obtain a 'static reference
    let static_config: &'static mut Config = Box::leak(heap_config);
    static_config.port = 3000;
    println!("Static port after Box::leak: {}", static_config.port);
}
```


**Smart Pointer Box<T> and Box::leak**
- `Box<T>` moves an object from stack to heap, retaining ownership over it.
- `Box::leak` cancels automatic `Drop` invocation and returns a mutable reference with a `'static` lifetime.
- This is useful for creating global configurations or long-lived singletons at runtime.


#### Step 3


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
    
    // Dereference operator overloading with *
    assert_eq!(*wrapped_num, 42);
    
    *wrapped_num += 10;
    println!("Updated value: {}", *wrapped_num);
}
```


**Dereference Operator Overloading: Deref and DerefMut**
- Implementing `Deref` enables using the dereference operator `*` on custom smart pointers.
- `DerefMut` extends dereferencing for mutable references (`*x = new_val`).
- Associated type `type Target = T` specifies what type dereferencing yields.


#### Step 4


```rust
// Function expects a standard string slice &str
fn print_message(msg: &str) {
    println!("Message: {msg}");
}

fn main() {
    let boxed_string: Box<String> = Box::new(String::from("Hello from Deref!"));
    
    // Deref Coercion chain: &Box<String> -> &String -> &str
    print_message(&boxed_string);
    
    // Direct invocation of inner type methods via smart pointer:
    println!("String length: {}", boxed_string.len());
}
```


**Implicit Reference Coercion (Deref Coercion)**
- The compiler automatically coerces smart pointer references into inner type references: `&Box<String>` -> `&String` -> `&str`.
- This applies across chains of types implementing `Deref`.
- Enables calling inner type methods directly through the smart pointer (e.g., `boxed_string.len()`).


<hr>

<h2 class="relative group">Test Your Knowledge!
    <div id="test-your-knowledge" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#test-your-knowledge" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Take a short quiz to consolidate your understanding of RAII, <code>Box&lt;T&gt;</code>, and <code>Deref</code> mechanisms.</p>
### 📝 Memory Management and Deref

**1. What is the core idea of the RAII (Resource Acquisition Is Initialization) idiom in Rust?**

- [ ] Resource allocation happens only at compile time, and cleanup occurs on system restart
- [x] Resource acquisition occurs during object initialization, and deallocation is guaranteed by the Drop destructor upon leaving scope
- [ ] All objects are forcibly copied to the heap to prevent stack overflow

> 💡 *RAII binds resource lifetime to object lifetime. When an object leaves scope, an automatic `drop()` call cleans up memory or closes resources.*


**2. What is the `Box::leak` method used for?**

- [ ] To urgently move data from heap to stack
- [x] To intentionally bypass deallocation and obtain a reference with `'static` lifetime
- [ ] To automatically optimize object memory size

> 💡 *`Box::leak` prevents the automatic `Drop` call for `Box<T>` and returns a static reference `&'static mut T`. This is useful for creating global configurations at runtime.*


**3. What is the primary purpose of the `Deref` trait?**

- [ ] Allowing cloning of any type without an explicit `.clone()` call
- [x] Overloading the dereference operator `*` and supporting implicit Deref Coercion
- [ ] Automatic data wrapping on the stack

> 💡 *`Deref` allows accessing internal smart pointer data via `*` operator and implicitly coerces references like `&SmartPointer<T>` to `&T`.*


**4. Which implicit type coercion (Deref Coercion) is supported by the Rust compiler?**

- [ ] From an immutable reference `&T` to a mutable `&mut U`
- [x] From a mutable reference `&mut T` to a mutable `&mut U` (if `T: DerefMut<Target = U>`)
- [ ] From string slice `&str` to `&mut String`

> 💡 *Rust allows Deref Coercion from `&T` to `&U`, `&mut T` to `&mut U`, and `&mut T` to `&U`. Coercing an immutable reference into a mutable one is forbidden for memory safety.*


**5. Why does the compiler forbid calling the `res.drop()` method directly?**

- [ ] The drop method does not exist at runtime
- [x] A direct call would lead to a double free error, as the destructor would still be called automatically at the end of the block
- [ ] The drop method can only be called inside compiler-generated code

> 💡 *If explicit `res.drop()` were allowed, the object would be destroyed again at the end of the scope. For explicit early deallocation, `std::mem::drop(res)` is used.*




