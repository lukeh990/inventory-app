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

let command = "";

prompt(terminal);

terminal.onData(e => {
  switch (e) {
    case '\u0003': // Ctrl+C
      terminal.write('^C');
      prompt(terminal);
      break;
    case '\r': // Enter
      //runCommand(terminal, command);
      command = '';
      prompt(terminal);
      break;
    case '\u007F': // Backspace (DEL)
      // Do not delete the prompt
      if (command != "") {
        terminal.write('\b \b');
        if (command.length > 0) {
          command = command.slice(0, command.length - 1);
        }
      }
      break;
    default: // Print all other characters for demo
      if (e >= String.fromCharCode(0x20) && e <= String.fromCharCode(0x7E) || e >= '\u00a0') {
        command += e;
        terminal.write(e);
      }
  }
});

function prompt(term: Terminal) {
  command = '';
  term.write("\r\nHello from \x1B[1;3;31mxterm.js\x1B[0m $ ");
}

