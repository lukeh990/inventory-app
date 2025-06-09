import "./style.css";
import "@xterm/xterm/css/xterm.css";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebglAddon } from "@xterm/addon-webgl";
import { ClipboardAddon } from "@xterm/addon-clipboard";
import { LigaturesAddon } from "@xterm/addon-ligatures";

const terminal = new Terminal({
  theme: {
    background: "#393939"
  },
  allowProposedApi: true
});
const fitAddon = new FitAddon();
const clipboardAddon = new ClipboardAddon();
const ligaturesAddon = new LigaturesAddon();

const element = document.getElementById("app") as HTMLElement;

terminal.loadAddon(fitAddon);
terminal.loadAddon(clipboardAddon);

terminal.open(element);

terminal.loadAddon(new WebglAddon());
terminal.loadAddon(ligaturesAddon);

fitAddon.fit();

onresize = (_) => {
  fitAddon.fit();
}

terminal.write("\x1b[1;32mWelcome to Inventory Tracker.\x1b[22m\r\nPlease wait while the client connects to the server\x1b[0m\r\n");
