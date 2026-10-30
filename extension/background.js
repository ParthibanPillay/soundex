// The native host (soundex binary) lives exactly as long as this port: it exits when the port closes.
let port = null;
let last = null; // latest {title, art, ...} from the tab
let priv = false; // privacy mode: hides the presence; toggled by clicking the toolbar icon
const post = () => last && port?.postMessage({ ...last, private: priv });
const badge = () => chrome.action.setBadgeText({ text: priv ? "off" : "" });
chrome.storage.local.get("priv").then((r) => { priv = !!r.priv; badge(); post(); });
chrome.action.onClicked.addListener(() => {
  chrome.storage.local.set({ priv: (priv = !priv) });
  badge();
  post();
});

async function sync() {
  const open = (await chrome.tabs.query({ url: "https://music.youtube.com/*" })).length > 0;
  if (open && !port) {
    port = chrome.runtime.connectNative("dev.soundex");
    port.onDisconnect.addListener(() => (port = null));
    post();
  } else if (!open && port) {
    port.disconnect();
    port = null;
  }
}

chrome.tabs.onCreated.addListener(sync);
chrome.tabs.onUpdated.addListener(sync);
chrome.tabs.onRemoved.addListener(sync);
chrome.runtime.onStartup.addListener(sync);
chrome.runtime.onInstalled.addListener(sync);

chrome.runtime.onMessage.addListener((msg) => {
  last = msg;
  post();
});
