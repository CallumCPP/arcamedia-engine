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