if (typeof window.RustPlayground === "undefined") {
  class RustPlayground {
    constructor() {
      this.isCodeRunning = false;
      this.themeObserver = new MutationObserver(() => this.setupCodeHover());
      this.init();
    }

    init() {
      if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", () =>
          this.initializeAfterDOM()
        );
      } else {
        this.initializeAfterDOM();
      }
    }

    initializeAfterDOM() {
      if (!document.getElementById("tooltips-disabled-style")) {
        const style = document.createElement("style");
        style.id = "tooltips-disabled-style";
        style.textContent = `
          body.tooltips-disabled .rust-keyword,
          html[data-tooltips-disabled="true"] .rust-keyword {
            pointer-events: none !important;
            cursor: text !important;
            border-bottom: none !important;
            text-decoration: none !important;
            background: transparent !important;
          }
          body.tooltips-disabled .tippy-box,
          html[data-tooltips-disabled="true"] .tippy-box {
            display: none !important;
            visibility: hidden !important;
            opacity: 0 !important;
            pointer-events: none !important;
          }
        `;
        document.head.appendChild(style);
      }

      const isDisabled = localStorage.getItem("pref_rust_tooltips") === "false";
      document.body.classList.toggle("tooltips-disabled", isDisabled);
      document.documentElement.setAttribute("data-tooltips-disabled", String(isDisabled));

      if (isDisabled) {
        document.querySelectorAll(".rust-keyword").forEach(el => {
          el.removeAttribute("title");
          if (el._tippy) el._tippy.disable();
        });
        return;
      }

      this.setupCodeHover();
      this.setupThemeObserver();
      this.setupEventListeners();
    }


    setupEventListeners() {
      // Можно добавить глобальные события по необходимости
    }


    switchSlide(current, target, container) {
      if (!current || !target) return;
      const isAnimEnabled = localStorage.getItem("pref_code_anim") !== "false";

      if (isAnimEnabled) {
        current.style.transition = "opacity 0.18s cubic-bezier(0.4, 0, 0.2, 1), transform 0.18s cubic-bezier(0.4, 0, 0.2, 1)";
        current.style.opacity = "0";
        current.style.transform = "translateY(-3px)";

        setTimeout(() => {
          current.hidden = true;
          current.style.opacity = "1";
          current.style.transform = "none";

          target.hidden = false;
          target.style.opacity = "0";
          target.style.transform = "translateY(5px)";
          target.style.transition = "opacity 0.26s cubic-bezier(0.4, 0, 0.2, 1), transform 0.26s cubic-bezier(0.4, 0, 0.2, 1)";

          requestAnimationFrame(() => {
            requestAnimationFrame(() => {
              target.style.opacity = "1";
              target.style.transform = "translateY(0)";
            });
          });
          this.updateSlideCounter(container);
        }, 180);
      } else {
        current.style.transition = "none";
        current.style.opacity = "1";
        current.style.transform = "none";
        current.hidden = true;

        target.hidden = false;
        target.style.transition = "none";
        target.style.opacity = "1";
        target.style.transform = "none";
        this.updateSlideCounter(container);
      }
    }


    goToNextSlide(container) {
      const slides = container.querySelectorAll(".slide");
      const current = container.querySelector(".slide:not([hidden])");
      let next = current ? current.nextElementSibling : null;

      if (!next || !next.classList.contains("slide")) {
        next = slides[0];
      }

      this.switchSlide(current, next, container);
    }

    goToPrevSlide(container) {
      const slides = container.querySelectorAll(".slide");
      const current = container.querySelector(".slide:not([hidden])");
      let prev = current ? current.previousElementSibling : null;

      if (!prev || !prev.classList.contains("slide")) {
        prev = slides[slides.length - 1];
      }

      this.switchSlide(current, prev, container);
    }


    updateSlideCounter(container) {
      const slides = container.querySelectorAll(".slide");
      const current = container.querySelector(".slide:not([hidden])");
      const currentIndex = Array.from(slides).indexOf(current) + 1;
      const counter = container.querySelector(".slide-counter");
      if (counter) counter.textContent = currentIndex;
    }

    getLang() {
      const path = window.location.pathname || "";
      if (path.startsWith("/en/") || path.includes("/en/")) return "en";
      if (window.RUST_PAGE_LANG === "en") return "en";
      const htmlLang = document.documentElement.lang || "";
      if (htmlLang.toLowerCase().startsWith("en")) return "en";
      return (window.RUST_PAGE_LANG || "ru");
    }


    getI18n() {
      const lang = this.getLang();
      const dict = {
        ru: {
          syntaxLabel: "Синтаксис:",
          exampleLabel: "Пример:",
          officialDoc: "📚 Документация 🦀",
          currentArticle: "📌 Эта статья",
          blogArticle: "✍️ Статья в блоге: ",
          compiling: "Компилируется...",
          running: "⏳ Запуск...",
          connError: "Ошибка подключения: "
        },
        en: {
          syntaxLabel: "Syntax:",
          exampleLabel: "Example:",
          officialDoc: "📚 Documentation 🦀",
          currentArticle: "📌 Current Article",
          blogArticle: "✍️ Blog Post: ",
          compiling: "Compiling...",
          running: "⏳ Running...",
          connError: "Connection error: "
        }
      };
      return dict[lang] || dict.ru;
    }

    async runCurrentSlide(container) {
      if (this.isCodeRunning) return;
      this.isCodeRunning = true;

      const i18n = this.getI18n();
      const currentSlide = container.querySelector(".slide:not([hidden])");
      const codeBlock = currentSlide.querySelector("pre code");
      const outputContainer = currentSlide.querySelector(".output-container");
      const outputContent = currentSlide.querySelector(".output-content");
      const runButton = container.querySelector(".run-button");
      const originalText = runButton.innerHTML;

      if (!codeBlock || !outputContainer || !outputContent) return;

      const lines = Array.from(
      currentSlide.querySelectorAll(".line:not(.gap-line) > .cl")
        ).map(cl => cl.textContent);

      const cleanedLines = lines.map((line) => {
        // 1. Номера строк в начале (с пробелами или без)
        return line.replace(/^\s*\d+\s*/, ""); // Удалить номер в начале
      });
      // Фильтруем полностью пустые строки (включая строки с одним номером)
      const code = cleanedLines
        .filter((line) => line.trim().length > 0)
        .join("\n")
        .trim();

      runButton.disabled = true;
      runButton.innerHTML = i18n.running;
      runButton.classList.add("bg-blue-600", "cursor-wait");
      runButton.classList.remove("bg-blue-500", "hover:bg-blue-600");

      outputContent.textContent = i18n.compiling;
      outputContainer.classList.remove("hidden");

      try {
        const response = await fetch(
          "https://play.rust-lang.org/evaluate.json",
          {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              version: "stable",
              optimize: "0",
              edition: "2024",
              code: code,
            }),
          }
        );

        const data = await response.json();

        if (data.error) {
          outputContent.textContent = data.error;
          outputContent.classList.add("text-red-400");
          outputContent.classList.remove("text-green-400");
        } else {
          outputContent.textContent = data.result;
          outputContent.classList.add("text-green-400");
          outputContent.classList.remove("text-red-400");
        }
      } catch (error) {
        outputContent.textContent = `${i18n.connError}${error.message}`;
        outputContent.classList.add("text-red-400");
      } finally {
        this.isCodeRunning = false;
        runButton.disabled = false;
        runButton.innerHTML = originalText;
        runButton.classList.remove("bg-blue-600", "cursor-wait");
        runButton.classList.add("bg-blue-500", "hover:bg-blue-600");
      }
    }

    setupCodeHover() {
      if (localStorage.getItem("pref_rust_tooltips") === "false") return;

      const tooltipData = window.RUST_KEYWORD_TOOLTIPS;
      if (!tooltipData) return;

      const keywordSet = new Set(tooltipData);
      const lang = this.getLang();
      const i18n = this.getI18n();
      const isEn = lang === "en";

      document
        .querySelectorAll("code.language-rust:not(.processed)")
        .forEach((codeBlock) => {
          codeBlock.classList.add("processed");

          // Ищем все элементы с текстом внутри .line > .cl
          codeBlock.querySelectorAll(".line > .cl").forEach((codeLine) => {
             codeLine
              .querySelectorAll("span:not(.ln):not(.rust-keyword)")
              .forEach((token) => {
                const text = token.textContent.trim();

                // Reconstruct the full path if part of a namespace (e.g., tokio::spawn)
                let path = text;
                let current = token.previousSibling;
                while (current) {
                  if (current.nodeType === Node.TEXT_NODE && current.textContent.trim() === "") {
                    current = current.previousSibling;
                    continue;
                  }
                  if (current.textContent.trim() === "::") {
                    let identSibling = current.previousSibling;
                    while (identSibling && identSibling.nodeType === Node.TEXT_NODE && identSibling.textContent.trim() === "") {
                      identSibling = identSibling.previousSibling;
                    }
                    if (identSibling) {
                      let identText = identSibling.textContent.trim();
                      if (/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(identText)) {
                        path = identText + "::" + path;
                        current = identSibling.previousSibling;
                        continue;
                      }
                    }
                  }
                  break;
                }

                const cleanText = text.replace(/\\/g, "");
                let targetKeyword = cleanText;

                if (cleanText === "for") {
                  let textBefore = "";
                  let p = token.previousSibling;
                  while (p) {
                    textBefore = (p.textContent || "") + textBefore;
                    p = p.previousSibling;
                  }
                  if (/\bimpl\b/.test(textBefore)) {
                    targetKeyword = "impl ... for";
                  } else {
                    let textAfter = "";
                    let n = token.nextSibling;
                    while (n && textAfter.length < 20) {
                      textAfter += (n.textContent || "");
                      n = n.nextSibling;
                    }
                    if (/^\s*<['a-zA-Z_]/.test(textAfter)) {
                      targetKeyword = "for<'a>";
                    }
                  }
                }

                for (const keywordObj of keywordSet) {
                  const kw = keywordObj.keyword.replace(/\\/g, "");
                  const cleanPath = path.replace(/\\/g, "");

                  if (
                    kw === targetKeyword ||
                    kw === cleanPath ||
                    (kw.includes("::") && cleanPath.endsWith(kw)) ||
                    (kw.includes("::") && kw.endsWith(cleanPath) && cleanPath.includes("::")) ||
                    (kw === cleanText && targetKeyword === cleanText && !cleanPath.includes("::"))
                  ) {
                    // Создаем новый элемент с подсказкой
                    const tooltipSpan = document.createElement("span");
                    tooltipSpan.className = "rust-keyword";
                    tooltipSpan.textContent = text;

                    const summaryText = (isEn && keywordObj.summary_en) ? keywordObj.summary_en : keywordObj.summary;
                    const syntaxText = (isEn && keywordObj.syntax_en) ? keywordObj.syntax_en : keywordObj.syntax;
                    const exampleText = (isEn && keywordObj.example_en) ? keywordObj.example_en : keywordObj.example;
                    const docText = (isEn && keywordObj.doc_en) ? keywordObj.doc_en : (isEn && keywordObj.summary ? keywordObj.summary : keywordObj.doc);


                    // Обновленное тело попапа
                    tooltipSpan.setAttribute(
                      "data-tippy-content",
                      `
<div class="rust-tooltip ${this.isDarkTheme() ? "dark" : "light"}">
    <header>
        <span class="keyword">${keywordObj.keyword.replace(/\\/g, "")}</span>
        <span class="summary">${summaryText}</span>
    </header>
    <div class="syntax"><strong>${i18n.syntaxLabel}</strong> ${syntaxText}</code></div>
    <div class="example"><strong>${i18n.exampleLabel} </strong><code>${exampleText}</code></div>
    <div class="doc">${docText}</div>
    
    <div class="doc-links">
        <a href="${keywordObj.docs.official}" target="_blank" 
           class="official-link">${i18n.officialDoc}</a>
        
        ${
          keywordObj.docs.blog.length > 0
            ? keywordObj.docs.blog
                .map((blog) => {
                  const currentPath = window.location.pathname.replace(/\/$/, "");
                  const currentSlug = currentPath.split("/").pop();
                  const blogUrl = blog.url || "";
                  const cleanBlogPath = blogUrl.replace(/#.*$/, "").replace(/\/$/, "");
                  const blogSlug = cleanBlogPath.split("/").pop();
                  const isCurrentArticle = Boolean(blogSlug && currentSlug && blogSlug === currentSlug);
                  const blogTitle = (isEn && blog.title_en) ? blog.title_en : (isEn ? "Rust Blog Post" : blog.title);


                  if (isCurrentArticle) {
                    return `<a href="${blog.url}" class="blog-link current-article">${i18n.currentArticle}</a>`;
                  } else {
                    return `<a href="${blog.url}" target="_blank" class="blog-link">${i18n.blogArticle}${blogTitle}</a>`;
                  }
                })
                .join("")
            : ``
        }
    </div>
</div>
`
                    );

                    // Заменяем оригинальный элемент
                    token.replaceWith(tooltipSpan);
                    break; // Прерываем цикл, если нашли совпадение
                  }
                }
              });
          });
        });

      // Инициализация tippy
      if (window.tippy) {
        const isDark =
          document.documentElement.classList.contains("dark") ||
          document.documentElement.getAttribute("data-theme") === "dark";

        tippy(".rust-keyword", {
          onShow(instance) {
            if (localStorage.getItem("pref_rust_tooltips") === "false" || document.body.classList.contains("tooltips-disabled")) {
              return false;
            }
          },
          content(reference) {
            return reference.getAttribute("data-tippy-content");
          },

          placement: "bottom",
          theme: isDark ? "dark" : "light",
          allowHTML: true,
          interactive: true,
          maxWidth: 500,
          arrow: true,
          animation: "fade",
          duration: 200,
          appendTo: () => document.body,
          boundary: 'viewport'
        });
      }
    }

    highlightKeywordsIn(element, tooltipData) {
      const walk = document.createTreeWalker(element, NodeFilter.SHOW_TEXT, {
        acceptNode(node) {
          // Игнорируем номера строк и уже обработанные узлы
          if (node.parentNode.closest(".ln")) return NodeFilter.FILTER_REJECT;
          if (node.parentNode.closest(".rust-keyword"))
            return NodeFilter.FILTER_REJECT;
          return NodeFilter.FILTER_ACCEPT;
        },
      });

      let node;
      const regex = new RegExp(
        `\\b(${Object.keys(tooltipData).join("|")})\\b`,
        "g"
      );

      while ((node = walk.nextNode())) {
        const originalText = node.textContent;
        const matches = [...originalText.matchAll(regex)];

        if (matches.length === 0) continue;

        const fragments = [];
        let lastIndex = 0;

        for (const match of matches) {
          const keyword = match[1];
          const index = match.index;

          if (index > lastIndex) {
            fragments.push(
              document.createTextNode(originalText.slice(lastIndex, index))
            );
          }

          const span = document.createElement("span");
          span.className = "rust-keyword";
          // Note: Do NOT set native 'title' attribute to prevent unwanted browser tooltips
          span.textContent = keyword;

          fragments.push(span);

          lastIndex = index + keyword.length;
        }

        if (lastIndex < originalText.length) {
          fragments.push(
            document.createTextNode(originalText.slice(lastIndex))
          );
        }

        const parent = node.parentNode;
        fragments.forEach((frag) => parent.insertBefore(frag, node));
        parent.removeChild(node);
      }
    }

    setupThemeObserver() {
      this.themeObserver.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ["class", "data-theme"],
      });
    }

    isDarkTheme() {
      return (
        document.documentElement.classList.contains("dark") ||
        document.documentElement.getAttribute("data-theme") === "dark"
      );
    }
  }

  window.RustPlayground = RustPlayground;
  window.rustPlayground = new RustPlayground();
}
