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
      this.setupCodeHover();
      this.setupThemeObserver();
      this.setupEventListeners();
    }

    setupEventListeners() {
      // Можно добавить глобальные события по необходимости
    }

    goToNextSlide(container) {
      const slides = container.querySelectorAll(".slide");
      const current = container.querySelector(".slide:not([hidden])");
      let next = current.nextElementSibling;

      if (!next || !next.classList.contains("slide")) {
        next = slides[0];
      }

      current.hidden = true;
      next.hidden = false;
      this.updateSlideCounter(container);
    }

    goToPrevSlide(container) {
      const slides = container.querySelectorAll(".slide");
      const current = container.querySelector(".slide:not([hidden])");
      let prev = current.previousElementSibling;

      if (!prev || !prev.classList.contains("slide")) {
        prev = slides[slides.length - 1];
      }

      current.hidden = true;
      prev.hidden = false;
      this.updateSlideCounter(container);
    }

    updateSlideCounter(container) {
      const slides = container.querySelectorAll(".slide");
      const current = container.querySelector(".slide:not([hidden])");
      const currentIndex = Array.from(slides).indexOf(current) + 1;
      const counter = container.querySelector(".slide-counter");
      if (counter) counter.textContent = currentIndex;
    }

    async runCurrentSlide(container) {
      if (this.isCodeRunning) return;
      this.isCodeRunning = true;

      const currentSlide = container.querySelector(".slide:not([hidden])");
      const codeBlock = currentSlide.querySelector("pre code");
      const outputContainer = currentSlide.querySelector(".output-container");
      const outputContent = currentSlide.querySelector(".output-content");
      const runButton = container.querySelector(".run-button");
      const originalText = runButton.innerHTML;

      if (!codeBlock || !outputContainer || !outputContent) return;

      const lines = codeBlock.textContent.split("\n");
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
      runButton.innerHTML = "⏳ Запуск...";
      runButton.classList.add("bg-blue-600", "cursor-wait");
      runButton.classList.remove("bg-blue-500", "hover:bg-blue-600");

      outputContent.textContent = "Компилируется...";
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
        outputContent.textContent = `Ошибка подключения: ${error.message}`;
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
      const tooltipData = window.RUST_KEYWORD_TOOLTIPS;

      if (!tooltipData) return;

      const keywordSet = new Set(tooltipData);

      document
        .querySelectorAll("code.language-rust:not(.processed)")
        .forEach((codeBlock) => {
          codeBlock.classList.add("processed");

          // Ищем все элементы с текстом внутри .line > .cl
          codeBlock.querySelectorAll(".line > .cl").forEach((codeLine) => {
            // Дочерние span элементы, которые содержат текст кода
            codeLine
              .querySelectorAll("span:not(.ln):not(.rust-keyword)")
              .forEach((token) => {
                const text = token.textContent.trim();

                for (const keywordObj of keywordSet) {
                  if (keywordObj.keyword === text) {
                    // Создаем новый элемент с подсказкой
                    const tooltipSpan = document.createElement("span");
                    tooltipSpan.className = "rust-keyword";
                    tooltipSpan.textContent = text;

                    // Обновленное тело попапа
                    tooltipSpan.setAttribute(
                      "data-tippy-content",
                      `
<div class="rust-tooltip ${this.isDarkTheme() ? "dark" : "light"}">
    <header>
        <span class="keyword">${keywordObj.keyword}</span>
        <span class="summary">${keywordObj.summary}</span>
    </header>
    <div class="syntax"><strong>Синтаксис:</strong> ${
      keywordObj.syntax
    }</code></div>
    <div class="example"><strong>Пример: </strong><code>${
      keywordObj.example
    }</code></div>
    <div class="doc">${keywordObj.doc}</div>
    
    <div class="doc-links">
        <a href="${keywordObj.docs.official}" target="_blank" 
           class="official-link">📚 Документация 🦀</a>
        
        ${
          keywordObj.docs.blog.length > 0
            ? keywordObj.docs.blog
                .map(
                  (blog) => `
                <a href="${blog.url}" target="_blank" 
                   class="blog-link">✍️ ${blog.title}</a>
            `
                )
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
          span.setAttribute("title", tooltipData[keyword]);
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
