// I do not know js, this is largely ai generated

export async function fetchStringFromServer(url) {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Failed to fetch string from server. Status code: ${response.status}`);
        }
        const text = await response.text();
        return text;
    } catch (error) {
        throw new Error(`An error occurred: ${error.message}`);
    }
}

export async function fetchBytesFromServer(url) {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Failed to fetch bytes from server. Status code: ${response.status}`);
        }
        const arrayBuffer = await response.arrayBuffer();
        return new Uint8Array(arrayBuffer);
    } catch (error) {
        throw new Error(`An error occurred: ${error.message}`);
    }
}

const textCtx = document.querySelector("#text").getContext("2d");

export function makeTextCanvas(text, width, height) {
    textCtx.canvas.width  = width;
    textCtx.canvas.height = height;
    textCtx.font = "50px monospace";
    textCtx.textAlign = "center";
    textCtx.textBaseline = "middle";
    textCtx.fillStyle = "black";
    textCtx.clearRect(0, 0, textCtx.canvas.width, textCtx.canvas.height);
    textCtx.fillText(text, width / 2, height / 2);
    return textCtx;
}

export function changeTextFillStyle(style) {
    textCtx.fillStyle = style;
}
