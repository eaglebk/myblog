# Trait Objects: How Dynamic Polymorphism Works in Rust

> Learn how trait objects (dyn Trait) work in Rust in simple terms: fat pointers, vtables, memory alignment, and object safety rules in practice.

- **URL:** https://eagle2000.space/en/rust/rust-dyn-traits/
- **Date:** 2026-08-14

- **Tags:** rust, dev, traits, dyn, vtable, middle

---


<h2 class="relative group">Introduction
    <div id="introduction" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#introduction" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>In our previous articles: <a href="../rust-raii-box-deref" >RAII, Box, and Deref</a> and <a href="../rust-traits" >Introduction to Traits</a>, we explored basic traits and heap allocations via <code>Box&lt;T&gt;</code>. By default, Rust strives to make your code as fast as possible: when using generic types, the compiler performs <strong>monomorphization</strong>. It takes a function and generates a concrete copy for every single type used, producing direct, zero-cost function calls.</p>
<p>However, real-world software engineering often presents tasks where data types are not known upfront at compile time.</p>
<p>Imagine we are designing a notification or background task system. We have different task structs: <code>NetworkTask</code> (HTTP requests), <code>DbTask</code> (database writes), and <code>TimerTask</code> (timer triggers). All of them implement a shared <code>Task</code> trait with an <code>execute()</code> method. We want to store all these tasks in a single list (<code>Vec</code>) and process them inside a loop.</p>
<p>How do we store different types inside a single <code>Vec</code>? A vector in Rust requires every element to have the exact same type and byte size in memory.</p>
<p>This is where <strong>trait objects</strong> and the <code>dyn</code> keyword come to the rescue. In this article, we will examine how dynamic method dispatch works in Rust, where &ldquo;fat&rdquo; pointers come from, and how the compiler keeps everything memory-safe.</p>
<hr>

<h2 class="relative group">1. Static vs. Dynamic Dispatch
    <div id="1-static-vs-dynamic-dispatch" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-static-vs-dynamic-dispatch" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>To choose the right tool for the job, let&rsquo;s compare these two approaches:</p>
<table>
	<thead>
			<tr>
					<th>Feature</th>
					<th>Static Polymorphism (Generics)</th>
					<th>Dynamic Polymorphism (<code>dyn Trait</code>)</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><strong>Method Resolution</strong></td>
					<td>Compile time</td>
					<td>Runtime</td>
			</tr>
			<tr>
					<td><strong>Binary Size</strong></td>
					<td>Slightly larger (code duplicated per type)</td>
					<td>Smaller (single function instance for all types)</td>
			</tr>
			<tr>
					<td><strong>Collections in Memory</strong></td>
					<td>Single type per vector (e.g., strictly <code>Vec&lt;i32&gt;</code>)</td>
					<td>Heterogeneous types in one vector (<code>Vec&lt;Box&lt;dyn Task&gt;&gt;</code>)</td>
			</tr>
			<tr>
					<td><strong>Execution Speed</strong></td>
					<td>Direct call (maximum performance)</td>
					<td>Indirect call via method table (small runtime overhead)</td>
			</tr>
	</tbody>
</table>

<h3 class="relative group">Heterogeneous Collections Example
    <div id="heterogeneous-collections-example" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#heterogeneous-collections-example" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>To store different types inside a single vector, we wrap them in a <code>Box</code> pointer and specify the <code>dyn</code> keyword:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Task</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">execute</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">);</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">NetworkTask</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">DbTask</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Implementing Task for these structs...
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Storing different types in a single vector
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">tasks</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">Box</span><span class="o">&lt;</span><span class="k">dyn</span><span class="w"> </span><span class="n">Task</span><span class="o">&gt;&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">NetworkTask</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">DbTask</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">task</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">tasks</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">task</span><span class="p">.</span><span class="n">execute</span><span class="p">();</span><span class="w"> </span><span class="c1">// Which exact function executes is resolved at runtime
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Note
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>If the exhaustive list of all possible types is known upfront and won&rsquo;t expand dynamically, a standard <code>enum</code> serves as a great alternative to <code>dyn Trait</code>.</p></div></div><hr>

