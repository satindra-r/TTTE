let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");
let textArea = document.getElementById("text");
let status = document.getElementById("status")

let peer;
let player = -1;
let dataChannel;
let ICE = [];

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

export function rand() {
	return Math.random();
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

const config = {
	iceServers: [{urls: "stun:stun.l.google.com:19302"}, {urls: 'stun:freestun.net:3478'}, {
		urls: 'turn:freestun.net:3478',
		username: 'free',
		credential: 'free'
	}]
};


function setupChannel() {
	dataChannel.binaryType = 'arraybuffer';

	dataChannel.onopen = () => {
		sendData("Join:" + player);
		textArea.value = "";
	};

	dataChannel.onmessage = (event) => {
		textArea.value = event.data;
		textArea.dispatchEvent(new Event("data"))
	};
}

export function getConnectionRequest() {

	peer = new RTCPeerConnection(config);
	ICE = [];

	peer.onicecandidate = (event) => {
		if (event.candidate) {
			ICE.push(event.candidate);
		}
	};

	dataChannel = peer.createDataChannel("moves");
	setupChannel();

	(async () => {
		const offer = await peer.createOffer();
		await peer.setLocalDescription(offer);

		await new Promise(resolve => {
			peer.onicegatheringstatechange = () => {
				if (peer.iceGatheringState === "complete") {
					resolve();
				}
			};
		});

		let SDP = JSON.stringify(peer.localDescription);
		textArea.value = JSON.stringify({SDP, ICE});
		setStatus("Request Generated, Send this to Opponent and ask them to Click Create Response")
	})();
}

export function getConnectionResponse() {
	let remoteJSON = ""
	let remoteSDP = "";
	let remoteICE = [];

	try {
		remoteJSON = JSON.parse(textArea.value);
		remoteSDP = JSON.parse(remoteJSON["SDP"]);
		remoteICE = remoteJSON["ICE"];
	} catch (error) {
		return;
	}

	player = 2
	peer = new RTCPeerConnection(config);
	ICE = [];

	peer.onicecandidate = (event) => {
		if (event.candidate) {
			ICE.push(event.candidate);
		}
	};

	peer.ondatachannel = (event) => {
		dataChannel = event.channel;
		setupChannel();
	};

	(async () => {
		await peer.setRemoteDescription(remoteSDP);

		const answer = await peer.createAnswer();
		await peer.setLocalDescription(answer);

		await new Promise(resolve => {
			peer.onicegatheringstatechange = () => {
				if (peer.iceGatheringState === "complete") {
					resolve();
				}
			};
		});

		for (const candidate of remoteICE) {
			await peer.addIceCandidate(candidate);
		}

		let SDP = JSON.stringify(peer.localDescription);
		textArea.value = JSON.stringify({SDP, ICE});
		setStatus("Response Generated, Send this to Opponent and ask them to Click Begin Connection")
	})();
}

export function setRemoteDesc() {
	let remoteJSON = ""
	let remoteSDP = "";
	let remoteICE = [];

	try {
		remoteJSON = JSON.parse(textArea.value);
		remoteSDP = JSON.parse(remoteJSON["SDP"]);
		remoteICE = remoteJSON["ICE"];
	} catch (error) {
		return;
	}

	player = 1;
	(async () => {
		await peer.setRemoteDescription(remoteSDP);
		for (const candidate of remoteICE) {
			await peer.addIceCandidate(candidate);
		}
	})();
}

export function sendData(data) {
	if (dataChannel && dataChannel.readyState === "open") {
		dataChannel.send(data);
	}
}

export function callAI(){
	setTimeout(()=>{
		textArea.dispatchEvent(new Event("AI"));
	},1000);

}

export function setStatus(data) {
	status.textContent = data;
}