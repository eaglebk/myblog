# Traits in Rust: From Fundamentals to Monomorphic Zero-Cost Code

> A comprehensive guide to traits in Rust: from basic syntax and impl blocks to default methods, generic constraints, impl Trait syntax, and zero-cost monomorphization.

- **URL:** https://eagle2000.space/en/rust/rust-traits/
- **Date:** 2026-08-07

- **Tags:** rust, dev, traits, generics, middle

---


<h2 class="relative group">Introduction: Why Do We Need Traits?
    <div id="introduction-why-do-we-need-traits" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#introduction-why-do-we-need-traits" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Imagine we are building a fitness application with different exercise types: <code>Pushups</code> and <code>Plank</code>. Both need to calculate calories burned, but do so using different logic:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Pushups</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">reps</span>: <span class="kt">u32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Plank</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">duration_sec</span>: <span class="kt">u32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Pushups</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">reps</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">1</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Plank</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">duration_sec</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">2</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Now we face a software engineering problem—writing a single function to calculate total calories burned across any workout exercises. Without abstractions, we would be forced to duplicate the function implementation for every distinct type.</p>
<hr>

<h3 class="relative group">How Rust Solves This Problem
    <div id="how-rust-solves-this-problem" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#how-rust-solves-this-problem" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>In many programming languages, this problem is solved using interfaces or base classes. However, without explicit type bounds, error messages often pop up deep inside template instantiation code, accompanied by massive compiler tracebacks.</p>
<p><strong>Rust solves this problem using Traits</strong>—explicit behavioral contracts.</p>
<p>First, we declare the <code>Workout</code> behavioral contract:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Workout</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Next, we declare that our structs implement this trait:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Workout</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Pushups</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">reps</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">1</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Workout</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Plank</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">duration_sec</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">2</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Now we can write <strong>a single unified function</strong> using a trait bound (<code>T: Workout</code>):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">total_calories</span><span class="o">&lt;</span><span class="n">T</span>: <span class="nc">Workout</span><span class="o">&gt;</span><span class="p">(</span><span class="n">exercises</span>: <span class="kp">&amp;</span><span class="p">[</span><span class="n">T</span><span class="p">])</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">exercises</span><span class="p">.</span><span class="n">iter</span><span class="p">().</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">e</span><span class="o">|</span><span class="w"> </span><span class="n">e</span><span class="p">.</span><span class="n">calories_burned</span><span class="p">()).</span><span class="n">sum</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">What Happens Under the Hood?
    <div id="what-happens-under-the-hood" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#what-happens-under-the-hood" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>The compiler does not invoke a single dynamic virtual method at runtime. Instead, it generates a specialized function copy for every concrete type at compile time (<strong>monomorphization</strong>). As a result, you gain clean architectural flexibility with the raw speed of pure C (Zero-cost abstractions).</p>
<p>Try our step-by-step interactive example below: in step 1, you will observe a compiler error when attempting to call a method on an unbounded type <code>T</code>, and in step 2—its resolution via the <code>Workout</code> trait:</p>
### 💻 Example: Traits in Practice (Fitness Workouts)


#### Step 1


```rust
// ?compile_fail
struct Pushups {
    reps: u32,
}

impl Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

// Attempting to write a generic function WITHOUT a trait bound T
fn total_calories<T>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let total = total_calories(&pushups);
    println!("Total calories: {}", total);
}
```


**Step 1: Compile error without trait bound**
- Attempting to invoke method `calories_burned()` on generic type `T` without contract declaration.
- The compiler does not know if type `T` possesses such a method and blocks compilation.


#### Step 2


```rust
pub trait Workout {
    fn calories_burned(&self) -> u32;
}

struct Pushups {
    reps: u32,
}

struct Plank {
    duration_sec: u32,
}

impl Workout for Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

impl Workout for Plank {
    fn calories_burned(&self) -> u32 {
        self.duration_sec * 2
    }
}

// Unified generic function with Trait Bound constraint (T: Workout)
fn total_calories<T: Workout>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let planks = vec![Plank { duration_sec: 60 }, Plank { duration_sec: 45 }];

    println!("Pushup calories: {}", total_calories(&pushups));
    println!("Plank calories: {}", total_calories(&planks));
}
```