<h2 class="relative group">2. Type Sizes and <code>?Sized</code>
    <div id="2-type-sizes-and-sized" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-type-sizes-and-sized" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>In Rust, almost every type has a fixed size in bytes known to the compiler at compile time. These are called <code>Sized</code> types. For example, <code>i64</code> always takes 8 bytes, and a struct of two <code>i32</code> fields takes 8 bytes. The compiler knows exactly how much stack space to allocate.</p>
<p>However, some types have a size that can only be determined at runtime. These are called <strong>DSTs (Dynamically Sized Types)</strong>:</p>
<ul>
<li>String slices <code>str</code></li>
<li>Array slices <code>[T]</code></li>
<li>Trait objects like <code>dyn Task</code></li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="caution">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
<path fill="currentColor"  d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-2.8-5.6-5.6-11.2-9.8-16.8l-50.6 58.8s-81.4-103.6-87.1-110.6C133.1 243.8 112 273.2 112 306.8C112 375.4 162.6 416 225.7 416z"/></svg></span></div>
        <div class="grow">
          Caution
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Variables of DST types cannot be stored directly on the stack or passed by value. A variable declaration like <code>let x: dyn Task</code> will fail to compile because Rust does not know how many bytes to allocate on the stack. Dynamic types must always live behind pointers or references: <code>&amp;str</code>, <code>&amp;[T]</code>, <code>Box&lt;dyn Task&gt;</code>.</p></div></div><p>By default, the compiler assumes every generic parameter must have a fixed size (<code>T: Sized</code>). If we want to write a function that works with dynamic types by reference, we must explicitly relax this constraint using the <code>?Sized</code> bound (&ldquo;size may be unknown&rdquo;):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// T can be either fixed-size or dynamically-sized (e.g., &amp;str or &amp;dyn Debug)
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_val</span><span class="o">&lt;</span><span class="n">T</span>: <span class="o">?</span><span class="nb">Sized</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">std</span>::<span class="n">fmt</span>::<span class="n">Debug</span><span class="o">&gt;</span><span class="p">(</span><span class="n">val</span>: <span class="kp">&amp;</span><span class="nc">T</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">val</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">3. Fat Pointers and the Method Table (vtable)
    <div id="3-fat-pointers-and-the-method-table-vtable" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-fat-pointers-and-the-method-table-vtable" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>A standard reference on a 64-bit architecture (like <code>&amp;i32</code> or <code>Box&lt;u64&gt;</code>) takes <strong>8 bytes</strong> (one machine word) in memory. It is simply a memory address.</p>
<p>However, a reference to a dynamic type like <code>&amp;dyn Task</code> or <code>Box&lt;dyn Task&gt;</code> takes <strong>16 bytes (two machine words)</strong>. Because of this doubled size, they are called <strong>fat pointers</strong>.</p>
<p>What do these 16 bytes contain?</p>
<ol>
<li><strong>First 8 bytes:</strong> Address of the actual data (where the struct lives on the heap or stack).</li>
<li><strong>Second 8 bytes:</strong> Address of the virtual method table (<code>vtable</code>) for that concrete type.</li>
</ol>
<div style="background-color:white; padding: 20px">
<pre class="not-prose mermaid">
graph TD
    subgraph FatPointer["Fat Pointer (16 bytes)"]
        direction LR
        DataPtr["1. Data Pointer (8 bytes)"]
        VtablePtr["2. vtable Pointer (8 bytes)"]
    end

    DataMemory["<b>Data in Memory</b><br/>──────────────────────<br/>NetworkTask<br/><i>(object payload)</i>"]
    VtableMemory["<b>Virtual Method Table (vtable)</b><br/>──────────────────────<br/>vtable for NetworkTask<br/>• Destructor function (Drop)<br/>• Object size in bytes (size)<br/>• Memory alignment (align)<br/>• Pointer to execute() method"]

    DataPtr --> DataMemory
    VtablePtr --> VtableMemory

    style FatPointer fill:#f4f4f6,stroke:#666,stroke-width:2px,color:#000
    style DataPtr fill:#fff,stroke:#333,color:#000
    style VtablePtr fill:#fff,stroke:#333,color:#000
    style DataMemory fill:#e8f4f8,stroke:#0288d1,stroke-width:2px,color:#000
    style VtableMemory fill:#efebe9,stroke:#5d4037,stroke-width:2px,color:#000
