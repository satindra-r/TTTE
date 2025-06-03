
export function hi(){
	return "ello"
}

import init, {greet} from "./pkg/LearningWASM.js";

init().then(() => {
	greet("WebAssembly");

});
let canvas = document.getElementById("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
let ctx = canvas.getContext("2d");

document.addEventListener("keydown", function (event) {
})