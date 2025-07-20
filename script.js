import init, {
	setHook,
	render,
	handleKeyDown,
	handleMouseClick,
	handleDataIn,
	createRequest,
	createResponse,
	beginConnection,
	enableAI,
	handleAIMove
} from "./pkg/LearningWASM.js";

let canvas = document.getElementById("canvas");
canvas.width = window.innerWidth / 2;
canvas.height = window.innerHeight;
let ctx = canvas.getContext("2d");
let textArea = document.getElementById("text")
ctx.imageSmoothingEnabled = false;

init().then(() => {
	setHook();
	render();
	document.addEventListener("keydown", async function (event) {
		handleKeyDown(event.key);
	});

	document.addEventListener("click", function (event) {
		handleMouseClick(event.x, event.y);
	});
	document.getElementById("Create Request").addEventListener("click", function () {
		createRequest();
	});
	document.getElementById("Create Response").addEventListener("click", function () {
		createResponse();
	});
	document.getElementById("Begin Connection").addEventListener("click", function () {
		beginConnection();
	});
	document.getElementById("Play Against AI").addEventListener("click", function () {
		enableAI();
	});
	textArea.addEventListener("data", function () {
		handleDataIn(textArea.value);
		textArea.value = "";
	})
	textArea.addEventListener("AI",function (){
		handleAIMove();
	})
});