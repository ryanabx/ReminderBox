const CACHE_NAME = 'spa-cache-v1';

// Install: nothing special yet
self.addEventListener('install', event => {
  self.skipWaiting();
});

// Activate: clean old caches
self.addEventListener('activate', event => {
  event.waitUntil(
    caches.keys().then(keys => {
      return Promise.all(
        keys
          .filter(key => key !== CACHE_NAME)
          .map(key => caches.delete(key))
      );
    })
  );

  self.clients.claim();
});

// Fetch: Network first, fallback to cache
self.addEventListener('fetch', event => {

  // Only handle GET requests
  if (event.request.method !== 'GET') return;

  event.respondWith(

    fetch(event.request)

      // If network works -> cache + return
      .then(networkResponse => {

        // Only cache valid responses
        if (
          networkResponse &&
          networkResponse.status === 200 &&
          networkResponse.type === 'basic'
        ) {

          const cloned = networkResponse.clone();

          caches.open(CACHE_NAME).then(cache => {
            cache.put(event.request, cloned);
          });
        }

        return networkResponse;
      })

      // If network fails -> cache
      .catch(() => {
        return caches.match(event.request);
      })
  );
});