</pre>

</div>

<h3 class="relative group">Virtual Method Table (vtable) in Practice
    <div id="virtual-method-table-vtable-in-practice" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#virtual-method-table-vtable-in-practice" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>For every type implementing a trait, the compiler constructs a small static <code>vtable</code> during compilation. It contains:</p>
<ul>
<li>Address of the cleanup function (<code>Drop</code>).</li>
<li>Object size and alignment requirements (so <code>Box</code> knows how many bytes to return to the OS on deallocation).</li>
<li>Addresses of all methods declared in the trait.</li>
</ul>
<p>When we invoke a method on a trait object:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">task</span>: <span class="nb">Box</span><span class="o">&lt;</span><span class="k">dyn</span><span class="w"> </span><span class="n">Task</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">NetworkTask</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">task</span><span class="p">.</span><span class="n">execute</span><span class="p">();</span></span></span></code></pre></div></div>
<p>The following steps occur under the hood:</p>
<ol>
<li>The program reads the second pointer from the fat pointer to locate the <code>vtable</code>.</li>
<li>It looks up the function address for <code>NetworkTask::execute</code> from the table slot.</li>
<li>It calls the function, passing the data address (from the first half of the fat pointer) as the first <code>&amp;self</code> argument.</li>
</ol>
<hr>

<h2 class="relative group">4. Combining Traits and Why <code>dyn (Fly + Swim)</code> Is Disallowed
    <div id="4-combining-traits-and-why-dyn-fly--swim-is-disallowed" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-combining-traits-and-why-dyn-fly--swim-is-disallowed" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>In Rust, you cannot directly construct a trait object from two independent traits:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Compilation error!
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">handle</span><span class="p">(</span><span class="n">obj</span>: <span class="kp">&amp;</span><span class="p">(</span><span class="k">dyn</span><span class="w"> </span><span class="n">Fly</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">Swim</span><span class="p">))</span><span class="w"> </span><span class="p">{}</span></span></span></code></pre></div></div>
<p>The reason is simple: supporting this would require a &ldquo;three-word&rdquo; fat pointer (data address + vtable1 + vtable2), adding memory overhead and complexity to pointer layouts.</p>

<h3 class="relative group">How to Achieve This in Code?
    <div id="how-to-achieve-this-in-code" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#how-to-achieve-this-in-code" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>We create a single supertrait that inherits the required traits, and then provide a <strong>Blanket Implementation</strong>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Fly</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">fly</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">String</span><span class="p">;</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Swim</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">swim</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">String</span><span class="p">;</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Combine constraints into a single trait
</span></span></span><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Duck</span>: <span class="nc">Fly</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">Swim</span><span class="w"> </span><span class="p">{}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Blanket implementation for any type implementing both Fly and Swim
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="o">&lt;</span><span class="n">T</span>: <span class="nc">Fly</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">Swim</span><span class="o">&gt;</span><span class="w"> </span><span class="n">Duck</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">T</span><span class="w"> </span><span class="p">{}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Now dyn Duck is completely valid and takes standard 16 bytes!
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">handle_duck</span><span class="p">(</span><span class="n">duck</span>: <span class="kp">&amp;</span><span class="nc">dyn</span><span class="w"> </span><span class="n">Duck</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Duck: </span><span class="si">{}</span><span class="s">, </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">duck</span><span class="p">.</span><span class="n">fly</span><span class="p">(),</span><span class="w"> </span><span class="n">duck</span><span class="p">.</span><span class="n">swim</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">5. Object Safety Rules
    <div id="5-object-safety-rules" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-object-safety-rules" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Not every trait can be converted into a <code>dyn Trait</code> object. For a trait to support dynamic polymorphism, it must adhere to <strong>Object Safety</strong> rules.</p>
<p>These rules become intuitive once you remember how <code>vtable</code> works: a virtual table can only store methods that have a fixed function pointer signature and receive a pointer to data.</p>

<h3 class="relative group">Why Do Certain Methods Break Object Safety?
    <div id="why-do-certain-methods-break-object-safety" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#why-do-certain-methods-break-object-safety" aria-label="Anchor">#</a>
    </span>
    
