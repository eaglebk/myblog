(function(){const e=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M8 5v14l11-7z"/></svg>`,n=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>`,s=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>`,i=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`,o=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;function t(){const p=document.querySelector(".gallery");if(!p)return;let i=[],a=0,m=null,u=!1,r=null;function g(){if(r&&clearTimeout(r),!t.classList.contains("is-open")||!t.classList.contains("meta-hidden")){t.classList.remove("controls-hidden");return}t.classList.remove("controls-hidden"),r=setTimeout(()=>{t.classList.contains("is-open")&&t.classList.contains("meta-hidden")&&t.classList.add("controls-hidden")},2500)}const t=document.createElement("div");t.className="gallery-lightbox",t.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Запустить слайд-шоу" title="Запустить слайд-шоу">${e}</button>
          <button class="toolbar-btn btn-toggle-info" aria-label="Скрыть/показать описание" title="Скрыть описание">${s}</button>
        </div>
        <div class="toolbar-right">
          <button class="lightbox-close" aria-label="Закрыть">${o}</button>
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
    `,document.body.appendChild(t);const d=t.querySelector(".lightbox-image"),H=t.querySelector(".lightbox-title"),R=t.querySelector(".camera-val"),L=t.querySelector(".lens-val"),N=t.querySelector(".params-val"),z=t.querySelector(".location-val"),T=t.querySelector(".date-val"),y=t.querySelector(".lightbox-music-bar"),M=t.querySelector(".music-track-name"),A=t.querySelector(".lightbox-close"),E=t.querySelector(".lightbox-prev"),k=t.querySelector(".lightbox-next"),h=t.querySelector(".btn-slideshow"),l=t.querySelector(".btn-toggle-info"),x=t.querySelectorAll(".swatch"),f=Array.from(p.querySelectorAll("img"));i=[...f];function _(){i=f.filter(e=>!e.classList.contains("filtered-out"))}function S(){if(u)return;u=!0,h.innerHTML=n,h.setAttribute("title","Приостановить слайд-шоу"),m=setInterval(()=>{c(a+1,!0)},3500)}function b(){if(!u)return;u=!1,h.innerHTML=e,h.setAttribute("title","Запустить слайд-шоу"),m&&(clearInterval(m),m=null)}function F(){u?b():S()}function c(e,n=!1){n||b(),e<0&&(e=i.length-1),e>=i.length&&(e=0),a=e;const s=i[a];if(!s)return;d.style.opacity="0",setTimeout(()=>{d.src=s.src,d.alt=s.alt||"Фотография";const f=s.getAttribute("data-title")||s.alt||"Фото",u=s.getAttribute("data-camera")||"—",l=s.getAttribute("data-lens")||"—",e=s.getAttribute("data-aperture"),o=s.getAttribute("data-shutter"),n=s.getAttribute("data-iso"),h=s.getAttribute("data-location")||"—",m=s.getAttribute("data-date")||"—",a=s.getAttribute("data-music"),i=s.getAttribute("data-story");let r="—";if(e||o||n){const t=[];e&&t.push(e),o&&t.push(o),n&&t.push(`ISO ${n}`),r=t.join(" • ")}H.textContent=f,R.textContent=u,L.textContent=l,N.textContent=r,z.textContent=h,T.textContent=m;const c=t.querySelector(".lightbox-story-box"),p=t.querySelector(".story-text");i?(p.textContent=i,c.style.display="block"):c.style.display="none",a?(M.textContent=a,y.style.display="flex"):y.style.display="none",d.onload=()=>{d.style.opacity="1"}},150),w(e+1),w(e-1)}function w(e){e<0&&(e=i.length-1),e>=i.length&&(e=0);const t=i[e];if(t&&t.src){const e=new Image;e.src=t.src}}function D(e){_();const n=i.indexOf(e);n!==-1&&(c(n),t.classList.add("is-open"),document.body.style.overflow="hidden")}function v(){b(),r&&(clearTimeout(r),r=null),t.classList.remove("controls-hidden"),t.classList.remove("meta-hidden"),l.textContent="👁️",l.setAttribute("title","Скрыть описание"),t.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{d.src=""},350)}p.addEventListener("click",function(e){const t=e.target.closest("img");t&&f.includes(t)&&D(t)}),A.addEventListener("click",v),E.addEventListener("click",()=>c(a-1)),k.addEventListener("click",()=>c(a+1)),h.addEventListener("click",e=>{e.stopPropagation(),F()}),l.addEventListener("click",e=>{e.stopPropagation(),t.classList.toggle("meta-hidden");const n=t.classList.contains("meta-hidden");n?(l.textContent="👁️‍🗨️",l.setAttribute("title","Показать описание"),g()):(l.textContent="👁️",l.setAttribute("title","Скрыть описание"),r&&(clearTimeout(r),r=null),t.classList.remove("controls-hidden"))}),x.forEach(e=>{e.addEventListener("click",n=>{n.stopPropagation(),x.forEach(e=>e.classList.remove("is-active")),e.classList.add("is-active");const s=e.getAttribute("data-color");t.style.setProperty("--lightbox-bg",s);const o=["#ffffff","#e5e7eb"].includes(s);o?t.classList.add("light-bg"):t.classList.remove("light-bg")})}),t.addEventListener("click",function(e){(e.target===t||e.target.classList.contains("lightbox-container"))&&v()}),t.addEventListener("mousemove",g),t.addEventListener("touchstart",g,{passive:!0}),document.addEventListener("keydown",function(e){if(!t.classList.contains("is-open"))return;e.key==="Escape"?v():e.key==="ArrowLeft"?c(a-1):e.key==="ArrowRight"&&c(a+1)});let O=0,j=0;t.addEventListener("touchstart",function(e){O=e.changedTouches[0].screenX},{passive:!0}),t.addEventListener("touchend",function(e){j=e.changedTouches[0].screenX,P()},{passive:!0});function P(){const e=j-O,t=60;e>t?c(a-1):e<-t&&c(a+1)}const C=document.querySelectorAll(".gallery-filters .filter-btn");C.forEach(e=>{e.addEventListener("click",function(){C.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(f.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),_(),window.Packery){const e=window.Packery.data(p);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",t):t()})()