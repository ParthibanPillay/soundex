// Every second, forward what the page is playing to the background worker (and on to the native host).
setInterval(() => {
  const m = navigator.mediaSession.metadata;
  const v = document.querySelector("video");
  if (!m || !v) return chrome.runtime.sendMessage({ title: "" }); // nothing loaded: host clears the status
  const id = new URLSearchParams(location.search).get("v");
  chrome.runtime.sendMessage({
    title: m.title,
    artists: m.artist,
    album: m.album,
    // Google-hosted covers accept any size in the URL; ask for a sharp one.
    art: (m.artwork.at(-1)?.src ?? "").replace(/=w\d+-h\d+/, "=w1024-h1024"),
    url: id ? `https://music.youtube.com/watch?v=${id}` : "",
    artist_url: document.querySelector('ytmusic-player-bar .byline a[href*="channel/"]')?.href ?? "",
    playing: !v.paused,
    pos: Math.floor(v.currentTime),
    len: Math.floor(v.duration), // NaN/Infinity (live) serialize to null; host treats as unknown
  });
}, 1000);
