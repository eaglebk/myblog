(function(){const t=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M8 5v14l11-7z"/></svg>`,s=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>`,e=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>`,o=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`,i=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;function n(){const d=document.querySelector(".gallery");if(!d)return;let a=[],r=0,g=null,m=!1,c=null;function v(){if(c&&clearTimeout(c),!n.classList.contains("is-open")||!n.classList.contains("meta-hidden")){n.classList.remove("controls-hidden");return}n.classList.remove("controls-hidden"),c=setTimeout(()=>{n.classList.contains("is-open")&&n.classList.contains("meta-hidden")&&n.classList.add("controls-hidden")},2500)}const n=document.createElement("div");n.className="gallery-lightbox",n.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Запустить слайд-шоу" title="Запустить слайд-шоу">${t}</button>
          <button class="toolbar-btn btn-toggle-info" aria-label="Скрыть/показать описание" title="Скрыть описание">${e}</button>
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
            <span class="music-icon-pulse">🎧</span>
            <span class="music-text">
              <span class="music-label">В наушниках играло:</span>
              <span class="music-track-name">-</span>
            </span>
          </div>
        </div>
      </div>
    `,document.body.appendChild(n);const h=n.querySelector(".lightbox-image"),$=n.querySelector(".lightbox-title"),B=n.querySelector(".camera-val"),I=n.querySelector(".lens-val"),H=n.querySelector(".params-val"),R=n.querySelector(".location-val"),L=n.querySelector(".date-val"),w=n.querySelector(".lightbox-music-bar"),D=n.querySelector(".music-track-name"),F=n.querySelector(".lightbox-close"),S=n.querySelector(".lightbox-prev"),M=n.querySelector(".lightbox-next"),p=n.querySelector(".btn-slideshow"),u=n.querySelector(".btn-toggle-info"),A=n.querySelectorAll(".swatch"),f=Array.from(d.querySelectorAll("img"));a=[...f];function y(){a=f.filter(e=>!e.classList.contains("filtered-out"))}const C=9;let O=C;function T(e=!1){e&&(O=C);const t=document.querySelector(".gallery-filters .filter-btn.is-active")?.getAttribute("data-filter")||"all";let n=0;if(f.forEach(e=>{const s=e.getAttribute("data-category"),o=t==="all"||s===t;o?n<O?(e.classList.remove("filtered-out"),e.style.display="",n++):(e.classList.add("filtered-out"),e.style.display="none"):(e.classList.add("filtered-out"),e.style.display="none")}),y(),window.Packery){const e=window.Packery.data(d);e&&(e.options.transitionDuration=0,e.reloadItems(),e.layout())}}T(!0);function z(){if(m)return;m=!0,p.innerHTML=s,p.setAttribute("title","Приостановить слайд-шоу"),g=setInterval(()=>{l(r+1,!0)},3500)}function j(){if(!m)return;m=!1,p.innerHTML=t,p.setAttribute("title","Запустить слайд-шоу"),g&&(clearInterval(g),g=null)}function N(){m?j():z()}function l(e,t=!1){t||j(),e<0&&(e=a.length-1),e>=a.length&&(e=0),r=e;const s=a[r];if(!s)return;h.style.opacity="0",setTimeout(()=>{h.src=s.src,h.alt=s.alt||"Фотография";const f=s.getAttribute("data-title")||s.alt||"Фото",d=s.getAttribute("data-camera")||"—",l=s.getAttribute("data-lens")||"—",e=s.getAttribute("data-aperture"),o=s.getAttribute("data-shutter"),t=s.getAttribute("data-iso"),u=s.getAttribute("data-location")||"—",m=s.getAttribute("data-date")||"—",a=s.getAttribute("data-music"),i=s.getAttribute("data-story");let r="—";if(e||o||t){const n=[];e&&n.push(e),o&&n.push(o),t&&n.push(`ISO ${t}`),r=n.join(" • ")}$.textContent=f,B.textContent=d,I.textContent=l,H.textContent=r,R.textContent=u,L.textContent=m;const c=n.querySelector(".lightbox-story-box"),p=n.querySelector(".story-text");i?(p.textContent=i,c.style.display="block"):c.style.display="none",a?(D.textContent=a,w.style.display="flex"):w.style.display="none",h.onload=()=>{h.style.opacity="1"}},150),_(e+1),_(e-1)}function _(e){e<0&&(e=a.length-1),e>=a.length&&(e=0);const t=a[e];if(t&&t.src){const e=new Image;e.src=t.src}}function P(e){y();const t=a.indexOf(e);t!==-1&&(l(t),n.classList.add("is-open"),document.body.style.overflow="hidden")}function b(){j(),c&&(clearTimeout(c),c=null),n.classList.remove("controls-hidden"),n.classList.remove("meta-hidden"),u.innerHTML=e,u.setAttribute("title","Скрыть описание"),n.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{h.src=""},350)}d.addEventListener("click",function(e){const t=e.target.closest("img");t&&f.includes(t)&&P(t)}),F.addEventListener("click",b),S.addEventListener("click",()=>l(r-1)),M.addEventListener("click",()=>l(r+1)),p.addEventListener("click",e=>{e.stopPropagation(),N()}),u.addEventListener("click",t=>{t.stopPropagation(),n.classList.toggle("meta-hidden");const s=n.classList.contains("meta-hidden");s?(u.innerHTML=o,u.setAttribute("title","Показать описание"),v()):(u.innerHTML=e,u.setAttribute("title","Скрыть описание"),c&&(clearTimeout(c),c=null),n.classList.remove("controls-hidden"))}),A.forEach(e=>{e.addEventListener("click",t=>{t.stopPropagation(),A.forEach(e=>e.classList.remove("is-active")),e.classList.add("is-active");const s=e.getAttribute("data-color");n.style.setProperty("--lightbox-bg",s);const o=["#ffffff","#e5e7eb"].includes(s);o?n.classList.add("light-bg"):n.classList.remove("light-bg")})}),n.addEventListener("click",function(e){(e.target===n||e.target.classList.contains("lightbox-container"))&&b()}),n.addEventListener("mousemove",v),n.addEventListener("touchstart",v,{passive:!0}),document.addEventListener("keydown",function(e){if(!n.classList.contains("is-open"))return;e.key==="Escape"?b():e.key==="ArrowLeft"?l(r-1):e.key==="ArrowRight"&&l(r+1)});let x=0,E=0;n.addEventListener("touchstart",function(e){x=e.changedTouches[0].screenX},{passive:!0}),n.addEventListener("touchend",function(e){E=e.changedTouches[0].screenX,V()},{passive:!0});function V(){const e=E-x,t=60;e>t?l(r-1):e<-t&&l(r+1)}const k=document.querySelectorAll(".gallery-filters .filter-btn");k.forEach(e=>{e.addEventListener("click",function(){k.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");d.classList.add("is-filtering"),setTimeout(()=>{if(f.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),y(),window.Packery){const e=window.Packery.data(d);e&&(e.options.transitionDuration=0,e.reloadItems(),e.layout())}void d.offsetHeight,d.classList.remove("is-filtering")},200)})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",n):n()})()