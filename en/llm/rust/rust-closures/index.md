# Closures in Rust: Functions with their own Memory

> A simple analysis of function pointers and closures in Rust. Learn how closures capture environment, the difference between Fn, FnMut, and FnOnce, and how they are structured under the hood.

- **URL:** https://eagle2000.space/en/rust/rust-closures/
- **Date:** 2026-07-16

- **Tags:** rust, dev, closures, advanced

---


<h2 class="relative group">Introduction
    <div id="introduction" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#introduction" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Closures (or lambda expressions) are anonymous functions that we can create on the fly and pass to other functions. But their main superpower is the ability to <strong>capture environment</strong>, meaning they remember variables from the scope where they were declared.</p>
<p>In programming languages with automatic garbage collection (GC), developers don&rsquo;t even think about how closures store these variables. But in Rust, where memory is managed by the compiler, the rules for closures are governed by strict ownership and borrowing semantics.</p>
<p>In this article, we will break down how closures work in Rust, how they differ from ordinary functions, how they are divided into <code>Fn</code>, <code>FnMut</code>, and <code>FnOnce</code> traits, and how the compiler generates them &ldquo;under the hood.&rdquo;</p>
<hr>

<h2 class="relative group">1. Function Pointers (<code>fn</code>)
    <div id="1-function-pointers-fn" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-function-pointers-fn" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Before we talk about closures, let&rsquo;s recall ordinary functions.</p>
<p>In Rust, a function&rsquo;s name can act as a pointer to it. The type of such a pointer is written in lowercase: for example, <code>fn(i32) -&gt; i32</code>.</p>
<ul>
<li><strong>How it is structured in memory:</strong>
<ul>
<li>The <code>fn</code> pointer is a <strong>&ldquo;thin&rdquo; pointer</strong>. It stores only one address — the address of the first instruction of the compiled code in memory.</li>
<li>Such functions <strong>have no state</strong>. They cannot look at outer variables or read anything that wasn&rsquo;t passed to them in the arguments.</li>
</ul>
</li>
</ul>
<hr>

<h2 class="relative group">2. Closures: Functions with a backpack
    <div id="2-closures-functions-with-a-backpack" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-closures-functions-with-a-backpack" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>A closure is an anonymous function that has an invisible &ldquo;backpack&rdquo; (context) behind its back, where it can put external variables.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">coefficient</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">10</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">multiply</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="o">|</span><span class="n">x</span><span class="o">|</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="n">coefficient</span><span class="p">;</span><span class="w"> </span><span class="c1">// The closure has captured coefficient!
</span></span></span></code></pre></div></div>
<p>Closure arguments are written inside two vertical bars <code>|...|</code>, followed by the body. Curly braces are only needed for multi-line code.</p>

<h3 class="relative group">Type Inference
    <div id="type-inference" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#type-inference" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The Rust compiler automatically infers what data types the closure accepts and returns, analyzing the place of its first invocation. However, if you want, you can specify the types manually:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">multiply</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="o">|</span><span class="n">x</span>: <span class="kt">i32</span><span class="o">|</span><span class="w"> </span>-&gt; <span class="kt">i32</span> <span class="p">{</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="n">coefficient</span><span class="w"> </span><span class="p">};</span></span></span></code></pre></div></div>

<h3 class="relative group">Capturing Environment and the <code>move</code> keyword
    <div id="capturing-environment-and-the-move-keyword" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#capturing-environment-and-the-move-keyword" aria-label="Anchor">#</a>
    </span>
    
</h3>
<ul>
<li><strong>By default (by reference):</strong> The closure tries to be as efficient as possible. It doesn&rsquo;t copy variables; it borrows them. The compiler decides what kind of reference is needed: a shared reference for reading (<code>&amp;T</code>) or a mutable reference for writing (<code>&amp;mut T</code>).</li>
<li><strong>Forced move (<code>move</code>):</strong> If the closure must outlive the surrounding code (for example, we return it from a function or pass it to another thread), we need to move ownership of the variables inside the closure. For this, write the keyword <code>move</code> before the vertical bars:
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">print_data</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">move</span><span class="w"> </span><span class="o">||</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Data: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">data</span><span class="p">);</span></span></span></code></pre></div></div>
</li>
</ul>
<hr>

