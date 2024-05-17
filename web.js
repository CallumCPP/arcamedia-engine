export function getStringFromServer(url) {
    const request = new XMLHttpRequest();
    request.open('GET', url, false); // synchronous request
    request.send(null);

    if (request.status === 200) {
        return request.responseText;
    } else {
        throw new Error(`Failed to fetch string from server. Status code: ${request.status}`);
    }
}