**Step 2: Resolution with explicit Workout trait**
- Trait `Workout` declares mandatory contract: presence of `calories_burned` method.
- Structs `Pushups` and `Plank` implement trait via `impl Workout for ...`.
- Function `total_calories` accepts any type `T: Workout`, while compiler generates optimized code for each type with zero runtime overhead.


<hr>

<h2 class="relative group">1. What Are Traits &amp; Basic Syntax
    <div id="1-what-are-traits--basic-syntax" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-what-are-traits--basic-syntax" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>A <strong>Trait</strong> is a specification of a set of methods that a type must be capable of executing. A trait itself stores no data payload; it strictly declares behavior.</p>
<p>Let&rsquo;s declare a simple trait <code>Say</code> and implement it for a <code>Dog</code> struct. By Rust convention, trait and struct names are written in <code>CamelCase</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Declare the contract: anything implementing Say can &#34;say&#34; something
</span></span></span><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Say</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">say</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Dog</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Implement the contract for Dog
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Say</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Dog</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">say</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Woof!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Now, when we instantiate a <code>Dog</code>, we can invoke the method using dot syntax:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">dog</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Dog</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">dog</span><span class="p">.</span><span class="n">say</span><span class="p">();</span><span class="w"> </span><span class="c1">// Outputs: Woof!
</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="important">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path fill="currentColor" d="M287.9 0C297.1 0 305.5 5.25 309.5 13.52L378.1 154.8L531.4 177.5C540.4 178.8 547.8 185.1 550.7 193.7C553.5 202.4 551.2 211.9 544.8 218.2L433.6 328.4L459.9 483.9C461.4 492.9 457.7 502.1 450.2 507.4C442.8 512.7 432.1 513.4 424.9 509.1L287.9 435.9L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.9 115.1 483.9L142.2 328.4L31.11 218.2C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C270.4 5.249 278.7 0 287.9 0L287.9 0zM287.9 78.95L235.4 187.2C231.9 194.3 225.1 199.3 217.3 200.5L98.98 217.9L184.9 303C190.4 308.5 192.9 316.4 191.6 324.1L171.4 443.7L276.6 387.5C283.7 383.7 292.2 383.7 299.2 387.5L404.4 443.7L384.2 324.1C382.9 316.4 385.5 308.5 391 303L476.9 217.9L358.6 200.5C350.7 199.3 343.9 194.3 340.5 187.2L287.9 78.95z"/></svg></span></div>
        <div class="grow">
          Important
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p><strong>Scope Requirement:</strong> Don&rsquo;t forget to import the trait! To call a trait method on a type, the trait itself must be brought into scope using the <code>use</code> keyword (e.g., <code>use crate::Say;</code>). Otherwise, the compiler will emit an error even if the type implements the trait.</p></div></div><hr>

<h2 class="relative group">2. Anatomy of Trait Members
    <div id="2-anatomy-of-trait-members" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-anatomy-of-trait-members" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Inside a trait, you can declare more than just instance methods.</p>

<h3 class="relative group">Instance Methods and Associated Functions
    <div id="instance-methods-and-associated-functions" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#instance-methods-and-associated-functions" aria-label="Anchor">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Instance Methods:</strong> Receive <code>self</code>, <code>&amp;self</code>, or <code>&amp;mut self</code> as their first argument. Called on an instance (<code>dog.say()</code>).</li>
<li><strong>Associated Functions:</strong> Do not take <code>self</code> as the first argument. These act as &ldquo;static methods&rdquo; called via namespace syntax (<code>Dog::species()</code>).</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Animal</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">name</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="kt">str</span><span class="p">;</span><span class="w">        </span><span class="c1">// Instance method
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">species</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="nb">&#39;static</span> <span class="kt">str</span><span class="p">;</span><span class="w"> </span><span class="c1">// Associated function without self
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Dog</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Animal</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Dog</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">name</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="kt">str</span> <span class="p">{</span><span class="w"> </span><span class="s">&#34;Rex&#34;</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">species</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="nb">&#39;static</span> <span class="kt">str</span> <span class="p">{</span><span class="w"> </span><span class="s">&#34;Canis lupus&#34;</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Associated function call:
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Species: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">Dog</span>::<span class="n">species</span><span class="p">());</span></span></span></code></pre></div></div>