<h2 class="relative group">3. Three Closure Traits: Fn, FnMut, and FnOnce
    <div id="3-three-closure-traits-fn-fnmut-and-fnonce" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-three-closure-traits-fn-fnmut-and-fnonce" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Depending on what exactly the closure does with the variables in its &ldquo;backpack,&rdquo; it automatically implements one of three traits:</p>
<div style="background-color:white; padding: 20px">
<pre class="not-prose mermaid">
graph TD
    FnOnce --> FnMut
    FnMut --> Fn
    
    style FnOnce fill:#f9f,stroke:#333,stroke-width:2px,color:#000
    style FnMut fill:#ccf,stroke:#333,stroke-width:1px,color:#000
    style Fn fill:#cfc,stroke:#333,stroke-width:1px,color:#000
</pre>

</div>
<ol>
<li><strong><code>FnOnce</code> (The Consumer)</strong>
<ul>
<li><strong>What it does:</strong> Takes captured variables by value and consumes them (destroys or passes them on).</li>
<li><strong>Limitation:</strong> Since the data is consumed on the first call, you can call such a closure <strong>only once</strong>.</li>
</ul>
</li>
<li><strong><code>FnMut</code> (The Mutator)</strong>
<ul>
<li><strong>What it does:</strong> Modifies captured variables (uses an exclusive reference <code>&amp;mut self</code>).</li>
<li><strong>Limitation:</strong> Can be called any number of times, but only sequentially (cannot run concurrently from different threads without synchronization).</li>
</ul>
</li>
<li><strong><code>Fn</code> (The Reader)</strong>
<ul>
<li><strong>What it does:</strong> Only reads data (uses a shared reference <code>&amp;self</code>).</li>
<li><strong>Limitation:</strong> No limitations! Can be called repeatedly and concurrently from different threads.</li>
</ul>
</li>
</ol>

<h3 class="relative group">Trait Hierarchy
    <div id="trait-hierarchy" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#trait-hierarchy" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>They form a nested structure:</p>
<ul>
<li>Any closure of type <code>Fn</code> automatically knows how to work as <code>FnMut</code> and <code>FnOnce</code>.</li>
<li>Regular function pointers (<code>fn</code>) have no state, so they always implement the top trait <code>Fn</code>.</li>
</ul>
<hr>

<h2 class="relative group">4. How Closures Work in Code
    <div id="4-how-closures-work-in-code" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-how-closures-work-in-code" aria-label="Anchor">#</a>
    </span>
    
</h2>

<h3 class="relative group">Passing to Functions
    <div id="passing-to-functions" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#passing-to-functions" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Since the exact type of a closure is anonymous, we use generics or <code>impl Trait</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">execute</span><span class="p">(</span><span class="k">mut</span><span class="w"> </span><span class="n">action</span>: <span class="nc">impl</span><span class="w"> </span><span class="nb">FnMut</span><span class="p">())</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">action</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Returning from Functions
    <div id="returning-from-functions" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#returning-from-functions" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>You cannot return a closure directly — the compiler doesn&rsquo;t know its exact size. Therefore, we return it via <code>impl Trait</code> with the mandatory <code>move</code> keyword (to take the captured variables with us):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">create_adder</span><span class="p">(</span><span class="n">x</span>: <span class="kt">i32</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">impl</span><span class="w"> </span><span class="nb">Fn</span><span class="p">(</span><span class="kt">i32</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">i32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">move</span><span class="w"> </span><span class="o">|</span><span class="n">y</span><span class="o">|</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">y</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Storing in Structs
    <div id="storing-in-structs" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#storing-in-structs" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>If a struct needs to store a closure, we have two ways:</p>
