let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

function setDrawColour(r, g, b) {
    ctx.strokeStyle = "rgb(" + r + "," + g + "," + b + ")";
}

function setFillColour(r, g, b) {
    ctx.fillStyle = "rgb(" + r + "," + g + "," + b + ")";
}

function setLineThickness(t) {
    ctx.lineWidth = t;
}

function getLighter(c) {
    return Math.min(c + 32, 255);
}

function getDarker(c) {
    return Math.max(c - 32, 0)
}

export function print(str) {
    console.log(str);
}

export function getWindowWidth() {
    return window.innerWidth;
}

export function getWindowHeight() {
    return window.innerHeight;
}

export function randRange(x, y) {
    return Math.random() * (y - x) + x;
}

export function drawRect(x, y, w, h, r, g, b, t) {
    setLineThickness(t);
    setDrawColour(r, g, b);
    ctx.strokeRect(x, y, w, h);
}

export function fillRect(x, y, w, h, r, g, b) {
    setFillColour(r, g, b);
    ctx.fillRect(x, y, w, h);
}

export function fill3DRect(x, y, w, h, r, g, b, t, raised) {
    fillRect(x + t, y + t, w - 2 * t, h - 2 * t, r, g, b);
    setLineThickness(t);
    if (raised) {
        setDrawColour(getLighter(r), getLighter(g), getLighter(b));
    } else {
        setDrawColour(getDarker(r), getDarker(g), getDarker(b));
    }

    ctx.beginPath();
    ctx.moveTo(x + t, y + t);
    ctx.lineTo(x + w - t, y + t);
    ctx.stroke();

    ctx.beginPath();
    ctx.moveTo(x + t, y + t);
    ctx.lineTo(x + t, y + h - t);
    ctx.stroke()

    if (raised) {
        setDrawColour(getDarker(r), getDarker(g), getDarker(b));
    } else {
        setDrawColour(getLighter(r), getLighter(g), getLighter(b));
    }

    ctx.beginPath();
    ctx.moveTo(x + w - t, y + h - t);
    ctx.lineTo(x + w - t, y + t);
    ctx.stroke();

    ctx.beginPath();
    ctx.moveTo(x + w - t, y + h - t);
    ctx.lineTo(x + t, y + h - t);
    ctx.stroke()
}

export function drawCross(x, y, s, r, g, b, t) {
    setDrawColour(r, g, b);
    setLineThickness(t);
    ctx.beginPath();
    ctx.moveTo(x - s, y - s);
    ctx.lineTo(x + s, y + s);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(x + s, y - s);
    ctx.lineTo(x - s, y + s);
    ctx.stroke();
}

export function drawCircle(x, y, s, r, g, b, t) {
    setDrawColour(r, g, b);
    setLineThickness(t)
    ctx.beginPath();
    ctx.arc(x, y, s, 0, 2 * Math.PI, false);
    ctx.stroke();
}