<h3 class="relative group">Default Implementations
    <div id="default-implementations" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#default-implementations" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>We can provide a default function body directly inside the trait declaration. Implementing types can either inherit this default behavior or override it when needed:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Greet</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">hello</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello!&#34;</span><span class="p">);</span><span class="w"> </span><span class="c1">// Default implementation
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Student</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Greet</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Student</span><span class="w"> </span><span class="p">{}</span><span class="w"> </span><span class="c1">// Empty impl body — uses default behavior
</span></span></span></code></pre></div></div>

<h3 class="relative group">Associated Constants
    <div id="associated-constants" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#associated-constants" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Traits can declare constants whose values are specified during implementation for concrete types. Their names are written in <code>UPPER_SNAKE_CASE</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">HasId</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">const</span><span class="w"> </span><span class="no">ID</span>: <span class="kt">usize</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">User</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">HasId</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">User</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">const</span><span class="w"> </span><span class="no">ID</span>: <span class="kt">usize</span> <span class="o">=</span><span class="w"> </span><span class="mi">42</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">3. Marker Traits: Copy vs. Clone
    <div id="3-marker-traits-copy-vs-clone" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-marker-traits-copy-vs-clone" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Rust features two fundamental traits that dictate how data payloads are copied in memory:</p>
<table>
	<thead>
			<tr>
					<th>Trait</th>
					<th>Category</th>
					<th>Mechanism</th>
					<th>Usage Code</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><strong><code>Clone</code></strong></td>
					<td>Explicit Trait</td>
					<td>Creates a deep copy of an object (can allocate heap memory and run arbitrary logic)</td>
					<td>Requires explicit <code>.clone()</code> call</td>
			</tr>
			<tr>
					<td><strong><code>Copy</code></strong></td>
					<td>Marker Auto-Trait</td>
					<td>Signals bitwise memcpy capability. Changes move semantics into implicit copy semantics</td>
					<td>Occurs automatically during assignment or function passing</td>
			</tr>
	</tbody>
</table>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">a</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;abc&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">b</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">a</span><span class="p">;</span><span class="w"> </span><span class="c1">// Move (ownership moves to b, &#39;a&#39; is no longer accessible)
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">5</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">x</span><span class="p">;</span><span class="w"> </span><span class="c1">// Copy (value of x is implicitly copied, both variables accessible)
</span></span></span></code></pre></div></div>
<p>The <code>Copy</code> trait is a subtrait of <code>Clone</code> (<code>trait Copy: Clone {}</code>). You cannot make a type <code>Copy</code> without also implementing <code>Clone</code>.</p>
<ul>
<li>Primitive numeric types (<code>i32</code>, <code>f64</code>), <code>bool</code>, <code>char</code> implement <code>Copy</code> natively.</li>
<li>For custom structs, we can derive them automatically: <code>#[derive(Copy, Clone)]</code>.</li>
</ul>
<hr>

<h2 class="relative group">4. Generics, Trait Bounds, and <code>impl Trait</code>
    <div id="4-generics-trait-bounds-and-impl-trait" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-generics-trait-bounds-and-impl-trait" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p><strong>Generics (generic types)</strong> allow writing structs and functions that operate across arbitrary data types.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Point</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span>: <span class="nc">T</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">y</span>: <span class="nc">T</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Trait Bounds
    <div id="trait-bounds" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#trait-bounds" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>When we want to perform operations on generic type <code>T</code>, we apply Trait Bounds:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">impl</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="n">Point</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">where</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">T</span>: <span class="nc">std</span>::<span class="n">ops</span>::<span class="n">Add</span><span class="o">&lt;</span><span class="n">Output</span><span class="o">=</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="c1">// Restricting type T to addition contract
</span></span></span><span class="line"><span class="cl"><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">add</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">other</span>: <span class="nc">Self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">Self</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">Self</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="n">x</span>: <span class="nc">self</span><span class="p">.</span><span class="n">x</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">other</span><span class="p">.</span><span class="n">x</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="n">y</span>: <span class="nc">self</span><span class="p">.</span><span class="n">y</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">other</span><span class="p">.</span><span class="n">y</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Ergonomic <code>impl Trait</code> in Arguments
    <div id="ergonomic-impl-trait-in-arguments" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#ergonomic-impl-trait-in-arguments" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>For most standard functions in Rust, instead of verbose <code>fn foo&lt;T: Trait&gt;(x: T)</code>, a more concise syntax is preferred: <code>impl Trait</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_workout</span><span class="p">(</span><span class="n">w</span>: <span class="nc">impl</span><span class="w"> </span><span class="n">Workout</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Calories: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">w</span><span class="p">.</span><span class="n">calories_burned</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Both forms are fundamentally equivalent: the compiler transforms <code>impl Trait</code> into a generic function at compile time.</p>