<ol>
<li><strong>Generics (fast):</strong> The struct is statically bound to the closure type at compile time. No overhead.
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Runner</span><span class="o">&lt;</span><span class="n">F</span>: <span class="nb">Fn</span><span class="p">()</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">action</span>: <span class="nc">F</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>
</li>
<li><strong>Box (flexible):</strong> The closure is hidden in the heap under a dynamic pointer. Allows storing different closures in the same struct type.
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Runner</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">action</span>: <span class="nb">Box</span><span class="o">&lt;</span><span class="k">dyn</span><span class="w"> </span><span class="nb">Fn</span><span class="p">()</span><span class="o">&gt;</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>
</li>
</ol>
<hr>

<h2 class="relative group">5. What happens under the hood?
    <div id="5-what-happens-under-the-hood" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-what-happens-under-the-hood" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Closures in Rust are just syntactic sugar. Under the hood, the compiler turns each closure into a regular structure:</p>
<ol>
<li>An <strong>anonymous struct</strong> is created, whose fields are the captured variables.</li>
<li>The implementation of the call (methods of traits <code>call</code>, <code>call_mut</code>, or <code>call_once</code>) is generated for this structure.</li>
<li>At the site of the closure call, the compiler simply substitutes a regular method call of this structure.</li>
</ol>
<hr>

<h2 class="relative group">6. Lifetimes on the fly (HRTBs)
    <div id="6-lifetimes-on-the-fly-hrtbs" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#6-lifetimes-on-the-fly-hrtbs" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Sometimes a complex situation arises: a closure accepts a reference to an object, but the lifetime of this reference is born and dies right inside the calling function. In such cases, the closure must be able to work with references of <strong>any</strong> lifetime.</p>
<p>For this, Higher-Rank Trait Bounds (or HRTBs) syntax is used via the <code>for&lt;'a&gt;</code> keyword:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">run_with_ref</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span><span class="p">(</span><span class="n">f</span>: <span class="nc">F</span><span class="p">)</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">where</span><span class="w"> </span><span class="n">F</span>: <span class="nc">for</span><span class="o">&lt;</span><span class="na">&#39;a</span><span class="o">&gt;</span><span class="w"> </span><span class="nb">Fn</span><span class="p">(</span><span class="o">&amp;</span><span class="na">&#39;a</span><span class="w"> </span><span class="kt">str</span><span class="p">)</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">local</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;data&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">f</span><span class="p">(</span><span class="o">&amp;</span><span class="n">local</span><span class="p">);</span><span class="w"> </span><span class="c1">// The lifetime of the reference is limited only to this function!
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>The construction <code>for&lt;'a&gt; Fn(&amp;'a str)</code> tells the compiler: &ldquo;This closure knows how to accept a string reference with absolutely any lifetime <code>'a</code> that we give it at the moment of call.&rdquo;</p>
<p>Let&rsquo;s see how closures of different types work in the interactive examples below:</p>
### 💻 Closures in Rust


#### Step 1


```rust
fn call_with_five(f: fn(i32) -> i32) {
    println!("Fn pointer call: {}", f(5));
}

fn main() {
    // 1. Function pointer to a standard function
    fn add_one(x: i32) -> i32 { x + 1 }
    call_with_five(add_one);
    
    // 2. Closure capturing environment
    let coefficient = 10;
    let closure = |x| x * coefficient;
    
    // call_with_five(closure); // Error! Closure with state cannot be coerced to a plain fn pointer.
    println!("Closure call: {}", closure(5));
}
```


**Function Pointers and Closures**
- Function pointer `fn` is a "thin" pointer: it stores only the machine code memory address.
- Closure (anonymous function) can capture external variables and carry payload state.
- Closures receive a unique anonymous type generated by the compiler.


#### Step 2


