var cacheName = 'puzzle';
var filesToCache = [
  './',
  './index.html',
  './puzzle.js',
  './puzzle_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    fetch(event.request)
      .then(response => {
        // 更新缓存
        const clone = response.clone();
        caches.open('dynamic-cache').then(cache => cache.put(event.request, clone));
        return response;
      })
      .catch(() => caches.match(event.request)) // 如果网络失败，回退缓存
  );
});