<hr>

<h2 class="relative group">5. Monomorphization Mechanics
    <div id="5-monomorphization-mechanics" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-monomorphization-mechanics" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Unlike languages like Java or C# where generics are erased at runtime, or Python where everything is checked dynamically, Rust utilizes <strong>monomorphization</strong>.</p>
<p>When the compiler encounters generic code, it inspects all concrete types passed into the function calls and generates physical function copies for every distinct type during compilation.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Generic source code
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="p">(</span><span class="n">x</span>: <span class="nc">T</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">foo</span><span class="p">(</span><span class="mi">5</span><span class="p">);</span><span class="w">       </span><span class="c1">// Called with i32
</span></span></span><span class="line"><span class="cl"><span class="n">foo</span><span class="p">(</span><span class="s">&#34;Hello&#34;</span><span class="p">);</span><span class="w"> </span><span class="c1">// Called with &amp;str
</span></span></span></code></pre></div></div>
<p>Under the hood, the compiler conceptually transforms this into two concrete functions:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo_i32</span><span class="p">(</span><span class="n">x</span>: <span class="kt">i32</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo_str</span><span class="p">(</span><span class="n">x</span>: <span class="kp">&amp;</span><span class="kt">str</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>

<h4 class="relative group">Advantages of Monomorphization:
    <div id="advantages-of-monomorphization" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#advantages-of-monomorphization" aria-label="Anchor">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Zero-Cost Abstractions:</strong> Execution runs at raw machine speed because types are fixed upfront—no runtime type checks or dynamic dispatch overhead.</li>
<li><strong>Aggressive Inlining:</strong> The compiler can easily inline short function bodies directly into call sites.</li>
</ul>

<h4 class="relative group">Disadvantages of Monomorphization:
    <div id="disadvantages-of-monomorphization" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#disadvantages-of-monomorphization" aria-label="Anchor">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Slower Compile Times:</strong> The compiler must compile and optimize multiple generated function copies.</li>
<li><strong>Binary Code Bloat:</strong> Machine code size expands as duplicate function copies accumulate.</li>
</ul>
<hr>
<p>Explore traits and static polymorphism in action in our interactive code slides below:</p>
### 💻 Traits and Static Polymorphism


#### Step 1


```rust
pub trait Say {
    fn say(&self) -> String;
}

struct Dog;

impl Say for Dog {
    fn say(&self) -> String {
        "Woof!".to_string()
    }
}

fn main() {
    let dog = Dog;
    println!("Dog says: {}", dog.say());
}
```


**Trait Declaration and Implementation**
- Trait `Say` describes contract: any type implementing it must be able to speak.
- Struct `Dog` implements method `say` from trait `Say`.
- After implementation, method becomes available on the struct using dot notation.


#### Step 2


```rust
pub trait HasId {
    const ID: usize;
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> where Self: Sized;
}

struct MyStruct;

impl HasId for MyStruct {
    const ID: usize = 10;
    type Err = std::convert::Infallible;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MyStruct)
    }
}

fn main() {
    println!("Struct ID: {}", MyStruct::ID);
    let _s = MyStruct::from_str("test").unwrap();
}
```


**Associated Items**
- Constants (`const ID`) and types (`type Err`) are declared inside trait and refined in implementations.
- Associated type acts as a type placeholder concrete upon implementation.
- Avoids polluting trait method signatures with extra generic parameters.


#### Step 3


```rust
use std::fmt;
// We cannot implement Display for Vec<i32> directly as both are foreign types.
// But we can wrap Vec inside our custom tuple struct (Newtype)!
struct MyVec(Vec<i32>);

impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My vector with {} elements", self.0.len())
    }
}

fn main() {
    let my_vec = MyVec(vec![100, 200, 300]);
    println!("{}", my_vec);
}
```


**Newtype Pattern (Wrapper)**
- Orphan Rules forbid implementing foreign traits for foreign types.
- Creating wrapper struct `MyVec` makes the type local.
- Now we can implement any external trait for it (such as `Display`).


#### Step 4


```rust
// Trait used as a constraint
trait Loud {
    fn loud_say(&self) -> String;
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Blanket implementation: implement Loud for any Point<T> if T implements Loud
impl<T: Loud> Loud for Point<T> {
    fn loud_say(&self) -> String {
        format!("X: {}, Y: {}", self.x.loud_say(), self.y.loud_say())
    }
}

struct Dog;
impl Loud for Dog {
    fn loud_say(&self) -> String {
        "WOOF!".to_string()
    }
}

fn main() {
    let p = Point { x: Dog, y: Dog };
    println!("Point shouts: {}", p.loud_say());
}
```


**Generics and Blanket Implementations**
- Trait bounds `T: Loud` guarantee that type `T` supports required operations.
- Blanket implementation allows implementing trait for a whole family of matching types at once.
- Compiler monomorphizes generic code, generating separate functions for each concrete type.


#### Step 5


```rust
use std::mem::size_of;
trait A {
    fn get(&self) -> &str;
}

trait B {
    fn get(&self) -> &str;
}

struct S;

impl A for S {
    fn get(&self) -> &str { "implementation A" }
}

impl B for S {
    fn get(&self) -> &str { "implementation B" }
}

fn main() {
    let s = S;
    // s.get(); // Compile error: method name get is ambiguous!
    
    // Fully Qualified Syntax resolves the conflict:
    let from_a = <S as A>::get(&s);
    let from_b = <S as B>::get(&s); 
    
    // Turbofish for explicit type specification:
    let float_size = size_of::<f64>();
    
    println!("A: {}, B: {}, f64 size: {}", from_a, from_b, float_size);
}
```


**Turbofish and Fully Qualified Syntax**
- Turbofish `::<>` hints types to compiler where inference cannot resolve automatically.
- Fully Qualified Syntax `<S as A>::get` resolves collisions when a single type implements multiple traits with identical method names.


<hr>

<h2 class="relative group">Test Your Knowledge!
    <div id="test-your-knowledge" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#test-your-knowledge" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Take a short quiz to consolidate your understanding of traits and static polymorphism in Rust.</p>
### 📝 Traits and Static Polymorphism

**1. Why are traits needed in Rust?**

- [ ] For dynamic memory allocation on the heap
- [x] To define behavioral contracts for types and implement static polymorphism without code duplication
- [ ] For automatic resource release without the Drop trait
- [ ] To hide private fields of a struct

> 💡 *Traits declare a set of methods (a behavior contract) that different data types can implement, enabling flexible and type-safe polymorphism.*


**2. What is the key difference between an associated function and a trait method?**

- [x] An associated function does not take a self parameter and is called via Type::func(), whereas a method takes self and is called on an object instance via dot syntax
- [ ] Methods cannot be overridden, but associated functions can
- [ ] Associated functions can only return numeric types

> 💡 *Associated functions do not receive a self argument (acting like static methods) and are called using double colons (Dog::species()), whereas methods take self and are invoked on instances (dog.say()).*


**3. What is the key difference between Copy and Clone traits?**

- [ ] Clone executes automatically on assignment, while Copy requires a method call
- [x] Copy performs implicit bitwise copying (memcpy) at the compiler level changing move semantics, while Clone creates a copy via an explicit `.clone()` method call
- [ ] Copy only works on the heap, while Clone works on the stack

> 💡 *Copy is a marker trait for automatic bitwise copying of data on assignment. Clone is a standard trait for explicit `.clone()` invocation with custom copy logic.*


**4. How does `impl Trait` syntax in function arguments relate to Trait Bounds `<T: Trait>`?**

- [ ] impl Trait switches the function to dynamic polymorphism via a vtable
- [x] impl Trait is syntactic sugar for Trait Bounds: the compiler converts it into a generic function and performs monomorphization
- [ ] impl Trait forbids passing structs derived via `#[derive]`

> 💡 *In function arguments, `fn foo(x: impl Trait)` is a convenient shorthand for `fn foo<T: Trait>(x: T)`. Both variants use static polymorphism and monomorphization.*


**5. What is generic monomorphization?**

- [ ] Casting types to dynamic trait objects at runtime
- [ ] Invoking virtual functions through a vtable
- [x] The compiler generating separate physical copies of code for each concrete type used with the generic code at build time

> 💡 *Monomorphization is the process of code duplication by the compiler, substituting real types in place of generics. This provides zero-cost abstractions.*




