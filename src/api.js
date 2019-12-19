const API_ENDPOINT = "http://localhost:8000"

export async function fetchInstanceInfo(backendUrl) {
    const response = await fetch(`${backendUrl}/info`);
    const data = response.json();
    return data;
}

export async function postBenchmark(data) {
    const response = await fetch(`${API_ENDPOINT}/benchmark`, {
        method: 'post',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });
    return response.json();
}

export async function fetchServerStatus() {
    const response = await fetch(`${API_ENDPOINT}/status`);
    const data = response.json();
    return data;
}

