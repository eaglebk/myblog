(function(){const e=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M8 5v14l11-7z"/></svg>`,n=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>`,t=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>`,o=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`,i=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;function s(){const p=document.querySelector(".gallery");if(!p)return;let d=[],l=0,f=null,b=!1,h=null,a=null,c="idle",r=5e3;function k(){if(h&&clearTimeout(h),!s.classList.contains("is-open")||!s.classList.contains("meta-hidden")){s.classList.remove("controls-hidden");return}s.classList.remove("controls-hidden"),h=setTimeout(()=>{s.classList.contains("is-open")&&s.classList.contains("meta-hidden")&&s.classList.add("controls-hidden")},2500)}const s=document.createElement("div");s.className="gallery-lightbox",s.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Слайд-шоу" title="Запустить слайд-шоу (5s)">
            <span class="slideshow-icon">${e}</span>
            <span class="slideshow-timeout-badge">5s</span>
          </button>
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
    `,document.body.appendChild(s);const j=s.querySelector(".lightbox-image"),W=s.querySelector(".lightbox-title"),U=s.querySelector(".camera-val"),J=s.querySelector(".lens-val"),Z=s.querySelector(".params-val"),X=s.querySelector(".location-val"),G=s.querySelector(".date-val"),L=s.querySelector(".lightbox-music-bar"),R=s.querySelector(".music-track-name"),Y=s.querySelector(".lightbox-close"),q=s.querySelector(".lightbox-prev"),I=s.querySelector(".lightbox-next"),m=s.querySelector(".btn-slideshow"),g=s.querySelector(".btn-toggle-info"),N=s.querySelectorAll(".swatch"),_=Array.from(p.querySelectorAll("img"));d=[..._];function T(){d=_.filter(e=>!e.classList.contains("filtered-out"))}const x=9;let w=x;function C(e=!1){e&&(w=x);const t=document.querySelector(".gallery-filters .filter-btn.is-active")?.getAttribute("data-filter")||"all";let n=0;if(_.forEach(e=>{const s=e.getAttribute("data-category"),o=t==="all"||s===t;o?n<w?(e.classList.remove("filtered-out"),e.style.display="",n++):(e.classList.add("filtered-out"),e.style.display="none"):(e.classList.add("filtered-out"),e.style.display="none")}),T(),window.Packery){const e=window.Packery.data(p);e&&(e.options.transitionDuration=0,e.reloadItems(),e.layout())}}C(!0);function $(){if(b)return;b=!0;const e=m.querySelector(".slideshow-icon");e&&(e.innerHTML=n),m.setAttribute("title",`Приостановить слайд-шоу (${r/1e3}s)`),f=setInterval(()=>{u(l+1,!0)},r)}function A(){if(!b)return;b=!1;const t=m.querySelector(".slideshow-icon");t&&(t.innerHTML=e),m.setAttribute("title",`Запустить слайд-шоу (${r/1e3}s)`),f&&(clearInterval(f),f=null)}function V(){b?A():$()}function B(){r===3e3?r=5e3:r===5e3?r=8e3:r===8e3?r=12e3:r===12e3?r=3e4:r=3e3;const e=m.querySelector(".slideshow-timeout-badge");e&&(e.textContent=`${r/1e3}s`),b?(m.setAttribute("title",`Приостановить слайд-шоу (${r/1e3}s)`),f&&clearInterval(f),f=setInterval(()=>{u(l+1,!0)},r)):m.setAttribute("title",`Запустить слайд-шоу (${r/1e3}s)`)}function O(){a&&(a.pause(),a=null),c="idle";const e=s.querySelector(".music-progress-bar"),t=s.querySelector(".music-progress-container");e&&(e.style.width="0%"),t&&(t.style.display="none"),v()}function v(){const e=s.querySelector(".music-play-btn");if(!e)return;const t=e.querySelector(".music-btn-icon-play"),n=e.querySelector(".music-btn-icon-pause"),o=e.querySelector(".music-btn-icon-spinner");c==="idle"||c==="paused"?(t.style.display="block",n.style.display="none",o.style.display="none"):c==="loading"?(t.style.display="none",n.style.display="none",o.style.display="block"):c==="playing"&&(t.style.display="none",n.style.display="block",o.style.display="none")}function P(e){O(),c="loading",v();const o=s.querySelector(".music-progress-container"),t=s.querySelector(".music-progress-bar"),n=s.querySelector(".music-external-link"),i=`https://itunes.apple.com/search?term=${encodeURIComponent(e)}&limit=1&entity=song`;fetch(i).then(e=>e.json()).then(s=>{if(s.results&&s.results.length>0){const i=s.results[0],r=i.previewUrl,l=i.trackViewUrl;r?(a=new Audio(r),a.addEventListener("timeupdate",()=>{if(a&&a.duration){const e=a.currentTime/a.duration*100;t&&(t.style.width=`${e}%`)}}),a.addEventListener("ended",()=>{O()}),a.addEventListener("error",()=>{y(e,!0)}),a.play().then(()=>{c="playing",o&&(o.style.display="block"),t&&(t.style.width="0%"),n&&(n.href=l,n.title="Открыть в Apple Music",n.style.display="block"),v()}).catch(t=>{console.error("Audio play failed:",t),y(e,!0)})):y(e,!1)}else y(e,!1)}).catch(t=>{console.error("iTunes search error:",t),y(e,!0)})}function y(e,t=!1){c="idle",v();const i=s.querySelector(".music-progress-bar"),a=s.querySelector(".music-progress-container"),n=s.querySelector(".music-external-link"),o=s.querySelector(".music-label");if(i&&(i.style.width="0%"),a&&(a.style.display="none"),o&&(t?o.textContent="Сеть/Adblock заблокировал превью. Ссылка:":o.textContent="Превью не найдено. Ссылка:",o.style.color="#fda4af"),n){const t=`https://www.youtube.com/results?search_query=${encodeURIComponent(e)}`;n.href=t,n.title="Искать на YouTube",n.style.display="block"}}function u(e,t=!1){const o=c==="playing"||c==="loading";O(),t||A(),e<0&&(e=d.length-1),e>=d.length&&(e=0),l=e;const n=d[l];if(!n)return;j.style.opacity="0",setTimeout(()=>{j.src=n.src,j.alt=n.alt||"Фотография";const f=n.getAttribute("data-title")||n.alt||"Фото",v=n.getAttribute("data-camera")||"—",g=n.getAttribute("data-lens")||"—",a=n.getAttribute("data-aperture"),r=n.getAttribute("data-shutter"),c=n.getAttribute("data-iso"),m=n.getAttribute("data-location")||"—",h=n.getAttribute("data-date")||"—",t=n.getAttribute("data-music"),l=n.getAttribute("data-story");let d="—";if(a||r||c){const e=[];a&&e.push(a),r&&e.push(r),c&&e.push(`ISO ${c}`),d=e.join(" • ")}W.textContent=f,U.textContent=v,J.textContent=g,Z.textContent=d,X.textContent=m,G.textContent=h;const u=s.querySelector(".lightbox-story-box"),p=s.querySelector(".story-text");l?(p.textContent=l,u.style.display="block"):u.style.display="none";const e=s.querySelector(".music-external-link"),i=s.querySelector(".music-label");if(t){if(R.textContent=t,L.style.display="flex",i&&(i.textContent="В наушниках играло:",i.style.color=""),e){const n=`https://www.youtube.com/results?search_query=${encodeURIComponent(t)}`;e.href=n,e.title="Искать на YouTube",e.style.display="block"}o&&P(t)}else L.style.display="none",e&&(e.style.display="none");j.onload=()=>{j.style.opacity="1"}},150),H(e+1),H(e-1)}function H(e){e<0&&(e=d.length-1),e>=d.length&&(e=0);const t=d[e];if(t&&t.src){const e=new Image;e.src=t.src}}function K(e){T();const t=d.indexOf(e);t!==-1&&(u(t),s.classList.add("is-open"),document.body.style.overflow="hidden")}function E(){O(),A(),h&&(clearTimeout(h),h=null),s.classList.remove("controls-hidden"),s.classList.remove("meta-hidden"),g.innerHTML=t,g.setAttribute("title","Скрыть описание"),s.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{j.src=""},350)}p.addEventListener("click",function(e){const t=e.target.closest("img");t&&_.includes(t)&&K(t)}),Y.addEventListener("click",E),q.addEventListener("click",()=>u(l-1)),I.addEventListener("click",()=>u(l+1));const F=s.querySelector(".music-play-btn");F&&F.addEventListener("click",e=>{e.stopPropagation();const t=R.textContent;if(!t||t==="-")return;c==="playing"?a&&(a.pause(),c="paused",v()):c==="paused"&&a?a.play().then(()=>{c="playing",v()}):P(t)}),m.addEventListener("click",e=>{e.stopPropagation();const t=e.target.closest(".slideshow-timeout-badge");t?B():V()}),g.addEventListener("click",e=>{e.stopPropagation(),s.classList.toggle("meta-hidden");const n=s.classList.contains("meta-hidden");n?(g.innerHTML=o,g.setAttribute("title","Показать описание"),k()):(g.innerHTML=t,g.setAttribute("title","Скрыть описание"),h&&(clearTimeout(h),h=null),s.classList.remove("controls-hidden"))}),N.forEach(e=>{e.addEventListener("click",t=>{t.stopPropagation(),N.forEach(e=>e.classList.remove("is-active")),e.classList.add("is-active");const n=e.getAttribute("data-color");s.style.setProperty("--lightbox-bg",n);const o=["#ffffff","#e5e7eb"].includes(n);o?s.classList.add("light-bg"):s.classList.remove("light-bg")})}),s.addEventListener("click",function(e){(e.target===s||e.target.classList.contains("lightbox-container"))&&E()}),s.addEventListener("mousemove",k),s.addEventListener("touchstart",k,{passive:!0}),document.addEventListener("keydown",function(e){if(!s.classList.contains("is-open"))return;e.key==="Escape"?E():e.key==="ArrowLeft"?u(l-1):e.key==="ArrowRight"&&u(l+1)});let M=0,z=0;s.addEventListener("touchstart",function(e){M=e.changedTouches[0].screenX},{passive:!0}),s.addEventListener("touchend",function(e){z=e.changedTouches[0].screenX,Q()},{passive:!0});function Q(){const e=z-M,t=60;e>t?u(l-1):e<-t&&u(l+1)}const D=document.querySelectorAll(".gallery-filters .filter-btn");D.forEach(e=>{e.addEventListener("click",function(){D.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active"),p.classList.add("is-filtering"),setTimeout(()=>{C(!0),void p.offsetHeight,p.classList.remove("is-filtering")},200)})});let S=!1;window.addEventListener("scroll",()=>{if(S)return;if(S=!0,setTimeout(()=>{S=!1},100),window.innerHeight+window.scrollY>=document.documentElement.scrollHeight-400){const e=document.querySelector(".gallery-filters .filter-btn.is-active")?.getAttribute("data-filter")||"all",t=_.filter(t=>e==="all"||t.getAttribute("data-category")===e).length;w<t&&(w+=x,C(!1))}})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",s):s()})()