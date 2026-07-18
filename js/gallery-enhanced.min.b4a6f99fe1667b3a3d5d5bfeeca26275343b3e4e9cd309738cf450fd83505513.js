(function(){const e=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M8 5v14l11-7z"/></svg>`,n=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>`,t=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>`,o=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`,i=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;function s(){const m=document.querySelector(".gallery");if(!m)return;let c=[],d=0,_=null,j=!1,l=null,a=null,r="idle";function E(){if(l&&clearTimeout(l),!s.classList.contains("is-open")||!s.classList.contains("meta-hidden")){s.classList.remove("controls-hidden");return}s.classList.remove("controls-hidden"),l=setTimeout(()=>{s.classList.contains("is-open")&&s.classList.contains("meta-hidden")&&s.classList.add("controls-hidden")},2500)}const s=document.createElement("div");s.className="gallery-lightbox",s.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Запустить слайд-шоу" title="Запустить слайд-шоу">${e}</button>
          <button class="toolbar-btn btn-toggle-info" aria-label="Скрыть/показать описание" title="Скрыть описание">${t}</button>
        </div>
        <div class="toolbar-right">
          <button class="lightbox-close" aria-label="Закрыть">${i}</button>
        </div>
      </div>
      <button class="lightbox-btn lightbox-prev" aria-label="Предыдущее фото">&#10094;</button>
      <button class="lightbox-btn lightbox-next" aria-label="Следующее фото">&#10095;</button>
      <div class="lightbox-container">
        <div class="lightbox-img-wrapper">
          <img class="lightbox-image" src="" alt="" />
        </div>
        <div class="lightbox-color-bar">
          <div class="color-swatches">
            <button class="swatch" data-color="#ffffff" style="background-color: #ffffff;" title="Белый"></button>
            <button class="swatch" data-color="#e5e7eb" style="background-color: #e5e7eb;" title="Светло-серый"></button>
            <button class="swatch" data-color="#7f7f7f" style="background-color: #7f7f7f;" title="Серый"></button>
            <button class="swatch" data-color="#374151" style="background-color: #374151;" title="Тёмно-серый"></button>
            <button class="swatch is-active" data-color="#18181b" style="background-color: #18181b;" title="Угольный (по умолчанию)"></button>
            <button class="swatch" data-color="#000000" style="background-color: #000000;" title="Чёрный"></button>
          </div>
        </div>
        <div class="lightbox-metadata-card">
          <div class="lightbox-title"></div>
          <div class="lightbox-story-box" style="display: none;">
            <span class="story-text"></span>
          </div>
          <div class="exif-grid">
            <div class="exif-item">
              <span class="exif-label">Камера</span>
              <span class="exif-value camera-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Объектив</span>
              <span class="exif-value lens-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Параметры</span>
              <span class="exif-value params-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Локация</span>
              <span class="exif-value location-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Дата</span>
              <span class="exif-value date-val">-</span>
            </div>
          </div>
          <div class="lightbox-music-bar">
            <button class="music-play-btn" aria-label="Воспроизвести превью" title="Воспроизвести превью">
              <span class="music-btn-icon-play">${e}</span>
              <span class="music-btn-icon-pause" style="display: none;">${n}</span>
              <span class="music-btn-icon-spinner" style="display: none;">
                <svg class="animate-spin" viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round"><circle cx="12" cy="12" r="10" stroke="currentColor" stroke-opacity="0.25"></circle><path d="M4 12a8 8 0 0 1 8-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 0 1 4 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" fill="currentColor"></path></svg>
              </span>
            </button>
            <div class="music-info-wrapper" style="flex-grow: 1; display: flex; flex-direction: column; gap: 4px;">
              <div class="music-text">
                <span class="music-label">В наушниках играло:</span>
                <span class="music-track-name">-</span>
              </div>
              <div class="music-progress-container" style="display: none; height: 3px; background: rgba(255,255,255,0.1); border-radius: 2px; width: 100%; overflow: hidden;">
                <div class="music-progress-bar" style="width: 0%; height: 100%; background: linear-gradient(90deg, #c084fc, #fda4af); transition: width 0.1s linear;"></div>
              </div>
            </div>
            <a class="music-external-link" href="#" target="_blank" title="Искать на YouTube" style="display: none; color: #fda4af; transition: color 0.2s;">
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
            </a>
          </div>
        </div>
      </div>
    `,document.body.appendChild(s);const p=s.querySelector(".lightbox-image"),H=s.querySelector(".lightbox-title"),B=s.querySelector(".camera-val"),U=s.querySelector(".lens-val"),P=s.querySelector(".params-val"),q=s.querySelector(".location-val"),K=s.querySelector(".date-val"),R=s.querySelector(".lightbox-music-bar"),L=s.querySelector(".music-track-name"),Q=s.querySelector(".lightbox-close"),X=s.querySelector(".lightbox-prev"),Y=s.querySelector(".lightbox-next"),v=s.querySelector(".btn-slideshow"),h=s.querySelector(".btn-toggle-info"),M=s.querySelectorAll(".swatch"),b=Array.from(m.querySelectorAll("img"));c=[...b];function F(){c=b.filter(e=>!e.classList.contains("filtered-out"))}const A=9;let w=A;function k(e=!1){e&&(w=A);const t=document.querySelector(".gallery-filters .filter-btn.is-active")?.getAttribute("data-filter")||"all";let n=0;if(b.forEach(e=>{const s=e.getAttribute("data-category"),o=t==="all"||s===t;o?n<w?(e.classList.remove("filtered-out"),e.style.display="",n++):(e.classList.add("filtered-out"),e.style.display="none"):(e.classList.add("filtered-out"),e.style.display="none")}),F(),window.Packery){const e=window.Packery.data(m);e&&(e.options.transitionDuration=0,e.reloadItems(),e.layout())}}k(!0);function $(){if(j)return;j=!0,v.innerHTML=n,v.setAttribute("title","Приостановить слайд-шоу"),_=setInterval(()=>{u(d+1,!0)},3500)}function x(){if(!j)return;j=!1,v.innerHTML=e,v.setAttribute("title","Запустить слайд-шоу"),_&&(clearInterval(_),_=null)}function V(){j?x():$()}function y(){a&&(a.pause(),a=null),r="idle";const e=s.querySelector(".music-progress-bar"),t=s.querySelector(".music-progress-container");e&&(e.style.width="0%"),t&&(t.style.display="none"),f()}function f(){const e=s.querySelector(".music-play-btn");if(!e)return;const t=e.querySelector(".music-btn-icon-play"),n=e.querySelector(".music-btn-icon-pause"),o=e.querySelector(".music-btn-icon-spinner");r==="idle"||r==="paused"?(t.style.display="block",n.style.display="none",o.style.display="none"):r==="loading"?(t.style.display="none",n.style.display="none",o.style.display="block"):r==="playing"&&(t.style.display="none",n.style.display="block",o.style.display="none")}function I(e){y(),r="loading",f();const o=s.querySelector(".music-progress-container"),t=s.querySelector(".music-progress-bar"),n=s.querySelector(".music-external-link"),i=`https://itunes.apple.com/search?term=${encodeURIComponent(e)}&limit=1&entity=song`;fetch(i).then(e=>e.json()).then(s=>{if(s.results&&s.results.length>0){const i=s.results[0],c=i.previewUrl,l=i.trackViewUrl;c?(a=new Audio(c),a.addEventListener("timeupdate",()=>{if(a&&a.duration){const e=a.currentTime/a.duration*100;t&&(t.style.width=`${e}%`)}}),a.addEventListener("ended",()=>{y()}),a.addEventListener("error",()=>{g(e)}),a.play().then(()=>{r="playing",o&&(o.style.display="block"),t&&(t.style.width="0%"),n&&(n.href=l,n.title="Открыть в Apple Music",n.style.display="block"),f()}).catch(t=>{console.error("Audio play failed:",t),g(e)})):g(e)}else g(e)}).catch(t=>{console.error("iTunes search error:",t),g(e)})}function g(e){r="idle",f();const n=s.querySelector(".music-progress-bar"),o=s.querySelector(".music-progress-container"),t=s.querySelector(".music-external-link");if(n&&(n.style.width="0%"),o&&(o.style.display="none"),t){const n=`https://www.youtube.com/results?search_query=${encodeURIComponent(e)}`;t.href=n,t.title="Искать на YouTube (Превью не найдено)",t.style.display="block"}}function u(e,t=!1){y(),t||x(),e<0&&(e=c.length-1),e>=c.length&&(e=0),d=e;const n=c[d];if(!n)return;p.style.opacity="0",setTimeout(()=>{p.src=n.src,p.alt=n.alt||"Фотография";const m=n.getAttribute("data-title")||n.alt||"Фото",g=n.getAttribute("data-camera")||"—",h=n.getAttribute("data-lens")||"—",i=n.getAttribute("data-aperture"),a=n.getAttribute("data-shutter"),t=n.getAttribute("data-iso"),d=n.getAttribute("data-location")||"—",u=n.getAttribute("data-date")||"—",o=n.getAttribute("data-music"),r=n.getAttribute("data-story");let c="—";if(i||a||t){const e=[];i&&e.push(i),a&&e.push(a),t&&e.push(`ISO ${t}`),c=e.join(" • ")}H.textContent=m,B.textContent=g,U.textContent=h,P.textContent=c,q.textContent=d,K.textContent=u;const l=s.querySelector(".lightbox-story-box"),f=s.querySelector(".story-text");r?(f.textContent=r,l.style.display="block"):l.style.display="none";const e=s.querySelector(".music-external-link");if(o){if(L.textContent=o,R.style.display="flex",e){const t=`https://www.youtube.com/results?search_query=${encodeURIComponent(o)}`;e.href=t,e.title="Искать на YouTube",e.style.display="block"}}else R.style.display="none",e&&(e.style.display="none");p.onload=()=>{p.style.opacity="1"}},150),N(e+1),N(e-1)}function N(e){e<0&&(e=c.length-1),e>=c.length&&(e=0);const t=c[e];if(t&&t.src){const e=new Image;e.src=t.src}}function W(e){F();const t=c.indexOf(e);t!==-1&&(u(t),s.classList.add("is-open"),document.body.style.overflow="hidden")}function O(){y(),x(),l&&(clearTimeout(l),l=null),s.classList.remove("controls-hidden"),s.classList.remove("meta-hidden"),h.innerHTML=t,h.setAttribute("title","Скрыть описание"),s.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{p.src=""},350)}m.addEventListener("click",function(e){const t=e.target.closest("img");t&&b.includes(t)&&W(t)}),Q.addEventListener("click",O),X.addEventListener("click",()=>u(d-1)),Y.addEventListener("click",()=>u(d+1));const T=s.querySelector(".music-play-btn");T&&T.addEventListener("click",e=>{e.stopPropagation();const t=L.textContent;if(!t||t==="-")return;r==="playing"?a&&(a.pause(),r="paused",f()):r==="paused"&&a?a.play().then(()=>{r="playing",f()}):I(t)}),v.addEventListener("click",e=>{e.stopPropagation(),V()}),h.addEventListener("click",e=>{e.stopPropagation(),s.classList.toggle("meta-hidden");const n=s.classList.contains("meta-hidden");n?(h.innerHTML=o,h.setAttribute("title","Показать описание"),E()):(h.innerHTML=t,h.setAttribute("title","Скрыть описание"),l&&(clearTimeout(l),l=null),s.classList.remove("controls-hidden"))}),M.forEach(e=>{e.addEventListener("click",t=>{t.stopPropagation(),M.forEach(e=>e.classList.remove("is-active")),e.classList.add("is-active");const n=e.getAttribute("data-color");s.style.setProperty("--lightbox-bg",n);const o=["#ffffff","#e5e7eb"].includes(n);o?s.classList.add("light-bg"):s.classList.remove("light-bg")})}),s.addEventListener("click",function(e){(e.target===s||e.target.classList.contains("lightbox-container"))&&O()}),s.addEventListener("mousemove",E),s.addEventListener("touchstart",E,{passive:!0}),document.addEventListener("keydown",function(e){if(!s.classList.contains("is-open"))return;e.key==="Escape"?O():e.key==="ArrowLeft"?u(d-1):e.key==="ArrowRight"&&u(d+1)});let z=0,D=0;s.addEventListener("touchstart",function(e){z=e.changedTouches[0].screenX},{passive:!0}),s.addEventListener("touchend",function(e){D=e.changedTouches[0].screenX,G()},{passive:!0});function G(){const e=D-z,t=60;e>t?u(d-1):e<-t&&u(d+1)}const S=document.querySelectorAll(".gallery-filters .filter-btn");S.forEach(e=>{e.addEventListener("click",function(){S.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active"),m.classList.add("is-filtering"),setTimeout(()=>{k(!0),void m.offsetHeight,m.classList.remove("is-filtering")},200)})});let C=!1;window.addEventListener("scroll",()=>{if(C)return;if(C=!0,setTimeout(()=>{C=!1},100),window.innerHeight+window.scrollY>=document.documentElement.scrollHeight-400){const e=document.querySelector(".gallery-filters .filter-btn.is-active")?.getAttribute("data-filter")||"all",t=b.filter(t=>e==="all"||t.getAttribute("data-category")===e).length;w<t&&(w+=A,k(!1))}})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",s):s()})()