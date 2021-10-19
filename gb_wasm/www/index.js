import { GameBoy } from "gb_wasm";
import { memory } from "gb_wasm/gb_wasm_bg";

const CELL_SIZE = 3;
const GRID_COLOR = "#CCCCCC";
const WIDTH = 160;
const HEIGHT = 144;

const canvas = document.getElementById("screen-canvas");
canvas.height = CELL_SIZE * HEIGHT + (2 * CELL_SIZE);
canvas.width = CELL_SIZE * WIDTH + (2 * CELL_SIZE);
const ctx = canvas.getContext("2d");
ctx.scale(CELL_SIZE, CELL_SIZE);
ctx.imageSmoothingEnabled = false;
ctx.mozImageSmoothingEnabled = false;
ctx.webkitImageSmoothingEnabled = false;
ctx.msImageSmoothingEnabled = false;

const auxCanvas = document.createElement('canvas');
auxCanvas.width = WIDTH;
auxCanvas.height = HEIGHT;
const auxCtx = auxCanvas.getContext('2d');

const debugDiv = document.getElementById('debug-div');
debugDiv.hidden = true;
const memoryDiv = document.getElementById('memory');

var debug = false;
var paused = false;
const debugBtn = document.getElementById('debug-btn');
const pauseBtn = document.getElementById('pause-btn');
const nextFrameBtn = document.getElementById('next-frame-btn');
const nextInstrBtn = document.getElementById('next-instr-btn');

debugBtn.onclick = function () {
    debug = !debug;
    debugDiv.hidden = !debug;
};

pauseBtn.onclick = function () {
    paused = !paused;
};

fetch("/super_mario.gb")
    .then((response) => response.arrayBuffer())
    .then((data) => {
        var cart = new Uint8Array(data);
        const gameBoy = GameBoy.new(cart);

        nextFrameBtn.onclick = function () {
            paused = true;
            gameBoy.tick();
            drawBorder();
            drawCells();
        };

        nextInstrBtn.onclick = function () {
            paused = true;
            gameBoy.instr_tick();
            drawBorder();
            drawCells();
        };        

        const renderLoop = () => {
            if (!paused) {
                gameBoy.tick();
                drawBorder();
                drawCells();
            }

            if (debug) {
                const mem = gameBoy.get_memory(48);
                memoryDiv.innerHTML = mem;
                
                const cpuInfo = gameBoy.cpu_debug()
                drawCpu(cpuInfo);
            }

            requestAnimationFrame(renderLoop);
        };

        requestAnimationFrame(renderLoop);

        window.onkeyup = function (e) {
            if (e.keyCode == 38) {
                gameBoy.set_up(false);
            } else if (e.keyCode == 39) {
                gameBoy.set_right(false);
            } else if (e.keyCode == 40) {
                gameBoy.set_down(false);
            } else if (e.keyCode == 37) {
                gameBoy.set_left(false);
            } else if (e.keyCode == 88) {
                gameBoy.set_a(false);
            } else if (e.keyCode == 90) {
                gameBoy.set_b(false);
            } else if (e.keyCode == 83) {
                gameBoy.set_start(false);
            } else if (e.keyCode == 65) {
                gameBoy.set_select(false);
            }
        };
        window.onkeydown = function (e) {
            if (e.keyCode == 38) {
                gameBoy.set_up(true);
            } else if (e.keyCode == 39) {
                gameBoy.set_right(true);
            } else if (e.keyCode == 40) {
                gameBoy.set_down(true);
            } else if (e.keyCode == 37) {
                gameBoy.set_left(true);
            } else if (e.keyCode == 88) {
                gameBoy.set_a(true);
            } else if (e.keyCode == 90) {
                gameBoy.set_b(true);
            } else if (e.keyCode == 83) {
                gameBoy.set_start(true);
            } else if (e.keyCode == 65) {
                gameBoy.set_select(true);
            }
        };

        const drawBorder = () => {
            ctx.beginPath();
            ctx.strokeStyle = GRID_COLOR;

            ctx.moveTo(0, 0);
            ctx.lineTo(WIDTH + 2, 0);

            ctx.moveTo(WIDTH + 2, 0);
            ctx.lineTo(WIDTH + 2, HEIGHT + 2);

            ctx.moveTo(WIDTH + 2, HEIGHT + 2);
            ctx.lineTo(0.0, HEIGHT + 2);

            ctx.moveTo(0.0, HEIGHT + 2);
            ctx.lineTo(0.0, 0.0);

            ctx.stroke();
        };

        const drawCells = () => {
            const cellsPtr = gameBoy.screen();
            const cells = new Uint8ClampedArray(memory.buffer, cellsPtr, WIDTH * HEIGHT * 4);

            var imageData = ctx.createImageData(WIDTH, HEIGHT);
            const data = imageData.data;
            for (var i = 0; i < data.length; i += 1) {
                data[i] = cells[i];
            }

            auxCtx.putImageData(imageData, 0, 0);

            ctx.drawImage(auxCanvas, 1, 1);
        };

        const drawCpu = (cpuInfo) => {
            document.getElementById('cpu-a').innerHTML = "<b>A:</b> " + cpuInfo.a();
            document.getElementById('cpu-b').innerHTML = "<b>B:</b> " + cpuInfo.b();
            document.getElementById('cpu-c').innerHTML = "<b>C:</b> " + cpuInfo.c();
            document.getElementById('cpu-d').innerHTML = "<b>D:</b> " + cpuInfo.d();
            document.getElementById('cpu-e').innerHTML = "<b>E:</b> " + cpuInfo.e();
            document.getElementById('cpu-hl').innerHTML = "<b>HL:</b> " + cpuInfo.hl();
            document.getElementById('cpu-pc').innerHTML = "<b>PC:</b> " + cpuInfo.pc();
            document.getElementById('cpu-sp').innerHTML = "<b>SP:</b> " + cpuInfo.sp();

            document.getElementById('cpu-fc').innerHTML = "<b>c:</b> " + (cpuInfo.flag_c() ? "1" : "0");
            document.getElementById('cpu-fhc').innerHTML = "<b>hc:</b> " + (cpuInfo.flag_hc() ? "1" : "0");
            document.getElementById('cpu-fs').innerHTML = "<b>sub:</b> " + (cpuInfo.flag_sub() ? "1" : "0");
            document.getElementById('cpu-fz').innerHTML = "<b>z:</b> " + (cpuInfo.flag_z() ? "1" : "0");
        }
    });
