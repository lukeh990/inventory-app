import "./style.css";
import "@xterm/xterm/css/xterm.css";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebglAddon } from "@xterm/addon-webgl";
import { ClipboardAddon } from "@xterm/addon-clipboard";

const terminal = new Terminal({
  theme: {
    background: "#393939"
  }
});
const fitAddon = new FitAddon();
const clipboardAddon = new ClipboardAddon();

const element = document.getElementById("app") as HTMLElement;

terminal.loadAddon(fitAddon);
terminal.loadAddon(clipboardAddon);

terminal.open(element);

terminal.loadAddon(new WebglAddon());
fitAddon.fit();

onresize = (_) => {
  fitAddon.fit();
}

terminal.write("Welcome to Inventory Tracker.\r\nPlease wait while the client connects to the server\r\n");
