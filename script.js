let canvas = document.getElementById("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
let ctx = canvas.getContext("2d");
ctx.imageSmoothingEnabled = false;

const config = {
	iceServers: [{ urls: "stun:stun.l.google.com:19302" }]
};
let peer;
let SDP = ""
let remoteSDP = ""
let ICE = []
let remoteICE = []
let dataChannel;

function setupChannel() {
	dataChannel.binaryType = 'arraybuffer';

	dataChannel.onopen = () => {
		console.log("Data channel open");
	};

	dataChannel.onmessage = (event) => {
		console.log("Peer: " + event.data);
	};
}

async function getConnectionRequest() {
	peer = new RTCPeerConnection(config);
	ICE = [];

	peer.onicecandidate = (event) => {
		if (event.candidate) {
			ICE.push(event.candidate);
		}
	};

	dataChannel = peer.createDataChannel("moves");
	setupChannel();

	const offer = await peer.createOffer();
	await peer.setLocalDescription(offer);

	await new Promise(resolve => {
		peer.onicegatheringstatechange = () => {
			if (peer.iceGatheringState === "complete") {
				resolve();
			}
		};
	});

	SDP = JSON.stringify(peer.localDescription);
	return JSON.stringify({ SDP, ICE });
}

async function getConnectionResponse(clipboard) {
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

	const parsed = JSON.parse(clipboard);
	remoteICE = parsed.ICE;
	await peer.setRemoteDescription(JSON.parse(parsed.SDP));

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

	SDP = JSON.stringify(peer.localDescription);
	return JSON.stringify({ SDP, ICE });
}

function send(data) {
	if (dataChannel && dataChannel.readyState === "open") {
		dataChannel.send(data);
	}
}

import init, { reset, handleKeyDown, handleMouseClick } from "./pkg/LearningWASM.js";

init().then(() => {
	reset();
	document.addEventListener("keydown", async function (event) {
		handleKeyDown(event.key);

		if (event.key === "Tab") {
			let clipboard = await navigator.clipboard.readText();
			let clipboardJSON = ""
			let clipboardSDP = "";
			let clipboardICE = [];

			try {
				clipboardJSON = JSON.parse(clipboard);
				clipboardSDP = JSON.parse(clipboardJSON["SDP"]);
				clipboardICE = clipboardJSON["ICE"];
			} catch (error) {
				clipboardJSON = "";
			}

			if (!clipboardJSON) {
				await navigator.clipboard.writeText(await getConnectionRequest());
			} else if (clipboardSDP["type"] === "offer" && JSON.stringify(clipboardSDP) !== SDP) {
				remoteSDP = clipboardSDP;
				remoteICE = clipboardICE;
				await navigator.clipboard.writeText(await getConnectionResponse(clipboard));
			} else if (clipboardSDP["type"] === "answer" && JSON.stringify(clipboardSDP) !== SDP) {
				remoteSDP = clipboardSDP;
				remoteICE = clipboardICE;
				await peer.setRemoteDescription(remoteSDP);
				for (const candidate of remoteICE) {
					await peer.addIceCandidate(candidate);
				}
			}
		} else if (event.key === "v") {
			await navigator.clipboard.writeText(await getConnectionResponse(await navigator.clipboard.readText()));
		}
	});

	document.addEventListener("click", function (event) {
		send(handleMouseClick(event.x, event.y));
	});
});