```rust
fn call_fn<F: Fn()>(f: F) { f(); }
fn call_fn_mut<F: FnMut()>(mut f: F) { f(); }
fn call_fn_once<F: FnOnce()>(f: F) { f(); }

fn main() {
    let mut greeting = "Hello".to_string();
    
    // Fn: only reads environment
    let closure_fn = || println!("Fn: {}", greeting);
    call_fn(closure_fn);
    
    // FnMut: mutates environment
    let closure_mut = || {
        greeting.push_str(" World");
        println!("FnMut: {}", greeting);
    };
    call_fn_mut(closure_mut);
    
    // FnOnce: consumes (moves) ownership of greeting
    let closure_once = move || {
        let consumed = greeting;
        println!("FnOnce: {}", consumed);
    };
    call_fn_once(closure_once);
    // call_fn_once(closure_once); // Error! Variable closure_once was already consumed.
}
```


**Traits Fn, FnMut, and FnOnce**
- `Fn`: captures via shared reference `&self`. Allows concurrent multi-threaded execution.
- `FnMut`: captures via exclusive reference `&mut self`. Allows mutating captured state.
- `FnOnce`: consumes captured state. Can be called exactly once.


#### Step 3


```rust
fn get_counter(mut start: u32) -> impl FnMut() -> u32 {
    move || {
        start += 1;
        start
    }
}

fn main() {
    let mut next = get_counter(100);
    println!("Counter: {}", next());
    println!("Counter: {}", next());
}
```


**Returning Closures from Functions**
- Because closure types are anonymous, we use `impl Trait` return position syntax.
- We must use the `move` keyword to transfer ownership of captured variables.
- Otherwise the returned closure would reference local variables destroyed upon function exit.


#### Step 4


```rust
fn process_borrowed<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let local_string = String::from("Local Data");
    f(&local_string); // Reference lives strictly inside function
}
fn main() {
    // Closure accepts a string slice whose lifetime is determined inside process_borrowed
    let print_len = |s: &str| println!("String length: {}", s.len());
    process_borrowed(print_len);
}
```


<hr>

<h2 class="relative group">Check Your Knowledge!
    <div id="check-your-knowledge" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#check-your-knowledge" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Take a short interactive test to reinforce your understanding of closures in Rust.</p>
### 📝 Closures in Rust

**1. What is the main low-level difference between a function pointer `fn` and a closure?**

- [ ] The `fn` pointer runs slower than closures
- [x] The `fn` pointer is a thin pointer and cannot contain external state (context), whereas a closure can capture variables from the environment
- [ ] Closures are always allocated on the heap, while `fn` pointers are on the stack

> 💡 *The `fn` type stores only the address of the machine code. Closures, on the other hand, compile into an anonymous structure storing values or references to the captured variables.*


**2. What is the inheritance (subtype) hierarchy between closure traits?**

- [ ] FnOnce is a subtype of FnMut, which is a subtype of Fn
- [x] Fn is a subtype of FnMut, which is a subtype of FnOnce
- [ ] These traits are independent and not related to each other

> 💡 *Any closure of type `Fn` can be used where `FnMut` or `FnOnce` is expected, as it imposes the least restrictive constraints on state (only immutable reading).*


**3. What happens if you apply the `move` keyword to a closure?**

- [ ] The closure will be moved to another execution thread
- [x] All external variables mentioned in the closure will be forced to be captured by value (ownership will move inside the closure)
- [ ] The compiler will forbid calling the closure more than once

> 💡 *`move` disables default capture by reference and forces the closure to take ownership of the captured variables. This is critical if the closure needs to outlive the scope of its creator.*


**4. Why are Higher-Rank Trait Bounds (HRTBs, `for<'a>` syntax) used in closure signatures?**

- [ ] To execute the closure concurrently in multiple threads
- [ ] To remove restrictions on recursive closure calls
- [x] To bind lifetimes so that the closure can accept references with absolutely any lifetime determined at runtime at the moment of invocation

> 💡 *HRTBs allow placing a universal quantifier over the lifetimes of closure parameters, guaranteeing to the compiler that it will work for any references passed by the calling function.*




