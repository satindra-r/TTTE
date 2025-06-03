let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

export function getWindowWidth() {
	return window.innerWidth;
}

export function getWindowHeight() {
	return window.innerHeight;
}

export function randRange(x, y) {
	return Math.random() * (y - x) + x;
}

export function setDrawColour(r, g, b) {
	ctx.strokeStyle = "rgb(" + r + "," + g + "," + b + ")";
}

export function setFillColour(r, g, b) {
	ctx.fillStyle = "rgb(" + r + "," + g + "," + b + ")";
}

export function drawRect(x, y, w, h) {
	ctx.strokeRect(x, y, w, h);
}

export function fillRect(x, y, w, h) {
	ctx.fillRect(x, y, w, h);
}