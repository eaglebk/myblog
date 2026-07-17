(function(){const t=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M8 5v14l11-7z"/></svg>`,o=`<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="display: block;"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>`,n=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>`,i=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`,s=`<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: block;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;function e(){const f=document.querySelector(".gallery");if(!f)return;let o=[],i=0,h=null,d=!1,a=null;function p(){if(a&&clearTimeout(a),!e.classList.contains("is-open")||!e.classList.contains("meta-hidden")){e.classList.remove("controls-hidden");return}e.classList.remove("controls-hidden"),a=setTimeout(()=>{e.classList.contains("is-open")&&e.classList.contains("meta-hidden")&&e.classList.add("controls-hidden")},2500)}const e=document.createElement("div");e.className="gallery-lightbox",e.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Запустить слайд-шоу" title="Запустить слайд-шоу">${t}</button>
          <button class="toolbar-btn btn-toggle-info" aria-label="Скрыть/показать описание" title="Скрыть описание">${n}</button>
        </div>
        <div class="toolbar-right">
          <button class="lightbox-close" aria-label="Закрыть">${s}</button>
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
    `,document.body.appendChild(e);const l=e.querySelector(".lightbox-image"),P=e.querySelector(".lightbox-title"),L=e.querySelector(".camera-val"),N=e.querySelector(".lens-val"),D=e.querySelector(".params-val"),T=e.querySelector(".location-val"),F=e.querySelector(".date-val"),j=e.querySelector(".lightbox-music-bar"),S=e.querySelector(".music-track-name"),k=e.querySelector(".lightbox-close"),C=e.querySelector(".lightbox-prev"),E=e.querySelector(".lightbox-next"),u=e.querySelector(".btn-slideshow"),c=e.querySelector(".btn-toggle-info"),O=e.querySelectorAll(".swatch"),m=Array.from(f.querySelectorAll("img"));o=[...m];function y(){o=m.filter(e=>!e.classList.contains("filtered-out"))}function A(){if(d)return;d=!0,u.textContent="⏸️",u.setAttribute("title","Приостановить слайд-шоу"),h=setInterval(()=>{r(i+1,!0)},3500)}function v(){if(!d)return;d=!1,u.textContent="▶️",u.setAttribute("title","Запустить слайд-шоу"),h&&(clearInterval(h),h=null)}function M(){d?v():A()}function r(t,n=!1){n||v(),t<0&&(t=o.length-1),t>=o.length&&(t=0),i=t;const s=o[i];if(!s)return;l.style.opacity="0",setTimeout(()=>{l.src=s.src,l.alt=s.alt||"Фотография";const f=s.getAttribute("data-title")||s.alt||"Фото",u=s.getAttribute("data-camera")||"—",d=s.getAttribute("data-lens")||"—",t=s.getAttribute("data-aperture"),o=s.getAttribute("data-shutter"),n=s.getAttribute("data-iso"),h=s.getAttribute("data-location")||"—",m=s.getAttribute("data-date")||"—",a=s.getAttribute("data-music"),i=s.getAttribute("data-story");let r="—";if(t||o||n){const e=[];t&&e.push(t),o&&e.push(o),n&&e.push(`ISO ${n}`),r=e.join(" • ")}P.textContent=f,L.textContent=u,N.textContent=d,D.textContent=r,T.textContent=h,F.textContent=m;const c=e.querySelector(".lightbox-story-box"),p=e.querySelector(".story-text");i?(p.textContent=i,c.style.display="block"):c.style.display="none",a?(S.textContent=a,j.style.display="flex"):j.style.display="none",l.onload=()=>{l.style.opacity="1"}},150),_(t+1),_(t-1)}function _(e){e<0&&(e=o.length-1),e>=o.length&&(e=0);const t=o[e];if(t&&t.src){const e=new Image;e.src=t.src}}function z(t){y();const n=o.indexOf(t);n!==-1&&(r(n),e.classList.add("is-open"),document.body.style.overflow="hidden")}function g(){v(),a&&(clearTimeout(a),a=null),e.classList.remove("controls-hidden"),e.classList.remove("meta-hidden"),c.textContent="👁️",c.setAttribute("title","Скрыть описание"),e.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{l.src=""},350)}f.addEventListener("click",function(e){const t=e.target.closest("img");t&&m.includes(t)&&z(t)}),k.addEventListener("click",g),C.addEventListener("click",()=>r(i-1)),E.addEventListener("click",()=>r(i+1)),u.addEventListener("click",e=>{e.stopPropagation(),M()}),c.addEventListener("click",t=>{t.stopPropagation(),e.classList.toggle("meta-hidden");const n=e.classList.contains("meta-hidden");n?(c.textContent="👁️‍🗨️",c.setAttribute("title","Показать описание"),p()):(c.textContent="👁️",c.setAttribute("title","Скрыть описание"),a&&(clearTimeout(a),a=null),e.classList.remove("controls-hidden"))}),O.forEach(t=>{t.addEventListener("click",n=>{n.stopPropagation(),O.forEach(e=>e.classList.remove("is-active")),t.classList.add("is-active");const s=t.getAttribute("data-color");e.style.setProperty("--lightbox-bg",s);const o=["#ffffff","#e5e7eb"].includes(s);o?e.classList.add("light-bg"):e.classList.remove("light-bg")})}),e.addEventListener("click",function(t){(t.target===e||t.target.classList.contains("lightbox-container"))&&g()}),e.addEventListener("mousemove",p),e.addEventListener("touchstart",p,{passive:!0}),document.addEventListener("keydown",function(t){if(!e.classList.contains("is-open"))return;t.key==="Escape"?g():t.key==="ArrowLeft"?r(i-1):t.key==="ArrowRight"&&r(i+1)});let w=0,b=0;e.addEventListener("touchstart",function(e){w=e.changedTouches[0].screenX},{passive:!0}),e.addEventListener("touchend",function(e){b=e.changedTouches[0].screenX,R()},{passive:!0});function R(){const e=b-w,t=60;e>t?r(i-1):e<-t&&r(i+1)}const x=document.querySelectorAll(".gallery-filters .filter-btn");x.forEach(e=>{e.addEventListener("click",function(){x.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(m.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),y(),window.Packery){const e=window.Packery.data(f);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",e):e()})()