</h3>
<ol>
<li><strong>Method takes <code>self</code> by value (<code>fn run(self)</code>):</strong> Passing by value requires placing the object on the stack, but the byte size of <code>dyn Trait</code> is not known at compile time.</li>
<li><strong>Method uses generics (<code>fn process&lt;T&gt;(&amp;self, val: T)</code>):</strong> The compiler generates a distinct function instance for every <code>T</code>. Storing infinite function variants in a fixed <code>vtable</code> is impossible.</li>
<li><strong>Method lacks a <code>self</code> parameter (<code>fn new() -&gt; Self</code>):</strong> Without a <code>self</code> argument, the program cannot dispatch to a specific vtable instance because there is no object instance to inspect.</li>
<li><strong>Method returns or takes <code>Self</code>:</strong> At runtime, different concrete structs may hide behind <code>dyn Trait</code>, so Rust cannot guarantee type safety when comparing or combining them.</li>
</ol>

<h3 class="relative group">Practical Solution with <code>where Self: Sized</code>
    <div id="practical-solution-with-where-self-sized" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#practical-solution-with-where-self-sized" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>If you need constructors or generic methods for static dispatch, but still want to use the trait via <code>dyn Trait</code>, add a <code>where Self: Sized</code> bound to problematic methods:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">SafeTrait</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">name</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">String</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// This method takes Self by value and would break Object Safety.
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// The `where Self: Sized` bound excludes it from the vtable!
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">copy_by_val</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">Self</span><span class="w"> </span><span class="k">where</span><span class="w"> </span><span class="bp">Self</span>: <span class="nb">Sized</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>With <code>where Self: Sized</code>, the compiler knows that this method is only available during static typing, so it simply omits it from the <code>dyn SafeTrait</code> vtable. The trait remains object-safe!</p>
<p>Let&rsquo;s see how all these mechanisms work in practice in our interactive code slides:</p>
### 💻 Trait Objects and Dynamic Polymorphism


#### Step 1


```rust
trait Task {
    fn execute(&self) -> String;
}

struct NetworkTask;
impl Task for NetworkTask {
    fn execute(&self) -> String { "Network".to_string() }
}

struct DbTask;
impl Task for DbTask {
    fn execute(&self) -> String { "Database".to_string() }
}

fn main() {
    // Vector holds different types under a common trait object
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(NetworkTask),
        Box::new(DbTask),
    ];
    for t in tasks {
        println!("Executing task: {}", t.execute());
    }
}
```


**Dynamic Polymorphism and dyn Trait**
- Trait object `dyn Task` enables working with heterogeneous collections where types are known only at runtime.
- For this reason, objects must live behind a pointer (e.g., `Box<dyn Task>` or `&dyn Task`).
- Invoked method resolution occurs at runtime via the virtual method table (vtable).


#### Step 2


```rust
// By default all generics have an implicit T: Sized bound.
// We relax it with ?Sized so the function can accept slices or dyn Trait.
fn print_debug<T: ?Sized + std::fmt::Debug>(val: &T) {
    println!("Value: {:?}", val);
}

fn main() {
    let s: &str = "Hello, world!"; // str is a DST (Dynamically Sized Type)
    print_debug(s);
}
```


**Implicit Sized bound and ?Sized**
- Most types have a size known at compile time (Sized) and can live on the stack.
- Dynamically Sized Types (DSTs), such as `str`, `[T]`, or `dyn Trait`, do not have a fixed size.
- Syntax `?Sized` relaxes the implicit fixed-size requirement, allowing DSTs to be passed by reference.


#### Step 3


```rust
trait Fly {
    fn fly(&self) -> String;
}

trait Swim {
    fn swim(&self) -> String;
}

// Rust forbids &(dyn Fly + Swim) directly.
// We create a combined supertrait:
trait Duck: Fly + Swim {}

// And write a Blanket Implementation for all matching types:
impl<T: Fly + Swim> Duck for T {}

struct Mallard;
impl Fly for Mallard {
    fn fly(&self) -> String { "Flying".to_string() }
}
impl Swim for Mallard {
    fn swim(&self) -> String { "Swimming".to_string() }
}

fn handle_duck(duck: &dyn Duck) {
    println!("Duck: {}, {}", duck.fly(), duck.swim());
}

fn main() {
    let d = Mallard;
    handle_duck(&d);
}
```


