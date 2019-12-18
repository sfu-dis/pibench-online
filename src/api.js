const API_ENDPOINT = "http://localhost:8000"

export async function fetchInstanceInfo() {
    const response = await fetch(`${API_ENDPOINT}/info`);
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