**Multi-Trait Objects**
- In Rust, you cannot directly combine multiple independent traits into a single trait object: `dyn Trait1 + Trait2` will not compile.
- As a workaround, we construct a supertrait `Duck` inheriting from both required traits.
- Blanket implementation automatically makes any matching type implement `Duck`.


#### Step 4


```rust
// Trait contains a `copy_by_val` method returning Self by value.
// This breaks Object Safety (dynamic compatibility).
// But we isolate it using `where Self: Sized`!
trait SafeTrait {
    fn name(&self) -> String;
    
    // This method will not be included in the vtable and won't break dyn SafeTrait
    fn copy_by_val(self) -> Self where Self: Sized {
        self
    }
}

struct Item;
impl SafeTrait for Item {
    fn name(&self) -> String { "Item".to_string() }
}

fn print_name(obj: &dyn SafeTrait) {
    println!("Name: {}", obj.name());
    // obj.copy_by_val(); // Error! Method unavailable for trait object.
}

fn main() {
    let item = Item;
    print_name(&item);
}
```


**Object Safety**
- Trait objects can only be created for Object-Safe traits (no Self by value, no generic methods, etc.).
- The `where Self: Sized` constraint allows excluding problematic methods from the vtable.
- The trait remains object-safe, while "unsafe" methods can only be invoked on static types.


<hr>

<h2 class="relative group">Test Your Knowledge!
    <div id="test-your-knowledge" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#test-your-knowledge" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Take a short quiz to test your understanding of trait objects, fat pointers, and dynamic polymorphism in Rust.</p>
### 📝 Trait Objects and Dynamic Polymorphism

**1. What is the key difference between DSTs (Dynamically Sized Types) and regular Sized types?**

- [ ] DST types can only be located on the stack, while Sized types are on the heap
- [x] DSTs have a size unknown at compile time. They cannot be stored directly on the stack or passed by value; they must be accessed behind pointers
- [ ] The compiler generates separate copies of functions for DST types during monomorphization

> 💡 *The size of DST types (such as `str`, `[T]`, or `dyn Trait`) is determined only at runtime, so the compiler cannot allocate a fixed stack frame for them. They are always used behind pointers.*


**2. What does a Fat Pointer to a trait object consist of in Rust?**

- [x] A pointer to the object's data in memory and a pointer to the virtual method table (vtable)
- [ ] A pointer to the data and the length of the object (number of elements)
- [ ] A pointer to the vtable and the number of methods in the trait

> 💡 *A fat pointer to a trait object takes 16 bytes (two machine words on 64-bit architecture) and contains the address of the object itself on the heap or stack plus the address of the static vtable.*


**3. Which information is NOT stored in the vtable for a trait object?**

- [ ] A pointer to the destructor (Drop) for cleaning up memory
- [ ] The size and alignment requirements of the concrete type in memory
- [x] String names of methods for dynamic lookup by name at runtime
- [ ] Addresses of compiled methods implementing the trait

> 💡 *Vtables do not contain method names. Function calls are resolved via fixed numeric offsets (indices in the vtable) calculated at compile time, providing high performance.*


**4. Which trait property violates Object Safety and prevents creating a `dyn Trait`?**

- [ ] Having methods that accept a mutable reference `&mut self`
- [ ] Having associated types
- [x] Having static methods (without a `self` argument) that lack a `where Self: Sized` bound

> 💡 *Functions without `self` cannot be invoked on a trait object at runtime because the compiler cannot retrieve a vtable without an object instance. If such a method is not isolated via `where Self: Sized`, the trait becomes object-unsafe.*


**5. How does the compiler know how many bytes of memory to free in the allocator when dropping `Box<dyn Trait>`?**

- [ ] The size is calculated by scanning the heap memory bytes
- [ ] The allocator tracks all allocation sizes without Rust's involvement
- [x] The compiler reads the size and alignment of the concrete type from the object's vtable

> 💡 *The vtable of a trait object stores metadata of the concrete type (size and align). When destroying `Box<dyn Trait>`, the runtime retrieves these values from the vtable and passes them to `__rust_dealloc(ptr, size, align)`.*




